//! CD-XA ADPCM audio sectors, decoded from the public format description
//! (psx-spx): 18 portions of 128 bytes per sector, 4- or 8-bit samples,
//! mono or stereo, 37800 or 18900 Hz, resampled to the SPU's 44100 Hz.
//! The CD mixer volumes (CdMix) are applied here.

use std::sync::Mutex;

const FILTERS: [(i32, i32); 4] = [(0, 0), (60, 0), (115, -52), (98, -55)];

/// CdMix matrix: CD left to SPU left, CD left to SPU right, CD right to SPU
/// right, CD right to SPU left (0x80 = 100%).
pub static MIX: Mutex<[u8; 4]> = Mutex::new([0x80, 0x00, 0x80, 0x00]);

#[derive(Default)]
pub struct XaDecoder {
    /// Filter history per channel: (old, older).
    hist: [(i32, i32); 2],
    pos: f64,
    prev: (f32, f32),
}

impl XaDecoder {
    /// Decodes one raw 2352-byte audio sector into 44.1 kHz stereo samples.
    pub fn decode_sector(&mut self, raw: &[u8]) -> Vec<(i16, i16)> {
        let coding = raw[19];
        let stereo = coding & 3 == 1;
        let rate = if coding & 4 != 0 { 18_900.0 } else { 37_800.0 };
        let eight_bit = (coding >> 4) & 3 == 1;
        let data = &raw[24..24 + 0x900];

        let mut decoded: Vec<(i16, i16)> = Vec::with_capacity(18 * 224);
        for portion in data.chunks_exact(128) {
            let blocks = if eight_bit { 4 } else { 8 };
            let mut units: Vec<[i16; 28]> = Vec::with_capacity(blocks);
            for b in 0..blocks {
                let ch = if stereo { b & 1 } else { 0 };
                let param = portion[4 + b];
                units.push(self.decode_unit(portion, b, param, eight_bit, ch));
            }
            if stereo {
                for pair in units.chunks_exact(2) {
                    decoded.extend(pair[0].iter().zip(pair[1].iter()).map(|(l, r)| (*l, *r)));
                }
            } else {
                for unit in &units {
                    decoded.extend(unit.iter().map(|s| (*s, *s)));
                }
            }
        }
        self.resample(&decoded, rate)
    }

    fn decode_unit(&mut self, portion: &[u8], block: usize, param: u8, eight_bit: bool, ch: usize) -> [i16; 28] {
        let range = i32::from(param & 0x0F);
        let (f0, f1) = FILTERS[usize::from((param >> 4) & 3)];
        let (mut old, mut older) = self.hist[ch];
        let mut out = [0i16; 28];
        for (i, slot) in out.iter_mut().enumerate() {
            let word = u32::from_le_bytes([
                portion[16 + i * 4],
                portion[17 + i * 4],
                portion[18 + i * 4],
                portion[19 + i * 4],
            ]);
            let t = if eight_bit {
                let byte = (word >> (block * 8)) as u8 as i8;
                (i32::from(byte) << 8) >> range.min(8)
            } else {
                let range = if range > 12 { 9 } else { range };
                let nibble = ((word >> (block * 4)) & 0xF) as u8;
                (i32::from((nibble << 4) as i8) << 8) >> range
            };
            let s = (t + (old * f0 + older * f1 + 32) / 64).clamp(-0x8000, 0x7FFF);
            older = old;
            old = s;
            *slot = s as i16;
        }
        self.hist[ch] = (old, older);
        out
    }

    /// Linear resampling to 44100 Hz, carried across sectors, with the CD
    /// mixer applied.
    fn resample(&mut self, input: &[(i16, i16)], rate: f64) -> Vec<(i16, i16)> {
        let mix = *MIX.lock().unwrap();
        let m = |v: u8| f32::from(v) / 128.0;
        let step = rate / 44_100.0;
        let mut out = Vec::with_capacity((input.len() as f64 / step) as usize + 2);
        for &(l, r) in input {
            let cur = (f32::from(l), f32::from(r));
            while self.pos < 1.0 {
                let t = self.pos as f32;
                let sl = self.prev.0 + (cur.0 - self.prev.0) * t;
                let sr = self.prev.1 + (cur.1 - self.prev.1) * t;
                let ol = sl * m(mix[0]) + sr * m(mix[3]);
                let or = sr * m(mix[2]) + sl * m(mix[1]);
                out.push((ol.clamp(-32768.0, 32767.0) as i16, or.clamp(-32768.0, 32767.0) as i16));
                self.pos += step;
            }
            self.pos -= 1.0;
            self.prev = cur;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A silent 37.8 kHz stereo sector yields silence at about 7/6 the
    /// sample count.
    #[test]
    fn silent_sector_resamples() {
        let mut raw = vec![0u8; 2352];
        raw[18] = 0x64; // submode: audio, real time, form 2
        raw[19] = 0x01; // stereo, 37800 Hz, 4-bit
        let mut d = XaDecoder::default();
        let out = d.decode_sector(&raw);
        let stereo_in = 18 * 112;
        let expected = stereo_in * 7 / 6;
        assert!((out.len() as i64 - expected as i64).abs() <= 2, "{} samples", out.len());
        assert!(out.iter().all(|&(l, r)| l == 0 && r == 0));
    }
}

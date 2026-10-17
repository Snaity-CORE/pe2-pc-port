//! The Sound Processing Unit, written from the public hardware documentation
//! (psx-spx): 512 KiB of sound RAM, 24 ADPCM voices with ADSR envelopes,
//! CD audio input, and the reverb unit. `tick` produces one stereo sample at
//! the SPU's 44100 Hz rate.
//!
//! Not modelled (unused by this game): noise, pitch modulation, volume
//! sweeps, capture buffers. Interpolation is linear instead of the
//! hardware's 4-point Gaussian filter.

use std::collections::VecDeque;

pub const RAM_SIZE: usize = 512 * 1024;
pub const VOICES: usize = 24;
/// The SPU's output rate.
pub const SAMPLE_RATE: u32 = 44_100;

/// SPU-ADPCM prediction filters (positive and negative coefficients, /64).
const FILTERS: [(i32, i32); 5] = [(0, 0), (60, 0), (115, -52), (98, -55), (122, -60)];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Phase {
    #[default]
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Default)]
pub struct Voice {
    // Registers.
    pub vol_l: u16,
    pub vol_r: u16,
    pub pitch: u16,
    pub start: u16,
    pub adsr1: u16,
    pub adsr2: u16,
    pub repeat: u16,
    // State.
    pub env: i32,
    pub phase: Phase,
    env_counter: u32,
    addr: u32,
    counter: u32,
    samples: [i16; 28],
    prev: i16,
    old: i32,
    older: i32,
    block_flags: u8,
}

/// Effective volume of a fixed-mode volume register (bits 0-14 hold
/// volume/2). Sweep mode is not modelled and plays at its current value.
fn fixed_volume(reg: u16) -> i32 {
    if reg & 0x8000 != 0 { 0x7FFF } else { ((reg << 1) as i16) as i32 }
}

impl Voice {
    fn key_on(&mut self, ram: &[u8]) {
        self.addr = u32::from(self.start) * 8;
        self.repeat = self.start;
        self.counter = 0;
        self.old = 0;
        self.older = 0;
        self.prev = 0;
        self.env = 0;
        self.env_counter = 0;
        self.phase = Phase::Attack;
        self.decode_block(ram);
    }

    fn key_off(&mut self) {
        if self.phase != Phase::Off {
            self.phase = Phase::Release;
            self.env_counter = 0;
        }
    }

    /// Decodes the 16-byte block at `addr`. Returns whether it holds the IRQ
    /// address (checked by the caller).
    fn decode_block(&mut self, ram: &[u8]) {
        let a = (self.addr as usize) & (RAM_SIZE - 16);
        let header = ram[a];
        let shift = i32::from(header & 0x0F).min(12);
        let (f0, f1) = FILTERS[usize::from((header >> 4) & 7).min(4)];
        self.block_flags = ram[a + 1];
        if self.block_flags & 4 != 0 {
            self.repeat = (a / 8) as u16;
        }
        for i in 0..28 {
            let byte = ram[a + 2 + i / 2];
            let nibble = if i % 2 == 0 { byte & 0x0F } else { byte >> 4 };
            let t = i32::from((nibble << 4) as i8) >> 4; // signed 4-bit
            let mut s = (t << 12) >> shift;
            s += (self.old * f0 + self.older * f1 + 32) / 64;
            let s = s.clamp(-0x8000, 0x7FFF);
            self.older = self.old;
            self.old = s;
            self.samples[i] = s as i16;
        }
    }

    /// Steps the ADSR envelope by one sample.
    fn step_envelope(&mut self) {
        let (exponential, decreasing, shift, step, target) = match self.phase {
            Phase::Off => return,
            Phase::Attack => (self.adsr1 & 0x8000 != 0, false, (self.adsr1 >> 10) & 0x1F, (self.adsr1 >> 8) & 3, None),
            Phase::Decay => {
                let sustain = (i32::from(self.adsr1 & 0xF) + 1) * 0x800;
                (true, true, (self.adsr1 >> 4) & 0xF, 0, Some(sustain))
            }
            Phase::Sustain => (
                self.adsr2 & 0x8000 != 0,
                self.adsr2 & 0x4000 != 0,
                (self.adsr2 >> 8) & 0x1F,
                (self.adsr2 >> 6) & 3,
                None,
            ),
            Phase::Release => (self.adsr2 & 0x20 != 0, true, self.adsr2 & 0x1F, 0, None),
        };
        let shift = i32::from(shift);
        let mut delta = 7 - i32::from(step);
        if decreasing {
            delta = !delta; // -8 + step
        }
        delta <<= (11 - shift).max(0);
        let mut increment = 0x8000_i32 >> (shift - 11).max(0);
        if exponential && !decreasing && self.env > 0x6000 {
            if shift < 10 {
                delta >>= 2;
            } else if shift >= 11 {
                increment >>= 2;
            } else {
                delta >>= 1;
                increment >>= 1;
            }
        }
        if exponential && decreasing {
            delta = (delta * self.env) >> 15;
        }
        self.env_counter += increment.max(1) as u32;
        if self.env_counter & 0x8000 != 0 {
            self.env_counter = 0;
            self.env = (self.env + delta).clamp(0, 0x7FFF);
        }
        match self.phase {
            Phase::Attack if self.env >= 0x7FFF => {
                self.phase = Phase::Decay;
                self.env_counter = 0;
            }
            Phase::Decay if self.env <= target.unwrap_or(0) => {
                self.phase = Phase::Sustain;
                self.env_counter = 0;
            }
            Phase::Release if self.env == 0 => self.phase = Phase::Off,
            _ => {}
        }
    }
}

/// Reverb registers, in hardware order from 0x1F801DC0.
#[derive(Clone, Copy, Default)]
pub struct ReverbRegs {
    pub d_apf1: u16,
    pub d_apf2: u16,
    pub v_iir: i16,
    pub v_comb: [i16; 4],
    pub v_wall: i16,
    pub v_apf1: i16,
    pub v_apf2: i16,
    pub m_same: [u16; 2],
    pub m_comb1: [u16; 2],
    pub m_comb2: [u16; 2],
    pub d_same: [u16; 2],
    pub m_diff: [u16; 2],
    pub m_comb3: [u16; 2],
    pub m_comb4: [u16; 2],
    pub d_diff: [u16; 2],
    pub m_apf1: [u16; 2],
    pub m_apf2: [u16; 2],
    pub v_in: [i16; 2],
}

impl ReverbRegs {
    /// From the 32 halfwords at 0x1F801DC0 (the layout used by presets).
    pub fn from_words(w: &[u16; 32]) -> Self {
        let s = |i: usize| w[i] as i16;
        ReverbRegs {
            d_apf1: w[0],
            d_apf2: w[1],
            v_iir: s(2),
            v_comb: [s(3), s(4), s(5), s(6)],
            v_wall: s(7),
            v_apf1: s(8),
            v_apf2: s(9),
            m_same: [w[10], w[11]],
            m_comb1: [w[12], w[13]],
            m_comb2: [w[14], w[15]],
            d_same: [w[16], w[17]],
            m_diff: [w[18], w[19]],
            m_comb3: [w[20], w[21]],
            m_comb4: [w[22], w[23]],
            d_diff: [w[24], w[25]],
            m_apf1: [w[26], w[27]],
            m_apf2: [w[28], w[29]],
            v_in: [s(30), s(31)],
        }
    }
}

pub struct Spu {
    pub ram: Box<[u8]>,
    pub voices: [Voice; VOICES],
    pub main_vol: [u16; 2],
    pub reverb_vol: [i16; 2],
    pub cd_vol: [i16; 2],
    /// ATTR (SPUCNT) register.
    pub control: u16,
    pub reverb_on: u32,
    pub endx: u32,
    pub irq_addr: u16,
    pub irq_flag: bool,
    pub reverb: ReverbRegs,
    /// Reverb work area start (ESA), in 8-byte units.
    pub reverb_base: u16,
    reverb_addr: u32,
    reverb_in: [i32; 2],
    reverb_out: [i32; 2],
    odd: bool,
    /// CD audio (XA) at 44.1 kHz, already scaled by the CD mixer.
    pub cd_input: VecDeque<(i16, i16)>,
    /// Mods: voices left out of the mix (bit per voice), a recording mixed
    /// in instead (as the CD input is, without reverb), and voices whose
    /// mix is also kept in `capture` (for dumping).
    pub muted_voices: u32,
    pub music_input: VecDeque<(i16, i16)>,
    pub captured_voices: u32,
    pub capture: Vec<(i16, i16)>,
}

pub const CONTROL_CD_ENABLE: u16 = 1 << 0;
pub const CONTROL_CD_REVERB: u16 = 1 << 2;
pub const CONTROL_IRQ_ENABLE: u16 = 1 << 6;
pub const CONTROL_REVERB: u16 = 1 << 7;
pub const CONTROL_UNMUTE: u16 = 1 << 14;
pub const CONTROL_ENABLE: u16 = 1 << 15;

impl Default for Spu {
    fn default() -> Self {
        Spu {
            ram: vec![0u8; RAM_SIZE].into_boxed_slice(),
            voices: std::array::from_fn(|_| Voice::default()),
            main_vol: [0; 2],
            reverb_vol: [0; 2],
            cd_vol: [0; 2],
            control: 0,
            reverb_on: 0,
            endx: 0,
            irq_addr: 0,
            irq_flag: false,
            reverb: ReverbRegs::default(),
            reverb_base: 0,
            reverb_addr: 0,
            reverb_in: [0; 2],
            reverb_out: [0; 2],
            odd: false,
            cd_input: VecDeque::new(),
            muted_voices: 0,
            music_input: VecDeque::new(),
            captured_voices: 0,
            capture: Vec::new(),
        }
    }
}

fn sat(v: i32) -> i32 {
    v.clamp(-0x8000, 0x7FFF)
}

impl Spu {
    pub fn key_on(&mut self, voices: u32) {
        for i in 0..VOICES {
            if voices & (1 << i) != 0 {
                let ram = &self.ram;
                self.voices[i].key_on(ram);
                self.endx &= !(1 << i);
                self.check_irq(self.voices[i].addr);
            }
        }
    }

    pub fn key_off(&mut self, voices: u32) {
        for i in 0..VOICES {
            if voices & (1 << i) != 0 {
                self.voices[i].key_off();
            }
        }
    }

    pub fn set_reverb_base(&mut self, base: u16) {
        self.reverb_base = base;
        self.reverb_addr = u32::from(base) * 8;
    }

    fn check_irq(&mut self, block_addr: u32) {
        if self.control & CONTROL_IRQ_ENABLE != 0 {
            let irq = u32::from(self.irq_addr) * 8;
            if irq >= block_addr && irq < block_addr + 16 {
                self.irq_flag = true;
            }
        }
    }

    /// A write into sound RAM by a transfer, which can also raise the IRQ.
    pub fn transfer_irq_check(&mut self, start: u32, len: u32) {
        if self.control & CONTROL_IRQ_ENABLE != 0 {
            let irq = u32::from(self.irq_addr) * 8;
            if irq >= start && irq < start + len {
                self.irq_flag = true;
            }
        }
    }

    fn voice_sample(&mut self, i: usize) -> i32 {
        let v = &mut self.voices[i];
        if v.phase == Phase::Off {
            return 0;
        }
        let index = (v.counter >> 12) as usize;
        let frac = ((v.counter >> 4) & 0xFF) as i32;
        let cur = i32::from(v.samples[index.min(27)]);
        let before = if index == 0 { i32::from(v.prev) } else { i32::from(v.samples[index - 1]) };
        let sample = before + (((cur - before) * frac) >> 8);
        let out = (sample * v.env) >> 15;
        v.step_envelope();

        v.counter += u32::from(v.pitch.min(0x4000));
        let mut block_end = None;
        if (v.counter >> 12) >= 28 {
            v.counter -= 28 << 12;
            v.prev = v.samples[27];
            let flags = v.block_flags;
            if flags & 1 != 0 {
                v.addr = u32::from(v.repeat) * 8;
                if flags & 2 == 0 {
                    v.phase = Phase::Off;
                    v.env = 0;
                }
                block_end = Some(true);
            } else {
                v.addr = (v.addr + 16) & (RAM_SIZE as u32 - 1);
                block_end = Some(false);
            }
        }
        if let Some(ended) = block_end {
            if ended {
                self.endx |= 1 << i;
            }
            let addr = self.voices[i].addr;
            let ram = &self.ram;
            self.voices[i].decode_block(ram);
            self.check_irq(addr);
        }
        out
    }

    fn reverb_read(&self, offset: u32) -> i32 {
        let addr = self.reverb_wrap(self.reverb_addr + offset * 8);
        i32::from(i16::from_le_bytes([self.ram[addr], self.ram[addr + 1]]))
    }

    fn reverb_read_back(&self, offset: u32, back: u32) -> i32 {
        let addr = self.reverb_wrap(self.reverb_addr + offset * 8 + 0x80000 - back * 2);
        i32::from(i16::from_le_bytes([self.ram[addr], self.ram[addr + 1]]))
    }

    fn reverb_write(&mut self, offset: u32, value: i32) {
        if self.control & CONTROL_REVERB == 0 {
            return;
        }
        let addr = self.reverb_wrap(self.reverb_addr + offset * 8);
        let b = (sat(value) as i16).to_le_bytes();
        self.ram[addr] = b[0];
        self.ram[addr + 1] = b[1];
    }

    /// Keeps a reverb address inside the work area [ESA*8, 0x80000).
    fn reverb_wrap(&self, addr: u32) -> usize {
        let base = u32::from(self.reverb_base) * 8;
        let size = 0x80000 - base;
        if size == 0 {
            return 0;
        }
        (base + (addr.wrapping_sub(base) % size)) as usize & (RAM_SIZE - 2)
    }

    /// One reverb update (22050 Hz) of one side: 0 left, 1 right.
    fn reverb_side(&mut self, side: usize) {
        let r = self.reverb;
        let other = 1 - side;
        let m = |v: i32, f: i16| (v * i32::from(f)) >> 15;
        let input = m(self.reverb_in[side], r.v_in[side]);

        let same = u32::from(r.m_same[side]);
        let same_prev = self.reverb_read_back(same, 1);
        let wall_same = self.reverb_read(u32::from(r.d_same[side]));
        let v = m(input + m(wall_same, r.v_wall) - same_prev, r.v_iir) + same_prev;
        self.reverb_write(same, v);

        let diff = u32::from(r.m_diff[side]);
        let diff_prev = self.reverb_read_back(diff, 1);
        let wall_diff = self.reverb_read(u32::from(r.d_diff[other]));
        let v = m(input + m(wall_diff, r.v_wall) - diff_prev, r.v_iir) + diff_prev;
        self.reverb_write(diff, v);

        let mut out = m(self.reverb_read(u32::from(r.m_comb1[side])), r.v_comb[0])
            + m(self.reverb_read(u32::from(r.m_comb2[side])), r.v_comb[1])
            + m(self.reverb_read(u32::from(r.m_comb3[side])), r.v_comb[2])
            + m(self.reverb_read(u32::from(r.m_comb4[side])), r.v_comb[3]);

        let apf1 = u32::from(r.m_apf1[side]);
        let apf1_tap = self.reverb_read(apf1.wrapping_sub(u32::from(r.d_apf1)) & 0xFFFF);
        out -= m(apf1_tap, r.v_apf1);
        self.reverb_write(apf1, out);
        out = m(sat(out), r.v_apf1) + apf1_tap;

        let apf2 = u32::from(r.m_apf2[side]);
        let apf2_tap = self.reverb_read(apf2.wrapping_sub(u32::from(r.d_apf2)) & 0xFFFF);
        out -= m(apf2_tap, r.v_apf2);
        self.reverb_write(apf2, out);
        out = m(sat(out), r.v_apf2) + apf2_tap;

        self.reverb_out[side] = sat(m(sat(out), self.reverb_vol[side]));
    }

    /// Produces one stereo sample.
    pub fn tick(&mut self) -> (i16, i16) {
        let (mut l, mut r) = (0i32, 0i32);
        let (mut rl, mut rr) = (0i32, 0i32);
        let (mut cl, mut cr) = (0i32, 0i32);
        if self.control & CONTROL_ENABLE != 0 {
            for i in 0..VOICES {
                let s = self.voice_sample(i);
                if s == 0 {
                    continue;
                }
                let v = &self.voices[i];
                let vl = (s * fixed_volume(v.vol_l)) >> 15;
                let vr = (s * fixed_volume(v.vol_r)) >> 15;
                if self.captured_voices & (1 << i) != 0 {
                    cl += vl;
                    cr += vr;
                }
                if self.muted_voices & (1 << i) != 0 {
                    continue;
                }
                l += vl;
                r += vr;
                if self.reverb_on & (1 << i) != 0 {
                    rl += vl;
                    rr += vr;
                }
            }
        }
        if let Some((cl, cr)) = self.cd_input.pop_front()
            && self.control & CONTROL_CD_ENABLE != 0
        {
            let cl = (i32::from(cl) * i32::from(self.cd_vol[0])) >> 15;
            let cr = (i32::from(cr) * i32::from(self.cd_vol[1])) >> 15;
            l += cl;
            r += cr;
            if self.control & CONTROL_CD_REVERB != 0 {
                rl += cl;
                rr += cr;
            }
        }

        if let Some((ml, mr)) = self.music_input.pop_front() {
            l += i32::from(ml);
            r += i32::from(mr);
        }
        if self.captured_voices != 0 {
            self.capture.push((sat(cl) as i16, sat(cr) as i16));
        }

        // Reverb runs at half rate, left and right on alternate ticks; its
        // input is the average of the two ticks it spans.
        self.reverb_in[0] = (self.reverb_in[0] + sat(rl)) / if self.odd { 2 } else { 1 };
        self.reverb_in[1] = (self.reverb_in[1] + sat(rr)) / if self.odd { 2 } else { 1 };
        if self.odd {
            self.reverb_side(0);
            self.reverb_side(1);
            self.reverb_addr = (self.reverb_addr + 2).max(u32::from(self.reverb_base) * 8);
            if self.reverb_addr >= 0x80000 {
                self.reverb_addr = u32::from(self.reverb_base) * 8;
            }
            self.reverb_in = [0, 0];
        }
        self.odd = !self.odd;
        l += self.reverb_out[0];
        r += self.reverb_out[1];

        if self.control & CONTROL_UNMUTE == 0 {
            return (0, 0);
        }
        let l = sat((sat(l) * fixed_volume(self.main_vol[0])) >> 15);
        let r = sat((sat(r) * fixed_volume(self.main_vol[1])) >> 15);
        (l as i16, r as i16)
    }

    /// libspu's key status: 1 on, 0 off, 2 off with the envelope still
    /// sounding (release), 3 on with the envelope silent.
    pub fn key_status(&self, i: usize) -> u8 {
        let v = &self.voices[i];
        match v.phase {
            Phase::Off => 0,
            Phase::Release => {
                if v.env > 0 {
                    2
                } else {
                    0
                }
            }
            _ => {
                if v.env > 0 {
                    1
                } else {
                    3
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-block looping square-ish wave plays and its envelope rises.
    #[test]
    fn voice_plays_and_loops() {
        let mut spu =
            Spu { control: CONTROL_ENABLE | CONTROL_UNMUTE, main_vol: [0x3FFF, 0x3FFF], ..Default::default() };
        // Block at 0x1000: shift 0, filter 0, loop start+end+repeat.
        let a = 0x1000;
        spu.ram[a] = 0x00;
        spu.ram[a + 1] = 0x07;
        for i in 2..16 {
            spu.ram[a + i] = 0x77; // +7 nibbles
        }
        let v = &mut spu.voices[0];
        v.start = (a / 8) as u16;
        v.pitch = 0x1000;
        v.vol_l = 0x3FFF;
        v.vol_r = 0x3FFF;
        v.adsr1 = 0x00FF; // fast linear attack, sustain level max
        v.adsr2 = 0x0000;
        spu.key_on(1);
        let mut peak = 0;
        for _ in 0..4410 {
            let (l, _) = spu.tick();
            peak = peak.max(l);
        }
        assert!(peak > 10_000, "peak {peak}");
        assert_ne!(spu.voices[0].phase, Phase::Off, "looping voice stopped");
        assert_ne!(spu.endx & 1, 0, "loop end not flagged");
    }

    /// A block ending without the repeat flag silences the voice.
    #[test]
    fn end_without_repeat_stops() {
        let mut spu = Spu { control: CONTROL_ENABLE | CONTROL_UNMUTE, ..Default::default() };
        let a = 0x2000;
        spu.ram[a + 1] = 0x01;
        let v = &mut spu.voices[3];
        v.start = (a / 8) as u16;
        v.pitch = 0x1000;
        v.adsr1 = 0x00FF;
        spu.key_on(1 << 3);
        for _ in 0..100 {
            spu.tick();
        }
        assert_eq!(spu.voices[3].phase, Phase::Off);
    }
}

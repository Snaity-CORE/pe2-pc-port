//! The MDEC (macroblock decoder), written from the public hardware
//! documentation (psx-spx): run-length coefficients, quantization and scale
//! tables, IDCT, and YUV to RGB output in 15 or 24 bits.
//!
//! It is driven at the register level (MDEC0/MDEC1 and DMA channels 0/1, see
//! `hw.rs`), so the game's own libpress code runs unchanged.

use std::collections::VecDeque;

/// Position in an 8x8 block of the k-th coefficient in zig-zag order.
const ZIGZAG: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21,
    28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54,
    47, 55, 62, 63,
];

pub struct Mdec {
    /// Words still expected by the current command.
    remaining: u32,
    command: u32,
    /// Halfwords of the current decode command.
    input: Vec<u16>,
    luma_q: [u8; 64],
    chroma_q: [u8; 64],
    scale: [i16; 64],
    /// Collected parameter words for table commands.
    table: Vec<u32>,
    pub output: VecDeque<u32>,
    pub enable_in: bool,
    pub enable_out: bool,
}

impl Default for Mdec {
    fn default() -> Self {
        Mdec {
            remaining: 0,
            command: 0,
            input: Vec::new(),
            luma_q: [0; 64],
            chroma_q: [0; 64],
            scale: [0; 64],
            table: Vec::new(),
            output: VecDeque::new(),
            enable_in: false,
            enable_out: false,
        }
    }
}

fn sext10(v: u16) -> i32 {
    (((v & 0x3FF) as i32) << 22) >> 22
}

impl Mdec {
    pub fn reset(&mut self) {
        self.remaining = 0;
        self.command = 0;
        self.input.clear();
        self.table.clear();
        self.output.clear();
    }

    /// A word written to MDEC0 (command or parameter).
    pub fn write_word(&mut self, w: u32) {
        if self.remaining == 0 {
            if w >> 29 == 1 {
                // A new decode abandons whatever the previous one produced
                // and the game did not read: the game skips the rest of a
                // frame when it falls behind. The hardware decodes as its
                // output is read, so nothing of the old frame remains; this
                // model decodes a whole command at once and must drop it.
                if super::trace("mdec") && !self.output.is_empty() {
                    eprintln!("[mdec] new decode drops {} unread words", self.output.len());
                }
                self.output.clear();
            }
            self.command = w;
            self.table.clear();
            self.input.clear();
            self.remaining = match w >> 29 {
                1 => w & 0xFFFF,
                2 => {
                    if w & 1 != 0 {
                        32
                    } else {
                        16
                    }
                }
                3 => 32,
                _ => 0,
            };
            if self.remaining == 0 && w >> 29 == 1 {
                self.decode();
            }
            return;
        }
        self.remaining -= 1;
        match self.command >> 29 {
            1 => {
                self.input.push(w as u16);
                self.input.push((w >> 16) as u16);
            }
            _ => self.table.push(w),
        }
        if self.remaining == 0 {
            match self.command >> 29 {
                1 => self.decode(),
                2 => {
                    let bytes: Vec<u8> = self.table.iter().flat_map(|w| w.to_le_bytes()).collect();
                    self.luma_q.copy_from_slice(&bytes[..64]);
                    if self.command & 1 != 0 {
                        self.chroma_q.copy_from_slice(&bytes[64..128]);
                    }
                }
                3 => {
                    for (i, half) in self.table.iter().flat_map(|w| [*w as u16, (*w >> 16) as u16]).enumerate() {
                        self.scale[i] = half as i16;
                    }
                }
                _ => {}
            }
        }
    }

    /// MDEC1 status for libpress's polling.
    /// The loaded quantization (luma, chroma) and scale tables, for tests.
    pub fn tables(&self) -> ([u8; 64], [u8; 64], [i16; 64]) {
        (self.luma_q, self.chroma_q, self.scale)
    }

    pub fn status(&self) -> u32 {
        let mut s = 0u32;
        if self.output.is_empty() {
            s |= 1 << 31;
        }
        if self.remaining > 0 {
            s |= 1 << 29; // command busy
        }
        if self.enable_in {
            s |= 1 << 28;
        }
        if self.enable_out && !self.output.is_empty() {
            s |= 1 << 27;
        }
        s |= ((self.command >> 27) & 3) << 25;
        s | (self.remaining.wrapping_sub(1) & 0xFFFF)
    }

    fn decode_block(&self, src: &[u16], pos: &mut usize, qt: &[u8; 64]) -> Option<[i32; 64]> {
        let mut blk = [0i32; 64];
        // Skip padding before the block.
        let mut n = loop {
            let n = *src.get(*pos)?;
            *pos += 1;
            if n != 0xFE00 {
                break n;
            }
        };
        let q_scale = i32::from((n >> 10) & 0x3F);
        let mut k = 0usize;
        let mut val = sext10(n) * i32::from(qt[0]);
        loop {
            if q_scale == 0 {
                val = sext10(n) * 2;
            }
            val = val.clamp(-0x400, 0x3FF);
            if q_scale > 0 {
                blk[ZIGZAG[k]] = val;
            } else {
                blk[k] = val;
            }
            n = *src.get(*pos)?;
            *pos += 1;
            k += usize::from((n >> 10) & 0x3F) + 1;
            if k > 63 {
                break;
            }
            val = (sext10(n) * i32::from(qt[k]) * q_scale + 4) / 8;
        }
        Some(self.idct(blk))
    }

    fn idct(&self, mut src: [i32; 64]) -> [i32; 64] {
        let mut dst = [0i32; 64];
        for _ in 0..2 {
            for x in 0..8 {
                for y in 0..8 {
                    let mut sum: i64 = 0;
                    for z in 0..8 {
                        sum += i64::from(src[y + z * 8]) * i64::from(self.scale[x + z * 8] / 8);
                    }
                    dst[x + y * 8] = ((sum + 0xFFF) / 0x2000) as i32;
                }
            }
            std::mem::swap(&mut src, &mut dst);
        }
        src
    }

    fn decode(&mut self) {
        let depth = (self.command >> 27) & 3;
        let signed = self.command & (1 << 26) != 0;
        let set_bit15 = self.command & (1 << 25) != 0;
        let input = std::mem::take(&mut self.input);
        let mut pos = 0;
        if depth < 2 {
            // Monochrome output is not used by the game's movies or images.
            eprintln!("[mdec] monochrome decode (depth {depth}) not implemented");
            return;
        }
        let mut bytes: Vec<u8> = Vec::new();
        let mut halfs: Vec<u16> = Vec::new();
        loop {
            let (lq, cq) = (self.luma_q, self.chroma_q);
            let Some(cr) = self.decode_block(&input, &mut pos, &cq) else { break };
            let Some(cb) = self.decode_block(&input, &mut pos, &cq) else { break };
            let mut ys = [[0i32; 64]; 4];
            let mut ok = true;
            for y in ys.iter_mut() {
                match self.decode_block(&input, &mut pos, &lq) {
                    Some(b) => *y = b,
                    None => ok = false,
                }
            }
            if !ok {
                break;
            }
            let mut rgb = [[0u8; 3]; 256];
            for (i, yblk) in ys.iter().enumerate() {
                let (xx, yy) = ((i % 2) * 8, (i / 2) * 8);
                for y in 0..8 {
                    for x in 0..8 {
                        let c = ((x + xx) / 2) + ((y + yy) / 2) * 8;
                        let (r, b) = (cr[c], cb[c]);
                        let g = (-88 * b - 183 * r) >> 8;
                        let (r, b) = ((359 * r) >> 8, (454 * b) >> 8);
                        let yv = yblk[x + y * 8];
                        let mut px = [(yv + r).clamp(-128, 127), (yv + g).clamp(-128, 127), (yv + b).clamp(-128, 127)];
                        if !signed {
                            for ch in px.iter_mut() {
                                *ch ^= 0x80;
                            }
                        }
                        rgb[(x + xx) + (y + yy) * 16] = [px[0] as u8, px[1] as u8, px[2] as u8];
                    }
                }
            }
            if depth == 2 {
                for px in rgb {
                    bytes.extend_from_slice(&px);
                }
            } else {
                for px in rgb {
                    let c = u16::from(px[0] >> 3) | (u16::from(px[1] >> 3) << 5) | (u16::from(px[2] >> 3) << 10);
                    halfs.push(c | if set_bit15 { 0x8000 } else { 0 });
                }
            }
        }
        if depth == 2 {
            bytes.resize(bytes.len().div_ceil(4) * 4, 0);
            self.output.extend(bytes.as_chunks::<4>().0.iter().map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])));
        } else {
            if halfs.len() % 2 == 1 {
                halfs.push(0);
            }
            self.output.extend(halfs.as_chunks::<2>().0.iter().map(|h| u32::from(h[0]) | (u32::from(h[1]) << 16)));
        }
    }
}

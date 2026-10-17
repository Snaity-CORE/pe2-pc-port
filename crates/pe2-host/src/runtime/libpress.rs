//! The MDEC calls of the data-compression library (libpress) the game makes,
//! written for the port. They drive the MDEC model through the same register
//! and DMA accesses the original makes (`hw.rs`), so decoding, its timing and
//! the completion callback behave as before.

use super::hw::{read32, write32};
use super::irq;

const MDEC0: u32 = 0x1F80_1820;
const MDEC1: u32 = 0x1F80_1824;
const DPCR: u32 = 0x1F80_10F0;
const DMA0_MADR: u32 = 0x1F80_1080;
const DMA0_BCR: u32 = 0x1F80_1084;
const DMA0_CHCR: u32 = 0x1F80_1088;
const DMA1_MADR: u32 = 0x1F80_1090;
const DMA1_BCR: u32 = 0x1F80_1094;
const DMA1_CHCR: u32 = 0x1F80_1098;

/// DPCR: MDEC in (channel 0) and out (channel 1) enabled.
const DPCR_MDEC: u32 = 0x88;
/// CHCR: start, sync mode 1 (blocks); bit 0 set for RAM to device.
const CHCR_START_TO_RAM: u32 = 0x0100_0200;
const CHCR_START_FROM_RAM: u32 = 0x0100_0201;

/// MDEC commands and status/control bits.
const MDEC_SET_QUANT_COLOR: u32 = 0x4000_0001;
const MDEC_SET_SCALE: u32 = 0x6000_0000;
const MDEC_RESET: u32 = 0x8000_0000;
const MDEC_ENABLE_DMA: u32 = 0x6000_0000;
/// Decode command bits DecDCTin sets from its mode: 15-bit output (rather
/// than 24-bit) and the mask bit in each output pixel.
const CMD_DEPTH_15: u32 = 1 << 27;
const CMD_MASK_BIT: u32 = 1 << 25;

/// DMA block control for `words` words in 32-word blocks.
fn block_control(words: u32) -> u32 {
    ((words >> 5) << 16) | 32
}

/// The default quantization table, for luma and chroma alike: MPEG-1's
/// intra matrix, in the zig-zag order the MDEC takes it.
pub(crate) fn default_quant() -> [u8; 64] {
    const INTRA: [u8; 64] = [
        2, 16, 19, 22, 26, 27, 29, 34, 16, 16, 22, 24, 27, 29, 34, 37, 19, 22, 26, 27, 29, 34, 34, 38, 22, 22, 26, 27,
        29, 34, 37, 40, 22, 26, 27, 29, 32, 35, 40, 48, 26, 27, 29, 32, 35, 40, 48, 58, 26, 27, 29, 34, 38, 46, 56, 69,
        27, 29, 35, 38, 46, 56, 69, 83,
    ];
    const ZIGZAG: [usize; 64] = [
        0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14,
        21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60,
        61, 54, 47, 55, 62, 63,
    ];
    std::array::from_fn(|i| INTRA[ZIGZAG[i]])
}

/// The IDCT scale table: c(u) cos((2x + 1) u pi / 16) in 15 fractional bits,
/// c(0) = 1/sqrt(2), rounded down.
pub(crate) fn default_scale() -> [i16; 64] {
    std::array::from_fn(|i| {
        let (u, x) = ((i / 8) as f64, (i % 8) as f64);
        let c = if u == 0.0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
        (c * ((2.0 * x + 1.0) * u * std::f64::consts::PI / 16.0).cos() * 32768.0).floor() as i16
    })
}

/// Sends a table command and its words to MDEC0.
fn send_table(command: u32, words: &[u32]) {
    read32(MDEC1);
    write32(DPCR, read32(DPCR) | DPCR_MDEC);
    write32(MDEC0, command);
    for &w in words {
        write32(MDEC0, w);
    }
}

/// DecDCTReset(mode): resets the MDEC and stops both of its DMA channels;
/// mode 0 also loads the default quantization and IDCT tables.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DecDCTReset(mode: i32) {
    let _g = irq::guard();
    write32(MDEC1, MDEC_RESET);
    write32(DMA0_CHCR, 0);
    write32(DMA1_CHCR, 0);
    write32(MDEC1, MDEC_ENABLE_DMA);
    if mode == 0 {
        let q = default_quant();
        let quant: Vec<u32> = q
            .iter()
            .chain(q.iter())
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| u32::from_le_bytes([*c[0], *c[1], *c[2], *c[3]]))
            .collect();
        send_table(MDEC_SET_QUANT_COLOR, &quant);
        let s = default_scale();
        let scale: Vec<u32> = s.chunks(2).map(|c| (c[0] as u16 as u32) | ((c[1] as u16 as u32) << 16)).collect();
        send_table(MDEC_SET_SCALE, &scale);
    }
}

/// DecDCTin(buf, mode): starts decoding the run-length data in buf. Its
/// first word is the MDEC decode command, which gets the output format from
/// mode (bit 0: 24-bit rather than 15-bit, bit 1: set the mask bit) and is
/// written back; the data follows it.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DecDCTin(buf: *mut u32, mode: i32) {
    let _g = irq::guard();
    // SAFETY: the game passes its run-length buffer in PSX RAM.
    let mut command = unsafe { buf.read() };
    command &= !(CMD_DEPTH_15 | CMD_MASK_BIT);
    if mode & 1 == 0 {
        command |= CMD_DEPTH_15;
    }
    if mode & 2 != 0 {
        command |= CMD_MASK_BIT;
    }
    // SAFETY: as above.
    unsafe { buf.write(command) };
    read32(MDEC1);
    write32(DPCR, read32(DPCR) | DPCR_MDEC);
    write32(DMA0_MADR, buf as u32 + 4);
    write32(DMA0_BCR, block_control(command & 0xFFFF));
    write32(MDEC0, command);
    write32(DMA0_CHCR, CHCR_START_FROM_RAM);
}

/// DecDCTout(buf, size): receives `size` words of decoded pixels into buf;
/// the DecDCToutCallback function runs when they are in.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DecDCTout(buf: *mut u32, size: i32) {
    let _g = irq::guard();
    read32(DMA1_CHCR);
    write32(DPCR, read32(DPCR) | DPCR_MDEC);
    write32(DMA1_CHCR, 0);
    write32(DMA1_MADR, buf as u32);
    write32(DMA1_BCR, block_control(size as u32));
    write32(DMA1_CHCR, CHCR_START_TO_RAM);
}

/// Copies the MDEC's tables (64 luma and 64 chroma quantization bytes, then
/// 64 scale halfwords) to `out`, for PE2_LIBTEST.
#[unsafe(no_mangle)]
pub extern "C" fn pc_mdec_tables(out: *mut u8) {
    let (l, c, s) = super::hw::mdec_tables();
    // SAFETY: the caller passes 256 writable bytes.
    let out = unsafe { std::slice::from_raw_parts_mut(out, 256) };
    out[..64].copy_from_slice(&l);
    out[64..128].copy_from_slice(&c);
    for (i, v) in s.iter().enumerate() {
        out[128 + 2 * i..130 + 2 * i].copy_from_slice(&v.to_le_bytes());
    }
}

/// DecDCToutCallback(func): `func` runs when a DecDCTout transfer completes;
/// returns the previous one.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DecDCToutCallback(func: u32) -> u32 {
    super::hw::pcrt_DMACallback(1, func)
}

//! The higher-resolution layer, drawn on the CPU (hires.rs) or on the
//! graphics card (hw.rs): every operation that changes the native VRAM is
//! repeated here.

use super::hires::Hires;
use super::hw::Hw;
use super::raster::{DrawState, Shading, Vertex, Vram};

pub enum Layer {
    Software(Hires),
    Hardware(Hw),
}

impl Layer {
    /// Before the native VRAM changes the rectangle (see hw.rs).
    pub fn before_native_write(&mut self, vram: &Vram, x: i32, y: i32, w: i32, h: i32) {
        if let Layer::Hardware(l) = self {
            l.before_native_write(vram, x, y, w, h);
        }
    }

    /// After the native VRAM changed the rectangle.
    pub fn after_native_write(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if let Layer::Hardware(l) = self {
            l.after_native_write(x, y, w, h);
        }
    }

    pub fn fill_command(&mut self, x: i32, y: i32, w: i32, h: i32, rgb: u32) {
        match self {
            Layer::Software(l) => l.fill_command(x, y, w, h, rgb),
            Layer::Hardware(l) => l.fill_command(x, y, w, h, rgb),
        }
    }

    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, rgb: u32) {
        match self {
            Layer::Software(l) => l.fill(x, y, w, h, rgb),
            Layer::Hardware(l) => l.fill(x, y, w, h, rgb),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn copy(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32, st: &DrawState) {
        match self {
            Layer::Software(l) => l.copy(sx, sy, dx, dy, w, h, st),
            Layer::Hardware(l) => l.copy(sx, sy, dx, dy, w, h),
        }
    }

    /// An image transfer; `st` when it honours the mask settings.
    #[allow(clippy::too_many_arguments)]
    pub fn upload(&mut self, vram: &Vram, x: i32, y: i32, w: i32, h: i32, pixels: &[u16], st: Option<&DrawState>) {
        match self {
            Layer::Software(l) if st.is_some() => {
                for (i, &px) in pixels.iter().enumerate().take((w * h).max(0) as usize) {
                    l.upload(x + i as i32 % w, y + i as i32 / w, px, st);
                }
            }
            Layer::Software(l) => l.upload_rect(x, y, w, h, pixels),
            Layer::Hardware(l) => l.upload_rect(vram, x, y, w, h, pixels),
        }
    }

    pub fn triangle(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, v: [Vertex; 3]) {
        match self {
            Layer::Software(l) => l.triangle(vram, st, sh, v),
            Layer::Hardware(l) => l.triangle(vram, st, sh, v),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rectangle(
        &mut self,
        vram: &Vram,
        st: &DrawState,
        sh: &Shading,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: (i32, i32, i32),
        u: i32,
        v: i32,
    ) {
        match self {
            Layer::Software(l) => l.rectangle(vram, st, sh, x, y, w, h, color, u, v),
            Layer::Hardware(l) => l.rectangle(vram, st, sh, x, y, w, h, color, u, v),
        }
    }

    pub fn line(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, a: Vertex, b: Vertex) {
        match self {
            Layer::Software(l) => l.line(vram, st, sh, a, b),
            Layer::Hardware(l) => l.line(vram, st, sh, a, b),
        }
    }
}

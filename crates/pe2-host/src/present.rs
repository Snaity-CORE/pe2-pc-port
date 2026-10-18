//! Shows the game's picture in the window through the graphics card,
//! letterboxed at 4:3: the hardware layer's latched picture, already on the
//! card, or else the CPU's (sent as a texture). Optionally as a CRT
//! television would ([video] crt): scanlines and an aperture grille; and
//! at whole multiples of its height ([video] integer_scaling).

use std::sync::Arc;

use winit::window::Window;

use crate::gfx::Gfx;

const SHADER: &str = r#"
struct Out {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// One triangle covering the viewport.
@vertex
fn vs(@builtin(vertex_index) i: u32) -> Out {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    var o: Out;
    o.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    o.uv = vec2<f32>(xy.x, 1.0 - xy.y);
    return o;
}

@group(0) @binding(0) var frame: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
// x: the console's lines in the picture (240, or 480 interlaced); y: 1 to
// take whole texels (the picture shown at whole multiples of its size);
// z, w: the picture's size in texels.
@group(0) @binding(2) var<uniform> params: vec4<f32>;

// The hardware layer keeps channels in the console's scale (5 bits times
// 8, up to 248): `gain` brings them to the full range.
override gain: f32 = 1.0;
// A CRT television's look.
override crt: bool = false;

// How much of a scanline's light reaches `d` lines from its centre: a
// gaussian beam, wider where the line is brighter, as a tube's spot is.
fn beam(d: f32, c: vec3<f32>) -> f32 {
    let luma = dot(c, vec3<f32>(0.299, 0.587, 0.114));
    let sigma = mix(0.26, 0.42, luma);
    return exp(-(d * d) / (2.0 * sigma * sigma));
}

@fragment
fn fs(o: Out) -> @location(0) vec4<f32> {
    var c: vec3<f32>;
    if (crt) {
        // The two scanlines around this point, each sampled at its centre
        // and weighted by its beam.
        let n = params.x;
        let y = o.uv.y * n - 0.5;
        let i = floor(y);
        let f = y - i;
        let c0 = textureSample(frame, smp, vec2<f32>(o.uv.x, (i + 0.5) / n)).rgb * gain;
        let c1 = textureSample(frame, smp, vec2<f32>(o.uv.x, (i + 1.5) / n)).rgb * gain;
        c = (c0 * beam(f, c0) + c1 * beam(1.0 - f, c1)) * 1.45;
        // Aperture grille: alternate red, green and blue stripes, column by
        // column of the window.
        let col = u32(o.pos.x) % 3u;
        let mask = select(select(vec3<f32>(0.82, 0.82, 1.08), vec3<f32>(0.82, 1.08, 0.82), col == 1u),
                          vec3<f32>(1.08, 0.82, 0.82), col == 0u);
        c = c * mask;
    } else if (params.y > 0.5) {
        let size = params.zw;
        c = textureLoad(frame, vec2<i32>(min(floor(o.uv * size), size - 1.0)), 0).rgb * gain;
    } else {
        c = textureSample(frame, smp, o.uv).rgb * gain;
    }
    return vec4<f32>(min(c, vec3<f32>(1.0)), 1.0);
}
"#;

pub struct Presenter {
    gfx: &'static Gfx,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    /// For CPU pictures, and for the hardware layer's (with the gain); as a
    /// CRT when asked.
    pipeline: wgpu::RenderPipeline,
    card_pipeline: wgpu::RenderPipeline,
    /// The shader's parameters (see `params` there), and what they hold.
    params: wgpu::Buffer,
    params_now: [f32; 4],
    /// The bind group of the hardware layer's picture, by its size.
    card: Option<((u32, u32), wgpu::BindGroup)>,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    /// The picture's texture, its size and its bind group.
    frame: Option<(wgpu::Texture, (u32, u32), wgpu::BindGroup)>,
    pixels: Vec<u32>,
    /// The surface wants configuring again (it said it was suboptimal):
    /// done before the next frame, never while one of its frames is held.
    stale: bool,
}

impl Presenter {
    pub fn new(gfx: &'static Gfx, window: Arc<Window>) -> Option<Presenter> {
        let size = window.inner_size();
        let surface = match gfx.instance.create_surface(window) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("pe2: window surface: {err}");
                return None;
            }
        };
        let caps = surface.get_capabilities(&gfx.adapter);
        // The picture's values are already gamma-encoded: an sRGB target
        // would encode them again.
        let format = caps.formats.iter().copied().find(|f| !f.is_srgb()).or(caps.formats.first().copied())?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        surface.configure(&gfx.device, &config);

        let device = &gfx.device;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("present"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("present"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let params = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("present params"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("present"),
            bind_group_layouts: &[Some(&layout)],
            ..Default::default()
        });
        let make_pipeline = |constants: &[(&str, f64)]| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("present"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions { constants, ..Default::default() },
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let crt = if crt() { 1.0 } else { 0.0 };
        let pipeline = make_pipeline(&[("crt", crt)]);
        let card_pipeline = make_pipeline(&[("gain", 255.0 / 248.0), ("crt", crt)]);
        // Nearest texels when the window is an integer multiple of the
        // picture; linear smooths other sizes.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("present"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Some(Presenter {
            gfx,
            surface,
            config,
            pipeline,
            card_pipeline,
            params,
            params_now: [0.0; 4],
            card: None,
            layout,
            sampler,
            frame: None,
            pixels: Vec::new(),
            stale: false,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.gfx.device, &self.config);
    }

    /// Draws the latched picture, letterboxed.
    pub fn present(&mut self) {
        if crate::runtime::gpu::front_on_card() {
            let front = crate::runtime::gpu::HW_FRONT.lock().unwrap();
            let Some(f) = front.as_ref() else { return };
            let size = (f.width, f.height);
            if self.card.as_ref().is_none_or(|c| c.0 != size) {
                let view = f.texture.create_view(&Default::default());
                let bind = self.bind(&view);
                self.card = Some((size, bind));
            }
            drop(front);
            let bind = self.card.as_ref().unwrap().1.clone();
            self.draw(&bind, true, size);
            return;
        }
        let Some((fw, fh)) = crate::runtime::gpu::snapshot(&mut self.pixels) else { return };
        let (fw, fh) = (fw as u32, fh as u32);
        let gfx = self.gfx;
        if self.frame.as_ref().is_none_or(|f| f.1 != (fw, fh)) {
            let texture = gfx.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("picture"),
                size: wgpu::Extent3d { width: fw, height: fh, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // 0x00RRGGBB words are B, G, R, 0 bytes.
                format: wgpu::TextureFormat::Bgra8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = texture.create_view(&Default::default());
            let bind = self.bind(&view);
            self.frame = Some((texture, (fw, fh), bind));
        }
        let (texture, _, bind) = self.frame.as_ref().unwrap();
        gfx.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck_u32(&self.pixels),
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(fw * 4), rows_per_image: Some(fh) },
            wgpu::Extent3d { width: fw, height: fh, depth_or_array_layers: 1 },
        );
        let bind = bind.clone();
        self.draw(&bind, false, (fw, fh));
    }

    fn bind(&self, view: &wgpu::TextureView) -> wgpu::BindGroup {
        self.gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("picture"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: self.params.as_entire_binding() },
            ],
        })
    }

    /// Draws the picture bound by `bind`, `size` texels, letterboxed, and
    /// presents it.
    fn draw(&mut self, bind: &wgpu::BindGroup, card: bool, size: (u32, u32)) {
        let gfx = self.gfx;
        let (x, y, pw, ph, whole) = viewport(self.config.width, self.config.height, size.0, size.1, integer_scaling());
        let lines = crate::runtime::gpu::picture_lines().max(1) as f32;
        let params = [lines, if whole { 1.0 } else { 0.0 }, size.0 as f32, size.1 as f32];
        if params != self.params_now {
            gfx.queue.write_buffer(&self.params, 0, &params.map(f32::to_le_bytes).concat());
            self.params_now = params;
        }
        if std::mem::take(&mut self.stale) {
            self.surface.configure(&gfx.device, &self.config);
        }
        let target = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) => t,
            wgpu::CurrentSurfaceTexture::Suboptimal(t) => {
                self.stale = true;
                t
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&gfx.device, &self.config);
                return;
            }
            _ => return,
        };
        let view = target.texture.create_view(&Default::default());
        let mut encoder = gfx.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_viewport(x as f32, y as f32, pw as f32, ph as f32, 0.0, 1.0);
            pass.set_pipeline(if card { &self.card_pipeline } else { &self.pipeline });
            pass.set_bind_group(0, bind, &[]);
            pass.draw(0..3, 0..1);
        }
        gfx.queue.submit([encoder.finish()]);
        gfx.queue.present(target);
    }
}

/// Where the picture goes in a `ww` x `wh` window: x, y, width, height,
/// and whether it is a whole multiple of the picture's `tw` x `th` texels
/// (shown with whole texels). The picture fills the window at 4:3, or,
/// with `integer`, takes the largest whole multiple of its height that
/// fits, at 4:3 (as wide as that height wants), unless not even one does.
fn viewport(ww: u32, wh: u32, tw: u32, th: u32, integer: bool) -> (u32, u32, u32, u32, bool) {
    let fit = || if ww * 3 > wh * 4 { (wh * 4 / 3, wh) } else { (ww, ww * 3 / 4) };
    let (pw, ph) = if integer && th > 0 {
        let mut k = wh / th;
        while k > 0 && th * k * 4 / 3 > ww {
            k -= 1;
        }
        if k == 0 { fit() } else { (th * k * 4 / 3, th * k) }
    } else {
        fit()
    };
    let whole = integer && tw > 0 && th > 0 && pw % tw == 0 && ph % th == 0;
    ((ww - pw) / 2, (wh - ph) / 2, pw, ph, whole)
}

/// Whether the picture is shown at whole multiples of its height:
/// PE2_INTEGER_SCALING=0/1, else pe2.toml's [video] integer_scaling, else
/// off.
fn integer_scaling() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("PE2_INTEGER_SCALING")
            .ok()
            .map(|v| v.trim() != "0")
            .or(crate::config::get().video.integer_scaling)
            .unwrap_or(false)
    })
}

/// Whether the picture looks like a CRT television's: PE2_CRT=0/1, else
/// pe2.toml's [video] crt, else off.
fn crt() -> bool {
    std::env::var("PE2_CRT").ok().map(|v| v.trim() != "0").or(crate::config::get().video.crt).unwrap_or(false)
}

fn bytemuck_u32(words: &[u32]) -> &[u8] {
    // SAFETY: u32 has no padding and u8 has alignment 1.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

#[cfg(test)]
mod tests {
    use super::viewport;

    /// Filling keeps 4:3 and centres the picture.
    #[test]
    fn fills_at_four_thirds() {
        assert_eq!(viewport(1920, 1080, 1280, 960, false), (240, 0, 1440, 1080, false));
        assert_eq!(viewport(800, 800, 320, 240, false), (0, 100, 800, 600, false));
    }

    /// Whole multiples of the picture's height, at 4:3; whole texels when
    /// the width is a multiple too; filling when not even one fits.
    #[test]
    fn integer_multiples() {
        assert_eq!(viewport(1920, 1080, 320, 240, true), (320, 60, 1280, 960, true));
        assert_eq!(viewport(1440, 1080, 1280, 960, true), (80, 60, 1280, 960, true));
        // 368 wide modes: 4:3 by height, not a multiple of the width.
        assert_eq!(viewport(1920, 1080, 368, 240, true), (320, 60, 1280, 960, false));
        // Too narrow for 4x at 1080 lines: 3x.
        assert_eq!(viewport(1200, 1080, 320, 240, true), (120, 180, 960, 720, true));
        // 960 lines do not fit 700: filling instead.
        assert_eq!(viewport(1000, 700, 1280, 960, true), (33, 0, 933, 700, false));
    }
}

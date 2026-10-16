//! Shows the game's picture in the window through the graphics card,
//! letterboxed at 4:3: the hardware layer's latched picture, already on the
//! card, or else the CPU's (sent as a texture).

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

// The hardware layer keeps channels in the console's scale (5 bits times
// 8, up to 248): `gain` brings them to the full range.
override gain: f32 = 1.0;

@fragment
fn fs(o: Out) -> @location(0) vec4<f32> {
    return vec4<f32>(min(textureSample(frame, smp, o.uv).rgb * gain, vec3<f32>(1.0)), 1.0);
}
"#;

pub struct Presenter {
    gfx: &'static Gfx,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    /// For CPU pictures, and for the hardware layer's (with the gain).
    pipeline: wgpu::RenderPipeline,
    card_pipeline: wgpu::RenderPipeline,
    /// The bind group of the hardware layer's picture, by its size.
    card: Option<((u32, u32), wgpu::BindGroup)>,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    /// The picture's texture, its size and its bind group.
    frame: Option<(wgpu::Texture, (u32, u32), wgpu::BindGroup)>,
    pixels: Vec<u32>,
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
            ],
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
                    targets: &[Some(wgpu::ColorTargetState { format, blend: None, write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: wgpu::PipelineCompilationOptions { constants, ..Default::default() },
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = make_pipeline(&[]);
        let card_pipeline = make_pipeline(&[("gain", 255.0 / 248.0)]);
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
            card: None,
            layout,
            sampler,
            frame: None,
            pixels: Vec::new(),
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
            self.draw(&bind, true);
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
            wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            bytemuck_u32(&self.pixels),
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(fw * 4), rows_per_image: Some(fh) },
            wgpu::Extent3d { width: fw, height: fh, depth_or_array_layers: 1 },
        );
        let bind = bind.clone();
        self.draw(&bind, false);
    }

    fn bind(&self, view: &wgpu::TextureView) -> wgpu::BindGroup {
        self.gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("picture"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        })
    }

    /// Draws the picture bound by `bind`, letterboxed, and presents it.
    fn draw(&mut self, bind: &wgpu::BindGroup, card: bool) {
        let gfx = self.gfx;
        let target = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) => t,
            wgpu::CurrentSurfaceTexture::Suboptimal(t) => {
                self.surface.configure(&gfx.device, &self.config);
                t
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&gfx.device, &self.config);
                return;
            }
            _ => return,
        };
        let view = target.texture.create_view(&Default::default());
        let (ww, wh) = (self.config.width as f32, self.config.height as f32);
        let (pw, ph) = if ww * 3.0 > wh * 4.0 { (wh * 4.0 / 3.0, wh) } else { (ww, ww * 3.0 / 4.0) };
        let mut encoder = gfx.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                })],
                ..Default::default()
            });
            pass.set_viewport(((ww - pw) / 2.0).floor(), ((wh - ph) / 2.0).floor(), pw.floor(), ph.floor(), 0.0, 1.0);
            pass.set_pipeline(if card { &self.card_pipeline } else { &self.pipeline });
            pass.set_bind_group(0, bind, &[]);
            pass.draw(0..3, 0..1);
        }
        gfx.queue.submit([encoder.finish()]);
        gfx.queue.present(target);
    }
}

fn bytemuck_u32(words: &[u32]) -> &[u8] {
    // SAFETY: u32 has no padding and u8 has alignment 1.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

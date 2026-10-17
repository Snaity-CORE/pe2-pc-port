//! Mods' textures on the card: one texture array whose layers are packed
//! with them (shelf by shelf), and a table telling the shader where each is
//! and how large the texture memory it replaces is (see textures.rs).

use std::collections::HashMap;

use crate::gfx::Gfx;
use crate::mods::{Picture, TextureKey};

/// Layers are this many pixels square; a texture larger is reduced to fit.
const SIZE: u32 = 2048;
/// Layers at most (16 MB each); beyond, the atlas starts over.
const MAX_LAYERS: u32 = 8;
/// Table entries at most; beyond, the atlas starts over.
const MAX_ENTRIES: u32 = 4096;

/// A row of a layer, filled left to right.
struct Shelf {
    y: u32,
    height: u32,
    next_x: u32,
}

pub struct Atlas {
    gfx: &'static Gfx,
    texture: wgpu::Texture,
    layers: u32,
    shelves: Vec<Vec<Shelf>>,
    /// Key -> table index + 1 (what vertices carry; 0 is none).
    entries: HashMap<TextureKey, u32>,
    table: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bind: wgpu::BindGroup,
}

impl Atlas {
    /// An atlas, of one layer when mods have textures (else of a pixel).
    pub fn new(gfx: &'static Gfx, wanted: bool) -> Atlas {
        let device = &gfx.device;
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty,
            count: None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("hw textures"),
            entries: &[
                entry(
                    0,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                ),
                entry(
                    1,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
            ],
        });
        let table = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hw texture table"),
            size: u64::from(if wanted { MAX_ENTRIES } else { 1 }) * 16,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let texture = Self::make_texture(gfx, if wanted { SIZE } else { 1 }, 1);
        let bind = Self::make_bind(gfx, &layout, &texture, &table);
        Atlas { gfx, texture, layers: 1, shelves: vec![Vec::new()], entries: HashMap::new(), table, layout, bind }
    }

    fn make_texture(gfx: &Gfx, size: u32, layers: u32) -> wgpu::Texture {
        gfx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hw textures"),
            size: wgpu::Extent3d { width: size, height: size, depth_or_array_layers: layers },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    }

    fn make_bind(
        gfx: &Gfx,
        layout: &wgpu::BindGroupLayout,
        texture: &wgpu::Texture,
        table: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hw textures"),
            layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: table.as_entire_binding() },
            ],
        })
    }

    /// The table index + 1 of texture `key`, if it is here.
    pub fn index(&self, key: TextureKey) -> Option<u32> {
        self.entries.get(&key).copied()
    }

    /// Adds `picture` as texture `key`, replacing `width` x `height` texels;
    /// None when full (the caller runs what was recorded and calls `clear`).
    pub fn add(&mut self, key: TextureKey, picture: &Picture, width: i32, height: i32) -> Option<u32> {
        if self.entries.len() as u32 >= MAX_ENTRIES {
            return None;
        }
        let (mut w, mut h) = (picture.width.max(1), picture.height.max(1));
        if w > SIZE || h > SIZE {
            let k = f64::from(SIZE) / f64::from(w.max(h));
            (w, h) = (((f64::from(w) * k) as u32).max(1), ((f64::from(h) * k) as u32).max(1));
        }
        let (layer, x, y) = match self.place(w, h) {
            Some(at) => at,
            None if self.layers < MAX_LAYERS => {
                self.grow();
                self.place(w, h)?
            }
            None => return None,
        };
        let rgba = if (w, h) == (picture.width, picture.height) {
            std::borrow::Cow::Borrowed(&picture.rgba)
        } else {
            std::borrow::Cow::Owned(crate::mods::resize(picture, w, h))
        };
        self.gfx.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: layer },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h) },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        let index = self.entries.len() as u32;
        let entry: [u32; 4] =
            [x | (y << 16), w | (h << 16), layer, (width as u32 & 0xFFFF) | ((height as u32 & 0xFFFF) << 16)];
        let bytes: Vec<u8> = entry.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.gfx.queue.write_buffer(&self.table, u64::from(index) * 16, &bytes);
        self.entries.insert(key, index + 1);
        Some(index + 1)
    }

    /// Forgets every texture (their space is reused).
    pub fn clear(&mut self) {
        self.entries.clear();
        for s in &mut self.shelves {
            s.clear();
        }
    }

    /// Room for `w` x `h` pixels: the first shelf they fit on, else a new
    /// shelf below the others.
    fn place(&mut self, w: u32, h: u32) -> Option<(u32, u32, u32)> {
        let size = self.texture.width();
        for (layer, shelves) in self.shelves.iter_mut().enumerate() {
            if let Some(s) = shelves.iter_mut().find(|s| s.height >= h && s.height <= h * 2 && s.next_x + w <= size) {
                let x = s.next_x;
                s.next_x += w;
                return Some((layer as u32, x, s.y));
            }
            let top = shelves.last().map_or(0, |s| s.y + s.height);
            if top + h <= size && w <= size {
                shelves.push(Shelf { y: top, height: h, next_x: w });
                return Some((layer as u32, 0, top));
            }
        }
        None
    }

    /// One layer more, the others copied over.
    fn grow(&mut self) {
        let texture = Self::make_texture(self.gfx, SIZE, self.layers + 1);
        let mut encoder = self.gfx.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_texture(
            self.texture.as_image_copy(),
            texture.as_image_copy(),
            wgpu::Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: self.layers },
        );
        self.gfx.queue.submit([encoder.finish()]);
        self.texture = texture;
        self.layers += 1;
        self.shelves.push(Vec::new());
        self.bind = Self::make_bind(self.gfx, &self.layout, &self.texture, &self.table);
    }
}

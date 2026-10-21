//! Mods' textures on the card: one texture array whose layers are packed
//! with them (shelf by shelf), a table telling the shader where each is and
//! how many texels of its sheet it replaces, and the pages primitives draw:
//! for each of a texture page's four columns, the texture replacing its
//! sheet (table index + 1, 0: none) and the VRAM row the sheet's top is at
//! (see textures.rs).
//!
//! Indexed replacements (mods/indexed.rs) are packed the same way into a
//! second texture array of the same size, of unsigned 8-bit values read as
//! they are (no filtering, no colour conversion): their table entries have
//! bit 31 of the layer set, and the shader looks their values up in the
//! CLUT the primitive is drawn with.

use std::collections::HashMap;

use crate::gfx::Gfx;
use crate::mods::indexed::Indexed;
use crate::mods::{Picture, TextureKey};

/// Layers are this many pixels square; a texture larger is reduced to fit.
const SIZE: u32 = 2048;
/// Layers at most (16 MB each); beyond, the atlas starts over.
const MAX_LAYERS: u32 = 8;
/// Table entries and pages at most; beyond, the atlas starts over.
const MAX_ENTRIES: u32 = 4096;
const MAX_PAGES: u32 = 4096;

/// A page: four textures (table index + 1), then four sheet tops.
pub type Page = [u32; 8];

/// A table entry's layer bit telling its texture is indexed.
pub const INDEXED: u32 = 1 << 31;

/// A row of a layer, filled left to right.
struct Shelf {
    y: u32,
    height: u32,
    next_x: u32,
}

pub struct Atlas {
    gfx: &'static Gfx,
    texture: wgpu::Texture,
    /// The indexed replacements' values, packed as `texture` is.
    indices: wgpu::Texture,
    layers: u32,
    shelves: Vec<Vec<Shelf>>,
    /// Key -> table index + 1 (what vertices carry; 0 is none).
    entries: HashMap<TextureKey, u32>,
    table: wgpu::Buffer,
    pages: HashMap<Page, u32>,
    page_buffer: wgpu::Buffer,
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
                entry(
                    2,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
                entry(
                    3,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
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
        let page_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hw texture pages"),
            size: u64::from(if wanted { MAX_PAGES } else { 1 }) * 32,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let size = if wanted { SIZE } else { 1 };
        let texture = Self::make_texture(gfx, size, 1, wgpu::TextureFormat::Rgba8Unorm);
        let indices = Self::make_texture(gfx, size, 1, wgpu::TextureFormat::R8Uint);
        let bind = Self::make_bind(gfx, &layout, &texture, &indices, &table, &page_buffer);
        Atlas {
            gfx,
            texture,
            indices,
            layers: 1,
            shelves: vec![Vec::new()],
            entries: HashMap::new(),
            table,
            pages: HashMap::new(),
            page_buffer,
            layout,
            bind,
        }
    }

    fn make_texture(gfx: &Gfx, size: u32, layers: u32, format: wgpu::TextureFormat) -> wgpu::Texture {
        gfx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hw textures"),
            size: wgpu::Extent3d { width: size, height: size, depth_or_array_layers: layers },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    }

    fn make_bind(
        gfx: &Gfx,
        layout: &wgpu::BindGroupLayout,
        texture: &wgpu::Texture,
        indices: &wgpu::Texture,
        table: &wgpu::Buffer,
        pages: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let array =
            wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() };
        let view = texture.create_view(&array);
        let index_view = indices.create_view(&array);
        gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hw textures"),
            layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: table.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: pages.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&index_view) },
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
        Some(self.enter(
            key,
            [x | (y << 16), w | (h << 16), layer, (width as u32 & 0xFFFF) | ((height as u32 & 0xFFFF) << 16)],
        ))
    }

    /// Adds indexed replacement `indexed` as texture `key`, replacing its
    /// sheet's texels; None when full (as `add`), or when larger than a
    /// layer. Its values go in as they are.
    pub fn add_indexed(&mut self, key: TextureKey, indexed: &Indexed) -> Option<u32> {
        let (w, h) = indexed.size;
        if self.entries.len() as u32 >= MAX_ENTRIES || w > SIZE || h > SIZE || w == 0 || h == 0 {
            return None;
        }
        let (layer, x, y) = match self.place(w, h) {
            Some(at) => at,
            None if self.layers < MAX_LAYERS => {
                self.grow();
                self.place(w, h)?
            }
            None => return None,
        };
        self.gfx.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.indices,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: layer },
                aspect: wgpu::TextureAspect::All,
            },
            &indexed.indices,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w), rows_per_image: Some(h) },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        let (nw, nh) = indexed.native;
        Some(self.enter(key, [x | (y << 16), w | (h << 16), layer | INDEXED, (nw & 0xFFFF) | ((nh & 0xFFFF) << 16)]))
    }

    /// Writes table entry `entry` for `key`; answers its index + 1.
    fn enter(&mut self, key: TextureKey, entry: [u32; 4]) -> u32 {
        let index = self.entries.len() as u32;
        let bytes: Vec<u8> = entry.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.gfx.queue.write_buffer(&self.table, u64::from(index) * 16, &bytes);
        self.entries.insert(key, index + 1);
        index + 1
    }

    /// The index + 1 of `page` in the page table; None when full.
    pub fn page(&mut self, page: Page) -> Option<u32> {
        if let Some(&i) = self.pages.get(&page) {
            return Some(i);
        }
        let index = self.pages.len() as u32;
        if index >= MAX_PAGES {
            return None;
        }
        let bytes: Vec<u8> = page.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.gfx.queue.write_buffer(&self.page_buffer, u64::from(index) * 32, &bytes);
        self.pages.insert(page, index + 1);
        Some(index + 1)
    }

    /// Forgets every texture (their space is reused).
    pub fn clear(&mut self) {
        self.entries.clear();
        self.pages.clear();
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

    /// One layer more (in both arrays), the others copied over.
    fn grow(&mut self) {
        let texture = Self::make_texture(self.gfx, SIZE, self.layers + 1, wgpu::TextureFormat::Rgba8Unorm);
        let indices = Self::make_texture(self.gfx, SIZE, self.layers + 1, wgpu::TextureFormat::R8Uint);
        let mut encoder = self.gfx.device.create_command_encoder(&Default::default());
        let extent = wgpu::Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: self.layers };
        encoder.copy_texture_to_texture(self.texture.as_image_copy(), texture.as_image_copy(), extent);
        encoder.copy_texture_to_texture(self.indices.as_image_copy(), indices.as_image_copy(), extent);
        self.gfx.queue.submit([encoder.finish()]);
        self.texture = texture;
        self.indices = indices;
        self.layers += 1;
        self.shelves.push(Vec::new());
        self.bind =
            Self::make_bind(self.gfx, &self.layout, &self.texture, &self.indices, &self.table, &self.page_buffer);
    }
}

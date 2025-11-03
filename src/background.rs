use crate::{
    PANEL_HEIGHT_BASE,
    spotify::{ARTIST_DATA_CACHE, IMAGES_CACHE, PLAYBACK_STATE, TRACK_DATA_CACHE, TrackData},
};
use anyhow::Result;
use auto_palette::Palette;
use bytemuck::{Pod, Zeroable};
use image::{Pixel, RgbaImage};
use itertools::Itertools;
use orx_parallel::{IntoParIter, ParIter};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::{
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    sync::LazyLock,
};
use vello::{
    Renderer,
    peniko::{Blob, ImageAlphaType, ImageData, ImageFormat},
    util::DeviceHandle,
    wgpu,
};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferDescriptor, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoderDescriptor, Extent3d, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp,
    TexelCopyBufferLayout, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension,
    VertexState,
};

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

/// Dimensions of the generated palette-based textures.
const PALETTE_IMAGE_HEIGHT: u32 = PANEL_HEIGHT_BASE as u32;
const PALETTE_IMAGE_WIDTH: u32 = PALETTE_IMAGE_HEIGHT * 3;

/// Number of refinement passes when synthesising the background texture.
const PALETTE_PASS_COUNT: usize = 8;
/// Maximum number of brush placements per pass.
const PALETTE_STROKES_PER_PASS: usize = 16;

const STALE_FRAME_BUDGET: u64 = 600;

static BRUSHES: LazyLock<[RgbaImage; 5]> = LazyLock::new(|| {
    let bytes = (
        include_bytes!("../assets/brushes/brush1.png"),
        include_bytes!("../assets/brushes/brush2.png"),
        include_bytes!("../assets/brushes/brush3.png"),
        include_bytes!("../assets/brushes/brush4.png"),
        include_bytes!("../assets/brushes/brush5.png"),
    );
    [
        image::load_from_memory(bytes.0).unwrap().to_rgba8(),
        image::load_from_memory(bytes.1).unwrap().to_rgba8(),
        image::load_from_memory(bytes.2).unwrap().to_rgba8(),
        image::load_from_memory(bytes.3).unwrap().to_rgba8(),
        image::load_from_memory(bytes.4).unwrap().to_rgba8(),
    ]
});

const WARP_SHADER_SRC: &str = include_str!("warp_background.wgsl");

const BACKGROUND_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct WarpUniforms {
    params: [f32; 4],
}

struct BackgroundSlot {
    texture: Texture,
    texture_size: (u32, u32),
    bind_group: BindGroup,
    output_view: TextureView,
    output_image: ImageData,
    last_frame: u64,
}

pub struct WarpBackground {
    pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,
    uniform_buffer: wgpu::Buffer,
    slots: HashMap<String, BackgroundSlot>,
}

impl WarpBackground {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("cantus_warp_shader"),
            source: ShaderSource::Wgsl(WARP_SHADER_SRC.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("cantus_warp_bind_group_layout"),
            entries: &[
                // Album Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // Uniform Buffer
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("cantus_warp_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("cantus_warp_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: BACKGROUND_FORMAT,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("cantus_warp_sampler"),
            address_mode_u: wgpu::AddressMode::MirrorRepeat,
            address_mode_v: wgpu::AddressMode::MirrorRepeat,
            address_mode_w: wgpu::AddressMode::MirrorRepeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("cantus_warp_uniform_buffer"),
            size: std::mem::size_of::<WarpUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            bind_group_layout,
            sampler,
            uniform_buffer,
            slots: HashMap::new(),
        }
    }

    pub fn render(
        &mut self,
        key: &str,
        device_handle: &DeviceHandle,
        renderer: &mut Renderer,
        image: &ImageData,
        elapsed_seconds: f32,
        frame_index: u64,
    ) -> ImageData {
        let device = &device_handle.device;
        let queue = &device_handle.queue;
        let image_size = (image.width, image.height);
        let slot = self.slots.entry(key.to_owned()).or_insert_with(|| {
            BackgroundSlot::new(
                device,
                renderer,
                &self.bind_group_layout,
                &self.sampler,
                &self.uniform_buffer,
                image,
            )
        });

        let needs_resize = slot.texture_size != image_size;
        if needs_resize {
            renderer.unregister_texture(slot.output_image.clone());
            *slot = BackgroundSlot::new(
                device,
                renderer,
                &self.bind_group_layout,
                &self.sampler,
                &self.uniform_buffer,
                image,
            );
        }

        queue.write_texture(
            slot.texture.as_image_copy(),
            image.data.data(),
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * image.width),
                rows_per_image: Some(image.height),
            },
            Extent3d {
                width: image.width,
                height: image.height,
                depth_or_array_layers: 1,
            },
        );

        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&WarpUniforms {
                params: [elapsed_seconds, 0.0, 0.0, 0.0],
            }),
        );

        slot.last_frame = frame_index;

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("cantus_warp_encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("cantus_warp_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &slot.output_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &slot.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        slot.output_image.clone()
    }

    pub fn purge_stale(&mut self, renderer: &mut Renderer, frame_index: u64) {
        self.slots.retain(|_, slot| {
            let keep = frame_index.saturating_sub(slot.last_frame) <= STALE_FRAME_BUDGET;
            if !keep {
                renderer.unregister_texture(slot.output_image.clone());
            }
            keep
        });
    }
}

impl BackgroundSlot {
    fn new(
        device: &wgpu::Device,
        renderer: &mut Renderer,
        bind_group_layout: &BindGroupLayout,
        sampler: &Sampler,
        uniform_buffer: &wgpu::Buffer,
        image: &ImageData,
    ) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("cantus_warp_texture"),
            size: Extent3d {
                width: image.width,
                height: image.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("cantus_warp_bind_group"),
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });

        let output_texture = device.create_texture(&TextureDescriptor {
            label: Some("cantus_warp_output_texture"),
            size: Extent3d {
                width: image.width,
                height: image.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: BACKGROUND_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let output_view = output_texture.create_view(&TextureViewDescriptor::default());
        let output_image = renderer.register_texture(output_texture);

        Self {
            texture,
            bind_group,
            texture_size: (image.width, image.height),
            output_view,
            output_image,
            last_frame: 0,
        }
    }
}

/// Downloads and caches an image from the given URL.
pub fn update_color_palettes() -> Result<()> {
    let state = PLAYBACK_STATE.lock().clone();
    let mut pending_palettes = Vec::new();
    for track in &state.queue {
        if !TRACK_DATA_CACHE.contains_key(&track.id)
            && let Some(image) = IMAGES_CACHE.get(&track.image_url)
            && let Some(artist_image_ref) = ARTIST_DATA_CACHE.get(&track.artist_id)
            && let Some(artist_image) = IMAGES_CACHE.get(&*artist_image_ref)
        {
            // Merge the images side by side
            let width = image.width;
            let height = image.height;
            let album_image =
                RgbaImage::from_raw(width, height, image.data.data().to_vec()).unwrap();

            // Get palette, try on the album image or if that doesn't get enough colours include the artist image
            let swatches = {
                let palette: Palette<f64> = Palette::builder()
                    .algorithm(auto_palette::Algorithm::SNIC)
                    .filter(ChromaFilter { threshold: 20 })
                    .build(&auto_palette::ImageData::new(width, height, &album_image)?)?;
                let swatches = palette
                    .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                    .or_else(|_| palette.find_swatches(NUM_SWATCHES))?;
                if swatches.len() < NUM_SWATCHES {
                    // Generate a new image with the artist image
                    let artist_new_width = (width as f32 * 0.1).round() as u32;
                    let mut new_img = RgbaImage::new(width + artist_new_width, height);
                    image::imageops::overlay(&mut new_img, &album_image, 0, 0);
                    let artist_img_resized = image::imageops::resize(
                        &image::RgbaImage::from_raw(
                            artist_image.width,
                            artist_image.height,
                            artist_image.data.data().to_vec(),
                        )
                        .unwrap(),
                        artist_new_width,
                        height,
                        image::imageops::FilterType::Triangle,
                    );
                    image::imageops::overlay(
                        &mut new_img,
                        &artist_img_resized,
                        i64::from(width),
                        0,
                    );

                    let palette: Palette<f64> = Palette::builder()
                        .algorithm(auto_palette::Algorithm::SLIC)
                        .filter(ChromaFilter { threshold: 20 })
                        .build(&auto_palette::ImageData::new(
                            width + artist_new_width,
                            height,
                            &new_img,
                        )?)?;
                    palette
                        .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                        .or_else(|_| palette.find_swatches(NUM_SWATCHES))?
                } else {
                    swatches
                }
            };

            // Sort out the ratios
            let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
            let primary_colors = swatches
                .iter()
                .map(|s| {
                    let rgb = s.color().to_rgb();
                    // Sometimes ratios can be tiny like 0.05%, this brings them a little closer to even
                    let lerped_ratio = lerp(
                        0.5,
                        (s.ratio() / total_ratio_sum) as f32,
                        1.0 / swatches.len() as f32,
                    );
                    [rgb.r, rgb.g, rgb.b, (lerped_ratio * 255.0).round() as u8]
                })
                .sorted_by(|a, b| b[3].cmp(&a[3]))
                .collect::<Vec<_>>();

            let palette_seed = {
                let mut hasher = DefaultHasher::new();
                track.id.hash(&mut hasher);
                hasher.finish()
            };
            pending_palettes.push((track.id.clone(), primary_colors, palette_seed));
        }
    }
    drop(state);

    let generated_data: Vec<_> = pending_palettes
        .into_par()
        .map(|(track_id, primary_colors, palette_seed)| {
            let palette_image = ImageData {
                data: Blob::from(generate_palette_image(&primary_colors, palette_seed)),
                format: ImageFormat::Rgba8,
                alpha_type: ImageAlphaType::Alpha,
                width: PALETTE_IMAGE_WIDTH,
                height: PALETTE_IMAGE_HEIGHT,
            };
            (
                track_id,
                TrackData {
                    primary_colors,
                    palette_image,
                },
            )
        })
        .collect();
    for (track_id, track_data) in generated_data {
        TRACK_DATA_CACHE.insert(track_id, track_data);
    }

    Ok(())
}

/// A filter that filters chroma values.
#[derive(Debug)]
pub struct ChromaFilter {
    threshold: u8,
}
impl auto_palette::Filter for ChromaFilter {
    fn test(&self, pixel: &auto_palette::Rgba) -> bool {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max - min) > self.threshold
    }
}

fn generate_palette_image(colors: &[[u8; 4]], seed: u64) -> Vec<u8> {
    let mut canvas = RgbaImage::from_pixel(
        PALETTE_IMAGE_WIDTH,
        PALETTE_IMAGE_HEIGHT,
        image::Rgba([12, 14, 18, 255]),
    );

    if colors.is_empty() {
        return canvas.into_raw();
    }

    let mut rng = SmallRng::seed_from_u64(seed);

    let mut targets = colors
        .iter()
        .map(|c| f32::from(c[3]).max(1.0))
        .collect::<Vec<_>>();
    let total_target = targets.iter().copied().sum::<f32>().max(1.0);
    for weight in &mut targets {
        *weight /= total_target;
    }

    let color_vectors = colors
        .iter()
        .map(|[r, g, b, _]| [f32::from(*r), f32::from(*g), f32::from(*b)])
        .collect::<Vec<_>>();

    let base = colors[0];
    for pixel in canvas.pixels_mut() {
        pixel.0 = [base[0], base[1], base[2], 255];
    }

    // Fill with the first colour; refinement passes will rebalance ratios.
    let total_pixels = (PALETTE_IMAGE_WIDTH * PALETTE_IMAGE_HEIGHT) as f32;

    let mut counts = vec![0u32; colors.len()];
    let mut coverage = vec![0.0f32; colors.len()];
    let mut per_color_strokes = vec![0u8; colors.len()];
    let mut available_indices = Vec::with_capacity(colors.len());

    for pass in 0..PALETTE_PASS_COUNT {
        let base_height = lerp(
            pass as f32 / PALETTE_PASS_COUNT as f32,
            PALETTE_IMAGE_HEIGHT as f32 * 0.7,
            PALETTE_IMAGE_HEIGHT as f32 * 0.3,
        );

        // Count pixels for each color, to get ratios
        counts.fill(0);
        for pixel in canvas.pixels() {
            let pr = f32::from(pixel[0]);
            let pg = f32::from(pixel[1]);
            let pb = f32::from(pixel[2]);
            let mut best_index = 0usize;
            let mut best_distance = f32::MAX;
            for (index, color) in color_vectors.iter().enumerate() {
                let dr = pr - color[0];
                let dg = pg - color[1];
                let db = pb - color[2];
                let distance = dr * dr + dg * dg + db * db;
                if distance < best_distance {
                    best_distance = distance;
                    best_index = index;
                }
            }
            counts[best_index] += 1;
        }
        for (index, ratio) in coverage.iter_mut().enumerate() {
            *ratio = counts[index] as f32 / total_pixels;
        }

        // Get how far we are off in total
        let total_coverage_diff = coverage
            .iter()
            .zip(targets.iter())
            .map(|(&c, &t)| (c - t).abs())
            .sum::<f32>()
            .abs();
        if total_coverage_diff <= f32::EPSILON {
            per_color_strokes.fill(0);
        } else {
            for (index, strokes) in per_color_strokes.iter_mut().enumerate() {
                *strokes = (((coverage[index] - targets[index]).abs() / total_coverage_diff)
                    * PALETTE_STROKES_PER_PASS as f32)
                    .floor() as u8;
            }
        }

        available_indices.clear();
        for (index, &count) in per_color_strokes.iter().enumerate() {
            if count > 0 {
                available_indices.push(index);
            }
        }

        for _ in 0..PALETTE_STROKES_PER_PASS {
            if available_indices.is_empty() {
                break;
            }

            // Randomly select an index from the available candidates
            let index_to_pick = rng.random_range(0..available_indices.len());
            let color_index = available_indices[index_to_pick];
            let strokes_left = &mut per_color_strokes[color_index];
            *strokes_left = strokes_left.saturating_sub(1);
            if *strokes_left == 0 {
                available_indices.swap_remove(index_to_pick);
            }
            let color = colors[color_index];

            // Pick a random brush
            let template = &BRUSHES[rng.random_range(0..BRUSHES.len())];
            let brush_factor = rng.random_range(0.75..1.2);
            let brush_size = (base_height * brush_factor)
                .round()
                .clamp(6.0, PALETTE_IMAGE_HEIGHT as f32) as u32;
            let stamp = image::imageops::resize(
                template,
                brush_size,
                brush_size,
                image::imageops::FilterType::Triangle,
            );

            // Overlay the stamp onto the canvas
            let fade_factor = rng.random_range(0.55..0.9);
            let x =
                i64::from(rng.random_range(0..=PALETTE_IMAGE_WIDTH)) - i64::from(brush_size / 2);
            let y =
                i64::from(rng.random_range(0..=PALETTE_IMAGE_HEIGHT)) - i64::from(brush_size / 2);
            let (bottom_width, bottom_height) = canvas.dimensions();
            let (top_width, top_height) = stamp.dimensions();

            // Crop our top image if we're going out of bounds
            let origin_bottom_x = x.clamp(0, i64::from(bottom_width)) as u32;
            let origin_bottom_y = y.clamp(0, i64::from(bottom_height)) as u32;

            let range_width = x
                .saturating_add(i64::from(top_width))
                .clamp(0, i64::from(bottom_width)) as u32
                - origin_bottom_x;
            let range_height = y
                .saturating_add(i64::from(top_height))
                .clamp(0, i64::from(bottom_height)) as u32
                - origin_bottom_y;

            let origin_top_x = x.saturating_mul(-1).clamp(0, i64::from(top_width)) as u32;
            let origin_top_y = y.saturating_mul(-1).clamp(0, i64::from(top_height)) as u32;

            for y in 0..range_height {
                for x in 0..range_width {
                    let stamp = {
                        let mut pixel = *stamp.get_pixel(origin_top_x + x, origin_top_y + y);
                        pixel[0] = color[0];
                        pixel[1] = color[1];
                        pixel[2] = color[2];
                        pixel[3] = ((f32::from(pixel[3]) * fade_factor)
                            .round()
                            .clamp(1.0, 255.0)) as u8;
                        pixel
                    };
                    let mut bottom_pixel =
                        *canvas.get_pixel(origin_bottom_x + x, origin_bottom_y + y);
                    bottom_pixel.blend(&stamp);

                    *canvas.get_pixel_mut(origin_bottom_x + x, origin_bottom_y + y) = bottom_pixel;
                }
            }
        }
    }

    // Blur the image
    image::imageops::blur(&canvas, 10.0).into_raw()
}

fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

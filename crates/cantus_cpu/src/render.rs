use crate::{
    CantusApp, MAX_RENDER_INSTANCES, PANEL_EXTENSION, PANEL_START, PARTICLE_COUNT,
    TRACK_SPACING_MS,
    art::{AlbumArt, ArtState},
    config::Config,
    model::{AudioFeatures, CondensedPlaylist, Rect, Track, playlist_icons},
    pipelines::{IMAGE_SIZE, MAX_TEXTURE_IMAGES},
    text_render::TextRenderer,
};
use arrayvec::ArrayVec;
use cantus_shared::{
    BackgroundPill, GlobalUniforms, ICON_SPACING, MAX_PILL_PLAYLIST_ICONS, PackedAudioFeatures,
    Particle, PlayheadUniforms, approach,
};
use glam::{FloatExt, Vec2, vec2};
use std::{f32::consts::TAU, mem, ops::Range, sync::Arc, time::Instant};
use wgpu::{
    BindGroup, Buffer, Color, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, Instance,
    LoadOp, Operations, Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, StoreOp, Surface, SurfaceConfiguration, Texture, TextureViewDescriptor,
};

/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f32 = 20.0;
/// Horizontal velocity range applied at spawn.
const SPARK_VELOCITY_X: Range<usize> = 40..60;
/// Vertical velocity range applied at spawn.
const SPARK_VELOCITY_Y: f32 = 5.0;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f32> = 1.2..1.5;

const DEFAULT_AUDIO_FEATURES: PackedAudioFeatures =
    PackedAudioFeatures::new([128, 128, 77, 102], [128, 51, 26, 213]);

const PLAYHEAD_START_DURATION: f32 = 0.7;
const PLAYHEAD_TRANSITION_SPEED: f32 = 5.5;
const DETAIL_FADE_DURATION: f32 = 0.2;

const fn flag(value: bool) -> f32 {
    if value { 1.0 } else { 0.0 }
}

fn layout_tracks(queue: &mut [Track], config: &Config, current_ms: f32) {
    let history_width = config.history_width;
    let height = config.height;
    let px_per_ms = config.px_per_ms();
    let timeline_end_ms = config.timeline_start_ms() + config.timeline_duration_ms();
    let timeline_end_x = history_width + config.timeline_width();
    let mut compact_count = 0;
    let mut transition = 0.0;
    let mut queue_offset = 0.0;

    for track in &mut *queue {
        track.runtime.width = 0.0;
        let start_ms = current_ms + queue_offset;
        queue_offset += track.queue_span_ms();
        if start_ms > timeline_end_ms {
            continue;
        }

        let natural_start = config.playhead_x() + start_ms * px_per_ms;
        let natural_end = natural_start + track.duration_ms as f32 * px_per_ms;
        let runtime = &mut track.runtime;
        runtime.start_ms = start_ms;
        if natural_end >= history_width + height {
            runtime.start_x = natural_start.max(history_width);
            runtime.width = natural_end.min(timeline_end_x) - runtime.start_x;
        } else if natural_end >= history_width {
            transition = (history_width + height - natural_end) / height;
            runtime.start_x = natural_end - height;
            runtime.width = height;
        } else {
            compact_count += 1;
        }
    }

    let stride = height * 0.55;
    let gap = TRACK_SPACING_MS * px_per_ms;
    for (index, track) in queue[..compact_count].iter_mut().enumerate() {
        let slot = compact_count - index - 1;
        let right = history_width - gap - (slot as f32 + transition) * stride;
        track.runtime.start_x = right - height;
        track.runtime.width = height;
    }
}

pub struct GpuResources {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub uniform_buffer: Buffer,
    pub playhead: GpuPass,
    pub background: GpuPass,
    pub text: GpuPass,
    pub particles: GpuPass,
    pub images: ImageAtlas,
    pub text_renderer: TextRenderer,
}

pub struct GpuPass {
    pub pipeline: RenderPipeline,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl GpuPass {
    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>, instances: u32) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..4, 0..instances);
    }

    fn draw_data<'pass, T: bytemuck::NoUninit>(
        &'pass self,
        queue: &Queue,
        pass: &mut RenderPass<'pass>,
        data: &[T],
    ) {
        if data.is_empty() {
            return;
        }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
        self.draw(pass, data.len() as u32);
    }
}

impl GpuResources {
    pub fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        if (self.surface_config.width, self.surface_config.height) != (width, height) {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.configure_surface();
        }
    }
}

pub struct ImageAtlas {
    pub texture: Texture,
    pub slots: [Option<Arc<AlbumArt>>; MAX_TEXTURE_IMAGES as usize],
    pub used: u32,
}

impl ImageAtlas {
    fn image_index(&mut self, queue: &Queue, art: &Arc<AlbumArt>) -> i32 {
        if let Some(index) = self
            .slots
            .iter()
            .position(|slot| slot.as_ref().is_some_and(|slot| Arc::ptr_eq(slot, art)))
        {
            self.used |= 1 << index;
            return index as i32;
        }

        let index = (!self.used).trailing_zeros();
        if index >= MAX_TEXTURE_IMAGES {
            return -1;
        }
        self.used |= 1 << index;
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: index,
                },
            },
            &art.pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * IMAGE_SIZE),
                rows_per_image: Some(IMAGE_SIZE),
            },
            wgpu::Extent3d {
                width: IMAGE_SIZE,
                height: IMAGE_SIZE,
                depth_or_array_layers: 1,
            },
        );
        self.slots[index as usize] = Some(Arc::clone(art));
        index as i32
    }
}

pub struct RenderState {
    pub instance: Instance,
    pub gpu: Option<GpuResources>,
    pub start_time: Instant,
    pub last_update: Instant,
    pub track_offset: f32,
    pub movement_speed: f32,
    pub last_toggle_playing: Instant,
    pub particles: [Particle; PARTICLE_COUNT],
    pub particles_accumulator: f32,
    /// Physical buffer pixels per logical Wayland surface pixel.
    pub scale: f32,
    pub surface_width: Option<f32>,
    pub uniforms: GlobalUniforms,
    pub pills: Vec<BackgroundPill>,
    pub playhead: PlayheadUniforms,
}
impl Default for RenderState {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            instance: Instance::default(),
            gpu: None,
            start_time: now,
            last_update: now,
            track_offset: 0.0,
            movement_speed: 0.0,
            last_toggle_playing: now,
            particles: [Particle::default(); PARTICLE_COUNT],
            particles_accumulator: 0.0,
            scale: 1.0,
            surface_width: None,
            uniforms: GlobalUniforms::default(),
            pills: Vec::with_capacity(MAX_RENDER_INSTANCES),
            playhead: PlayheadUniforms::default(),
        }
    }
}

impl CantusApp {
    pub fn emit_click_particles(&mut self, position: Vec2) {
        let time = self.render.start_time.elapsed().as_secs_f32();
        for particle in self
            .render
            .particles
            .iter_mut()
            .filter(|particle| time > particle.end_time)
            .take(20)
        {
            let angle = fastrand::f32() * TAU;
            let speed = 30.0 + fastrand::f32() * 20.0;
            let duration = 0.5.lerp(1.5, fastrand::f32());
            particle.spawn_pos = position;
            particle.spawn_vel = Vec2::from_angle(angle) * speed;
            particle.color =
                u32::from_le_bytes([255, 215, 50, (duration * 100.0).min(255.0) as u8]);
            particle.end_time = time + duration;
        }
    }

    pub fn logical_surface_size(&self) -> (f32, f32) {
        (
            self.render.surface_width.unwrap_or(self.config.width),
            self.config.height + PANEL_START + PANEL_EXTENSION,
        )
    }

    pub fn buffer_size(&self) -> (u32, u32) {
        let (width, height) = self.logical_surface_size();
        (
            (width * self.render.scale).round() as u32,
            (height * self.render.scale).round() as u32,
        )
    }

    pub fn playhead_rect(&self) -> Rect {
        let x = self.config.playhead_x();
        Rect::from_center(
            vec2(x, PANEL_START + self.config.height * 0.5),
            vec2(self.config.height * 0.25, self.config.height * 0.5),
        )
    }

    /// Render a frame and report whether the surface must be recreated.
    pub fn render(&mut self) -> bool {
        while let Ok(update) = self.app_updates.try_recv() {
            update(self);
        }
        self.start_missing_art_downloads();

        let Some(gpu) = self.render.gpu.as_mut() else {
            return false;
        };
        let (surface_texture, reconfigure_after_present) = match gpu.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => (texture, false),
            CurrentSurfaceTexture::Suboptimal(texture) => (texture, true),
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return false,
            CurrentSurfaceTexture::Outdated => {
                gpu.configure_surface();
                return false;
            }
            CurrentSurfaceTexture::Lost => return true,
            CurrentSurfaceTexture::Validation => {
                tracing::error!("surface texture acquisition failed validation");
                return false;
            }
        };

        gpu.images.used = 0;
        gpu.text_renderer.glyphs.clear();
        self.create_scene();

        let gpu = self.render.gpu.as_mut().unwrap();
        gpu.queue.write_buffer(
            &gpu.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.render.uniforms),
        );
        gpu.queue.write_buffer(
            &gpu.particles.buffer,
            0,
            bytemuck::cast_slice(&self.render.particles),
        );
        gpu.queue.write_buffer(
            &gpu.playhead.buffer,
            0,
            bytemuck::bytes_of(&self.render.playhead),
        );

        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            gpu.background
                .draw_data(&gpu.queue, &mut pass, &self.render.pills);
            gpu.text
                .draw_data(&gpu.queue, &mut pass, &gpu.text_renderer.glyphs);
            gpu.particles.draw(&mut pass, PARTICLE_COUNT as u32);
            gpu.playhead.draw(&mut pass, 1);
        }

        gpu.queue.submit([encoder.finish()]);
        gpu.queue.present(surface_texture);
        if reconfigure_after_present {
            gpu.configure_surface();
        }
        false
    }

    fn get_image_index(&mut self, art: &ArtState) -> i32 {
        let (Some(gpu), ArtState::Ready(art)) = (self.render.gpu.as_mut(), art) else {
            return -1;
        };
        gpu.images.image_index(&gpu.queue, art)
    }

    pub fn create_scene(&mut self) {
        let now = Instant::now();
        let dt = now
            .duration_since(self.render.last_update)
            .as_secs_f32()
            .min(0.1);
        self.render.last_update = now;

        let px_per_ms = self.config.px_per_ms();
        let playhead_x = self.config.playhead_x();

        let mut playback_state = mem::take(&mut self.playback);
        if playback_state.queue.is_empty() {
            self.render.pills.clear();
            self.playback = playback_state;
            return;
        }

        let drag_offset_ms = self.interaction.drag_origin.map_or(0.0, |origin| {
            (self.render.uniforms.mouse_pos.x - origin.x) / px_per_ms
        });
        let cur_idx = playback_state
            .queue_index
            .min(playback_state.queue.len() - 1);

        if self.interaction.dragging {
            self.interaction.drag_track = None;
        }

        // Lerp the progress based on when the data was last updated, get the start time of the current track
        let playback_elapsed = playback_state.estimated_progress();

        let current_queue_offset = playback_state.queue[..cur_idx]
            .iter()
            .map(Track::queue_span_ms)
            .sum::<f32>();
        let mut current_ms = -playback_elapsed - current_queue_offset + drag_offset_ms;
        let diff = current_ms - self.render.track_offset;
        self.render.uniforms.expansion_xy.x += diff * px_per_ms * dt;
        if !self.render.uniforms.expansion_xy.x.is_finite() {
            self.render.uniforms.expansion_xy.x = playhead_x;
        }
        if !self.interaction.dragging && diff.abs() > 200.0 {
            current_ms = self.render.track_offset + diff * 3.5 * dt;
        }

        self.render.movement_speed = self.render.movement_speed.lerp(
            (current_ms - self.render.track_offset) * dt,
            (dt * 10.0).min(1.0),
        );
        self.render.track_offset = current_ms;

        layout_tracks(&mut playback_state.queue, &self.config, current_ms);

        self.render.uniforms.time = self.render.start_time.elapsed().as_secs_f32();
        let (screen_width, screen_height) = self.logical_surface_size();
        self.render.uniforms.screen_size = vec2(screen_width, screen_height);
        self.render.uniforms.bar_height = vec2(PANEL_START, self.config.height);
        self.render.uniforms.playhead_x = playhead_x;

        approach(
            &mut self.render.uniforms.mouse_pressure,
            self.interaction.mouse_pressure,
            5.0 * dt,
        );

        let hovered_track = (!self.interaction.dragging && self.interaction.mouse_pressure > 0.0)
            .then(|| self.hovered_track(&playback_state.queue))
            .flatten();
        self.interaction.hovered_track = hovered_track;

        self.render.pills.clear();
        let current_track = playback_state.queue.iter().position(Track::is_current);
        let playlists = &playback_state.playlists;
        let render_order = (0..playback_state.queue.len())
            .filter(|&index| Some(index) != hovered_track)
            .chain(hovered_track);
        for queue_index in render_order {
            if self.render.pills.len() == MAX_RENDER_INSTANCES {
                break;
            }
            let track = &mut playback_state.queue[queue_index];
            if track.runtime.rect(self.config.height).is_some() {
                let hovered = Some(queue_index) == hovered_track;
                self.draw_track(track, playhead_x, hovered, dt, playlists);
            }
        }

        self.render_playhead_particles(
            dt,
            &playback_state.queue[current_track.unwrap_or(cur_idx)],
            playhead_x,
            self.render.movement_speed,
            playback_state.playing,
        );
        self.playback = playback_state;
    }

    fn hovered_track(&self, queue: &[Track]) -> Option<usize> {
        let mouse_pos = self.render.uniforms.mouse_pos;
        let in_track = |track: &Track| {
            track
                .runtime
                .rect(self.config.height)
                .is_some_and(|rect| rect.contains(mouse_pos))
                || self
                    .icon_row_rects(track)
                    .into_iter()
                    .flatten()
                    .any(|rect| rect.contains(mouse_pos))
        };

        if let Some(index) = self.interaction.hovered_track
            && queue.get(index).is_some_and(in_track)
        {
            return Some(index);
        }

        queue
            .iter()
            .enumerate()
            .rev()
            .find(|(_, track)| in_track(track))
            .map(|(index, _)| index)
    }

    fn draw_track(
        &mut self,
        track: &mut Track,
        origin_x: f32,
        hovered: bool,
        dt: f32,
        playlists: &[CondensedPlaylist],
    ) {
        let width = track.runtime.width;
        let start_x = track.runtime.start_x;
        // If dragging, set the drag target to this track, and the position within the track
        if self.interaction.dragging && track.is_current() {
            let (hit_start, hit_end) = track.natural_x_range(origin_x, self.config.px_per_ms());
            self.interaction.drag_track = Some((
                track.id,
                (origin_x.max(start_x) - hit_start) / (hit_end - hit_start),
            ));
        }

        let image_index = self.get_image_index(&track.art);
        let colors = track.palette();
        let show_details = width > self.config.height;
        approach(
            &mut track.runtime.detail_alpha,
            flag(width >= self.config.height),
            dt / DETAIL_FADE_DURATION,
        );
        let detail_alpha = track.runtime.detail_alpha;
        approach(
            &mut track.runtime.playlist_expansion,
            flag(hovered && show_details && detail_alpha >= 1.0),
            dt.min(0.1) * 6.0,
        );
        let playlist_expansion = track.runtime.playlist_expansion;
        let audio_features = track
            .audio_features
            .map_or(DEFAULT_AUDIO_FEATURES, AudioFeatures::packed);
        let mut pill = BackgroundPill {
            x: start_x,
            width,
            colors,
            alpha: detail_alpha,
            image_index,
            rating: -1,
            audio_features,
            playlist_images: [-1; MAX_PILL_PLAYLIST_ICONS],
            ..Default::default()
        };

        if show_details
            && detail_alpha > 0.0
            && let Some(gpu) = &mut self.render.gpu
        {
            gpu.text_renderer
                .render(&gpu.queue, track, detail_alpha, self.render.scale);
        }

        // Expand the hitbox vertically so it includes the playlist buttons
        if show_details {
            self.populate_playlist_buttons(track, playlist_expansion, playlists, &mut pill);
        }
        if hovered
            && pill.rating >= 0
            && let Some((index, right_half)) = pill
                .icon_rows(PANEL_START, self.config.height)
                .0
                .hit(self.render.uniforms.mouse_pos)
            && index < 5
        {
            pill.rating = index as i32 * 2 + 1 + i32::from(right_half);
        }
        track.runtime.primary_playlist_count = pill.primary_playlist_count as u8;
        track.runtime.secondary_playlist_count = pill.secondary_playlist_count as u8;
        let primary_icons = pill.star_count() + pill.primary_playlist_count as f32;
        approach(
            &mut track.runtime.primary_icon_alpha,
            flag(
                primary_icons > 0.0
                    && (playlist_expansion > 0.0 || width >= ICON_SPACING * 1.05 * primary_icons),
            ),
            dt / DETAIL_FADE_DURATION,
        );
        pill.primary_alpha = track.runtime.primary_icon_alpha;
        pill.secondary_expansion = playlist_expansion;
        self.render.pills.push(pill);
    }

    fn populate_playlist_buttons(
        &mut self,
        track: &Track,
        secondary_expansion: f32,
        playlists: &[CondensedPlaylist],
        pill: &mut BackgroundPill,
    ) {
        let Some(track_id) = track.id else {
            return;
        };
        let icons = playlist_icons(track_id, playlists, true)
            .chain(playlist_icons(track_id, playlists, false))
            .take(MAX_PILL_PLAYLIST_ICONS)
            .collect::<ArrayVec<_, MAX_PILL_PLAYLIST_ICONS>>();
        let primary_count = icons.partition_point(|playlist| playlist.tracks.contains(&track_id));
        let visible_count = if secondary_expansion > 0.0 {
            icons.len()
        } else {
            primary_count
        };
        for (slot, playlist) in pill.playlist_images.iter_mut().zip(&icons[..visible_count]) {
            *slot = self.get_image_index(&playlist.art);
        }

        pill.rating = if self.config.ratings_enabled {
            i32::from(
                playlists
                    .iter()
                    .find_map(|playlist| {
                        playlist
                            .rating_index
                            .filter(|_| playlist.tracks.contains(&track_id))
                    })
                    .map_or(0, |rating| rating + 1),
            )
        } else {
            -1
        };
        pill.primary_playlist_count = primary_count as u32;
        pill.secondary_playlist_count = (visible_count - primary_count) as u32;
    }

    fn render_playhead_particles(
        &mut self,
        dt: f32,
        track: &Track,
        playhead_x: f32,
        avg_speed: f32,
        playing: bool,
    ) {
        let palette = track.palette();

        // Emit new particles while playing
        let emit_count = if avg_speed.abs() > 0.00001 {
            self.render.particles_accumulator += dt * SPARK_EMISSION;
            let count = self.render.particles_accumulator.floor() as u8;
            self.render.particles_accumulator -= f32::from(count);
            count
        } else {
            self.render.particles_accumulator = 0.0;
            0
        };
        let horizontal_bias = (avg_speed.abs().powf(0.2) * avg_speed.signum()).clamp(-3.0, 3.0);
        let time = self.render.uniforms.time;

        for particle in self
            .render
            .particles
            .iter_mut()
            .filter(|particle| time > particle.end_time)
            .take(emit_count as usize)
        {
            let y_fraction = fastrand::f32();

            particle.spawn_pos = vec2(
                playhead_x,
                PANEL_START + self.config.height * y_fraction.remap(0.0, 1.0, 0.1, 0.95),
            );
            particle.spawn_vel = vec2(
                fastrand::usize(SPARK_VELOCITY_X) as f32 * horizontal_bias,
                (y_fraction - 0.5) * 2.0 * SPARK_VELOCITY_Y,
            );
            let duration = SPARK_LIFETIME
                .start
                .lerp(SPARK_LIFETIME.end, fastrand::f32());
            let packed_duration = (duration * 100.0).min(255.0) as u8;
            let base_color = palette[fastrand::usize(0..palette.len())];
            particle.color = (base_color & 0x00FF_FFFF) | (u32::from(packed_duration) << 24);
            particle.end_time = time + duration;
        }

        let speed = PLAYHEAD_TRANSITION_SPEED * dt;
        let playhead_hovered = self
            .playhead_rect()
            .contains(self.render.uniforms.mouse_pos)
            && self.interaction.mouse_pressure > 0.0;
        let last_toggle =
            self.render.last_toggle_playing.elapsed().as_secs_f32() / PLAYHEAD_START_DURATION;

        let play_intro_active = !playhead_hovered && playing && last_toggle < 1.0;
        if play_intro_active {
            self.render.playhead.bar_split = 1.0 - last_toggle;
            self.render.playhead.icon_presence = 1.0 - last_toggle;
            approach(&mut self.render.playhead.icon_morph, 1.0, speed * 1.5);
        } else {
            let show_icon = flag(playhead_hovered || !playing);
            let play_icon = flag(playhead_hovered && !playing);
            approach(&mut self.render.playhead.bar_split, show_icon, speed);
            if show_icon > self.render.playhead.icon_presence {
                self.render.playhead.icon_presence = show_icon;
            }
            approach(&mut self.render.playhead.icon_presence, show_icon, speed);
            approach(&mut self.render.playhead.icon_morph, play_icon, speed);
        }
    }
}

use crate::{
    CantusApp, MAX_RENDER_INSTANCES, PANEL_EXTENSION, PANEL_OVERFLOW, PANEL_START, PARTICLE_COUNT,
    interaction::{InteractionState, Rect, TrackAction},
    spotify::Track,
};
use art::{AlbumArt, ArtState};
use cantus_shared::{
    GAP, GlobalUniforms, Particle, PlayheadUniforms, RipplePulse,
    status::{StatusPill, WIDTH as STATUS_WIDTH},
    tempo::{WIDTH as TEMPO_WIDTH, WeatherPill, pill_x as tempo_pill_x, sun_position},
    track::{ICON_SPACING, TrackPill},
};
use glam::{FloatExt, Vec2, vec2};
use pipelines::{IMAGE_SIZE, MAX_TEXTURE_IMAGES, write_texture_region};
use std::{
    f32::consts::TAU,
    mem,
    ops::Range,
    slice,
    sync::{Arc, Weak},
    time::Instant,
};
use text::{TextRenderer, TextStyle};
use wgpu::{
    BindGroup, Buffer, Color, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, Instance, LoadOp,
    Operations, Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    StoreOp, Surface, SurfaceConfiguration, Texture, TextureViewDescriptor,
};

fn approach(current: &mut f32, target: f32, speed: f32) {
    *current += (target - *current).clamp(-speed, speed);
}

pub mod art;
pub mod pipelines;
pub mod status;
pub mod tempo;
pub mod text;
pub mod track;

const SPARK_EMISSION: f32 = 20.0;
const SPARK_VELOCITY_X: Range<usize> = 40..60;
const SPARK_VELOCITY_Y: f32 = 5.0;
const SPARK_LIFETIME: Range<f32> = 1.2..1.5;

const PLAYHEAD_START_DURATION: f32 = 0.7;
const PLAYHEAD_TRANSITION_SPEED: f32 = 5.5;

#[derive(Clone, Copy)]
pub struct Timeline {
    pub px_per_ms: f32,
    pub playhead_x: f32,
}

pub struct GpuResources {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub globals: Buffer,
    pub playhead: GpuPass,
    pub track: GpuPass,
    pub weather: Option<GpuPass>,
    pub status: Option<GpuPass>,
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
    fn draw_range<'pass>(&'pass self, pass: &mut RenderPass<'pass>, instances: Range<u32>) {
        if instances.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..4, instances);
    }

    fn upload_data<T: bytemuck::NoUninit>(&self, queue: &Queue, data: &[T]) {
        if !data.is_empty() {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
        }
    }
}

pub struct ImageAtlas {
    pub texture: Texture,
    pub slots: [Weak<AlbumArt>; MAX_TEXTURE_IMAGES as usize],
    pub used: u32,
}

impl ImageAtlas {
    fn image_index(&mut self, queue: &Queue, art: &Arc<AlbumArt>) -> i32 {
        if let Some(index) = self
            .slots
            .iter()
            .position(|slot| slot.as_ptr() == Arc::as_ptr(art))
        {
            self.used |= 1 << index;
            return index as i32;
        }

        let index = (!self.used).trailing_zeros();
        if index >= MAX_TEXTURE_IMAGES {
            return -1;
        }
        self.used |= 1 << index;
        write_texture_region(queue, &self.texture, [0, 0, index], [IMAGE_SIZE; 2], &art.pixels);
        self.slots[index as usize] = Arc::downgrade(art);
        index as i32
    }
}

pub struct RenderState {
    pub instance: Instance,
    pub gpu: Option<GpuResources>,
    pub start_time: Instant,
    last_update: Instant,
    track_offset: f32,
    movement_speed: f32,
    pub last_toggle_playing: Instant,
    particles: [Particle; PARTICLE_COUNT],
    particles_accumulator: f32,
    /// Physical buffer pixels per logical Wayland surface pixel.
    pub scale: f32,
    pub surface_width: Option<f32>,
    pub uniforms: GlobalUniforms,
    track_pills: Vec<TrackPill>,
    track_glyphs: Vec<Range<u32>>,
    pub status: StatusPill,
    playhead: PlayheadUniforms,
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
            track_pills: Vec::with_capacity(MAX_RENDER_INSTANCES),
            track_glyphs: Vec::with_capacity(MAX_RENDER_INSTANCES),
            status: StatusPill::default(),
            playhead: PlayheadUniforms::default(),
        }
    }
}

impl RenderState {
    fn expired_particles(&mut self, time: f32) -> impl Iterator<Item = &mut Particle> {
        self.particles
            .iter_mut()
            .filter(move |particle| time > particle.end_time)
    }

    /// The GPU device and its dependent resources, valid once the Wayland surface is configured.
    const fn gpu(&mut self) -> &mut GpuResources {
        self.gpu.as_mut().expect("render called before gpu configured")
    }
}

fn particle_color(rgb: u32, duration: f32) -> u32 {
    (rgb & 0x00FF_FFFF) | (u32::from((duration * 100.0).min(255.0) as u8) << 24)
}

impl CantusApp {
    fn finish_interaction(&mut self, mut interaction: InteractionState) {
        let pulse = interaction.end_frame();
        self.interaction = interaction;
        if let Some(origin) = pulse {
            let globals = &mut self.render.uniforms;
            let ripple = globals
                .ripples
                .iter_mut()
                .min_by(|a, b| a.animation.x.total_cmp(&b.animation.x))
                .unwrap();
            *ripple = RipplePulse {
                origin,
                animation: vec2(globals.time, 1.0),
            };
        }
    }

    pub fn timeline(&self) -> Timeline {
        let config = &self.config;
        let mut reserved = config.history_width + GAP;
        if config.weather_enabled {
            reserved += TEMPO_WIDTH + GAP;
        }
        if config.status_enabled {
            reserved += STATUS_WIDTH + GAP;
        }
        let px_per_ms = (self.logical_surface_size().0 - reserved).max(84.0)
            / (config.timeline_future_minutes * 60_000.0);
        Timeline {
            px_per_ms,
            playhead_x: config.history_width + config.timeline_past_minutes * 60_000.0 * px_per_ms,
        }
    }

    pub fn logical_surface_size(&self) -> (f32, f32) {
        let extension = if self.config.weather_enabled {
            PANEL_EXTENSION
        } else {
            PANEL_OVERFLOW
        };
        (
            self.render.surface_width.unwrap_or(1920.0),
            self.config.height + PANEL_START + extension,
        )
    }

    pub fn buffer_size(&self) -> (u32, u32) {
        let (width, height) = self.logical_surface_size();
        (
            (width * self.render.scale).round() as u32,
            (height * self.render.scale).round() as u32,
        )
    }

    pub fn render(&mut self) {
        while let Ok(update) = self.app_updates.try_recv() {
            update(self);
        }
        self.start_missing_art_downloads();

        let Some(gpu) = self.render.gpu.as_mut() else {
            return;
        };
        let (surface_texture, reconfigure_after_present) = match gpu.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => (texture, false),
            CurrentSurfaceTexture::Suboptimal(texture) => (texture, true),
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return,
            CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                gpu.surface.configure(&gpu.device, &gpu.surface_config);
                return;
            }
            CurrentSurfaceTexture::Validation => {
                tracing::error!("surface texture acquisition failed validation");
                return;
            }
        };

        gpu.images.used = 0;
        gpu.text_renderer.glyphs.clear();
        self.render.track_glyphs.clear();
        let (weather, weather_glyph_end) = self.create_scene();

        let gpu = self.render.gpu.as_mut().unwrap();
        gpu.queue
            .write_buffer(&gpu.globals, 0, bytemuck::bytes_of(&self.render.uniforms));
        gpu.particles.upload_data(&gpu.queue, &self.render.particles);
        gpu.playhead
            .upload_data(&gpu.queue, slice::from_ref(&self.render.playhead));
        gpu.track.upload_data(&gpu.queue, &self.render.track_pills);
        gpu.text.upload_data(&gpu.queue, &gpu.text_renderer.glyphs);
        if let (Some(pass), Some(weather)) = (&gpu.weather, weather.as_ref()) {
            pass.upload_data(&gpu.queue, slice::from_ref(weather));
        }
        if let Some(pass) = &gpu.status {
            pass.upload_data(&gpu.queue, slice::from_ref(&self.render.status));
        }

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
            if let Some(weather) = &gpu.weather {
                weather.draw_range(&mut pass, 0..1);
            }
            gpu.text.draw_range(&mut pass, 0..weather_glyph_end);
            for (index, glyphs) in self.render.track_glyphs.iter().enumerate() {
                let index = index as u32;
                gpu.track.draw_range(&mut pass, index..index + 1);
                gpu.text.draw_range(&mut pass, glyphs.clone());
            }
            if let Some(status) = &gpu.status {
                status.draw_range(&mut pass, 0..1);
            }
            gpu.particles.draw_range(&mut pass, 0..PARTICLE_COUNT as u32);
            gpu.playhead.draw_range(&mut pass, 0..1);
        }

        gpu.queue.submit([encoder.finish()]);
        gpu.queue.present(surface_texture);
        if reconfigure_after_present {
            gpu.surface.configure(&gpu.device, &gpu.surface_config);
        }
    }

    fn get_image_index(&mut self, art: &ArtState) -> i32 {
        let Some(art) = art.ready() else {
            return -1;
        };
        let gpu = self.render.gpu();
        gpu.images.image_index(&gpu.queue, art)
    }

    fn create_scene(&mut self) -> (Option<WeatherPill>, u32) {
        let dt = self.render.last_update.elapsed().as_secs_f32().min(0.1);
        self.render.last_update = Instant::now();

        self.render.uniforms.time = self.render.start_time.elapsed().as_secs_f32();
        let (screen_width, screen_height) = self.logical_surface_size();
        self.render.uniforms.screen_size = vec2(screen_width, screen_height);
        self.render.uniforms.bar_height = vec2(PANEL_START, self.config.height);
        let mouse = self.render.uniforms.mouse_pos;
        let mut ui = mem::take(&mut self.interaction);
        ui.begin_frame(mouse);
        self.render.status = self.status.as_mut().map_or_else(StatusPill::default, |status| {
            status.damp_readings(dt);
            status.pill(self.render.uniforms.time)
        });

        let weather = if let Some(weather_state) = &mut self.weather {
            let (weather, weather_label, hour) =
                weather_state.scene(screen_width, self.config.height, &ui, dt);
            self.render.uniforms.weather_hour = hour;
            self.render.status.sun_height = sun_position(hour, weather.sun_hours)[1];
            self.render.status.conditions = weather.hourly[0].values();
            let scale = self.render.scale;
            let label_y = PANEL_START + self.config.height * 0.46;
            let gpu = self.render.gpu.as_mut().unwrap();
            gpu.text_renderer.render_centered_label(
                &gpu.queue,
                &weather_label,
                vec2(tempo_pill_x(screen_width) + TEMPO_WIDTH * 0.5, label_y),
                TextStyle::WEATHER,
                1.0,
                scale,
            );
            weather_state.calendar_labels(
                screen_width,
                self.config.height,
                &mut ui,
                |text, position, alpha, style| {
                    gpu.text_renderer
                        .render_centered_label(&gpu.queue, text, position, style, alpha, scale);
                },
            );
            Some(weather)
        } else {
            None
        };
        if let Some(status) = &mut self.status {
            status.interact(&mut self.render.status, screen_width, self.config.height, &mut ui);
        }
        let weather_glyph_end = self.render.gpu.as_ref().unwrap().text_renderer.glyphs.len() as u32;

        let timeline = self.timeline();
        self.render.uniforms.playhead_x = timeline.playhead_x;

        let playhead = ui.interact(
            vec2(timeline.playhead_x, PANEL_START + self.config.height * 0.5),
            vec2(self.config.height * 0.25, self.config.height * 0.5),
        );
        if playhead.clicked {
            self.toggle_playing();
        }
        let volume_rect = Rect::new(
            timeline.playhead_x - 100.0,
            PANEL_START,
            timeline.playhead_x + 100.0,
            PANEL_START + self.config.height,
        );
        let volume_scroll = ui.scroll_surface(volume_rect);
        if volume_scroll != 0 {
            self.adjust_playback_volume(volume_scroll);
        }
        if ui.released() {
            if ui.dragging
                && let Some(track) = self.playback.queue.iter().find(|track| track.is_current())
                && let Some(track_id) = track.id
            {
                let (start, end) = track.natural_x_range(timeline.playhead_x, timeline.px_per_ms);
                let position = (timeline.playhead_x.max(track.runtime.start_x) - start) / (end - start);
                self.handle_track_action(TrackAction::Seek(track_id, position));
            }
            ui.cancel_drag();
        }

        let mut playback_state = mem::take(&mut self.playback);
        if playback_state.queue.is_empty() {
            self.render.track_pills.clear();
            self.playback = playback_state;
            self.finish_interaction(ui);
            return (weather, weather_glyph_end);
        }
        // Lerp the progress based on when the data was last updated.
        let playback_elapsed = playback_state.estimated_progress();
        let queue = &mut playback_state.queue;

        let drag_offset_ms = if ui.dragging {
            (self.render.uniforms.mouse_pos.x - ui.press_origin.x) / timeline.px_per_ms
        } else {
            0.0
        };
        let cur_idx = playback_state.queue_index.min(queue.len() - 1);

        let mut current_ms = -playback_elapsed
            - queue[..cur_idx].iter().map(Track::queue_span_ms).sum::<f32>()
            + drag_offset_ms;
        let diff = current_ms - self.render.track_offset;
        if !ui.dragging && diff.abs() > 200.0 {
            current_ms = self.render.track_offset + diff * 3.5 * dt;
        }

        self.render.movement_speed = self
            .render
            .movement_speed
            .lerp((current_ms - self.render.track_offset) * dt, (dt * 10.0).min(1.0));
        self.render.track_offset = current_ms;

        track::layout(queue, &self.config, timeline, current_ms);
        let gpu = self.render.gpu();
        for track in queue.iter_mut() {
            let primary_icon_count = usize::from(self.config.ratings_enabled) * 5
                + track.runtime.primary_playlist_count as usize;
            track.runtime.primary_icons_fit = primary_icon_count > 0
                && track.runtime.width >= ICON_SPACING * 1.05 * primary_icon_count as f32;
            let expansion = track.runtime.playlist_expansion_curve();
            if expansion > 0.0 {
                let extra_width =
                    (gpu.text_renderer.track_width(track) - track.runtime.width).max(0.0) * expansion;
                track.runtime.start_x -= extra_width * 0.5;
                track.runtime.width += extra_width;
            }
        }

        approach(
            &mut self.render.uniforms.mouse_pressure,
            ui.mouse_pressure,
            5.0 * dt,
        );

        let playlists = &playback_state.playlists;
        self.render.track_pills.clear();
        self.render.track_glyphs.clear();
        let mut foreground = None;
        let mut track_action = None;
        if ui.dragging {
            ui.hover_claimed = true;
        }
        for queue_index in (0..queue.len()).rev() {
            if self.render.track_pills.len() + usize::from(foreground.is_some()) == MAX_RENDER_INSTANCES
            {
                break;
            }
            let track = &mut queue[queue_index];
            if track.runtime.width > 0.0 && track.runtime.start_x + track.runtime.width > 0.0 {
                let (pill, glyphs, hovered, action) =
                    self.draw_track(track, dt, playlists, timeline, &mut ui);
                track_action = action.or(track_action);
                if hovered {
                    foreground = Some((pill, glyphs));
                } else {
                    self.render.track_pills.insert(0, pill);
                    self.render.track_glyphs.insert(0, glyphs);
                }
            }
        }
        if let Some((pill, glyphs)) = foreground {
            self.render.track_pills.push(pill);
            self.render.track_glyphs.push(glyphs);
        }

        self.render_playhead_particles(
            dt,
            &queue[queue.iter().position(Track::is_current).unwrap_or(cur_idx)],
            playback_state.playing,
            playhead.hovered,
        );
        self.playback = playback_state;
        if let Some(action) = track_action {
            if matches!(action, TrackAction::Rate(..) | TrackAction::TogglePlaylist(..)) {
                let time = self.render.uniforms.time;
                for particle in self.render.expired_particles(time).take(20) {
                    let duration = 0.5.lerp(1.5, fastrand::f32());
                    particle.spawn_pos = ui.pointer();
                    particle.spawn_vel =
                        Vec2::from_angle(fastrand::f32() * TAU) * (30.0 + fastrand::f32() * 20.0);
                    particle.color = particle_color(0x32_D7_FF, duration);
                    particle.end_time = time + duration;
                }
            }
            self.handle_track_action(action);
        }
        self.finish_interaction(ui);
        (weather, weather_glyph_end)
    }

    fn render_playhead_particles(
        &mut self,
        dt: f32,
        track: &Track,
        playing: bool,
        playhead_hovered: bool,
    ) {
        let palette = track.runtime.art.palette();
        let playhead_x = self.render.uniforms.playhead_x;
        let avg_speed = self.render.movement_speed;

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

        for particle in self.render.expired_particles(time).take(emit_count as usize) {
            let y_fraction = fastrand::f32();

            particle.spawn_pos = vec2(
                playhead_x,
                PANEL_START + self.config.height * y_fraction.remap(0.0, 1.0, 0.1, 0.95),
            );
            particle.spawn_vel = vec2(
                fastrand::usize(SPARK_VELOCITY_X) as f32 * horizontal_bias,
                (y_fraction - 0.5) * 2.0 * SPARK_VELOCITY_Y,
            );
            let duration = SPARK_LIFETIME.start.lerp(SPARK_LIFETIME.end, fastrand::f32());
            particle.color = particle_color(palette[fastrand::usize(0..palette.len())], duration);
            particle.end_time = time + duration;
        }

        let speed = PLAYHEAD_TRANSITION_SPEED * dt;
        let last_toggle =
            self.render.last_toggle_playing.elapsed().as_secs_f32() / PLAYHEAD_START_DURATION;

        if !playhead_hovered && playing && last_toggle < 1.0 {
            self.render.playhead.bar_split = 1.0 - last_toggle;
            self.render.playhead.icon_presence = 1.0 - last_toggle;
            approach(&mut self.render.playhead.icon_morph, 1.0, speed * 1.5);
        } else {
            let show_icon = f32::from(playhead_hovered || !playing);
            let play_icon = f32::from(playhead_hovered && !playing);
            approach(&mut self.render.playhead.bar_split, show_icon, speed);
            if show_icon > self.render.playhead.icon_presence {
                self.render.playhead.icon_presence = show_icon;
            }
            approach(&mut self.render.playhead.icon_presence, show_icon, speed);
            approach(&mut self.render.playhead.icon_morph, play_icon, speed);
        }
    }
}

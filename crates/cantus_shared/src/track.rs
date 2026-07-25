use glam::Vec2;

/// Maximum number of playlist artwork icons carried by one pill instance.
pub const MAX_PILL_PLAYLIST_ICONS: usize = 8;

/// Visual width, in pixels, of rating and playlist icons before hover growth.
pub const ICON_WIDTH: f32 = 21.6;

/// Center-to-center icon spacing for rating stars and playlist artwork.
pub const ICON_SPACING: f32 = 18.0;

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct TrackPill {
    pub x: f32,
    pub width: f32,
    pub colors: [u32; 4],
    pub visibility: f32,
    pub image_index: i32,
    pub rating: i32,
    pub primary_playlist_count: u32,
    pub secondary_playlist_count: u32,
    pub primary_alpha: f32,
    pub secondary_expansion: f32,
    pub audio_features: AudioFeatures,
    pub playlist_images: [i32; MAX_PILL_PLAYLIST_ICONS],
}

impl TrackPill {
    pub const fn star_count(&self) -> f32 {
        if self.rating >= 0 { 5.0 } else { 0.0 }
    }

    pub fn icon_rows(&self, bar_start_y: f32, bar_height: f32) -> (PillIconRow, PillIconRow) {
        pill_icon_rows(
            self.x + self.width * 0.5,
            pill_icon_primary_center_y(bar_start_y, bar_height),
            self.star_count() + self.primary_playlist_count as f32,
            self.secondary_playlist_count as f32,
            self.secondary_expansion,
        )
    }
}

pub fn pill_icon_primary_center_y(bar_start_y: f32, bar_height: f32) -> f32 {
    bar_start_y + bar_height * 0.975 - 3.0
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(
    feature = "cpu",
    derive(bytemuck::Pod, bytemuck::Zeroable, serde::Deserialize)
)]
pub struct AudioFeatures {
    pub energy: f32,
    pub danceability: f32,
    pub acousticness: f32,
    pub tempo: f32,
    pub valence: f32,
    pub instrumentalness: f32,
    pub loudness: f32,
}

#[derive(Copy, Clone)]
pub struct PillIconRow {
    pub center: Vec2,
    pub count: f32,
    pub expansion: f32,
}

impl PillIconRow {
    pub fn hit(self, point: Vec2) -> Option<(usize, bool)> {
        if self.expansion <= 0.0 {
            return None;
        }
        let index = (point.x - self.center.x) / (ICON_SPACING * self.expansion)
            + (self.count - 1.0).max(0.0) * 0.5
            + 0.5;
        if !(0.0..self.count).contains(&index) {
            return None;
        }
        let index = index as usize;
        let center = self.icon_center(index as f32);
        let delta = (point - center).abs();
        (delta.x <= ICON_WIDTH * 0.5 && delta.y <= ICON_WIDTH * 0.5)
            .then_some((index, point.x >= center.x))
    }

    pub fn half_span(self) -> f32 {
        (self.count - 1.0).max(0.0) * ICON_SPACING * self.expansion * 0.5
    }

    pub fn half_size(self, radius: f32) -> Vec2 {
        Vec2::new(self.half_span() + radius, radius)
    }

    pub fn backplate_center(self) -> Vec2 {
        self.center + Vec2::new(0.0, -ICON_WIDTH * 0.25)
    }

    pub fn icon_center(self, index: f32) -> Vec2 {
        let row_center = (self.count - 1.0).max(0.0) * 0.5;
        Vec2::new(
            self.center.x + (index - row_center) * ICON_SPACING * self.expansion,
            self.center.y + 2.0,
        )
    }
}

pub fn pill_icon_rows(
    center_x: f32,
    primary_center_y: f32,
    primary_count: f32,
    secondary_count: f32,
    secondary_expansion: f32,
) -> (PillIconRow, PillIconRow) {
    (
        PillIconRow {
            center: Vec2::new(center_x, primary_center_y),
            count: primary_count,
            expansion: 1.0,
        },
        PillIconRow {
            center: Vec2::new(
                center_x,
                primary_center_y + ICON_SPACING * secondary_expansion,
            ),
            count: secondary_count,
            expansion: secondary_expansion,
        },
    )
}

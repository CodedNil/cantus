use crate::{CantusLayer, spotify};
use rspotify::model::{PlaylistId, TrackId};
use std::time::{Duration, Instant};
use vello::kurbo::{Point, Rect};
use wayland_client::QueueHandle;

#[derive(Clone, Debug)]
pub struct IconHitbox {
    pub rect: Rect,
    pub track_id: TrackId<'static>,
    pub playlist_id: Option<PlaylistId<'static>>,
    pub rating_index: Option<usize>,
}

impl CantusLayer {
    /// Handle pointer click events.
    pub fn handle_pointer_click(&self) -> bool {
        let point = Point::new(self.pointer_position.0, self.pointer_position.1);
        if let Some(hitbox) = self
            .icon_hitboxes
            .iter()
            .find(|hitbox| hitbox.rect.contains(point))
        {
            println!(
                "Clicked button for track {:?}, playlist {:?}, rating index {:?})",
                hitbox.track_id, hitbox.playlist_id, hitbox.rating_index
            );
            return true;
        }
        if let Some((id, rect)) = self
            .track_hitboxes
            .iter()
            .find(|(_, rect)| rect.contains(point))
        {
            let id = id.clone();
            let rect = *rect;
            tokio::spawn(async move {
                spotify::skip_to_track(id, point, rect).await;
            });
            return true;
        }
        false
    }

    /// Update the input region for the surface.
    pub fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        if self.last_hitbox_update.elapsed() <= Duration::from_millis(500) {
            return;
        }

        let (Some(wl_surface), Some(compositor)) = (&self.wl_surface, &self.compositor) else {
            return;
        };

        let region = compositor.create_region(qhandle, ());
        for rect in self.track_hitboxes.values() {
            region.add(
                rect.x0.round() as i32,
                rect.y0.round() as i32,
                (rect.x1 - rect.x0).round() as i32,
                (rect.y1 - rect.y0).round() as i32,
            );
        }
        for hitbox in &self.icon_hitboxes {
            let rect = &hitbox.rect;
            region.add(
                rect.x0.round() as i32,
                rect.y0.round() as i32,
                (rect.x1 - rect.x0).round() as i32,
                (rect.y1 - rect.y0).round() as i32,
            );
        }

        wl_surface.set_input_region(Some(&region));
        wl_surface.commit();
        self.last_hitbox_update = Instant::now();
    }
}

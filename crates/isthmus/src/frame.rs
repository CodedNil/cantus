use crate::{
    Image,
    backend::canvas::Canvas,
    contract::{PushBlock, Quad, ShaderSpec, SurfaceHandle},
    data::ImageHandle,
    text::{Line, Text, TextScope},
};
use core::ops::{Deref, DerefMut};

pub struct Frame<'a> {
    pub time: f32,
    pub screen_size: glam::Vec2,
    pub delta_time: f32,
    text: &'a mut Text,
    canvas: &'a mut Canvas,
    surface: SurfaceHandle,
}

impl<'a> Frame<'a> {
    pub(crate) const fn new(push: &'a PushBlock, time: f32, delta_time: f32, text: &'a mut Text, canvas: &'a mut Canvas, surface: SurfaceHandle) -> Self {
        Self {
            time,
            screen_size: push.screen_size,
            delta_time,
            text,
            canvas,
            surface,
        }
    }

    /// Sets app-defined shader data shared by every paint on this surface frame.
    pub fn set_globals<T: crate::ShaderData>(&mut self, globals: T) {
        self.canvas.set_globals(self.surface, globals);
    }

    /// Starts a bounded group whose layers are composited before later paints.
    pub fn group(&mut self) -> PaintGroup<'_, 'a> {
        self.canvas.begin_group(self.surface);
        PaintGroup { frame: self }
    }

    #[doc(hidden)]
    pub fn __image(&mut self, image: &Image) -> ImageHandle {
        self.canvas.image(image.size(), image.pixels())
    }

    pub const fn text(&mut self) -> TextScope<'_> {
        TextScope::new(self.text)
    }

    /// Paints a quad with an inline Rust-GPU fragment shader.
    ///
    /// The inline closure receives [`crate::Fragment`], followed by explicitly typed
    /// values inferred from its surrounding host function.
    ///
    pub fn paint_quad<S, Geometry, Payload>(&mut self, geometry: Geometry, payload: Payload)
    where
        S: ShaderSpec,
        Geometry: Copy + Into<Quad>,
        Payload: FnOnce(&mut Self, Geometry) -> S::Instance,
    {
        let value = payload(self, geometry);
        self.canvas.emit::<S>(self.surface, geometry.into(), value);
    }

    /// Paints shaped text with an inline Rust-GPU fragment shader.
    ///
    /// The closure inputs match [`Self::paint_quad`], starting with [`crate::TextFragment`].
    ///
    pub fn paint_text<S, Payload>(&mut self, line: Line, payload: Payload)
    where
        S: ShaderSpec,
        Payload: FnOnce(&mut Self, Line) -> S::Instance,
    {
        let value = payload(self, line);
        self.canvas.emit::<S>(self.surface, line.quad(), value);
    }
}

/// A bounded set of sibling paints that may be ordered into layers.
pub struct PaintGroup<'frame, 'canvas> {
    frame: &'frame mut Frame<'canvas>,
}

impl<'canvas> PaintGroup<'_, 'canvas> {
    /// Paints through the front or ordinary layer of this group.
    pub fn front(&mut self, front: bool) -> PaintLayer<'_, 'canvas> {
        PaintLayer {
            frame: self.frame,
            layer: u8::from(front),
        }
    }

    /// Finishes the group, returning subsequent paints to immediate order.
    pub fn finish(self) {
        drop(self);
    }
}

impl<'canvas> Deref for PaintGroup<'_, 'canvas> {
    type Target = Frame<'canvas>;

    fn deref(&self) -> &Self::Target {
        self.frame
    }
}

impl DerefMut for PaintGroup<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.frame
    }
}

impl Drop for PaintGroup<'_, '_> {
    fn drop(&mut self) {
        self.frame.canvas.end_group();
    }
}

/// Paint access fixed to one layer of a [`PaintGroup`].
pub struct PaintLayer<'frame, 'canvas> {
    frame: &'frame mut Frame<'canvas>,
    layer: u8,
}

impl<'canvas> PaintLayer<'_, 'canvas> {
    pub fn paint_quad<S, Geometry, Payload>(&mut self, geometry: Geometry, payload: Payload)
    where
        S: ShaderSpec,
        Geometry: Copy + Into<Quad>,
        Payload: FnOnce(&mut Frame<'canvas>, Geometry) -> S::Instance,
    {
        let value = payload(self.frame, geometry);
        self.frame.canvas.emit_layer::<S>(self.layer, geometry.into(), value);
    }

    pub fn paint_text<S, Payload>(&mut self, line: Line, payload: Payload)
    where
        S: ShaderSpec,
        Payload: FnOnce(&mut Frame<'canvas>, Line) -> S::Instance,
    {
        let value = payload(self.frame, line);
        self.frame.canvas.emit_layer::<S>(self.layer, line.quad(), value);
    }
}

impl<'canvas> Deref for PaintLayer<'_, 'canvas> {
    type Target = Frame<'canvas>;

    fn deref(&self) -> &Self::Target {
        self.frame
    }
}

impl DerefMut for PaintLayer<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.frame
    }
}

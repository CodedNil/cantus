use crate::{
    Image, Rect, Response, ShaderData,
    backend::canvas::Canvas,
    contract::{Quad, ShaderSpec, SurfaceHandle},
    data::ImageHandle,
    input::Input,
    text::{Line, Text, TextScope},
};
pub struct Frame<'a, Shared: ShaderData, Services = ()> {
    pub globals: &'a mut Shared,
    pub delta_time: f32,
    pub services: Services,
    text: &'a mut Text,
    canvas: &'a mut Canvas<Shared>,
    input: &'a mut Input,
    surface: SurfaceHandle,
}

impl<'a, Shared: ShaderData, Services> Frame<'a, Shared, Services> {
    pub(crate) const fn new(
        globals: &'a mut Shared,
        input: &'a mut Input,
        delta_time: f32,
        text: &'a mut Text,
        canvas: &'a mut Canvas<Shared>,
        surface: SurfaceHandle,
        services: Services,
    ) -> Self {
        Self {
            globals,
            delta_time,
            services,
            text,
            canvas,
            input,
            surface,
        }
    }

    #[track_caller]
    pub fn interact(&mut self, rect: Rect) -> Response {
        self.input.interact(rect)
    }

    #[track_caller]
    pub fn drag(&mut self, rect: Rect) -> Response {
        self.input.drag(rect)
    }

    pub const fn press_position(&self) -> Option<glam::Vec2> {
        self.input.press_position()
    }

    pub const fn pointer(&self) -> glam::Vec2 {
        self.input.pointer()
    }

    pub fn pointer_in(&mut self, rect: Rect) -> bool {
        self.input.pointer_in(rect)
    }

    pub fn input_region(&mut self, rect: Rect) {
        self.input.input_region(rect);
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
    /// In source, the closure receives [`crate::Fragment`] first, the program's push-constant
    /// globals second, then explicitly typed values inferred from its surrounding host function.
    /// [`crate::paint`] lowers that closure to the private payload builder accepted here.
    /// Example: `frame.paint_quad(quad, |fragment: Fragment, globals: Globals, alpha: f32| { ... })`.
    ///
    pub fn paint_quad<S, Geometry, Payload>(&mut self, geometry: Geometry, payload: Payload)
    where
        S: ShaderSpec<Shared = Shared>,
        Geometry: Copy + Into<Quad>,
        Payload: FnOnce(&mut Self, Geometry) -> S::Instance,
    {
        let value = payload(self, geometry);
        self.canvas.emit::<S>(self.surface, geometry.into(), value);
    }

    /// Paints shaped text with an inline Rust-GPU fragment shader.
    ///
    /// The closure receives [`crate::TextFragment`] first, the program's push-constant
    /// globals second, then inferred draw values, matching [`Self::paint_quad`].
    ///
    pub fn paint_text<S, Payload>(&mut self, line: Line, payload: Payload)
    where
        S: ShaderSpec<Shared = Shared>,
        Payload: FnOnce(&mut Self, Line) -> S::Instance,
    {
        let value = payload(self, line);
        self.canvas.emit::<S>(self.surface, line.quad(), value);
    }
}

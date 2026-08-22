use crate::{
    __private::{Canvas, Paint},
    FrameGlobals, ImageArray, Rect, Response, SurfaceHandle,
    backend::renderer::SurfaceSlot,
    text::{Line, ShapedLine, Shaper, Text, TextLayout, TextStyle},
};
use core::ops::Range;
use core::ops::{Deref, DerefMut};

pub struct SurfaceState<'a, Shared> {
    surfaces: &'a mut [Option<SurfaceSlot<Shared>>],
    surface: SurfaceHandle,
}

impl<'a, Shared> SurfaceState<'a, Shared> {
    const fn new(surfaces: &'a mut [Option<SurfaceSlot<Shared>>]) -> Self {
        Self {
            surfaces,
            surface: SurfaceHandle::PRIMARY,
        }
    }

    const fn replace_surface(&mut self, surface: SurfaceHandle) -> SurfaceHandle {
        let previous = self.surface;
        self.surface = surface;
        previous
    }

    const fn slot(&self) -> &SurfaceSlot<Shared> {
        self.surfaces[self.surface.index()].as_ref().expect("frame targets a missing surface")
    }

    const fn slot_mut(&mut self) -> &mut SurfaceSlot<Shared> {
        self.surfaces[self.surface.index()].as_mut().expect("frame targets a missing surface")
    }
}

impl<Shared> Deref for SurfaceState<'_, Shared> {
    type Target = Shared;

    fn deref(&self) -> &Self::Target {
        &self.slot().globals
    }
}

impl<Shared> DerefMut for SurfaceState<'_, Shared> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slot_mut().globals
    }
}

pub struct Frame<'a, Shared: FrameGlobals, Services = ()> {
    pub globals: SurfaceState<'a, Shared>,
    pub delta_time: f32,
    pub services: Services,
    text: &'a mut Text,
    canvas: &'a mut Canvas<Shared>,
}

impl<'a, Shared: FrameGlobals, Services> Frame<'a, Shared, Services> {
    pub(crate) const fn new(surfaces: &'a mut [Option<SurfaceSlot<Shared>>], delta_time: f32, text: &'a mut Text, canvas: &'a mut Canvas<Shared>, services: Services) -> Self {
        Self {
            globals: SurfaceState::new(surfaces),
            delta_time,
            text,
            services,
            canvas,
        }
    }

    #[track_caller]
    pub fn interact(&mut self, rect: Rect) -> Response {
        self.globals.slot_mut().input.interact(rect)
    }

    pub const fn click_position(&self) -> Option<glam::Vec2> {
        self.globals.slot().input.click()
    }

    pub const fn pointer(&self) -> glam::Vec2 {
        self.globals.slot().input.pointer()
    }

    pub const fn pointer_down(&self) -> bool {
        self.globals.slot().input.down()
    }

    pub const fn dragging(&self) -> bool {
        self.globals.slot().input.dragging()
    }

    pub const fn drag_origin(&self) -> glam::Vec2 {
        self.globals.slot().input.drag_origin()
    }

    pub const fn enable_drag(&mut self) {
        self.globals.slot_mut().input.enable_drag();
    }

    pub const fn cancel_drag(&mut self) {
        self.globals.slot_mut().input.cancel_drag();
    }

    pub const fn released(&self) -> bool {
        self.globals.slot().input.released()
    }

    pub fn scroll(&mut self, rect: Rect) -> i32 {
        self.globals.slot_mut().input.scroll(rect)
    }

    pub fn input_region(&mut self, rect: Rect) {
        self.globals.slot_mut().input.input_region(rect);
    }

    pub fn image_array(&self, size: [u32; 2], layers: usize) -> ImageArray {
        self.canvas.image_array(size, layers)
    }

    pub fn text(&mut self, content: &str, style: TextStyle) -> TextLayout<'_> {
        TextLayout::new(self.text, content, style)
    }

    pub fn measure_text(&self, content: &str, style: TextStyle) -> f32 {
        self.text.width(content, style)
    }

    pub fn fit_text(&mut self, shaped: &ShapedLine, y: f32, bounds: Range<f32>) -> Line {
        self.text.fit_shaped(shaped, y, bounds.start, bounds.end)
    }

    pub fn visible_text(&mut self, shaped: &ShapedLine, left: glam::Vec2, clip: Range<f32>) -> Line {
        self.text.place_visible(shaped, left, clip)
    }

    pub fn text_shaper(&self) -> Shaper {
        self.text.shaper()
    }

    pub fn paint(&mut self, paint: impl Paint<Shared>) {
        paint.paint(self.canvas);
    }

    pub fn draw_text(&mut self, line: Line) {
        self.canvas.emit_text(line);
    }

    pub fn surface(&mut self, surface: SurfaceHandle, draw: impl FnOnce(&mut Self)) {
        let previous_surface = self.canvas.replace_surface(surface);
        let previous_globals = self.globals.replace_surface(surface);
        self.sync_input();
        draw(self);
        self.canvas.replace_surface(previous_surface);
        self.globals.replace_surface(previous_globals);
        self.sync_input();
    }

    fn sync_input(&mut self) {
        let pointer = self.globals.slot().input.pointer();
        let pressure = self.globals.slot().input.pressure();
        let info = self.globals.frame_info_mut();
        info.mouse_pos = pointer;
        info.mouse_pressure = pressure;
    }
}

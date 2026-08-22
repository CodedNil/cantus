use crate::{Quad, contract::IntoQuad, glam::Vec2};
use std::{mem, panic::Location, vec::Vec};

#[derive(Clone, Copy)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Id(u64);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Callsite {
    file: &'static str,
    line: u32,
    column: u32,
}

#[derive(Clone, Copy)]
struct Widget {
    id: Id,
    rect: Rect,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pointer {
    Outside,
    Hovering,
    Held,
}

#[derive(Clone, Copy)]
enum ButtonEvent {
    Press,
    Release,
}

pub struct Input {
    state: Pointer,
    dragging: bool,
    drag_enabled: bool,
    press_origin: Vec2,
    pointer: Vec2,
    event: Option<ButtonEvent>,
    hot: Option<Id>,
    active: Option<Id>,
    callsites: Vec<(Callsite, u32)>,
    click: Option<Vec2>,
    scroll: i32,
    pressure: f32,
    previous_widgets: Vec<Widget>,
    widgets: Vec<Widget>,
    regions: Vec<Rect>,
}

pub struct Response {
    flags: u8,
}

#[derive(Clone, Copy)]
pub enum InputEvent {
    Enter(Vec2),
    Motion(Vec2),
    Leave,
    Press,
    Release,
    CancelDrag,
    Scroll(i32),
}

impl Response {
    const CONTAINS_POINTER: u8 = 1;
    const HOVERED: u8 = 2;
    const PRESSED: u8 = 4;
    const CLICKED: u8 = 8;

    pub const fn contains_pointer(&self) -> bool {
        self.flags & Self::CONTAINS_POINTER != 0
    }

    pub const fn hovered(&self) -> bool {
        self.flags & Self::HOVERED != 0
    }

    pub const fn pressed(&self) -> bool {
        self.flags & Self::PRESSED != 0
    }

    pub const fn clicked(&self) -> bool {
        self.flags & Self::CLICKED != 0
    }
}

impl Rect {
    pub const fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    pub fn from_center(center: Vec2, half_size: Vec2) -> Self {
        let (min, max) = (center - half_size, center + half_size);
        Self::new(min.x, min.y, max.x, max.y)
    }

    pub(crate) fn contains(self, point: Vec2) -> bool {
        point.x >= self.x0 && point.x <= self.x1 && point.y >= self.y0 && point.y <= self.y1
    }
}

impl IntoQuad for Rect {
    fn into_quad(self) -> Quad {
        Quad::from_min_max(Vec2::new(self.x0, self.y0), Vec2::new(self.x1, self.y1))
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            state: Pointer::Outside,
            dragging: false,
            drag_enabled: false,
            press_origin: Vec2::ZERO,
            pointer: Vec2::ZERO,
            event: None,
            hot: None,
            active: None,
            callsites: Vec::new(),
            click: None,
            scroll: 0,
            pressure: 0.0,
            previous_widgets: Vec::new(),
            widgets: Vec::new(),
            regions: Vec::new(),
        }
    }
}

impl Input {
    pub(crate) const fn pointer(&self) -> Vec2 {
        self.pointer
    }

    pub const fn dragging(&self) -> bool {
        self.dragging
    }

    pub(crate) const fn drag_origin(&self) -> Vec2 {
        self.press_origin
    }

    fn hit_test(&self, point: Vec2) -> Option<Id> {
        if self.down() {
            self.active
                .filter(|&id| self.previous_widgets.iter().any(|widget| widget.id == id && widget.rect.contains(point)))
        } else {
            self.previous_widgets.iter().rfind(|widget| widget.rect.contains(point)).map(|widget| widget.id)
        }
    }

    #[track_caller]
    pub fn interact(&mut self, rect: Rect) -> Response {
        let location = Location::caller();
        let callsite = Callsite {
            file: location.file(),
            line: location.line(),
            column: location.column(),
        };
        let index = self.callsites.iter().position(|known| known.0 == callsite).unwrap_or_else(|| {
            self.callsites.push((callsite, 0));
            self.callsites.len() - 1
        });
        let occurrence = self.callsites[index].1;
        self.callsites[index].1 += 1;
        let id = Id((index as u64) << 32 | u64::from(occurrence));
        self.interact_id(id, rect)
    }

    fn interact_id(&mut self, id: Id, rect: Rect) -> Response {
        self.widgets.push(Widget { id, rect });
        self.regions.push(rect);
        let contains_pointer = self.state != Pointer::Outside && rect.contains(self.pointer);
        let hovered = self.state != Pointer::Outside && self.hot == Some(id);
        let pressed = self.active == Some(id) && matches!(self.event, Some(ButtonEvent::Press));
        let clicked = self.active == Some(id) && hovered && !self.dragging && matches!(self.event, Some(ButtonEvent::Release));
        if clicked {
            self.click = Some(self.pointer);
        }
        let flag = |enabled, bit| if enabled { bit } else { 0 };
        Response {
            flags: flag(contains_pointer, Response::CONTAINS_POINTER) | flag(hovered, Response::HOVERED) | flag(pressed, Response::PRESSED) | flag(clicked, Response::CLICKED),
        }
    }

    pub(crate) fn scroll(&mut self, rect: Rect) -> i32 {
        self.regions.push(rect);
        if rect.contains(self.pointer) { mem::take(&mut self.scroll) } else { 0 }
    }

    const fn target_pressure(&self) -> f32 {
        match self.state {
            Pointer::Outside => 0.0,
            Pointer::Hovering => 1.0,
            Pointer::Held => 2.0,
        }
    }

    pub(crate) const fn pressure(&self) -> f32 {
        self.pressure
    }

    pub(crate) const fn down(&self) -> bool {
        matches!(self.state, Pointer::Held)
    }

    pub(crate) fn input_region(&mut self, rect: Rect) {
        self.regions.push(rect);
    }

    pub(crate) fn take_regions(&mut self) -> impl Iterator<Item = Rect> + '_ {
        self.regions.drain(..)
    }

    pub(crate) const fn released(&self) -> bool {
        matches!(self.event, Some(ButtonEvent::Release))
    }

    pub(crate) fn begin_frame(&mut self, delta_time: f32) {
        self.click = None;
        self.hot = (self.state != Pointer::Outside).then(|| self.hit_test(self.pointer)).flatten();
        for (_, count) in &mut self.callsites {
            *count = 0;
        }
        self.widgets.clear();
        self.pressure += (self.target_pressure() - self.pressure).clamp(-5.0 * delta_time, 5.0 * delta_time);
    }

    pub(crate) fn end_frame(&mut self) {
        if !self.down() {
            self.cancel_drag();
            self.active = None;
        }
        self.previous_widgets.clear();
        self.previous_widgets.append(&mut self.widgets);
        self.event = None;
        self.scroll = 0;
    }

    fn press(&mut self, position: Vec2) {
        self.state = Pointer::Held;
        self.press_origin = position;
        self.active = self.previous_widgets.iter().rfind(|widget| widget.rect.contains(position)).map(|widget| widget.id);
        self.hot = self.active;
        self.event = Some(ButtonEvent::Press);
        self.dragging = false;
    }

    fn press_current(&mut self) {
        self.press(self.pointer);
    }

    fn release(&mut self) {
        self.event = self.down().then_some(ButtonEvent::Release);
        self.state = Pointer::Hovering;
        self.hot = self.hit_test(self.pointer);
    }

    fn motion(&mut self, position: Vec2) {
        self.pointer = position;
        self.hot = self.hit_test(position);
        if self.drag_enabled {
            self.dragging |= (position - self.press_origin).abs().max_element() >= 2.0;
        }
    }

    pub const fn cancel_drag(&mut self) {
        self.drag_enabled = false;
        self.dragging = false;
    }

    const fn hover(&mut self) {
        self.state = Pointer::Hovering;
    }

    const fn leave(&mut self) {
        self.state = Pointer::Outside;
        self.cancel_drag();
    }

    const fn scroll_input(&mut self, direction: i32) {
        self.scroll = direction;
    }

    pub(crate) const fn enable_drag(&mut self) {
        self.drag_enabled = true;
    }

    pub(crate) const fn click(&self) -> Option<Vec2> {
        self.click
    }

    pub(crate) fn apply(&mut self, event: InputEvent) {
        match event {
            InputEvent::Enter(position) => {
                self.motion(position);
                self.hover();
            }
            InputEvent::Motion(position) => self.motion(position),
            InputEvent::Leave => self.leave(),
            InputEvent::Press => self.press_current(),
            InputEvent::Release => self.release(),
            InputEvent::CancelDrag => {
                self.cancel_drag();
                self.hover();
            }
            InputEvent::Scroll(direction) => self.scroll_input(direction),
        }
    }
}

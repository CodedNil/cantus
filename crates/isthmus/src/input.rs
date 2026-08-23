use crate::{Quad, glam::Vec2};
use std::{mem, panic::Location, vec::Vec};

#[derive(Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Id {
    location: &'static Location<'static>,
    occurrence: u32,
}

#[derive(Clone, Copy)]
struct Widget {
    id: Id,
    rect: Rect,
    draggable: bool,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Pointer {
    #[default]
    Outside,
    Hovering,
    Held,
}

#[derive(Clone, Copy)]
enum ButtonEvent {
    Press,
    Release,
}

#[derive(Default)]
pub struct Input {
    state: Pointer,
    dragging: bool,
    press_origin: Vec2,
    pointer: Vec2,
    event: Option<ButtonEvent>,
    hot: Option<Id>,
    active: Option<Id>,
    scroll: i32,
    pressure: f32,
    held_seconds: f32,
    previous_held_seconds: f32,
    previous_widgets: Vec<Widget>,
    widgets: Vec<Widget>,
    regions: Vec<Rect>,
}

#[derive(Clone, Copy)]
pub struct Response {
    hovered: bool,
    gesture: Gesture,
    pub scroll: i32,
    pub held_seconds: f32,
    previous_held_seconds: f32,
    pub drag_origin: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Gesture {
    Idle,
    Clicked,
    Held,
    Dragging,
    DragReleased,
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
    pub const fn hovered(&self) -> bool {
        self.hovered
    }

    pub const fn clicked(&self) -> bool {
        matches!(self.gesture, Gesture::Clicked)
    }

    pub const fn held(&self) -> bool {
        matches!(self.gesture, Gesture::Held | Gesture::Dragging)
    }

    pub const fn dragging(&self) -> bool {
        matches!(self.gesture, Gesture::Dragging | Gesture::DragReleased)
    }

    pub const fn released(&self) -> bool {
        matches!(self.gesture, Gesture::DragReleased)
    }

    pub fn held_for(&self, seconds: f32) -> bool {
        self.held() && self.previous_held_seconds < seconds && self.held_seconds >= seconds
    }
}

impl Rect {
    pub const fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self {
            min: Vec2::new(x0, y0),
            max: Vec2::new(x1, y1),
        }
    }

    pub fn from_center(center: Vec2, half_size: Vec2) -> Self {
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub(crate) fn contains(self, point: Vec2) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }
}

impl From<Rect> for Quad {
    fn from(rect: Rect) -> Self {
        Self::from_min_max(rect.min, rect.max)
    }
}

impl Input {
    pub(crate) const fn pointer(&self) -> Vec2 {
        self.pointer
    }

    pub(crate) fn pointer_in(&mut self, rect: Rect) -> bool {
        self.regions.push(rect);
        self.state != Pointer::Outside && rect.contains(self.pointer)
    }

    pub(crate) const fn press_position(&self) -> Option<Vec2> {
        if matches!(self.event, Some(ButtonEvent::Press)) { Some(self.pointer) } else { None }
    }

    pub(crate) const fn dragging(&self) -> bool {
        self.dragging
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
    pub(crate) fn interact(&mut self, rect: Rect) -> Response {
        self.interact_with(rect, false)
    }

    #[track_caller]
    pub(crate) fn drag(&mut self, rect: Rect) -> Response {
        self.interact_with(rect, true)
    }

    #[track_caller]
    fn interact_with(&mut self, rect: Rect, drag: bool) -> Response {
        let location = Location::caller();
        let occurrence = self.widgets.iter().filter(|widget| widget.id.location == location).count() as u32;
        let id = Id { location, occurrence };
        self.interact_id(id, rect, drag)
    }

    fn interact_id(&mut self, id: Id, rect: Rect, drag: bool) -> Response {
        self.widgets.push(Widget { id, rect, draggable: drag });
        let hovered = self.state != Pointer::Outside && self.hot == Some(id);
        let clicked = self.active == Some(id) && hovered && !self.dragging && matches!(self.event, Some(ButtonEvent::Release));
        let held = self.active == Some(id) && self.down();
        let dragging = drag && self.active == Some(id) && self.dragging;
        let released = drag && self.active == Some(id) && matches!(self.event, Some(ButtonEvent::Release));
        let scroll = if hovered { mem::take(&mut self.scroll) } else { 0 };
        let gesture = if released {
            Gesture::DragReleased
        } else if dragging {
            Gesture::Dragging
        } else if clicked {
            Gesture::Clicked
        } else if held {
            Gesture::Held
        } else {
            Gesture::Idle
        };
        Response {
            hovered,
            gesture,
            scroll,
            held_seconds: self.held_seconds,
            previous_held_seconds: self.previous_held_seconds,
            drag_origin: self.press_origin,
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
        self.regions.drain(..).chain(self.previous_widgets.iter().map(|widget| widget.rect))
    }

    pub(crate) fn begin_frame(&mut self, delta_time: f32) {
        self.hot = (self.state != Pointer::Outside).then(|| self.hit_test(self.pointer)).flatten();
        self.widgets.clear();
        self.previous_held_seconds = self.held_seconds;
        if self.down() {
            self.held_seconds += delta_time;
        } else {
            self.held_seconds = 0.0;
            self.previous_held_seconds = 0.0;
        }
        let target_pressure = match self.state {
            Pointer::Outside => 0.0,
            Pointer::Hovering => 1.0,
            Pointer::Held => 2.0,
        };
        self.pressure += (target_pressure - self.pressure).clamp(-5.0 * delta_time, 5.0 * delta_time);
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
        self.held_seconds = 0.0;
        self.previous_held_seconds = 0.0;
    }

    fn release(&mut self) {
        self.event = self.down().then_some(ButtonEvent::Release);
        self.state = Pointer::Hovering;
        self.hot = self.hit_test(self.pointer);
    }

    fn motion(&mut self, position: Vec2) {
        self.pointer = position;
        self.hot = self.hit_test(position);
        if self
            .active
            .is_some_and(|active| self.previous_widgets.iter().any(|widget| widget.id == active && widget.draggable))
        {
            self.dragging |= (position - self.press_origin).abs().max_element() >= 2.0;
        }
    }

    pub(crate) const fn cancel_drag(&mut self) {
        self.dragging = false;
    }

    const fn leave(&mut self) {
        self.state = Pointer::Outside;
        self.cancel_drag();
    }

    pub(crate) fn apply(&mut self, event: InputEvent) {
        match event {
            InputEvent::Enter(position) => {
                self.motion(position);
                self.state = Pointer::Hovering;
            }
            InputEvent::Motion(position) => self.motion(position),
            InputEvent::Leave => self.leave(),
            InputEvent::Press => self.press(self.pointer),
            InputEvent::Release => self.release(),
            InputEvent::CancelDrag => {
                self.cancel_drag();
                self.state = Pointer::Hovering;
            }
            InputEvent::Scroll(direction) => self.scroll = direction,
        }
    }
}

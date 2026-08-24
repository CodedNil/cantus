use crate::render::{Globals, RipplePulse};
use isthmus::{Quad, glam::Vec2};
use std::{mem, vec::Vec};

#[derive(Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
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

    pub fn contains(self, point: Vec2) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }
}

impl From<Rect> for Quad {
    fn from(rect: Rect) -> Self {
        Self::from_min_max(rect.min, rect.max)
    }
}

#[derive(Clone, Copy)]
struct Widget {
    rect: Rect,
    drag: Option<u64>,
}

#[derive(Clone, Copy)]
enum Pointer {
    Outside(Vec2),
    Hovering(Vec2),
    Held { position: Vec2, origin: Vec2, dragging: bool },
}

impl Default for Pointer {
    fn default() -> Self {
        Self::Outside(Vec2::ZERO)
    }
}

#[derive(Clone, Copy)]
enum ButtonEvent {
    Press,
    Release,
}

#[derive(Clone, Copy)]
enum Active {
    Widget(usize),
    Drag(u64),
}

#[derive(Default)]
pub struct Interaction {
    pointer: Pointer,
    pressure: f32,
    ripples: [RipplePulse; 4],
    event: Option<ButtonEvent>,
    hot: Option<usize>,
    active: Option<Active>,
    scroll: i32,
    held_seconds: f32,
    previous_held_seconds: f32,
    previous_widgets: Vec<Widget>,
    widgets: Vec<Widget>,
    regions: Vec<Rect>,
    launcher_active: bool,
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

impl Interaction {
    pub fn set_launcher_active(&mut self, active: bool) {
        if self.launcher_active == active {
            return;
        }
        self.launcher_active = active;
        self.hot = None;
        self.active = None;
        self.pointer = Pointer::Outside(self.position());
        self.previous_widgets.clear();
        self.widgets.clear();
        self.regions.clear();
    }

    pub fn begin_frame(&mut self, delta_time: f32, time: f32) {
        self.hot = self.pointer().and_then(|pointer| self.hit_test(pointer));
        self.widgets.clear();
        self.previous_held_seconds = self.held_seconds;
        if self.down() {
            self.held_seconds += delta_time;
        } else {
            self.held_seconds = 0.0;
            self.previous_held_seconds = 0.0;
        }
        let target_pressure = match self.pointer {
            Pointer::Outside(_) => 0.0,
            Pointer::Hovering(_) => 1.0,
            Pointer::Held { .. } => 2.0,
        };
        self.pressure += (target_pressure - self.pressure).clamp(-5.0 * delta_time, 5.0 * delta_time);
        if matches!(self.event, Some(ButtonEvent::Press)) {
            let pulse = RipplePulse {
                origin: self.position(),
                start_time: time,
            };
            if let Some(ripple) = self.ripples.iter_mut().min_by(|a, b| a.start_time.total_cmp(&b.start_time)) {
                *ripple = pulse;
            }
        }
    }

    pub fn end_frame(&mut self) {
        if !self.down() {
            self.active = None;
        }
        self.previous_widgets.clear();
        self.previous_widgets.append(&mut self.widgets);
        self.event = None;
        self.scroll = 0;
    }

    pub fn interact(&mut self, rect: Rect) -> Response {
        self.interact_with(rect, None)
    }

    pub fn drag(&mut self, key: u64, rect: Rect) -> Response {
        self.interact_with(rect, Some(key))
    }

    pub fn pointer_in(&mut self, rect: Rect) -> bool {
        self.regions.push(rect);
        self.pointer().is_some_and(|pointer| rect.contains(pointer))
    }

    pub fn input_region(&mut self, rect: Rect) {
        self.regions.push(rect);
    }

    pub fn take_regions(&mut self) -> impl Iterator<Item = Rect> + '_ {
        self.regions.drain(..).chain(self.previous_widgets.iter().map(|widget| widget.rect))
    }

    pub const fn position(&self) -> Vec2 {
        match self.pointer {
            Pointer::Outside(position) | Pointer::Hovering(position) | Pointer::Held { position, .. } => position,
        }
    }

    pub const fn dragging(&self) -> bool {
        matches!(self.pointer, Pointer::Held { dragging: true, .. })
    }

    pub const fn globals(&self, bar_height: f32) -> Globals {
        Globals {
            pointer: self.position(),
            pressure: self.pressure,
            bar_height,
            ripples: self.ripples,
        }
    }

    pub fn apply(&mut self, event: InputEvent) {
        match event {
            InputEvent::Enter(position) => {
                self.pointer = Pointer::Hovering(position);
                self.hot = self.hit_test(position);
            }
            InputEvent::Motion(position) => self.motion(position),
            InputEvent::Leave => self.pointer = Pointer::Outside(self.position()),
            InputEvent::Press => self.press(),
            InputEvent::Release => self.release(),
            InputEvent::CancelDrag => self.pointer = Pointer::Hovering(self.position()),
            InputEvent::Scroll(direction) => self.scroll = direction,
        }
    }

    fn hit_test(&self, point: Vec2) -> Option<usize> {
        if self.down() {
            match self.active {
                Some(Active::Widget(slot)) if self.previous_widgets.get(slot).is_some_and(|widget| widget.rect.contains(point)) => Some(slot),
                _ => None,
            }
        } else {
            self.previous_widgets.iter().rposition(|widget| widget.rect.contains(point))
        }
    }

    fn interact_with(&mut self, rect: Rect, drag: Option<u64>) -> Response {
        let slot = self.widgets.len();
        self.widgets.push(Widget { rect, drag });
        let active = if let Some(key) = drag {
            matches!(self.active, Some(Active::Drag(active)) if active == key)
        } else {
            matches!(self.active, Some(Active::Widget(active)) if active == slot)
        };
        let hovered = self.pointer().is_some()
            && if drag.is_some() && active {
                rect.contains(self.position())
            } else {
                self.hot == Some(slot)
            };
        let release = matches!(self.event, Some(ButtonEvent::Release));
        let gesture = if drag.is_some() && active && self.dragging() && release {
            Gesture::DragReleased
        } else if active && hovered && release {
            Gesture::Clicked
        } else if drag.is_some() && active && self.dragging() {
            Gesture::Dragging
        } else if active && self.down() {
            Gesture::Held
        } else {
            Gesture::Idle
        };
        Response {
            hovered,
            gesture,
            scroll: if hovered { mem::take(&mut self.scroll) } else { 0 },
            held_seconds: self.held_seconds,
            previous_held_seconds: self.previous_held_seconds,
            drag_origin: match self.pointer {
                Pointer::Held { origin, .. } => origin,
                _ => self.position(),
            },
        }
    }

    const fn pointer(&self) -> Option<Vec2> {
        match self.pointer {
            Pointer::Outside(_) => None,
            Pointer::Hovering(position) | Pointer::Held { position, .. } => Some(position),
        }
    }

    const fn down(&self) -> bool {
        matches!(self.pointer, Pointer::Held { .. })
    }

    fn press(&mut self) {
        let position = self.position();
        self.pointer = Pointer::Held {
            position,
            origin: position,
            dragging: false,
        };
        self.hot = self.previous_widgets.iter().rposition(|widget| widget.rect.contains(position));
        self.active = self.hot.map(|slot| self.previous_widgets[slot].drag.map_or(Active::Widget(slot), Active::Drag));
        self.event = Some(ButtonEvent::Press);
        self.held_seconds = 0.0;
        self.previous_held_seconds = 0.0;
    }

    fn release(&mut self) {
        self.event = self.down().then_some(ButtonEvent::Release);
        let position = self.position();
        self.pointer = Pointer::Hovering(position);
        self.hot = self.hit_test(position);
    }

    fn motion(&mut self, position: Vec2) {
        self.hot = self.hit_test(position);
        self.pointer = match self.pointer {
            Pointer::Held { origin, dragging, .. } => Pointer::Held {
                position,
                origin,
                dragging: dragging || matches!(self.active, Some(Active::Drag(_))) && (position - origin).abs().max_element() >= 2.0,
            },
            Pointer::Hovering(_) => Pointer::Hovering(position),
            Pointer::Outside(_) => Pointer::Outside(position),
        };
    }
}

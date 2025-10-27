use std::sync::atomic::AtomicI64;
use undo_2::{Action, Commands};

pub type Point2d = (f64, f64);

#[derive(Debug)]
pub enum EditEvent {
    CreateCircle(i64, Point2d),
    EditRadius(i64, f64),
}

#[derive(Debug)]
pub struct Drawing {
    pub canvas: Vec<Circle>,
    pub command: Commands<EditEvent>,
}
impl Drawing {
    pub fn new() -> Self {
        Self {
            canvas: Vec::new(),
            command: Commands::new(),
        }
    }

    pub fn draw(&mut self, x: f64, y: f64) {
        let circle = Circle::new(x, y);
        let id = circle.id;
        let x = circle.x;
        let y = circle.y;
        self.canvas.push(circle);
        self.command.push(EditEvent::CreateCircle(id, (x, y)));
    }
    pub fn edit_radius(&mut self, id: i64, radius: f64) {
        if let Some(circle) = self.canvas.iter_mut().find(|c| c.id == id) {
            let delta = radius - circle.radius ; 
            circle.radius = radius;
            self.command.push(EditEvent::EditRadius(id, delta));
        }
        dbg!("{}",&self.command);
    }
    pub fn undo(&mut self) {
        dbg!("Pre undo: {}",&self.command);
        for action in self.command.undo() {
            materialize(&mut self.canvas, action)
        }
        dbg!("Post undo: {}",&self.command);
    }
    pub fn redo(&mut self) {
        dbg!("Pre redo: {}",&self.command);
        for action in self.command.redo() {
            materialize(&mut self.canvas, action)
        }
        dbg!("Post redo: {}",&self.command);
    }
}

fn materialize(data: &mut Vec<Circle>, action: (Action, &EditEvent)) {
    use EditEvent as E;
    match action {
        (Action::Do, E::CreateCircle(_, (x, y))) => data.push(Circle::new(*x, *y)),
        (Action::Undo, E::CreateCircle(id, _)) => {
            data.retain(|s| s.id != *id);
            dbg!(data);
        }
        (Action::Do, E::EditRadius(id, delta)) => {
            if let Some(svg) = data.iter_mut().find(|d| d.id == *id) {
                svg.radius = svg.radius + delta;
            }
        }
        (Action::Undo, E::EditRadius(id, delta)) => {
            if let Some(svg) = data.iter_mut().find(|d| d.id == *id) {
                svg.radius = svg.radius - delta;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub id: i64,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

static CIRCLE_UID: AtomicI64 = AtomicI64::new(0);
impl Circle {
    pub fn new(x: f64, y: f64) -> Self {
        let id = CIRCLE_UID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Self {
            id,
            x,
            y,
            radius: 50.,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

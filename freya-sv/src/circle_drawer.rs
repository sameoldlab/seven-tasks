use freya::prelude::*;
use std::sync::atomic::AtomicI64;
use undo_2::{Action, Commands};

type Point2d = (f64, f64);

#[derive(Debug)]
enum EditEvent {
    CreateCircle(i64, Point2d),
    EditRadius(i64, f64),
}

#[derive(Debug)]
struct Drawing {
    canvas: Vec<Circle>,
    command: Commands<EditEvent>,
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
struct Circle {
    id: i64,
    x: f64,
    y: f64,
    radius: f64,
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

#[component]
pub fn CircleDrawer() -> Element {
    let mut drawing = use_signal(|| Drawing::new());
    let mut selected = use_signal::<Option<usize>>(|| None);
    let mut temp = use_signal::<Option<Circle>>(|| None);

    rsx!(
        rect {
            direction: "horizontal",
            margin: "16 0 0 0",
            spacing: "8",

            Button {
                onclick: move |_|  drawing.write().undo(),
                label {"Undo"}
            }
            Button {
                onclick: move |_| drawing.write().redo(),
                label {"Redo"}
            }
        }
        // Canvas
        CursorArea {
            icon: CursorIcon::Crosshair,
            rect {
                width: "100%",
                height: "100%",
                background: "#333333",
                margin: "16",
                border: "1 center fill",
                onclick: move |e| {
                    let coords = e.data.clone().element_coordinates.clone();
                    drawing.write().draw(coords.x, coords.y);
                },
                if let Some(circle) = &*temp.read() {
                    svg {
                        width: (circle.radius * 2.).to_string(),
                        height: (circle.radius * 2.).to_string(),
                        stroke: "red",
                        fill: "red",
                        position: "absolute",
                        position_top: (circle.y - circle.radius).to_string(),
                        position_left: (circle.x - circle.radius).to_string(),
                        svg_content: r##"
                            <svg width="100" height="100" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <circle cx="50" cy="50" r="48" />
                            </svg>
                        "##,
                    }
                }
                for (i, circle) in drawing.read().canvas.iter().enumerate() {
                    svg {
                        width: (circle.radius * 2.).to_string(),
                        height: (circle.radius * 2.).to_string(),
                        stroke: if !selected.read().is_some_and(|i2| i2 == i) { "red"} else {"transparent"},
                        position: "absolute",
                        position_top: (circle.y - circle.radius).to_string(),
                        position_left: (circle.x - circle.radius).to_string(),
                        svg_content: r##"
                            <svg width="100" height="100" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <circle cx="50" cy="50" r="48" />
                            </svg>
                        "##,
                        onclick: move |e| {
                            e.stop_propagation();
                            selected.set(Some(i.clone()));
                            temp.set(Some(drawing.read().canvas[i].clone()));
                        },
                    }

                }
            }
        }
        if let Some(circle) = temp.read().clone() {
            Popup {
                oncloserequest: move |_| {
                    drawing.write().edit_radius(circle.id, circle.radius);
                    selected.set(None);
                    temp.set(None);
                },
                PopupContent {
                    rect{
                        padding: "16",
                        label { "Adjust diameter of circle at {circle}."}
                        Slider {
                            size: "90%",
                            value: circle.radius,
                            onmoved: move |value: f64| {
                                temp.with_mut(|circle| {
                                    if let Some(c) = circle {
                                        c.radius = value;
                                    }
                                })
                            },
                        }
                    }
                }
            }
        }
    )
}

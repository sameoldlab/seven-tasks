use std::sync::atomic::AtomicI64;

use freya::prelude::*;
enum EditEvent {
    CreateCircle(Circle),
    AdjustDiameter(AtomicI64, i64),
}

static HISTORY: Vec<String> = Vec::new();
static ID: AtomicI64 = AtomicI64::new(0);

#[derive(Debug, Clone, PartialEq)]
struct Circle {
    id: i64,
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        let id = ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id,
            x,
            y,
            radius,
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
    let mut selected = use_signal::<Option<usize>>(|| None);
    let mut circles = use_signal::<Vec<Circle>>(|| Vec::new());

    rsx!(
        rect {
            direction: "horizontal",
            margin: "16 0 0 0",
            spacing: "8",

            Button {
                onclick: move |_| println!(),
                label {"Undo"}
            }
            Button {
                onclick: move |_| println!(),
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
                    circles.write().push(Circle::new( coords.x, coords.y, 40. ));
                },
                for (i, circle) in circles().iter().enumerate() {
                    svg {
                        width: (circle.radius * 2.).to_string(),
                        height: (circle.radius * 2.).to_string(),
                        stroke: "red",
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
                        },
                    }

                }
            }
        }
        if let Some(idx) = *selected.read() {
            Popup {
                oncloserequest: move |_| {
                    selected.set(None);
                },
                PopupContent {
                    rect{
                        padding: "16",
                        label { "Adjust diameter of circle at {circles()[idx]}."}
                        Slider {
                            size: "90%",
                            value: circles()[idx].radius,
                            onmoved: move |v: f64| {
                                circles.write()[idx].radius = v.clone();
                            }
                        }
                    }
                }
            }
        }
    )
}

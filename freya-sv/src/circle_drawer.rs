use freya::prelude::*;
use state::circles::{Circle, Drawing};

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

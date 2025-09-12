use std::time::Duration;

use xilem::core::fork;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::{Background, Style};
use xilem::tokio::time;
use xilem::view::{
    self as v, Axis, FlexExt, button, flex, flex_col, flex_row, indexed_stack, label, portal,
    progress_bar, sized_box, task, text_input,
};
use xilem::winit::error::EventLoopError;
use xilem::{Color, EventLoop, WidgetView, WindowOptions, Xilem, tokio};

#[derive(Default)]
struct AppState {
    tab: Tabs,
    num: i32,
    /// temperature in celsius
    temp: Option<f32>,
    elapsed_time: f64,
    total_time: f64,
}

#[repr(usize)]
#[derive(Default, Clone, Copy, PartialEq)]
enum Tabs {
    #[default]
    Counter,
    TempConverter,
    FlightBooker,
    Timer,
    Crud,
    Circles,
    Cells,
}
fn app_logic(data: &mut AppState) -> impl WidgetView<AppState> + use<> {
    let null_string = String::new();
    let tabs = vec![
        ("Counter", Tabs::Counter),
        ("Temp Converter", Tabs::TempConverter),
        ("Flight Booker", Tabs::FlightBooker),
        ("Timer", Tabs::Timer),
        ("CRUD", Tabs::Crud),
        ("Circles", Tabs::Circles),
        ("Cells", Tabs::Cells),
    ]
    .into_iter()
    .map(|(l, t)| {
        button(l, move |data: &mut AppState| data.tab = t)
            .disabled(data.tab == t)
            .background(Background::Color(Color::from_rgb8(15, 15, 15)))
            .disabled_background(Background::Color(Color::from_rgb8(00, 00, 00)))
            .corner_radius(0.)
            .border_color(Color::TRANSPARENT)
            .hovered_border_color(Color::from_rgb8(63, 63, 70))
        // .border_width(0.)
    })
    // .iter()
    .collect::<Vec<_>>();

    flex(
        Axis::Vertical,
        (
            flex_row(tabs)
                .main_axis_alignment(v::MainAxisAlignment::Start)
                .must_fill_major_axis(true)
                .cross_axis_alignment(v::CrossAxisAlignment::Start)
                .gap(0.px())
                // .border(Color::from_rgb8(63, 63, 70), 1.)
                .background_color(Color::from_rgb8(15, 15, 15)),
            sized_box(
                indexed_stack((
                    // Counter
                    flex_col(flex_row((
                        button("-", |data: &mut AppState| data.num -= 1),
                        label(format!("{}", data.num)),
                        button("+", |data: &mut AppState| data.num += 1),
                    ))),
                    // Temp Converter
                    flex_col(flex_row((
                        v::text_input(
                            data.temp
                                .map_or(null_string.clone(), |v| format!("{}", v.round())),
                            |data: &mut AppState, input| {
                                if input.len() == 0 {
                                    data.temp = None;
                                } else {
                                    match input.parse::<f32>() {
                                        Ok(celsius) => data.temp = Some(celsius),
                                        Err(_) => {}
                                    }
                                }
                            },
                        )
                        .flex(1.),
                        label("Celsius = "),
                        v::text_input(
                            data.temp
                                .map_or(null_string, |v| format!("{}", as_fahrenheit(v).round())),
                            |data: &mut AppState, input| {
                                if input.len() == 0 {
                                    data.temp = None;
                                } else {
                                    match input.parse::<f32>() {
                                        Ok(fahrenheit) => {
                                            data.temp = Some((fahrenheit - 32.0) * (5.0 / 9.0))
                                        }
                                        Err(_) => {}
                                    }
                                }
                            },
                        )
                        .flex(1.),
                        label("Fahrenheit"),
                    ))),
                    // Flight Booker
                    flex_col((
                        // v::dropdown,
                        text_input("".to_string(), |_, _| {}),
                        text_input("".to_string(), |_, _| {}).disabled(true),
                        button("Book", |_| {}).disabled(true),
                        if false { Some(label("")) } else { None },
                    )),
                    // Timer
                    flex_col((
                        fork(
                            flex_row((
                                label("Elapsed Time: "),
                                progress_bar(Some(data.elapsed_time / data.total_time)),
                            )),
                            task(
                                |proxy| async move {
                                    let mut interval = time::interval(Duration::from_millis(100));
                                    loop {
                                        interval.tick().await;
                                        let Ok(()) = proxy.message(()) else {
                                            break;
                                        };
                                    }
                                },
                                |data: &mut AppState, _| {
                                    if data.elapsed_time < data.total_time {
                                        data.elapsed_time += 0.1;
                                    }
                                },
                            ),
                        ),
                        label(format!("{:.2}s", data.elapsed_time)),
                        flex_row((
                            label("Duration: "),
                            // "Slider"
                        )),
                        button("Reset", |data: &mut AppState| {
                            data.elapsed_time = 0.;
                            data.total_time = 30.;
                        }),
                    )),
                    // Crud
                    flex_col((
                        flex_row((
                            label("Filter prefix: "),
                            text_input("".to_string(), |_, _| {}),
                        )),
                        flex_row((
                            portal(flex_col(())),
                            flex_col((
                                flex_row((label("Name: "), text_input("".to_string(), |_, _| {}))),
                                flex_row((
                                    label("Surname: "),
                                    text_input("".to_string(), |_, _| {}),
                                )),
                            )),
                        )),
                        flex_row((
                            button("Create", |_| {}),
                            button("Update", |_| {}),
                            button("Delete", |_| {}),
                        )),
                    )),
                    // Circles
                    flex_col(()),
                    // Cells
                    flex_col(()),
                ))
                .active(data.tab as usize),
            )
            .width(500.px())
            .height(500.px()),
        ),
    )
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppState {
            tab: Tabs::Counter,
            num: 0,
            temp: Some(24.),
            elapsed_time: 0.,
            total_time: 30.,
        },
        app_logic,
        WindowOptions::new("Seven Tasks"),
    );
    app.run_in(EventLoop::with_user_event())
}
fn as_fahrenheit(v: f32) -> f32 {
    v * (9.0 / 5.0) + 32.0
}

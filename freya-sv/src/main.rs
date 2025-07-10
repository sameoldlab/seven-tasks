use freya::{launch::launch, prelude::*};
// use freya_router::prelude::*;
use dioxus_router::prelude::{Outlet, Routable, Router};
mod flight_booker;
use flight_booker::FlightBooker;
mod timer;
use timer::Timer;
mod crud;
use crud::Crud;
mod circle_drawer;
use circle_drawer::CircleDrawer;
mod state;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(Router::<Route> {})
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(TabBar)]
        #[route("/")]
        Home,
        #[route("/counter")]
        Counter,
        #[route("/temp")]
        TempConverter,
        #[route("/flight")]
        FlightBooker,
        #[route("/timer")]
        Timer,
        #[route("/crud")]
        Crud,
        #[route("/drawer")]
        CircleDrawer,
}
const THEME: Theme = Theme { ..DARK_THEME };

#[component]
fn TabBar() -> Element {
    rsx!(
        ThemeProvider {
            theme: THEME,
        NativeRouter { Body {
            Tabsbar {
                Link {
                    to: Route::Counter,
                    Tab { label { "Counter" } }
                }
                Link {
                    to: Route::TempConverter,
                    Tab { label { "TempConverter" } }
                }
                Link {
                    to: Route::FlightBooker,
                    Tab { label { "FlightBooker" } }
                }
                Link {
                    to: Route::Timer,
                    Tab { label { "Timer" } }
                }
                Link {
                    to: Route::Crud,
                    Tab { label { "Crud" } }
                }
                Link {
                    to: Route::CircleDrawer,
                    Tab { label { "Circle Drawer" } }
                }
            }
            rect {
                main_align: "center",
                cross_align: "center",
                width: "100%",
                Outlet::<Route> {}
            }
        } } }
    )
}

#[component]
fn Home() -> Element {
    rsx!(rect {
        height: "100%",
        width: "100%",
        cross_align: "center",
        main_align: "center",
        label {
            "SEVEN TASKS"
        }
    })
}

#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
       rect {
           direction: "horizontal",
           Button {
               onclick: move |_| count -= 1,
               label { "-" }
           }
           rect {
               padding: 4,
               label {
                    "{count}"
                }
           }
           Button {
               onclick: move |_| count += 1,
               label { "+" },
           }
       }
    )
}

#[component]
fn TempConverter() -> Element {
    let mut c = use_signal(|| 0.0);
    let mut f = use_signal(|| 0.0);

    rsx!(
       rect {
           direction: "horizontal",
           padding: "16",
           spacing: "4",
           cross_align: "center",
           width: "100%",
           Input {
               placeholder: "0.0",
               value: c.to_string(),
               onvalidate: move |validator: InputValidator| {
                   validator.set_valid(validator.text().parse::<f32>().is_ok())
               },
               onchange: move |t: String| {
                   match t.parse::<f32>() {
                    Ok(val) => {
                        c.set(val);
                        f.set( val * 9.0/5.0 + 32.0);
                        val
                    },
                    Err(e) => {
                        eprintln!("{e}");
                        return
                    },
                };
               }
           }
           label { "Celsius =" }
           Input {
               placeholder: "0.0",
               value: f.to_string(),
               onvalidate: move |validator: InputValidator| {
                   if validator.text().ends_with('.'){
                       validator.set_valid(validator.text().trim_matches('.').parse::<f32>().is_ok())
                   }
                   else { validator.set_valid(validator.text().parse::<f32>().is_ok()) }
               },
               onchange: move |t: String| {
                   match t.parse::<f32>() {
                    Ok(val) => {
                        f.set(val);
                           c.set( (val - 32.0) * (5.0/9.0));
                        val
                    },
                    Err(e) => {
                        eprintln!("{e}");
                        return
                    },
                };
               }
           }
           label { "Fahrenheit" }


       }
    )
}

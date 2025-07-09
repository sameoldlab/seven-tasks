use freya::prelude::*;
use std::fmt::{Display, Error, Formatter};
use time::{Date, OffsetDateTime, macros::format_description};

#[derive(Debug, Clone, PartialEq)]
enum TripType {
    OneWay,
    Return,
}
impl Display for TripType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TripType::OneWay => write!(f, "One-way Trip"),
            TripType::Return => write!(f, "Round Trip"),
        }
    }
}

#[component]
pub fn FlightBooker() -> Element {
    let format = format_description!("[year]-[month]-[day]");
    let now = OffsetDateTime::now_utc().date();
    let now_string = now.format(format).unwrap();

    let mut depart_date = use_signal(|| now);
    let mut depart = use_signal(|| now_string.clone());
    let mut arrival_date = use_signal(|| now);
    let mut arrival = use_signal(|| now_string.clone());

    let mut is_valid = use_signal(|| true);
    let mut message = use_signal(|| String::new());
    let mut is_arrival_valid = use_signal(|| true);
    let mut is_depart_valid = use_signal(|| true);

    let flight_types = use_hook(|| vec![TripType::OneWay, TripType::Return]);
    let mut selected_flight_type = use_signal(|| TripType::OneWay);

    rsx!(
        rect {
            direction: "vertical",
            padding: "16",
            spacing: "4",
            cross_align: "center",
            main_align: "center",
            content: "flex",
            width: "100%",
            height: "100%",
            max_width: "300",

            Dropdown {
                value: selected_flight_type.read().clone().to_string(),
                theme: theme_with!(DropdownTheme {
                    width: "flex(1)".into(),
                }),
                for ft in flight_types {
                    DropdownItem {
                        value: ft.to_string(),
                        onpress: {
                            to_owned![ft];
                            move |_| selected_flight_type.set(ft.clone())
                        },
                        label { "{ft}" }
                    }
                }
            }
            Input {
                value: depart,
                width: "fill-min",
                theme: theme_with!(InputTheme {
                    background: if !is_depart_valid.read().clone() { "red".into() } else { "rgb(20, 20, 20)".into() }
                }),
                onchange: move |t: String| {
                    depart.set(t);
                    // 🍝
                    match Date::parse(&depart.read(), &format) {
                        Ok(date) => {
                            if (date - now).is_negative() {
                                message.set("Departure must be in the future".to_string());
                                is_valid.set(false);
                                is_depart_valid.set(false);
                            } else {
                                message.set("".to_string());
                                is_valid.set(true);
                                is_depart_valid.set(true);
                                depart_date.set(date);
                            }
                        }
                        Err(e) => {
                            message.set(format!("Error reading daparture date: {e}"));
                            is_valid.set(false);
                            is_depart_valid.set(false);
                        }
                    }
                }
            }
            rect {
                opacity: if selected_flight_type.read().clone() == TripType::OneWay {
                    "0.3"
                } else {
                    "1"
                },
                // the builtin components have very basic customization.
                // Similar issue to Slint where only a few of the underlying props were exposed.
                // I'd need to fork the Input accessibility implementation to change
                // the opacity. This doesn't just affect styling but also states
                // like valid, invalid, disabled, success, loading.
                Input {
                    value: arrival,
                    width: "fill-min",
                    theme: theme_with!(InputTheme {
                        background: if !is_arrival_valid.read().clone() { "red".into() } else { "rgb(20, 20, 20)".into() }
                    }),
                    onvalidate: move |validator: InputValidator| {
                        validator.set_valid(selected_flight_type.read().clone() == TripType::Return);
                    },
                    onchange: move |t: String| {
                        arrival.set(t);
                        match Date::parse(&arrival.read(), &format) {
                            Ok(date) => {
                                if (date - *depart_date.read()).is_negative() {
                                    message.set("Arrival must be after departure".to_string());
                                    is_valid.set(false);
                                    is_arrival_valid.set(false);
                                } else {
                                    message.set("".to_string());
                                    is_valid.set(true);
                                    is_arrival_valid.set(true);
                                    arrival_date.set(date)
                                }
                            }
                            Err(e) => {
                                message.set(format!("Error reading arrival date: {e}"));
                                is_valid.set(false);
                                is_arrival_valid.set(false);
                            }
                        }
                    }
                }
            }
            rect {
                opacity: if *is_valid.read() { "1" } else { "0.3" },
                Button {
                     theme: theme_with!(ButtonTheme {
                        width: "flex(1)".into(),
                    }),
                    onpress: move |_| {
                        if !*is_valid.read() {return}
                        message.set("Scheduled trip".to_string());
                    },
                    label {"book"}
                }
            }
            label { "{message}" }
       }
    )
}

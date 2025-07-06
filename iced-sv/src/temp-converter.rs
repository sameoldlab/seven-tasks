use iced::{widget, Element, Theme};

fn main() -> iced::Result {
    iced::application("Temps", update, view)
        .centered()
        .theme(theme)
        .run()
}

#[derive(Default)]
pub struct Converter {
    celsius: String,
    fahrenheit: String,
}

#[derive(Debug, Clone)]
enum Message {
    CelsiusChanged(String),
    FahrenheitChanged(String),
}
fn theme(_state: &Converter) -> Theme {
    Theme::CatppuccinMocha
}
fn _title(state: &Converter) -> String {
    state.celsius.to_string()
}
fn update(state: &mut Converter, message: Message) {
    match message {
        Message::CelsiusChanged(string) => {
            state.celsius = string;
            match state.celsius.parse::<f64>() {
                Ok(celsius) => {
                    state.fahrenheit = (celsius * (9.0 / 5.0) + 32.0).round().to_string()
                }

                Err(err) => println!("{:#?}", err),
            }
        }
        Message::FahrenheitChanged(string) => {
            state.fahrenheit = string;
            match state.fahrenheit.parse::<f64>() {
                Ok(fahrenheit) => {
                    state.celsius = ((fahrenheit - 32.0) * (5.0 / 9.0)).round().to_string()
                }

                Err(err) => println!("{:#?}", err),
            }
        }
    }
}
fn view(state: &Converter) -> Element<Message> {
    let widget = widget::row![
        widget::text_input("0", &state.celsius).on_input(Message::CelsiusChanged),
        widget::text("Celsius = "),
        widget::text_input("0", &state.fahrenheit).on_input(Message::FahrenheitChanged),
        widget::text("Fahrenheit"),
    ];

    widget.into()
}

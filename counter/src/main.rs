use iced::{widget, Element, Theme};

fn main() -> iced::Result {
    iced::application("title", update, view)
        .centered()
        .theme(theme)
        .run()
}

#[derive(Default)]
pub struct Counter {
    val: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}
fn theme(_state: &Counter) -> Theme {
    Theme::CatppuccinMocha
}
fn _title(counter: &Counter) -> String {
    counter.val.to_string()
}
fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.val += 1,
        Message::Decrement => counter.val -= 1,
    }
}
fn view(counter: &Counter) -> Element<Message> {
    let widget = widget::row![
        widget::text(counter.val),
        widget::button("Add").on_press(Message::Increment),
        widget::button("Minus").on_press(Message::Decrement),
    ];

    widget.into()
}

use iced::{widget, Element, Theme};

fn main() -> iced::Result {
    iced::application("Timer", update, view)
        .centered()
        .theme(theme)
        .run()
}

#[derive(Default)]
pub struct Timer {
    elapsed: f32,
    duration: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Reset,
    DurationChanged(f32)
}
fn theme(_state: &Timer) -> Theme {
    Theme::CatppuccinMocha
}
fn update(state: &mut Timer, message: Message) {
    match message {
        Message::Reset => state.elapsed = 0.0,
        Message::DurationChanged(d) => {},
    }
}
fn view(state: &Timer) -> Element<Message> {
    let widget = widget::column![
        widget::row![
            widget::text("Elapsed Time: "),
            widget::progress_bar(0.0..=state.duration, state.elapsed)
        ],
        widget::text("{elapsed}s"),
        widget::row![
            widget::text("Duration: "),
            widget::slider(0.0..=60.0, state.duration, Message::DurationChanged),
        ],
        widget::button("Reset").on_press(Message::Reset),
    ];

    widget.into()
}

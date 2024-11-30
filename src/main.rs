use iced::{
    widget::{button, text},
    Element, Theme,
};

fn main() -> iced::Result {
    iced::application("Rust Desktop 2024", update, view)
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct State {
    counter: u32,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.counter += 1,
    }
}

fn view(state: &State) -> Element<Message> {
    button(text(state.counter))
        .on_press(Message::Increment)
        .into()
}

fn theme(_state: &State) -> Theme {
    Theme::TokyoNight
}

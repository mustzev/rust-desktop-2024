mod screen;

use iced::{
    application,
    widget::{button, text},
    Element, Theme,
};
use screen::{screen1::Screen1, screen2::Screen2};

fn main() -> iced::Result {
    application("Rust Desktop 2024", update, view)
        .theme(theme)
        .run()
}

pub struct App {
    screen: Screen,
}

pub enum Screen {
    Screen1(Screen1),
    Screen2(Screen2),
}

#[derive(Default)]
struct State {
    counter: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn theme(_state: &State) -> Theme {
    Theme::TokyoNight
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

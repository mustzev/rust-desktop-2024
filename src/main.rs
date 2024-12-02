mod screen;

use iced::{application, widget::text, Element, Task, Theme};
use screen::{screen1, screen2};

fn main() -> iced::Result {
    application("Rust Desktop 2024", App::update, App::view)
        .theme(App::theme)
        .run_with(move || App::new(true))
}

enum Screen {
    Screen1(screen1::Screen1),
    Screen2(screen2::Screen2),
}

struct App {
    screen: Screen,
}

#[derive(Debug)]
enum Message {
    Screen1(screen1::Message),
    Screen2(screen2::Message),
}

impl App {
    fn new(initialize: bool) -> (App, Task<Message>) {
        match initialize {
            true => (
                App {
                    screen: Screen::Screen1(screen1::Screen1 {}),
                },
                Task::none(),
            ),
            false => todo!(),
        }
    }

    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        text("main").into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

mod screen;

use iced::{application, Element, Task, Theme};
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

    fn update(&mut self, message: Message) {
        match message {
            Message::Screen1(message) => match message {
                screen1::Message::GoToScreen2 => self.screen = Screen::Screen2(screen2::Screen2 {}),
            },
            Message::Screen2(message) => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let screen = match &self.screen {
            Screen::Screen1(screen1) => screen1.view().map(Message::Screen1),
            Screen::Screen2(screen2) => screen2.view().map(Message::Screen2),
        };
        screen.into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

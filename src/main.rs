mod screen;

use iced::{futures::future, system, Element, Task, Theme};
use screen::{screen1, screen2, Screen};

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .run_with(App::new)
}

struct App {
    screen: Screen,
    system: Option<system::Information>,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded { system: Box<system::Information> },
    Screen1(screen1::Message),
    Screen2(screen2::Message),
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Loading,
                system: None,
            },
            Task::future(future::ok::<bool, bool>(true)).then(|_| {
                system::fetch_information()
                    .map(Box::new)
                    .map(move |system| Message::Loaded { system })
            }),
        )
    }

    fn title(&self) -> String {
        match &self.screen {
            Screen::Loading => "Loading".to_owned(),
            Screen::Screen1(screen1) => screen1.title(),
            Screen::Screen2(screen2) => screen2.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded { system } => {
                self.system = Some(*system);
                let (screen1, task) = screen1::Screen1::new();
                self.screen = Screen::Screen1(screen1);
                task.map(Message::Screen1)
            }
            Message::Screen1(message) => {
                if let Screen::Screen1(screen1) = &mut self.screen {
                    let action = screen1.update(message);
                    match action {
                        screen1::Action::GoToScreen2 => {
                            let (screen2, task) = screen2::Screen2::new();
                            self.screen = Screen::Screen2(screen2);
                            task.map(Message::Screen2)
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::Screen2(message) => {
                if let Screen::Screen2(screen2) = &mut self.screen {
                    let action = screen2.update(message);
                    match action {
                        screen2::Action::None => Task::none(),
                        screen2::Action::GoToScreen1 => {
                            let (screen1, task) = screen1::Screen1::new();
                            self.screen = Screen::Screen1(screen1);
                            task.map(Message::Screen1)
                        }
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Loading => screen::loading(),
            Screen::Screen1(screen1) => screen1.view(self.theme()).map(Message::Screen1),
            Screen::Screen2(screen2) => screen2.view().map(Message::Screen2),
        }
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

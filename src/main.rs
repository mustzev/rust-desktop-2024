mod data;
mod screen;
mod widget;

use iced::{futures::future, system, widget::row, Element, Task, Theme};
use screen::{poetry, screen1, screen2, Screen};
use widget::sidebar::{self, Sidebar};

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .run_with(App::new)
}

struct App {
    system: Option<system::Information>,
    sidebar: Sidebar,
    screen: Screen,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded { system: Box<system::Information> },
    Sidebar(sidebar::Message),
    Screen1(screen1::Message),
    Screen2(screen2::Message),
    Poetry(poetry::Message),
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                system: None,
                sidebar: Sidebar::new(Screen::Loading),
                screen: Screen::Loading,
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
            Screen::Poetry(poetry) => poetry.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded { system } => {
                self.system = Some(*system);
                let (poetry, task) = poetry::Poetry::new();
                self.screen = Screen::Poetry(poetry);
                task.map(Message::Poetry)
            }
            Message::Sidebar(message) => {
                let screen = self.sidebar.update(message);
                self.screen = screen;
                match self.screen {
                    Screen::Loading => Task::none(),
                    Screen::Screen1(_) => Task::none(),
                    Screen::Screen2(_) => Task::none(),
                    Screen::Poetry(_) => {
                        let (_poetry, task) = poetry::Poetry::new();
                        task.map(Message::Poetry)
                    }
                }
            }
            Message::Screen1(message) => {
                if let Screen::Screen1(screen1) = &mut self.screen {
                    let action = screen1.update(message);
                    match action {
                        screen1::Action::GoToScreen2 => {
                            let (screen2, task) = screen2::Screen2::new();
                            self.screen = screen::Screen::Screen2(screen2);
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
            Message::Poetry(message) => {
                if let Screen::Poetry(poetry) = &mut self.screen {
                    let task = poetry.update(message);
                    task.map(Message::Poetry)
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar.view().map(Message::Sidebar);
        let screen = match &self.screen {
            Screen::Loading => screen::loading(),
            Screen::Screen1(screen1) => screen1.view(self.theme()).map(Message::Screen1),
            Screen::Screen2(screen2) => screen2.view().map(Message::Screen2),
            Screen::Poetry(poetry) => poetry.view().map(Message::Poetry),
        };
        row![sidebar, screen].into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

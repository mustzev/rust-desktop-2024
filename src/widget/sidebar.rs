use iced::{
    widget::{mouse_area, text, Column},
    Element,
};

use crate::screen::{poetry::Poetry, screen1::Screen1, screen2::Screen2, Screen};

pub struct Sidebar {
    screen: Screen,
}

#[derive(Debug, Clone)]
pub enum Message {
    ScreenChanged(Screen),
}

impl Sidebar {
    pub fn new(screen: Screen) -> Self {
        Self { screen }
    }

    pub fn update(&mut self, message: Message) -> Screen {
        match message {
            Message::ScreenChanged(screen) => {
                self.screen = screen;
                self.screen.clone()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let items = vec![
            ("screen1", Screen::Screen1(Screen1::default())),
            ("screen2", Screen::Screen2(Screen2::default())),
            ("poetry", Screen::Poetry(Poetry::default())),
        ];
        Column::with_children(
            items
                .iter()
                .map(|(label, screen)| {
                    mouse_area(text(label.to_owned()))
                        .on_press(Message::ScreenChanged(screen.clone()))
                        .into()
                })
                .collect::<Vec<Element<Message>>>(),
        )
        .padding(10)
        .into()
    }
}

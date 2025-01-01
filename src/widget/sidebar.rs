use iced::{
    widget::{text, Column},
    Element, Task,
};

use crate::screen::Screen;

pub struct Sidebar {
    screen: Screen,
}

#[derive(Debug, Clone)]
pub enum Message {
    ScreenChanged(String),
}

impl Sidebar {
    pub fn new(screen: Screen) -> Self {
        Self { screen }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ScreenChanged(_) => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let items = vec!["screen1", "screen2", "poetry"];
        Column::with_children(
            items
                .iter()
                .map(|item| text(item.to_owned()).into())
                .collect::<Vec<Element<Message>>>(),
        )
        .padding(10)
        .into()
    }
}

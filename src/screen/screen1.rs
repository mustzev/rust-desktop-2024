use iced::{
    widget::{button, column, text},
    Element,
};

pub struct Screen1 {}

#[derive(Debug, Clone)]
pub enum Message {
    GoToScreen2,
}

impl Screen1 {
    pub fn new() -> Self {
        Screen1 {}
    }

    pub fn update(&mut self, message: Message) {}

    pub fn view<'a>(&self) -> Element<'a, Message> {
        column![]
            .push(text("screen-1"))
            .push(button(text("go to screen-2")).on_press(Message::GoToScreen2))
            .into()
    }
}

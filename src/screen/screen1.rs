use iced::{widget::text, Element};

pub struct Screen1 {}

#[derive(Debug)]
pub enum Message {}

impl Screen1 {
    pub fn new() -> Self {
        Screen1 {}
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        text("screen-1").into()
    }
}

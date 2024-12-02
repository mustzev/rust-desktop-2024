use iced::{widget::text, Element};

pub struct Screen2 {}

#[derive(Debug)]
pub enum Message {}

impl Screen2 {
    pub fn new() -> Self {
        Screen2 {}
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        text("screen-2").into()
    }
}

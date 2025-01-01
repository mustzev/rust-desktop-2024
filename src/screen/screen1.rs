use iced::{
    widget::{button, center, column, text},
    Alignment::Center,
    Element, Font, Task, Theme,
};

#[derive(Clone)]
pub struct Screen1 {}

#[derive(Debug, Clone)]
pub enum Message {
    GoToScreen2,
}

pub enum Action {
    GoToScreen2,
}

const NAME: &str = "screen1";

impl Screen1 {
    pub fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    pub fn title(&self) -> String {
        format!("{NAME} - App")
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToScreen2 => Action::GoToScreen2,
        }
    }

    pub fn view(&self, _theme: Theme) -> Element<Message> {
        let title = text(NAME).font(Font::MONOSPACE);
        let button = button(text("Go to screen2")).on_press(Message::GoToScreen2);
        center(
            column![title, button]
                .max_width(600)
                .spacing(10)
                .align_x(Center),
        )
        .padding(10)
        .into()
    }
}

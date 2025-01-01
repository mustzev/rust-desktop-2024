use iced::{
    widget::{button, column, container, text, text_input},
    Element, Task,
};

#[derive(Clone)]
pub struct Screen2 {
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    GoToScreen1,
}

pub enum Action {
    None,
    GoToScreen1,
}

const NAME: &str = "screen2";

impl Screen2 {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                input: String::new(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        format!("{NAME} - App")
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
                Action::None
            }
            Message::GoToScreen1 => Action::GoToScreen1,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let text_input =
            text_input("Type something here...", &self.input).on_input(Message::InputChanged);
        let button = button(text("Go to screen1")).on_press(Message::GoToScreen1);
        container(column![text_input, button].spacing(10))
            .padding(10)
            .into()
    }
}

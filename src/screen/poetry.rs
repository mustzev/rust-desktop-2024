use iced::{
    widget::{column, container},
    Element, Task,
};

pub struct Poetry {
    search: String,
    authors: Vec<String>,
    author: Option<String>,
    titles: Vec<String>,
    title: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    AuthorSelected(Option<String>),
    TitleSelected(Option<String>),
}

pub enum Action {
    None,
}

const NAME: &str = "Poetry";

impl Poetry {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                search: String::new(),
                authors: vec![],
                author: None,
                titles: vec![],
                title: None,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        format!("{NAME} - App")
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SearchChanged(search) => {
                self.search = search;
                Action::None
            }
            Message::AuthorSelected(_) => todo!(),
            Message::TitleSelected(_) => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = column![];
        container(content).padding(10).into()
    }
}

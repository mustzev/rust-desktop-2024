use iced::{
    widget::{button, column, container, text, text_input, Column},
    Element, Task,
};

use crate::data::poetry::{FetchPoetry, Poetry as PoetryStruct};

pub struct Poetry {
    search: String,
    fetch_poetry: FetchPoetry,
    authors: Vec<String>,
    author: Option<String>,
    titles: Vec<String>,
    title: Option<String>,
    poetry: Option<PoetryStruct>,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackButtonPressed,
    SearchChanged(String),
    AuthorsFetched(Vec<String>),
    AuthorSelected(Option<String>),
    TitlesFetched(Vec<String>),
    TitleSelected(Option<String>),
    PoetryFetched(PoetryStruct),
}

pub enum Action {
    None,
}

const NAME: &str = "Poetry";

impl Poetry {
    pub fn new() -> (Self, Task<Message>) {
        let fetch_poetry = FetchPoetry::new();
        (
            Self {
                search: String::new(),
                fetch_poetry: fetch_poetry.clone(),
                authors: vec![],
                author: None,
                titles: vec![],
                title: None,
                poetry: None,
            },
            Task::future(fetch_poetry.fetch_authors())
                .map(|result| Message::AuthorsFetched(result.unwrap())),
        )
    }

    pub fn title(&self) -> String {
        format!("{NAME} - App")
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::BackButtonPressed => todo!(),
            Message::SearchChanged(search) => {
                self.search = search;
                Action::None
            }
            Message::AuthorsFetched(authors) => {
                self.authors = authors;
                Action::None
            }
            Message::AuthorSelected(_) => todo!(),
            Message::TitlesFetched(_) => todo!(),
            Message::TitleSelected(_) => todo!(),
            Message::PoetryFetched(_poetry) => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let to_select_author = self.author.is_none();
        let _to_select_title = self.title.is_none();
        let content = column![]
            .push(button(text("<")).on_press(Message::BackButtonPressed))
            .push(
                text_input(
                    if to_select_author {
                        "Search author..."
                    } else {
                        "Search title.."
                    },
                    &self.search,
                )
                .on_input(Message::SearchChanged),
            )
            .push(text(if to_select_author {
                "Authors"
            } else {
                "Titles"
            }))
            .push(if to_select_author {
                Column::with_children(
                    self.authors
                        .iter()
                        .map(|author| text(author).into())
                        .collect::<Vec<Element<Message>>>(),
                )
            } else {
                Column::with_children(
                    self.titles
                        .iter()
                        .map(|title| text(title).into())
                        .collect::<Vec<Element<Message>>>(),
                )
            })
            .spacing(10);
        container(content).padding(10).into()
    }
}

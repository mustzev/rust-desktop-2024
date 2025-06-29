use iced::{
    widget::{button, column, container, mouse_area, row, scrollable, text, text_input, Column},
    Element, Task,
};

use crate::data::poetry::{FetchPoetry, Poetry as PoetryStruct};

#[derive(Debug, Default, Clone)]
pub struct Poetry {
    search: String,
    fetch_poetry: FetchPoetry,
    authors: Vec<String>,
    author: Option<String>,
    titles: Vec<String>,
    title: Option<String>,
    filtered_authors: Option<Vec<String>>,
    filtered_titles: Option<Vec<String>>,
    poetry: Option<PoetryStruct>,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackButtonPressed,
    SearchChanged(String),
    ClearButtonPressed(),
    SearchButtonPressed(),
    AuthorsFetched(Vec<String>),
    AuthorSelected(Option<String>),
    TitlesFetched(Vec<String>),
    TitleSelected(Option<String>),
    PoetryFetched(PoetryStruct),
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
                filtered_authors: None,
                filtered_titles: None,
                poetry: None,
            },
            Task::future(fetch_poetry.fetch_authors())
                .map(|result| Message::AuthorsFetched(result.unwrap())),
        )
    }

    pub fn title(&self) -> String {
        format!("{NAME} - App")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BackButtonPressed => todo!(),
            Message::SearchChanged(search) => {
                self.search = search.to_lowercase();
                Task::none()
            }
            Message::ClearButtonPressed() => {
                self.search = String::from("");
                self.author = None;
                self.title = None;
                self.titles = vec![];
                self.poetry = None;
                self.filtered_authors = None;
                self.filtered_titles = None;
                Task::none()
            }
            Message::SearchButtonPressed() => {
                let mut filter_authors = || {
                    self.filtered_authors = Some(
                        self.authors
                            .iter()
                            .filter(|author| author.to_lowercase().contains(&self.search))
                            .cloned()
                            .collect(),
                    );
                };
                let mut filter_titles = || {
                    self.filtered_titles = Some(
                        self.titles
                            .iter()
                            .filter(|title| title.to_lowercase().contains(&self.search))
                            .cloned()
                            .collect(),
                    );
                };
                match (&self.author, &self.title) {
                    (Some(_author), Some(_title)) => filter_titles(),
                    (Some(_author), None) => filter_titles(),
                    (None, Some(_title)) => filter_authors(),
                    (None, None) => filter_authors(),
                }
                Task::none()
            }
            Message::AuthorsFetched(authors) => {
                self.authors = authors;
                Task::none()
            }
            Message::AuthorSelected(author) => {
                self.author = author.to_owned();
                if author.is_some() {
                    Task::future(self.fetch_poetry.to_owned().fetch_titles(author.unwrap()))
                        .map(|result| Message::TitlesFetched(result.unwrap()))
                } else {
                    Task::none()
                }
            }
            Message::TitlesFetched(titles) => {
                self.titles = titles;
                Task::none()
            }
            Message::TitleSelected(title) => {
                self.title = title.to_owned();
                if title.is_some() {
                    Task::future(self.fetch_poetry.to_owned().fetch_poetry(title.unwrap()))
                        .map(|result| Message::PoetryFetched(result.unwrap()))
                } else {
                    Task::none()
                }
            }
            Message::PoetryFetched(poetry) => {
                self.poetry = Some(poetry);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let to_select_author = self.author.is_none();
        let content = column![]
            .push(button(text("back")).on_press(Message::BackButtonPressed))
            .push(
                row![]
                    .push(
                        text_input(
                            if to_select_author {
                                "Search author..."
                            } else {
                                "Search title..."
                            },
                            &self.search,
                        )
                        .on_input(Message::SearchChanged),
                    )
                    .push(button(text("clear")).on_press(Message::ClearButtonPressed()))
                    .push(button(text("search")).on_press(Message::SearchButtonPressed()))
                    .spacing(10),
            )
            .push(
                row![]
                    .push(
                        column![
                            text("Authors"),
                            scrollable(Column::with_children(
                                self.filtered_authors
                                    .as_ref()
                                    .unwrap_or(&self.authors)
                                    .iter()
                                    .map(|author| {
                                        mouse_area(text(author))
                                            .on_press(Message::AuthorSelected(Some(
                                                author.to_owned(),
                                            )))
                                            .into()
                                    })
                                    .collect::<Vec<Element<Message>>>(),
                            ))
                            .width(300),
                        ]
                        .spacing(10),
                    )
                    .push(
                        column![
                            text("Titles"),
                            scrollable(Column::with_children(
                                self.filtered_titles
                                    .as_ref()
                                    .unwrap_or(&self.titles)
                                    .iter()
                                    .map(|title| {
                                        mouse_area(text(title))
                                            .on_press(Message::TitleSelected(Some(
                                                title.to_owned(),
                                            )))
                                            .into()
                                    })
                                    .collect::<Vec<Element<Message>>>(),
                            ))
                            .width(300),
                        ]
                        .spacing(10),
                    )
                    .push(
                        column![
                            text("Poetry"),
                            match &self.poetry {
                                Some(poetry) => column![
                                    text(format!("{} - {}", poetry.author, poetry.title)),
                                    Column::with_children(
                                        poetry
                                            .lines
                                            .iter()
                                            .map(|line| text(line).into())
                                            .collect::<Vec<Element<Message>>>(),
                                    )
                                ]
                                .spacing(10),
                                None => column![text("Select author and title")],
                            }
                        ]
                        .spacing(10),
                    )
                    .spacing(20),
            )
            .spacing(20);
        container(content).padding(15).into()
    }
}

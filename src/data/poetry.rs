use reqwest::{Client, Error};
use serde::Deserialize;

const POETRYDB_URL: &str = "https://poetrydb.org";

#[derive(Debug, Clone)]
pub struct FetchPoetry {
    client: Client,
}

#[derive(Debug, Deserialize)]
struct AuthorsRes {
    authors: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Title {
    title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Poetry {
    author: String,
    title: String,
    lines: Vec<String>,
    linecount: String,
}

impl FetchPoetry {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_authors(self) -> Result<Vec<String>, Error> {
        let authors_res = self
            .client
            .get(POETRYDB_URL.to_owned() + "/author")
            .send()
            .await?
            .json::<AuthorsRes>()
            .await?;
        Ok(authors_res.authors)
    }

    pub async fn fetch_titles(self, author: &str) -> Result<Vec<String>, Error> {
        let titles_res = self
            .client
            .get(format!("{POETRYDB_URL}/author/{author}/title"))
            .send()
            .await?
            .json::<Vec<Title>>()
            .await?;
        let titles = titles_res
            .iter()
            .map(|title| title.title.to_owned())
            .collect();
        Ok(titles)
    }

    pub async fn fetch_poetry(self, title: &str) -> Result<Poetry, Error> {
        let poetry_res = self
            .client
            .get(format!("{POETRYDB_URL}/title/{title}:abs"))
            .send()
            .await?
            .json::<Vec<Poetry>>()
            .await?;
        Ok(poetry_res[0].clone())
    }
}

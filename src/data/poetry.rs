use reqwest::{Client, Error};

pub async fn fetch() -> Result<String, Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    Ok(body)
}

const POETRYDB_URL: &str = "https://poetrydb.org";

pub async fn fetch_authors() -> Result<String, Error> {
    let client = Client::new();
    let body = client
        .get(POETRYDB_URL.to_owned() + "/auhtor")
        .send()
        .await?
        .json()
        .await?;
    Ok(body)
}

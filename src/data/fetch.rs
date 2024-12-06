use reqwest::Error;

pub async fn fetch() -> Result<String, Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    Ok(body)
}

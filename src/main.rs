use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Manga {
    id: i32,
    hid: String,
    title: String,
    desc: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let res = fetch_manga_list(&args[1]).await?;
    println!("{:#?}", res);
    Ok(())
}

async fn fetch_manga_list(_query: &str) -> Result<Vec<Manga>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(format!("https://api.comick.fun/v1.0/search?q=naruto&tachiyomi=true"))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .header("Accept", "application/json")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Referer", "https://comick.fun/")
            .header("Origin", "https://comick.fun")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-site")
            .send().await?;
    let body = response.text().await?;
    match serde_json::from_str::<Vec<Manga>>(&body) {
        Ok(value) => {
            return Ok(value);
        }
        Err(e) => {
            eprintln!("error during deseriaization {}", e);
            eprintln!("response body {}", body);
            return Err(e.into());
        }
    };
}

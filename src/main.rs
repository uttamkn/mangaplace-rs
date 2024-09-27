use clap::{Arg, Command};
use reqwest::Client;

fn main() {
    let matches = Command::new("My Program")
        .version("1.0")
        .author("shashank the great")
        .about("Demonstrates query, info, and downloading mangas")
        .subcommand(
            Command::new("search").about("Search related mangas").arg(
                Arg::new("SEARCH")
                    .help("used to search similar mangas")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("info").about("Get information").arg(
                Arg::new("INFO")
                    .help("used to get info about manga")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("download").about("Download something").arg(
                Arg::new("DOWNLOAD")
                    .help("give manga download [name] to download the manga name")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    // let value = if let Some(matches) = matches.subcommand_matches("search") {
    //     matches.get_one::<String>("SEARCH").unwrap()
    // } else if let Some(matches) = matches.subcommand_matches("info") {
    //     matches.get_one::<String>("INFO").unwrap()
    // } else if let Some(matches) = matches.subcommand_matches("download") {
    //     matches.get_one::<String>("DOWNLOAD").unwrap()
    // } else {
    //     println!("No command was used");
    //     return;
    // };
    //
    let mut value = String::new();
    if let Some(matches) = matches.subcommand_matches("search") {
        value = matches.get_one::<String>("SEARCH").unwrap().to_owned();
    }

    else if let Some(matches) = matches.subcommand_matches("info") {
        value = matches.get_one::<String>("INFO").unwrap().to_owned();
    }

    else if let Some(matches) = matches.subcommand_matches("download") {
        value = matches.get_one::<String>("DOWNLOAD").unwrap().to_owned();
    }

    println!("The value is: {}", value);
}

#[allow(dead_code)]
async fn fetch_manga_list(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(format!("https://api.comick.fun/comic/{}/chapters?lang=en&limit=99999&tachiyomi=true",query))
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
    .header("Accept", "application/json, text/plain, */*")
    .header("Accept-Language", "en-US,en;q=0.9")
    .header("Referer", "https://comick.fun/")
    .header("Origin", "https://comick.fun")
    .header("Sec-Fetch-Dest", "empty")
    .header("Sec-Fetch-Mode", "cors")
    .header("Sec-Fetch-Site", "same-site")
    .send().await?;

    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}

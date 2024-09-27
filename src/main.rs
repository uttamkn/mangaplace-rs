use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

        let _res = fetch_manga_with_similar_names(&value).await?;
    } else if let Some(matches) = matches.subcommand_matches("info") {
        value = matches.get_one::<String>("INFO").unwrap().to_owned();
    } else if let Some(matches) = matches.subcommand_matches("download") {
        value = matches.get_one::<String>("DOWNLOAD").unwrap().to_owned();
    }

    println!("The value is: {}", value);
    return Ok(());
}

#[allow(dead_code)]
async fn fetch_manga_with_similar_names(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.comick.fun/v1.0/search?q={}&tachiyomi=true",
        query
    );
    let header = headers();
    let res = client.get(&url).headers(header).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => println!("i got something"),
        reqwest::StatusCode::FORBIDDEN => println!("it is saying forbidden"),
        _ => println!("i don't care"),
    }
    return Ok(());
}

#[allow(dead_code)]
fn headers() -> reqwest::header::HeaderMap {
    let mut header = reqwest::header::HeaderMap::new();
    header.insert("User-Agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
    header.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json, text/plain, */*"),
    );
    header.insert(
        "Accept-Language",
        reqwest::header::HeaderValue::from_static("en-US,en;q=0.9"),
    );
    header.insert(
        "Referer",
        reqwest::header::HeaderValue::from_static("https://comick.fun/"),
    );
    header.insert(
        "Origin",
        reqwest::header::HeaderValue::from_static("https://comick.fun"),
    );
    header.insert(
        "Sec-Fetch-Dest",
        reqwest::header::HeaderValue::from_static("empty"),
    );
    header.insert(
        "Sec-Fetch-Mode",
        reqwest::header::HeaderValue::from_static("cors"),
    );
    header.insert(
        "Sec-Fetch-Site",
        reqwest::header::HeaderValue::from_static("same-site"),
    );

    return header;
}

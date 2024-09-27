use clap::{Arg, Command};

// NOTE: i will handle getting query in the main function itself
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

    match matches.subcommand() {
        Some(("search", matching)) => {
            let res = fetch_manga_with_similar_names(
                matching
                    .get_one::<String>("SEARCH")
                    .expect("expected a search arguement")
                    .to_owned(),
            )
            .await;

            match res {
                Ok(out) => {
                    let val: serde_json::Value = serde_json::from_str(&out).expect("string to json failed");
                    println!("{:#?}", val);
                },
                Err(e) => {
                    eprintln!("the error you got is {e}");
                }
            }
        }
        Some(("info", matching)) => {
            println!("{}", matching.get_one::<String>("INFO").unwrap().to_owned())
        }
        Some(("download", matching)) => println!(
            "{}",
            matching.get_one::<String>("DOWNLOAD").unwrap().to_owned()
        ),
        Some((name, _matching)) => println!("unimplemented: {:?}", name),
        None => unreachable!("subcommand required"),
    }

    return Ok(());
}

#[allow(dead_code)]
async fn fetch_manga_with_similar_names(
    query: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.comick.fun/v1.0/search?q={}&tachiyomi=true",
        query
    );
    let header = headers();
    let res = client.get(&url).headers(header).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("Successfully received a response");
            let text = res.text().await?;
            return Ok(text);
        }
        reqwest::StatusCode::FORBIDDEN => {
            eprintln!("Access forbidden: Check your headers or API access.");
        }
        reqwest::StatusCode::NOT_FOUND => {
            eprintln!("Manga not found.");
        }
        _ => {
            eprintln!("Unexpected response status: {}", res.status());
        }
    }
    return Ok(res.json().await?);
}

#[allow(dead_code)]
fn headers() -> reqwest::header::HeaderMap {
    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_static("HTTPie/3.2.3"),
    );
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

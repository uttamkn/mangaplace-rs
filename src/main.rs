// files
mod apis;
mod models;

// imports
use apis::fetch_manga_list;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let res = fetch_manga_list(&args[1]).await?;
    println!("{:#?}", res);
    Ok(())
}

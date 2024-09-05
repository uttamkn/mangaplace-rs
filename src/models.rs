use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Manga {
    id: i32,
    hid: String,
    title: String,
    desc: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BookMetadata {
    pub id: usize,
    pub title: String,
    pub author: String,
    pub page_count: usize,
    pub cover_img: Box<[u8]>,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BookPage {
    pub id: usize,
    pub content: String,
}

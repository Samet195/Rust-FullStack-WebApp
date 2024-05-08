use serde::{Deserialize, Serialize};

pub use page::*;
pub use section::*;

mod page;
mod section;

#[derive(Deserialize, Serialize)]
pub struct BookContent {
    pub sections: Vec<BookSection>,
    pub pages: Vec<BookPage>,
}

use serde::{Deserialize, Serialize};

pub use content::*;
pub use metadata::*;

mod content;
mod metadata;

#[derive(Deserialize, Serialize)]
pub struct Book {
    pub metadata: BookMetadata,
    pub content: BookContent,
}

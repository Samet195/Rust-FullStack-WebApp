use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BookSection {
    pub id: usize,
    pub name: String,
}

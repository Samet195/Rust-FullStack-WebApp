use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorJson {
    pub message: String,
}

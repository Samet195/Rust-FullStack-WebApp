use std::{fs::ReadDir, slice::Iter};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TestData {
    pub content: Vec<String>,
}

impl TestData {
    pub fn iter(&self) -> Iter<'_, String> {
        self.content.iter()
    }
}

impl From<ReadDir> for TestData {
    fn from(value: ReadDir) -> Self {
        Self {
            content: value
                .map(|k| k.unwrap().path().to_str().unwrap().to_string())
                .collect(),
        }
    }
}

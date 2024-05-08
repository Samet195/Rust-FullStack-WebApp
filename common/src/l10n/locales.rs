use std::collections::BTreeMap;
use std::ops::Deref;

use rust_embed::RustEmbed;
use rust_i18n::Backend;

#[derive(RustEmbed)]
#[folder = "locales"]
struct EmbeddedLocales;

pub struct Locales {
    locales: BTreeMap<Box<str>, BTreeMap<Box<str>, Box<str>>>,
}

impl Locales {
    pub fn new() -> Self {
        let mut locales = BTreeMap::new();

        for file in EmbeddedLocales::iter() {
            let locale = file
                .strip_suffix(".yml")
                .unwrap()
                .to_string()
                .into_boxed_str();

            let content: BTreeMap<Box<str>, Box<str>> = serde_yaml::from_slice(
                EmbeddedLocales::get(&file)
                    .unwrap()
                    .data
                    .to_vec()
                    .as_slice(),
            )
            .unwrap();

            locales.insert(locale, content);
        }

        Self { locales }
    }
}

impl Default for Locales {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for Locales {
    fn available_locales(&self) -> Vec<&str> {
        self.locales.keys().map(|key| &**key).collect()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        self.locales.get(locale)?.get(key).map(Deref::deref)
    }
}

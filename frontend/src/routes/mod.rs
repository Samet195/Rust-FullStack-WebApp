// Standart Library Imports
use std::fmt::{Debug, Display};

// Internal Module Imports
use crate::utils::close_menu;

// Internal Modules
mod home;
mod not_found;
mod search;

pub use self::home::TabsRouter;

// Extenal Crates Imports
use gloo_utils::document;
use rust_i18n::t;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

pub type Link = yew_router::prelude::Link<Route>;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/*tab")]
    Tabs { tab: TabsRouter },

    #[at("/search")]
    Search,

    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn switch(self) -> Html {
        use Route::*;
        self.route_hook().unwrap();

        match self {
            Home => {
                html! {<home::Home tab={TabsRouter::About} />}
            }
            Tabs { tab } => html! {<home::Home {tab} />},
            Search => html! {<search::Search />},
            NotFound => html! { <not_found::NotFound /> },
        }
    }

    fn route_hook(&self) -> Result<(), JsValue> {
        if let Some(el) = document().query_selector("title")? {
            let sep = el.get_attribute("sep").unwrap();
            let title = el.text_content().unwrap();
            let base = title.split(&sep).next().unwrap().trim();

            let title = if self.to_string() == *"" {
                base.to_string()
            } else {
                format!("{} {} {}", t!("title"), sep, self)
            };

            el.set_text_content(Some(title.as_str()));

            let _ = js_sys::eval(
                format!(
                    "window.webkit.messageHandlers.external.postMessage('{}')",
                    &title
                )
                .as_str(),
            );
        };

        close_menu::<&str>("left_menu");
        close_menu::<&str>("right_menu");

        Ok(())
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Route::*;

        match self {
            Home => write!(f, ""),
            Tabs { tab } => write!(f, "{}", tab),
            _ => write!(f, "{}", t!(format!("{:?}", self).as_str())),
        }
    }
}

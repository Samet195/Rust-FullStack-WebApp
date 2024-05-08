// Extenal Crates Imports
use rust_i18n::t;
use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        {t!("not_found")}
    }
}

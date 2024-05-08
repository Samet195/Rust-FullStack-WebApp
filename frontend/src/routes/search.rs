// Extenal Crates Imports
use yew::prelude::*;

pub struct Search;

impl Component for Search {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {<>
            <div class="searchbar-backdrop"></div>
                <div class="list list-strong-ios list-outline-ios list-dividers-ios simple-list searchbar-not-found">
                <ul>
                    <li>{"Nothing found"}</li>
                </ul>
            </div>
            <div class="list list-strong-ios list-outline-ios list-dividers-ios search-list searchbar-found">

            </div>
        </>}
    }
}

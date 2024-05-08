// Internal Module Imports
use crate::{
    routes::{Link, Route},
    utils::open_menu_cb,
};

// Extenal Crates Imports
use rust_i18n::t;
use yew::prelude::*;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="header pink">
                <div class="left">
                    <button class="icon ion-navicon-round" onclick={open_menu_cb("left_menu")}></button>
                </div>
                <h1>{t!("title")}</h1>
                <div class="right">
                    <Link to={Route::Search}><button class="icon ion-search"></button></Link>
                    <button class="icon ion-android-more-vertical" onclick={open_menu_cb("right_menu")}></button>
                </div>
            </div>
        }
    }
}

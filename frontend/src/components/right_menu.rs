// Internal Module Imports
use crate::utils::close_menu_cb;

// Extenal Crates Imports
use rust_i18n::t;
use yew::prelude::*;

pub struct RightMenu;

impl Component for RightMenu {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="menu menu-right green" id="right_menu">
                <button class="green-100 full" onclick={close_menu_cb("right_menu")}>{t!("Close menu")}</button>
            </div>
        }
    }
}

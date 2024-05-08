// Internal Module Imports
use crate::routes::{Link, Route};

// Extenal Crates Imports
use rust_i18n::t;
use yew::prelude::*;

pub struct LeftMenu;

impl Component for LeftMenu {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="menu red padding" id="left_menu">
                <Link to={Route::Home}><button class="white full margin-bottom">{t!("Page1")}</button></Link>
                <Link to={Route::Search}><button class="white full margin-bottom">{t!("Page2")}</button></Link>
                <Link to={Route::NotFound}><button class="white full margin-bottom">{t!("Page3")}</button></Link>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            js_sys::eval("window.menu.enableSwiper('left_menu')").unwrap();
        }
    }
}

// Internal Modules
mod components;
mod routes;
mod services;
mod utils;

// Internal Module Imports
use crate::components::{Head, Header, LeftMenu, RightMenu};
use crate::routes::Route;

// Extenal Crates Imports
use common::l10n::prelude::*;
use rust_i18n::t;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::{html, Component, Context, Html};
use yew_router::{BrowserRouter, Switch};

struct App;

pub enum Msg {
    ChangeLang(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeLang(lang) => rust_i18n::set_locale(&lang),
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let cb_en = ctx.link().callback(|_| Msg::ChangeLang("en".into()));
        // let cb_tr = ctx.link().callback(|_| Msg::ChangeLang("tr".into()));

        html! {
            <BrowserRouter>
                <Head />
                <body class="padding grey-200 has-header">
                    <Header />
                    <LeftMenu />
                    <RightMenu />
                    // <button class="red" onclick={cb_en}>{t!("EN")}</button>
                    // <button class="blue" onclick={cb_tr}>{t!("TR")}</button>
                    <main class="content">
                        <Switch<Route> render={Route::switch} />
                    </main>
                </body>
            </BrowserRouter>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            js_sys::eval(format!("document.title='{0}'", t!("title")).as_str()).unwrap();
        }
    }
}

#[wasm_bindgen(start)]
fn main() {
    let lang = common::l10n::detect_locale().unwrap_or("en".into());
    rust_i18n::set_locale(lang.as_str());
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    let root = gloo_utils::document().document_element().unwrap();
    yew::Renderer::<App>::with_root(root).render();
}

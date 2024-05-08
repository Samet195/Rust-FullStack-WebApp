// Extenal Crates Imports
use rust_i18n::t;
use yew::prelude::*;

pub struct Head;
impl Component for Head {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {<>
            <meta charset="utf-8" />
            <meta name="viewport" content="user-scalable=no, initial-scale=1, maximum-scale=1, minimum-scale=1, width=device-width" />
            <title sep="-">{t!("title")}</title>
            <style>{"html,body{max-width:100%;overflow-x:hidden}"}</style>
            <style>{include_str!("../../css/mobileui.min.css")}</style>
            // <script>{include_str!("../../js/cordova.min.js")}</script>
            <script>{include_str!("../../js/mobileui.min.js")}</script>
        </>}
    }
}

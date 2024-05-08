use std::{fmt::Display, str::FromStr};

use rust_i18n::t;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::TabBar;

pub struct Home;

#[derive(PartialEq, Properties)]
pub struct HomeProps {
    pub tab: TabsRouter,
}

impl Component for Home {
    type Message = ();
    type Properties = HomeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let body = gloo_utils::body();
        let mut classes = body.class_name();

        classes.push_str(" has-sub-header");
        body.set_class_name(classes.as_str());

        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {<>
            <TabBar<TabsRouter> active_tab={ctx.props().tab.clone()} />
            <Switch<TabsRouter> render={TabsRouter::switch} />
        </>}
    }
    fn destroy(&mut self, _ctx: &Context<Self>) {
        let body = gloo_utils::body();
        let classes = body.class_name().replace(" has-sub-header", "");

        body.set_class_name(classes.as_str());
    }
}

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum TabsRouter {
    #[at("/")]
    About,

    #[at("/events")]
    Events,

    #[at("/messages")]
    Messages,

    #[at("/groups")]
    Groups,

    #[not_found]
    #[at("/404")]
    NotFound,
}

impl TabsRouter {
    pub fn switch(self) -> Html {
        use TabsRouter::*;

        match self {
            About => html! {
                <div class="tab-content active">
                    <h2 class="text-strong padding text-pink">{t!("About")}</h2>
                    <p>{t!("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")}</p>
                </div>
            },
            Events => html! {
                <div class="tab-content active">
                    <h2 class="text-strong padding text-pink">{t!("Events")}</h2>
                    <p>{t!("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")}</p>
                </div>
            },
            Messages => html! {
                <div class="tab-content active">
                    <h2 class="text-strong padding text-pink">{t!("Messages")}</h2>
                    <p>{t!("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")}</p>
                </div>
            },
            Groups => html! {
                <div class="tab-content active">
                    <h2 class="text-strong padding text-pink">{t!("Groups")}</h2>
                    <p>{t!("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")}</p>
                </div>
            },
            NotFound => html! {
                <h2 class="text-strong padding text-pink">{t!("NotFound")}</h2>
            },
        }
    }
}

impl Display for TabsRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", t!(format!("{:?}", self).as_str()))
    }
}

impl FromStr for TabsRouter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::recognize(s).ok_or(())
    }
}

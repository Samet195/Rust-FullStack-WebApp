use rust_i18n::t;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::TabsRouter;

pub struct TabBar<R: Routable + 'static> {
    active_tab: R,
}

pub enum TabBarMsg<R: Routable + 'static> {
    SwitchTab(R),
}

#[derive(PartialEq, Properties)]
pub struct TabBarProps<R: Routable + 'static> {
    pub active_tab: R,
}

impl Component for TabBar<TabsRouter> {
    type Message = TabBarMsg<TabsRouter>;
    type Properties = TabBarProps<TabsRouter>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            active_tab: ctx.props().active_tab.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabBarMsg::SwitchTab(tab) => {
                self.active_tab = tab.clone();
                ctx.link().navigator().unwrap().push(&tab);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use TabsRouter::*;

        let cb = |tab: TabsRouter| {
            ctx.link()
                .callback(move |_| TabBarMsg::SwitchTab(tab.clone()))
        };

        let active = match self.active_tab {
            About => ("active", "", "", ""),
            Events => ("", "active", "", ""),
            Messages => ("", "", "active", ""),
            Groups => ("", "", "", "active"),
            NotFound => ("", "", "", ""),
        };

        html! {
            <div class="header pink sub shadow tab">
                <button class={format!("icon ion-android-person {}",active.0)} onclick={cb(About)}>{t!("About")}</button>
                <button class={format!("icon ion-android-list {}",active.1)} onclick={cb(Events)}>{t!("Events")}  </button>
                <button class={format!("icon ion-android-chat {}",active.2)} onclick={cb(Messages)}>{t!("Messages")}</button>
                <button class={format!("icon ion-android-people {}",active.3)} onclick={cb(Groups)}>{t!("Groups")}</button>
            </div>
        }
    }
}

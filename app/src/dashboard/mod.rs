mod api;

use crate::utils::icons::Icon;
use leptos::prelude::*;
use leptos_meta::Title;
use std::string::ToString;
use stylance::import_style;

import_style!(style, "./dashboard.module.scss");

#[component]
fn nav_button_url(icon:String,name:String,url:String) -> impl IntoView{
    view! {
        <div class="nav-button">
            <a href="/">
                <Icon name=icon />
            </a>
        </div>
    }
}

type CallbackList = Vec<Action<(),()>>;
#[island]
fn nav_button_onclick(icon:String,name:String,callback_index:usize) -> impl IntoView
where {
    let callback = expect_context::<CallbackList>();
    view! {
        <div class="nav-button" on:click=move |_| {callback[callback_index].dispatch(());}>
            <span>
                <Icon name=icon />
            </span>
        </div>
    }
}


#[island]
pub fn dashboard() -> impl IntoView {
    let test_action = Action::new(
        |&()| async move{
        todo!();
    });
    let context:CallbackList =  vec!(test_action);
    provide_context(context);
    view! {
        <Title text="Dashboard" />
        <div class=style::dashboardcontainer>
            <nav>
                <NavButtonUrl
                    icon="ri-home-fill".to_string()
                    name="Home".to_string()
                    url="/".to_string()
                />
                <NavButtonOnclick
                    icon="ri-home-fill".to_string()
                    name="Home".to_string()
                    callback_index=0
                />
            </nav>
            <div>POTATO</div>
        </div>
    }
}

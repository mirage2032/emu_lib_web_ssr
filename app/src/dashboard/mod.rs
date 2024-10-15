mod api;

use crate::utils::icons::Icon;
use leptos::prelude::*;
use leptos_meta::Title;
use std::string::ToString;
use leptos::task::spawn_local;
use stylance::{classes, import_style};
use crate::utils::cookie;
use crate::utils::cookie::CookieKey;

import_style!(style, "./dashboard.module.scss");

#[component]
fn nav_button_url(
    icon: String,
    name: String,
    url: String,
    #[prop(optional)] extra_class: String,
) -> impl IntoView {
    view! {
        <div class=format!("nav-button{}", extra_class)>
            <a href="/">
                <Icon name=icon />
            </a>
        </div>
    }
}

type CallbackList = Vec<Action<(), ()>>;
#[island]
fn nav_button_onclick(
    icon: String,
    name: String,
    callback_index: usize,
    extra_class: Option<String>,
) -> impl IntoView
where {
    let extra = match extra_class {
        Some(data) => format!(" {}", data),
        None => String::default(),
    };
    let callback = expect_context::<CallbackList>();
    view! {
        <div
            class=format!("nav-button{}", extra)
            on:click=move |_| {
                callback[callback_index].dispatch(());
            }
        >
            <span>
                <Icon name=icon />
            </span>
        </div>
    }
}

#[island]
fn inner_dashboard() -> impl IntoView {
    cookie::wasm::set(&CookieKey::Other("ba"),"dd",std::time::Duration::from_secs(60*24));
    let test_action = Action::new(|&()| async move {
        todo!();
    });
    let context: CallbackList = vec![test_action];
    provide_context(context);
    view! {
        <div class=style::maincontainer>
            <div class=style::navcontainer>
                <nav>
                    <div class=classes!(style::navgroup,
        style::fullheight)>
                        <NavButtonUrl
                            icon="ri-home-fill".to_string()
                            name="Home".to_string()
                            url="/".to_string()
                        />
                        <NavButtonOnclick
                            icon="ri-logout-box-fill".to_string()
                            name="Home".to_string()
                            extra_class=None
                            callback_index=0
                        />
                        <NavButtonOnclick
                            icon="ri-cpu-line".to_string()
                            name="Home".to_string()
                            extra_class=None
                            callback_index=0
                        />
                    </div>
                    <div class=style::navgroup style:height="5rem">
                        <NavButtonOnclick
                            icon="ri-logout-box-fill".to_string()
                            name="Home".to_string()
                            extra_class=Some(style::nobottompadding.to_string())
                            callback_index=0
                        />
                    </div>
                </nav>
            </div>
        </div>
    }
}
#[component]
pub fn dashboard() -> impl IntoView {
    view!{
        <Title text="Dashboard" />
        <InnerDashboard/>
    }
}
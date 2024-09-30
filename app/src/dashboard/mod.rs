mod api;

use crate::utils::icons::Icon;
use leptos::prelude::*;
use leptos_meta::Title;
use std::string::ToString;
use stylance::import_style;

import_style!(style, "./dashboard.module.scss");
#[component]
pub fn dashboard() -> impl IntoView {
    view! {
        <Title text="Dashboard" />
        <div class=style::dashboardcontainer>
            <nav>
                <Icon name="ri-home-fill".to_string() />
            </nav>
            <div>POTATO</div>
        </div>
    }
}

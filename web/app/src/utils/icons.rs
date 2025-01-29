use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn IconsCDN() -> impl IntoView {
    view! { 
        <Stylesheet href="https://cdn.jsdelivr.net/npm/remixicon@4.3.0/fonts/remixicon.css" />
    }
}

#[component]
pub fn Icon(name: String) -> impl IntoView {
    view! { <i class=name></i> }
}

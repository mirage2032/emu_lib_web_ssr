use leptos::{component, view, IntoView};
use leptos_router::A;

stylance::import_style!(style,"./header.module.scss");
#[component]
pub fn simple_header(title:String) -> impl IntoView {
    view! {
        <header class=style::simpleheader>
                <A href="/">"Home"</A>
                <h1>{title}</h1>
                <div></div>
        </header>
    }
}
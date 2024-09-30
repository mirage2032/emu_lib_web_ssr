use leptos::prelude::*;
use leptos_router::components::A;

stylance::import_style!(style, "./footer.module.scss");
#[component]
pub fn footer() -> impl IntoView {
    view! {
        <footer class=style::footerclass>
            <div class=style::footercontainer>
                <p>Z80 Emulator</p>
                <p>Built by <A href="https://github.com/mirage2032">Popescu Ionut Alexandru</A></p>
            </div>
        </footer>
    }
}

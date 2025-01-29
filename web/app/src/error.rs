use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use thiserror::Error;

stylance::import_style!(style, "./error.module.scss");

#[component]
pub fn Error(#[prop(optional)] code: Option<i32>, message: String) -> impl IntoView {
    view! {
        <Title text="Error" />
        <div class=style::errorcontainer>
            <SimpleHeader title="Error".to_string() />
                <div class=style::errormain>
                    <h2>
                        {match code {
                            Some(code) => {
                                Some(
                                    view! {
                                        <span>{code}</span>
                                        " "
                                    },
                                )
                            }
                            None => None,
                        }} {message}
                    </h2>
                </div>
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Not Found" />
            <div class=style::errorcontainer>
                <SimpleHeader title="Error".to_string() />
                    <div class=style::errormain>
                        <h2><span>{404}</span>" Not Found"</h2>
                    </div>
            </div>
    }
}

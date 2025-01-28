use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use thiserror::Error;

stylance::import_style!(style, "./error.module.scss");

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("404 - Not Found")]
    NotFound,
    #[error("Error! code: {0}, message: {1}")]
    CustomCodeMsg(i32, String),
    #[error("Error! message: {0}")]
    CustomMsg(String),
}

impl AppError {
    pub fn into_view(self) -> impl IntoView {
        match self {
            AppError::NotFound => view! { <Error code=404 message="Not Found".to_string() /> },
            AppError::CustomCodeMsg(code, message) => view! { <Error code=code message=message /> },
            AppError::CustomMsg(message) => view! { <Error message=message /> },
        }
    }
}
#[component]
pub fn Error(#[prop(optional)] code: Option<i32>, message: String) -> impl IntoView {
    // provide_meta_context();
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

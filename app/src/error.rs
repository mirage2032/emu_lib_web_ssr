use leptos::{component, view, IntoView, View};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::A;
use thiserror::Error;

stylance::import_style!(style,"./error.module.scss");

#[derive(Clone, Debug,Error)]
pub enum AppError {
    #[error("404 - Not Found")]
    NotFound,
    #[error("Error! code: {0}, message: {1}")]
    Custom(i32,String),
}

impl IntoView for AppError {
    fn into_view(self) -> View {
        match self {
            AppError::NotFound => view! { <Error code=404 message="Not Found".to_string() /> },
            AppError::Custom(code,message) => view! { <Error code=code message=message /> }
        }
    }
}
#[component]
pub fn Error(
    #[prop(optional)]
    code: Option<i32>,
    message: String,
) -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Error" />
        <div class=style::errorcontainer>
            <header>
                <A href="/">"Home"</A>
                <h1>"ERROR"</h1>
                <div></div>
            </header>
            <main>
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
            </main>
        </div>
    }
}
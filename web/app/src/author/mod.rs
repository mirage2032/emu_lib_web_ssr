use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::Title;

mod resume_data;
mod resume_view;

stylance::import_style!(style, "./resume.module.scss");

#[component]
fn ResumeError(message: String) -> impl IntoView {
    view! {
        <div class=style::errorcontainer>
            <p>{message}</p>
        </div>
    }
}

#[component]
pub fn ResumeTransition() -> impl IntoView {
    let resume = Resource::new(
        || (),
        move |_| async move { resume_data::fetch_resume().await },
    );
    view! {
        <Transition fallback=|| view! { <ResumeError message="Loading...".to_string() /> }>
            <p>
                {move || Suspend::new(async move {
                    resume
                        .with(|resume| {
                            match resume {
                                Some(e) => {
                                    match e.as_ref() {
                                        Ok(resume) => {
                                            let resume = Signal::from(resume.clone());
                                            view! { <resume_view::Resume resume /> }.into_any()
                                        }
                                        _ => {
                                            view! {
                                                <ResumeError message="Failed to load resume".to_string() />
                                            }
                                                .into_any()
                                        }
                                    }
                                }
                                _ => {
                                    view! {
                                        <ResumeError message="Failed to load resume".to_string() />
                                    }
                                        .into_any()
                                }
                            }
                        })
                })}
            </p>
        </Transition>
    }
}
#[component]
pub fn Author() -> impl IntoView {
    view! {
        <Title text="Author" />
        <SimpleHeader title="Author".to_string() />
        <div class=style::authorcontainer>
            <ResumeTransition />
        </div>
    }
}

use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::Title;

mod resume;
stylance::import_style!(style, "./resume.module.scss");

#[component]
pub fn ResumeTransition() -> impl IntoView {
    let resume = Resource::new(|| (), move |_| async move { resume::fetch_resume().await });
    // let resume = LocalResource::new(resume::fetch_resume);
    view! {
        <Transition fallback=|| view! { <div class=style::greenbox>LOADING</div> }>
            <p>
                {move || Suspend::new(async move {
                    resume
                        .with(|resume| {
                            match resume {
                                Some(e) => {
                                    match e.as_ref() {
                                        Ok(resume) => resume.name.first.clone(),
                                        _ => "ERROR".to_string(),
                                    }
                                }
                                _ => "ERROR".to_string(),
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
        <ResumeTransition />
    }
}

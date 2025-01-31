use super::style;
use crate::author::resume_data;
use leptos::prelude::*;
use url::Url;
#[component]
pub fn Image(url: Url) -> impl IntoView {
    view! {
        <div class=style::photocontainer>
            <img src=url.to_string() alt="Avatar" class="avatar" />
        </div>
    }
}

// #[component]
// pub fn Name(name: resume_data::Name) -> impl IntoView {
//     view! {
//         <div class=style::namecontainer>
//             <p class="firstname">{name.first}</p>
//             <p class="middlename">{name.middle}</p>
//             <p class="lastname">{name.last}</p>
//         </div>
//     }
// }

// #[component]
// pub fn Description(description: String) -> impl IntoView {
//     view! {
//         <div class=style::descriptioncontainer>
//             <p>{description}</p>
//         </div>
//     }
// }

// #[component]
// pub fn Website(website: Signal(Vec<resume_data::Website>)) -> impl IntoView {
//     view! {
//         <div class=style::websitecontainer>
//             <For each=move || webste.get() key=|w| w.clone() let:website>
//                 <div class=style::singlewebsitecontainer>
//                     <p>{website.name}</p>
//                     <p>{website.uri}</p>
//                 </div>
//             </For>
//         </div>
//     }
// }

// #[component]
// pub fn Contact(contact: resume_data::Contact) -> impl IntoView {
//     view! {
//         <div class=style::contactcontainer>
//             <p>{contact.phone}</p>
//             <p>{contact.email}</p>
//             <p>{contact.location.city}</p>
//             <p>{contact.location.country}</p>
//             <Website website=contact.website />
//         </div>
//     }
// }
//
// #[component]
// pub fn Education(education: Vec<resume_data::Education>) -> impl IntoView {
//     view! {
//         <div class=style::educationcontainer>
//             <For each=move || education key=|e| *e let:education>
//                 <div class=style::singleeducationcontainer>
//                     <p>{education.name}</p>
//                     <p>{education.location.city}</p>
//                     <p>{education.location.country}</p>
//                     <p>{education.degree}</p>
//                     <p>{education.major}</p>
//                     <p>{education.interval.start.to_string()}</p>
//                     <p>{education.interval.end.map(|v| v.to_string())}</p>
//                 </div>
//             </For>
//         </div>
//     }
// }
//
// #[component]
// pub fn Work(work: Vec<resume_data::Work>) -> impl IntoView {
//     view! {
//         <div class=style::workcontainer>
//             <For each=move || work key=|w| *w let:work>
//                 <div class=style::singleworkcontainer>
//                     <p>{work.name}</p>
//                     <p>{work.position}</p>
//                     <p>{work.interval.start.to_string()}</p>
//                     <p>{work.interval.end.map(|v| v.to_string())}</p>
//                     <div class=style::worklines>
//                         <For each=move || work.lines key=|l| l let:line>
//                             <p>{line}</p>
//                         </For>
//                     </div>
//                     <div class=style::workskills>
//                         <For each=move || work.skills key=|s| s let:skill>
//                             <p>{skill}</p>
//                         </For>
//                     </div>
//                 </div>
//             </For>
//         </div>
//     }
// }
//
// #[component]
// pub fn Languages(languages: Vec<resume_data::Language>) -> impl IntoView {
//     view! {
//         <div class=style::languagecontainer>
//             <For each=move || languages key=|l| l.clone() let:language>
//                 <div class=style::singlelanguagecontainer>
//                     <p>{language.name}</p>
//                     <p>{language.level}</p>
//                 </div>
//             </For>
//         </div>
//     }
// }
//
// #[component]
// pub fn Skills(skills: Vec<String>) -> impl IntoView {
//     view! {
//         <div class=style::skillscontainer>
//             <For each=move || skills key=|s| s let:skill>
//                 <p>{skill}</p>
//             </For>
//         </div>
//     }
// }
//
// #[component]
// pub fn Technologies(technologies: resume_data::Technologies) -> impl IntoView {
//     view! {
//         <div class=style::technologiescontainer>
//             <div class=style::languages>
//                 <h3>"Programing languages"</h3>
//                 <For each=move || technologies.languages key=|l| l let:language>
//                     <p>{language}</p>
//                 </For>
//             </div>
//             <div class=style::others>
//                 <h3>"Other technologies"</h3>
//                 <For each=move || technologies.others key=|o| o let:other>
//                     <p>{other}</p>
//                 </For>
//             </div>
//         </div>
//     }
// }
//
// #[component]
// pub fn Project(project: Vec<resume_data::Project>) -> impl IntoView {
//     view! {
//         <div class=style::projectcontainer>
//             <For each=|| project key=|p| p.clone() let:project>
//                 <div class=style::singleprojectcontainer>
//                     <p>{project.name}</p>
//                     <p>{project.description}</p>
//                     <p>{project.uri.to_string()}</p>
//                 </div>
//             </For>
//         </div>
//     }
// }

#[component]
pub fn Resume(resume: Signal<resume_data::Resume>) -> impl IntoView {
    view! {
        <div class=style::resumecontainer>
            <Image url=resume.with(|resume|resume.photo.clone()) />
            // <Name name=resume.with(|resume|resume.name.clone()) />
            // </Description description=resume.description />
            // <Contact contact=resume.contact />
            // <Education education=resume.education />
            // <Work work=resume.work />
            // <Languages languages=resume.languages />
            // <Skills skills=resume.skills />
            // <Technologies technologies=resume.technologies />
            // <Project project=resume.projects />
        </div>
    }
}

use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use crate::db::models::program::Program;
use http::StatusCode;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::prelude::codee::string::FromToStringCodec;
use leptos_use::{use_cookie, use_cookie_with_options, UseCookieOptions};
use serde::{Deserialize, Serialize};
use stylance::classes;
use crate::utils::cookie::CookieKey;
#[derive(Serialize, Deserialize, Clone)]
pub struct AccountLoadables {
    pub c_programs: Vec<Program>,
}

#[server(GetAccountLoadables, endpoint = "/account_loadable")]
pub async fn get_account_loadables() -> Result<AccountLoadables, ServerFnError<String>> {
    use crate::db::models::user::UserData;
    use crate::db::AppState;
    use axum::Extension;
    use leptos_axum::{extract, ResponseOptions};
    let state = expect_context::<AppState>();
    let response = expect_context::<ResponseOptions>();
    let userdata: Result<Extension<UserData>, _> = extract().await;
    match userdata {
        Ok(userdata) => Ok(AccountLoadables {
            c_programs: Program::get_by_owner_id(userdata.id, &state.pool)
                .map_err(|e| e.to_string())?,
        }),
        Err(_) => {
            response.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::ServerError("Unauthorized".to_string()))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Directory {
    name: String,
    open: bool,
    loadables: Vec<Loadable>,
}

#[derive(Serialize, Deserialize, Clone)]
enum Loadable {
    CProgram(Program),
    Directory(Directory),
}

impl Loadable {
    fn new_dir(name: String, loadables: Vec<Loadable>) -> Self {
        Self::Directory(Directory {
            name,
            open: false,
            loadables,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct LoadablesTree {
    root: Loadable,
}
impl LoadablesTree {
    fn new(account_loadables: Option<AccountLoadables>) -> Self {
        if let Some(account_loadables) = account_loadables {
            let c_loadables = account_loadables
                .c_programs
                .into_iter()
                .map(|program| Loadable::CProgram(program))
                .collect::<Vec<_>>();
            let c_loadables_dir = Loadable::new_dir("C Programs".to_string(), c_loadables);
            let emu_states_dir = Loadable::new_dir("Emulator States".to_string(), vec![]);
            let root_dir =
                Loadable::new_dir("Account".to_string(), vec![c_loadables_dir, emu_states_dir]);
            LoadablesTree { root: root_dir }
        } else {
            LoadablesTree {
                root: Loadable::new_dir("Account".to_string(), vec![]),
            }
        }
    }
}

#[island]
pub fn AccountMenu(loadable: LoadablesTree) -> impl IntoView {
    // let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    // let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    view! {}
}

#[island]
pub fn Account() -> impl IntoView {
    let loadable_resources =
        Resource::new(|| (), move |_| async move { get_account_loadables().await });
    let userdata_resource = Resource::new(
        || (),
        move |_| async move {
            crate::auth::api::userdata().await
        },
    );
    let cookie_options = UseCookieOptions::default()
        .path("/")
        .same_site(leptos_use::SameSite::Lax);
    let (session_data, set_session_data) = use_cookie_with_options::<String, FromToStringCodec>(CookieKey::Session.into(),cookie_options);
    let login = move || {
        let _ = window().location().set_href("/login");
    };
    let logout = move || {
        set_session_data(None);
        loadable_resources.refetch();
        userdata_resource.refetch();
    };
    let username = move || Suspend::new(async move {
        let userdata = userdata_resource.await;
        match userdata {
            Ok(data) => data.username,
            Err(_) => "Guest".to_string(),
        }
    });

    let logbutton = move || Suspend::new(async move {
        if userdata_resource.await.is_ok() {
            view! { <span class=classes!(emu_style::logout,emu_style::log)
                on:click=move|_| logout()>
                 LogOut</span> }
                .into_any()
        } else {
            view! { <span class=classes!(emu_style::login,emu_style::log)
                on:click=move|_| login()>
                LogIn</span> }.into_any()
        }
    });
    view! {
        <div class=emu_style::account>
            <Transition fallback=move || { "".to_string() }>
                <div class=emu_style::sectop>
                    <span>Account</span>
                </div>
                <div class=emu_style::secmid>
                    <span>{{ username }}</span>
                    {{ logbutton }}
                </div>
                <div class=emu_style::secbottom>
                    {move || Suspend::new(async move {
                        let d = loadable_resources.await;
                        match d {
                            Ok(d) => {
                                let loadables = LoadablesTree::new(Some(d));
                                view! { <AccountMenu loadable=loadables /> }.into_any()
                            }
                            Err(_) => {
                                let loadables = LoadablesTree::new(None);
                                view! { <AccountMenu loadable=loadables /> }.into_any()
                            }
                        }
                    })}
                </div>
            </Transition>
        </div>
    }
}

use axum::Extension;
use http::StatusCode;
use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use crate::db::models::program::Program;
use crate::emulator::editor::EditorContext;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;

struct AccountLoadables {
    pub c_programs: Vec<Program>,
}

#[server(GetAccountLoadables, endpoint = "/account_loadable")]
pub async fn get_account_loadables() -> Result<AccountLoadables, String> {
    use crate::db::models::user::UserData;
    use crate::db::AppState;
    use leptos_axum::{extract, ResponseOptions};
    let state = expect_context::<AppState>();
    let response = expect_context::<ResponseOptions>();
    let userdata: Result<Extension<UserData>, _> = extract().await;
    match userdata {
        Ok(userdata) => {
            Ok(AccountLoadables {
                c_programs: Program::get_by_owner_id(userdata.id,&state.pool).map_err(|e| e.to_string())?,
            })
        }
        Err(_) => {
            response.set_status(StatusCode::UNAUTHORIZED);
            Err("Unauthorized".to_string())
        }
    }
}

pub enum EmuLoadable {
    CProgram(String),
    EmuState(String),
}

impl EmuLoadable {
    pub fn load(&mut self, emu_ctx: &mut EmulatorContext, editor_ctx: &mut EditorContext) {
        match self {
            EmuLoadable::CProgram(data) => {
                editor_ctx.c_buffer = data.clone();
                log!("Loading C program: {}", data);
            }
            EmuLoadable::EmuState(data) => {
                if let Err(err) = emu_ctx.emu.load(data.clone().into(), true, true) {
                    log!("Error loading emulator state: {}", err);
                } else {
                    log!("Emulator state loaded successfully");
                }
            }
        }
    }
}

enum TreeEntryData {
    Loadable(EmuLoadable),
    Directory(Vec<TreeEntry>),
}

struct TreeEntry {
    name: String,
    data: TreeEntryData,
}

impl From<AccountLoadables> for TreeEntry {
    fn from(loadables: AccountLoadables) -> Self {
        let mut entries = vec![];
        let mut programs_entry = TreeEntry {
            name: "C Programs".to_string(),
            data: TreeEntryData::Directory(
                loadables.c_programs
                    .iter()
                    .map(|program| TreeEntry {
                        name: program.name.clone(),
                        data: TreeEntryData::Loadable(EmuLoadable::CProgram(program.data.clone())),
                    })
                    .collect(),
            ),
        };
        entries.push(programs_entry);
        TreeEntry {
            name: "Loadables".to_string(),
            data: TreeEntryData::Directory(entries),
        }
    }
}
#[island]
pub fn Account() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let loadables_resource = Resource::new(
        ||(),
        move |_| async move { get_account_loadables().await }
    );
    view! {
        <div class=emu_style::account>
            <div class=emu_style::sectop>
                <span>Account</span>    
            </div>
        </div>
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use crate::db::models::program::{NewProgram, Program};
    pub use crate::db::models::user::UserData;
    pub use crate::db::models::user::UserType;
    pub use crate::db::AppState;
    pub use axum::Extension;
    pub use http::StatusCode;
    pub use leptos::wasm_bindgen::convert::IntoWasmAbi;
    pub use leptos_axum::extract;
    pub use leptos_axum::ResponseOptions;
    pub use std::string::ToString;
}

// #[server(AddProgramApi, endpoint = "/program/new")]
// pub async fn add_program(
//     program: String,
//     name: String,
//     description: Option<String>,
// ) -> Result<(), ServerFnError> {
//     use server_imports::*;
//     let state = expect_context::<AppState>();
//     let pool = state.pool;
//     let response = expect_context::<ResponseOptions>();
//     let userdata: Result<Extension<UserData>, _> = extract().await;
//     match userdata {
//         Ok(userdata) => {
//             let new_program = NewProgram::new(userdata.id, name, description, program);
//             match Program::new(new_program, &pool) {
//                 Ok(_program) => Ok(()),
//                 Err(e) => {
//                     response.set_status(StatusCode::BAD_REQUEST);
//                     let msg = format!("{}", e);
//                     Err(ServerFnError::Response(msg))
//                 }
//             }
//         }
//         Err(_) => {
//             response.set_status(StatusCode::UNAUTHORIZED);
//             let msg = "User not authenticated".to_string();
//             Err(ServerFnError::Response(msg))
//         }
//     }
// }
//
// #[server(DeleteProgramApi, endpoint = "/program/delete")]
// pub async fn delete_program(program_id: i32) -> Result<(), ServerFnError> {
//     use server_imports::*;
//     let state = expect_context::<AppState>();
//     let pool = state.pool;
//     let response = expect_context::<ResponseOptions>();
//     let userdata: Result<Extension<UserData>, _> = extract().await;
//     let program = Program::get_by_id(program_id, &pool);
//     match (userdata, program) {
//         (Ok(userdata), Ok(program)) => {
//             if program.owner_id.map(|v| v == userdata.id).unwrap_or(false)
//                 || userdata.user_type == UserType::Admin
//             {
//                 match program.delete(&pool) {
//                     Ok(_) => Ok(()),
//                     Err(err) => {
//                         let msg = format!("Failed to login user: {}", err);
//                         response.set_status(StatusCode::INTERNAL_SERVER_ERROR);
//                         Err(ServerFnError::Response(msg))
//                     }
//                 }
//             } else {
//                 let msg = "Not Admin or owner of resource".to_string();
//                 response.set_status(StatusCode::FORBIDDEN);
//                 Err(ServerFnError::Response(msg))
//             }
//         }
//         (Err(e), _) => {
//             response.set_status(StatusCode::UNAUTHORIZED);
//             let msg = "User not authenticated".to_string();
//             Err(ServerFnError::Response(msg))
//         }
//         (_, Err(e)) => {
//             response.set_status(StatusCode::UNAUTHORIZED);
//             let msg = "User not authenticated".to_string();
//             Err(ServerFnError::Response(msg))
//         }
//     }
// }

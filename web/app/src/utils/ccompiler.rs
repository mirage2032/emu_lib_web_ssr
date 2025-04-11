use base64::Engine;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use axum::Extension;
    pub use crate::db::AppState;
    pub use crate::utils::cookie::{self, CookieKey};
    pub use http::StatusCode;
    pub use leptos_axum::extract;
    pub use leptos_axum::ResponseOptions;
}

const COMPILER_URL: &str = env!("COMPILER_URL");

#[derive(Serialize, Deserialize)]
struct RequestBody {
    b64data: String,
}

impl RequestBody {
    pub fn new(code: String) -> Self {
        let b64data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &code);
        Self { b64data }
    }
}
#[derive(Serialize, Deserialize)]
struct EncResponseBody {
    rc: i32,
    b64stdout: String,
    b64stderr: String,
    b64data: String,
}

impl EncResponseBody {
    pub fn decode(&self) -> Result<CompileData, ServerFnError<String>> {
        let decode_str = |data: &str| {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(data)
                .map_err(|e| format!("Failed to decode base64 data: {}", e))?;
            String::from_utf8(decoded)
                .map_err(|e| format!("Failed to convert decoded data to string: {}", e))
        };
        let stdout = decode_str(&self.b64stdout).map_err(|e| {
            ServerFnError::ServerError(format!("Failed to decode base64 data: {}", e).to_string())
        })?;
        let stderr = decode_str(&self.b64stderr).map_err(|e| {
            ServerFnError::ServerError(format!("Failed to decode stderr: {}", e).to_string())
        })?;
        let data = base64::engine::general_purpose::STANDARD
            .decode(&self.b64data)
            .map_err(|e| format!("Failed to decode base64 data: {}", e))?;
        Ok(CompileData {
            rc: self.rc,
            stdout,
            stderr,
            data,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct CompileData {
    pub rc: i32,
    pub stdout: String,
    pub stderr: String,
    pub data: Vec<u8>,
}

#[server(CCompile, endpoint = "/ccompile")]
pub async fn c_compile(code: String) -> Result<CompileData, ServerFnError<String>> {
    use server_imports::*;
    use crate::db::models::user::UserData;
    //encode code in b64
    let data = RequestBody::new(code);
    let state = expect_context::<AppState>();
    let response = expect_context::<ResponseOptions>();
    let userdata: Result<Extension<UserData>, _> = extract().await;
    match userdata {
        Ok(_) => {
            let reqwest_client = &state.reqwest_client;
            let response = reqwest_client
                .post("http://".to_string() + COMPILER_URL)
                .header("Content-Length", serde_json::to_string(&data).unwrap().len())
                .json(&data)
                .send()
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?
                .json::<EncResponseBody>()
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
            response.decode()
        }
        Err(_) => {
            response.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::Response(
                "Only authenticated users can use the compiler".to_string(),
            ))
        }
    }
}

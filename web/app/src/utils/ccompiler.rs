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

const COMPILER_HOST: &str = env!("COMPILER_HOST");

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
struct EncCompileData {
    rc: i32,
    b64stdout: String,
    b64stderr: String,
    b64data: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompileData {
    pub rc: i32,
    pub stdout: String,
    pub stderr: String,
    pub data: Vec<u8>,
}

impl EncCompileData {
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
pub struct EncFormatData {
    b64data: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormatData {
    pub data: String,
}

impl EncFormatData {
    pub fn decode(&self) -> Result<FormatData, ServerFnError<String>> {
        let data = base64::engine::general_purpose::STANDARD
            .decode(&self.b64data)
            .map_err(|e| format!("Failed to decode base64 data: {}", e))?;
        let data = String::from_utf8(data)
            .map_err(|e| format!("Failed to convert decoded data to string: {}", e))?;
        Ok(FormatData { data })
    }
}

#[derive(Serialize, Deserialize)]
pub struct EncSyntaxCheckData {
    pub rc: i32,
    pub b64stderr: String,
}

#[derive(Serialize, Deserialize)]
pub struct SyntaxCheckData {
    pub rc: i32,
    pub stderr: String,
}

impl EncSyntaxCheckData {
    pub fn decode(&self) -> Result<SyntaxCheckData, ServerFnError<String>> {
        let stderr = base64::engine::general_purpose::STANDARD
            .decode(&self.b64stderr)
            .map_err(|e| format!("Failed to decode base64 data: {}", e))?;
        let stderr = String::from_utf8(stderr)
            .map_err(|e| format!("Failed to convert decoded data to string: {}", e))?;
        Ok(SyntaxCheckData {
            rc: self.rc,
            stderr,
        })
    }
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
                .post(format!("http://{COMPILER_HOST}/compile"))
                .header("Content-Length", serde_json::to_string(&data).unwrap().len())
                .json(&data)
                .send()
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?
                .json::<EncCompileData>()
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

#[server(CFormat, endpoint = "/cformat")]
pub async fn c_format(code:String) -> Result<FormatData, ServerFnError<String>> {
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
                .post(format!("http://{COMPILER_HOST}/format"))
                .header("Content-Length", serde_json::to_string(&data).unwrap().len())
                .json(&data)
                .send()
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?
                .json::<EncFormatData>()
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

#[server(CSyntaxCheck, endpoint = "/csyntax_check")]
pub async fn c_syntax_check(code: String) -> Result<SyntaxCheckData, ServerFnError<String>> {
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
                .post(format!("http://{COMPILER_HOST}/syntax_check"))
                .header("Content-Length", serde_json::to_string(&data).unwrap().len())
                .json(&data)
                .send()
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?
                .json::<EncSyntaxCheckData>()
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

use std::collections::HashMap;

use anyhow::Result;
use axum::{routing::get, Router};
use once_cell::sync::OnceCell;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct AppState(HashMap<String, String>);

static APP_GLOBAL_STATE: OnceCell<AppState> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the global state
    APP_GLOBAL_STATE.get_or_init(AppState::new);

    let app = Router::new().route("/", get(index_hanlder));

    let addr = "0.0.0.0:8080";
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn index_hanlder() -> String {
    // 不建议使用全局状态，但如果非要使用，可以这样获取
    let state = AppState::get();
    format!("Hello, {}!", state.0.get("hello").unwrap())
}

impl AppState {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("hello".to_string(), "world".to_string());
        Self(map)
    }

    pub fn get() -> &'static Self {
        APP_GLOBAL_STATE
            .get()
            .expect("app global state is not initialized")
    }
}

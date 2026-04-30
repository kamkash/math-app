use axum::Json;
use log;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct NameList {
    pub names: Vec<String>,
}

pub async fn hello(Json(payload): Json<HelloRequest>) -> String {
    log::info!("Received request: {:?}", payload);
    format!("Hello, {}!", payload.name)
}

pub async fn list_names() -> Json<NameList> {
    let names = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
    ];
    log::info!("Listing names: {:?}", names);
    Json(NameList { names })
}

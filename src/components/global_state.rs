use askama::Template;
use axum::{extract::State, response::IntoResponse};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::{AppState, HtmlTemplate};

#[derive(Template)]
#[template(path = "components/home/global_state.html")]
struct GlobalStateTemplate {
    highest_atx: String,
    previous_atx: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActivationServiceHighestResult {
    atx: Atx,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Atx {
    id: Id,
    layer: AtxLayer,
    coinbase: Coinbase,
    prevAtx: Id,
}

#[derive(Debug, Serialize, Deserialize)]
struct Id {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AtxLayer {
    number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Coinbase {
    address: String,
}

pub async fn global_state_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let grpcurl_result = Command::new("grpcurl")
        .args([
            "-plaintext",
            "192.168.7.10:9092",
            "spacemesh.v1.ActivationService.Highest",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let result: ActivationServiceHighestResult =
        serde_json::from_str(String::from_utf8(grpcurl_result).unwrap().as_str()).unwrap();

    let template = GlobalStateTemplate {
        highest_atx: base64_to_hex(result.atx.id.id),
        previous_atx: base64_to_hex(result.atx.prevAtx.id),
    };

    HtmlTemplate(template)
}

fn base64_to_hex(base64: String) -> String {
    let mut buffer = Vec::<u8>::new();
    general_purpose::STANDARD
        .decode_vec(base64, &mut buffer)
        .unwrap();

    return buffer
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("");
}

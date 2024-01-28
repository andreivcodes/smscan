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
    genesis_time: String,
    current_layer: u64,
    current_epoch: u64,
    epoch_num_layers: u64,
    layer_duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshServiceGenesisTimeResult {
    unixtime: MeshServicValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshServiceCurrentLayerResult {
    layernum: MeshServiceNum,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshServiceCurrentEpochResult {
    epochnum: MeshServiceNum,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshServiceEpochNumLayersResult {
    numlayers: MeshServiceNum,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshServiceLayerDurationResult {
    duration: MeshServicValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshServiceNum {
    number: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshServicValue {
    value: String,
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
    let grpcurl_highest: Vec<u8> = Command::new("grpcurl")
        .args([
            "-plaintext",
            "192.168.7.10:9092",
            "spacemesh.v1.ActivationService.Highest",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let highest_result: ActivationServiceHighestResult =
        serde_json::from_str(String::from_utf8(grpcurl_highest).unwrap().as_str()).unwrap();

    let grpcurl_genesistime: Vec<u8> = Command::new("grpcurl")
        .args([
            "-plaintext",
            "192.168.7.10:9092",
            "spacemesh.v1.MeshService.GenesisTime",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let genesis_time_result: MeshServiceGenesisTimeResult =
        serde_json::from_str(String::from_utf8(grpcurl_genesistime).unwrap().as_str()).unwrap();

    let grpcurl_currentlayer: Vec<u8> = Command::new("grpcurl")
        .args([
            "-plaintext",
            "192.168.7.10:9092",
            "spacemesh.v1.MeshService.CurrentLayer",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let current_layer_result: MeshServiceCurrentLayerResult =
        serde_json::from_str(String::from_utf8(grpcurl_currentlayer).unwrap().as_str()).unwrap();

    let grpcurl_currentepoch: Vec<u8> = Command::new("grpcurl")
        .args([
            "-plaintext",
            "192.168.7.10:9092",
            "spacemesh.v1.MeshService.CurrentEpoch",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let current_epoch_result: MeshServiceCurrentEpochResult =
        serde_json::from_str(String::from_utf8(grpcurl_currentepoch).unwrap().as_str()).unwrap();

    let grpcurl_epochnumlayers: Vec<u8> = Command::new("grpcurl")
        .args([
            "-plaintext",
            "192.168.7.10:9092",
            "spacemesh.v1.MeshService.EpochNumLayers",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let epoch_num_layers_result: MeshServiceEpochNumLayersResult =
        serde_json::from_str(String::from_utf8(grpcurl_epochnumlayers).unwrap().as_str()).unwrap();

    let grpcurl_layerduration: Vec<u8> = Command::new("grpcurl")
        .args([
            "-plaintext",
            "192.168.7.10:9092",
            "spacemesh.v1.MeshService.LayerDuration",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let layer_duration_result: MeshServiceLayerDurationResult =
        serde_json::from_str(String::from_utf8(grpcurl_layerduration).unwrap().as_str()).unwrap();

    let template = GlobalStateTemplate {
        highest_atx: base64_to_hex(highest_result.atx.id.id),
        previous_atx: base64_to_hex(highest_result.atx.prevAtx.id),
        genesis_time: genesis_time_result.unixtime.value,
        current_layer: current_layer_result.layernum.number,
        current_epoch: current_epoch_result.epochnum.number,
        epoch_num_layers: epoch_num_layers_result.numlayers.number,
        layer_duration: layer_duration_result.duration.value,
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

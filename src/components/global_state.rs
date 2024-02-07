use crate::{AppState, GlobalState, HtmlTemplate};
use askama::Template;
use axum::{extract::State, response::IntoResponse};
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::process::Command;


#[derive(Template, Debug)]
#[template(path = "components/home/global_state.html")]
struct GlobalStateTemplate {
    highest_atx: String,
    previous_atx: String,
    genesis_timestamp: String,
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

pub async fn global_state_handler(State(state): State<AppState>) -> impl IntoResponse {
    let global_state_read = state.global_state.read().await;

    if (global_state_read.last_state_fetch - Utc::now().naive_utc()) > Duration::seconds(1) {
        let mut global_state_write = state.global_state.write().await;
        let new_global_state = fetch_global_state().await.unwrap();

        global_state_write.highest_atx = new_global_state.highest_atx;
        global_state_write.previous_atx = new_global_state.previous_atx;
        global_state_write.genesis_timestamp = new_global_state.genesis_timestamp;
        global_state_write.genesis_time = new_global_state.genesis_time;
        global_state_write.current_layer = new_global_state.current_layer;
        global_state_write.current_epoch = new_global_state.current_epoch;
        global_state_write.epoch_num_layers = new_global_state.epoch_num_layers;
        global_state_write.layer_duration = new_global_state.layer_duration;
        global_state_write.last_state_fetch = Utc::now().naive_utc();
    }

    let template = GlobalStateTemplate {
        highest_atx: global_state_read.highest_atx.to_string(),
        previous_atx: global_state_read.previous_atx.to_string(),
        genesis_timestamp: global_state_read.genesis_timestamp.to_string(),
        genesis_time: global_state_read.genesis_time.to_string(),
        current_layer: global_state_read.current_layer,
        current_epoch: global_state_read.current_epoch,
        epoch_num_layers: global_state_read.epoch_num_layers,
        layer_duration: global_state_read.layer_duration.to_string(),
    };

    HtmlTemplate(template)
}

async fn fetch_global_state() -> anyhow::Result<GlobalState> {
    let node_host = std::env::var("NODE_HOST").unwrap();

    let grpcurl_highest: Vec<u8> = Command::new("grpcurl")
        .args([
            "-plaintext",
            node_host.as_str(),
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
            node_host.as_str(),
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
            node_host.as_str(),
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
            node_host.as_str(),
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
            node_host.as_str(),
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
            node_host.as_str(),
            "spacemesh.v1.MeshService.LayerDuration",
        ])
        .output()
        .await
        .unwrap()
        .stdout;

    let layer_duration_result: MeshServiceLayerDurationResult =
        serde_json::from_str(String::from_utf8(grpcurl_layerduration).unwrap().as_str()).unwrap();

    Ok(GlobalState {
        highest_atx: base64_to_hex(highest_result.atx.id.id),
        previous_atx: base64_to_hex(highest_result.atx.prevAtx.id),
        genesis_timestamp: genesis_time_result.unixtime.value.clone(),
        genesis_time: DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDateTime::from_timestamp_millis(
                genesis_time_result.unixtime.value.parse::<i64>().unwrap() * 1000,
            )
            .unwrap(),
            Utc,
        )
        .format("%Y-%m-%d %H:%M:%S")
        .to_string(),
        current_layer: current_layer_result.layernum.number,
        current_epoch: current_epoch_result.epochnum.number,
        epoch_num_layers: epoch_num_layers_result.numlayers.number,
        layer_duration: layer_duration_result.duration.value,
        last_state_fetch: Utc::now().naive_utc(),
    })
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

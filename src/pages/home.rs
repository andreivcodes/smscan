use crate::{
    db_entities::{layers, rewards, transactions},
    AppState, HtmlTemplate,
};
use askama::Template;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

pub fn home_routes() -> Router<AppState> {
    Router::new()
        .route("/home/layers", get(layers_handler))
        .route("/home/transactions", get(transactions_handler))
        .route("/home/rewards", get(rewards_handler))
        .route("/", get(page))
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate;

async fn page() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Layer {
    id: i32,
    processed: bool,
    hash: String,
    state_hash: String,
    applied_block: String,
}
#[derive(Template)]
#[template(path = "components/home/layers.html")]
struct LayersListTemplate {
    layers: Vec<Layer>,
}
async fn layers_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = &state.database;

    let db_layers = layers::Entity::find()
        .order_by_desc(layers::Column::Id)
        .limit(10)
        .all(conn)
        .await
        .unwrap();

    let layers = db_layers
        .iter()
        .map(|layer| Layer {
            id: layer.id,
            hash: layer
                .aggregated_hash
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .rev()
                .take(6)
                .rev()
                .collect(),
            processed: layer.processed.unwrap_or(0) == 1,
            state_hash: layer
                .state_hash
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .rev()
                .take(6)
                .rev()
                .collect(),
            applied_block: layer
                .applied_block
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .rev()
                .take(6)
                .rev()
                .collect(),
        })
        .collect();

    let template = LayersListTemplate { layers };
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    tx: String,
    header: String,
    result: String,
    layer: i32,
    block: String,
    principal: String,
    principal_short: String,
    nonce: u64,
    timestamp: String,
}
#[derive(Template)]
#[template(path = "components/home/transactions.html")]
struct TransactionsListTemplate {
    txs: Vec<Transaction>,
}
async fn transactions_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = &state.database;

    let db_txs = transactions::Entity::find()
        .order_by_desc(transactions::Column::Layer)
        .limit(10)
        .all(conn)
        .await
        .unwrap();

    let txs = db_txs
        .iter()
        .map(|tx| Transaction {
            tx: tx
                .tx
                .clone()
                .unwrap()
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            header: tx
                .header
                .clone()
                .unwrap()
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            result: tx
                .result
                .clone()
                .unwrap()
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            layer: tx.layer.unwrap(),
            block: tx
                .block
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            principal: tx
                .principal
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            principal_short: tx
                .principal
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .rev()
                .take(6)
                .rev()
                .collect(),
            nonce: {
                let nonce_vec = tx.nonce.clone().unwrap();

                let nonce_array = [
                    nonce_vec[0],
                    nonce_vec[1],
                    nonce_vec[2],
                    nonce_vec[3],
                    nonce_vec[4],
                    nonce_vec[5],
                    nonce_vec[6],
                    nonce_vec[7],
                ];

                u64::from_be_bytes(nonce_array)
            },
            timestamp: NaiveDateTime::from_timestamp_millis(tx.timestamp / 1000000)
                .unwrap()
                .to_string(),
        })
        .collect();
    let template = TransactionsListTemplate { txs };
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Reward {
    coinbase: String,
    coinbase_short: String,
    layer: i32,
    total_reward: f32,
    layer_reward: f32,
}
#[derive(Template)]
#[template(path = "components/home/rewards.html")]
struct RewardsListTemplate {
    rewards: Vec<Reward>,
}
async fn rewards_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = &state.database;

    let db_rewards = rewards::Entity::find()
        .order_by_desc(rewards::Column::Layer)
        .limit(10)
        .all(conn)
        .await
        .unwrap();

    let rewards = db_rewards
        .iter()
        .map(|reward| Reward {
            coinbase: reward
                .coinbase
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            coinbase_short: reward
                .coinbase
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .rev()
                .take(6)
                .rev()
                .collect(),
            layer: reward.layer,
            total_reward: (reward.total_reward.unwrap_or(0) as f32 / 1_000_000_000.0),
            layer_reward: (reward.layer_reward.unwrap_or(0) as f32 / 1_000_000_000.0),
        })
        .collect();
    let template = RewardsListTemplate { rewards };
    HtmlTemplate(template)
}

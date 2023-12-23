use crate::{
    db_entities::{blocks, layers, rewards, transactions},
    AppState, HtmlTemplate,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

pub fn layer_routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(page))
        .route("/:id/layer", get(layer_handler))
        .route("/:id/blocks", get(blocks_handler))
        .route("/:id/transactions", get(transactions_handler))
        .route("/:id/rewards", get(rewards_handler))
}

#[derive(Template)]
#[template(path = "pages/layer.html")]
struct LayerTemplate {
    id: i32,
}

async fn page(Path(id): Path<i32>) -> impl IntoResponse {
    let template = LayerTemplate { id };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "components/layer/layer_tab.html")]
struct LayerTabTemplate {
    id: i32,
    processed: bool,
    hash: String,
    state_hash: String,
    applied_block: String,
}

async fn layer_handler(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let conn = &state.database;
    let db_layer = layers::Entity::find_by_id(id).all(conn).await.unwrap();

    let layer = LayerTabTemplate {
        id: db_layer.first().unwrap().id,
        hash: db_layer
            .first()
            .unwrap()
            .aggregated_hash
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect(),
        processed: db_layer.first().unwrap().processed.unwrap_or(0) == 1,
        state_hash: db_layer
            .first()
            .unwrap()
            .state_hash
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect(),
        applied_block: db_layer
            .first()
            .unwrap()
            .applied_block
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect(),
    };

    HtmlTemplate(layer)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlockTabTemplate {
    id: String,
    validity: bool,
    block_data: String,
}

#[derive(Template)]
#[template(path = "components/layer/blocks_tab.html")]
struct BlocksTabTemplate {
    blocks: Vec<BlockTabTemplate>,
}

async fn blocks_handler(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let conn = &state.database;
    let db_blocks = blocks::Entity::find()
        .filter(blocks::Column::Layer.eq(id))
        .all(conn)
        .await
        .unwrap();

    let blocks = db_blocks
        .iter()
        .map(|block| BlockTabTemplate {
            id: block
                .id
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            validity: block.validity.unwrap_or(0) == 1,
            block_data: block
                .block
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
        })
        .collect();

    let template = BlocksTabTemplate { blocks };
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransactionTabTemplate {
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
#[template(path = "components/layer/transactions_tab.html")]
struct TransactionsTabTemplate {
    txs: Vec<TransactionTabTemplate>,
}

async fn transactions_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let conn = &state.database;
    let db_transactions = transactions::Entity::find()
        .filter(transactions::Column::Layer.eq(id))
        .all(conn)
        .await
        .unwrap();

    let txs = db_transactions
        .iter()
        .map(|tx| TransactionTabTemplate {
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

    let template = TransactionsTabTemplate { txs };
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RewardTabTemplate {
    coinbase: String,
    coinbase_short: String,
    layer: i32,
    total_reward: f32,
    layer_reward: f32,
}

#[derive(Template)]
#[template(path = "components/layer/rewards_tab.html")]
struct RewardsTabTemplate {
    rewards: Vec<RewardTabTemplate>,
}

async fn rewards_handler(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let conn = &state.database;
    let db_rewards = rewards::Entity::find()
        .filter(rewards::Column::Layer.eq(id))
        .all(conn)
        .await
        .unwrap();

    let rewards = db_rewards
        .iter()
        .map(|reward| RewardTabTemplate {
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

    let template = RewardsTabTemplate { rewards };
    HtmlTemplate(template)
}

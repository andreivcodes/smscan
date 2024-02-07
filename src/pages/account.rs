use crate::{
    db_entities::{accounts, rewards, transactions},
    AppState, HtmlTemplate,
};
use askama::Template;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

pub fn account_route() -> Router<AppState> {
    Router::new()
        .route("/:id", get(page))
        .route("/:id/account", get(account_handler))
        .route("/:id/transactions", get(transactions_handler))
        .route("/:id/rewards", get(rewards_handler))
}

#[derive(Template)]
#[template(path = "pages/account.html")]
struct AddressTemplate {
    id: String,
}

async fn page(Path(id): Path<String>) -> impl IntoResponse {
    let template = AddressTemplate { id };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "components/account/account_tab.html")]
struct AccountTabTemplate {
    address: String,
    balance: f64,
    next_nonce: i64,
    template: String,
    state: String,
}

async fn account_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = &state.database;

    let db_account = accounts::Entity::find()
        .filter(accounts::Column::Address.eq(hex::decode(id).unwrap()))
        .all(conn)
        .await
        .unwrap();

    let layer = AccountTabTemplate {
        address: db_account
            .first()
            .unwrap()
            .address
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect(),
        balance: db_account.first().unwrap().balance as f64 / 1_000_000_000.0,
        next_nonce: db_account.first().unwrap().next_nonce,
        template: db_account
            .first()
            .unwrap()
            .template
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect(),
        state: db_account
            .first()
            .unwrap()
            .state
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect(),
    };

    HtmlTemplate(layer)
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
#[template(path = "components/account/transactions_tab.html")]
struct TransactionsTabTemplate {
    id: String,
    txs: Vec<TransactionTabTemplate>,
    txs_count: u64,
    skip: u64,
}

#[derive(Deserialize)]
struct Pagination {
    skip: Option<u64>,
}

async fn transactions_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    pagination: Query<Pagination>,
) -> impl IntoResponse {
    let conn = &state.database;

    let db_transactions = transactions::Entity::find()
        .filter(transactions::Column::Principal.eq(hex::decode(id.clone()).unwrap()))
        .order_by_desc(transactions::Column::Layer)
        .offset(pagination.skip)
        .limit(20)
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

    let template = TransactionsTabTemplate {
        id,
        txs,
        txs_count: db_transactions.len() as u64,
        skip: pagination.skip.unwrap_or(0) + 20,
    };
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
#[template(path = "components/account/rewards_tab.html")]
struct RewardsTabTemplate {
    id: String,
    rewards: Vec<RewardTabTemplate>,
    rewards_count: u64,
    skip: u64,
}

async fn rewards_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    pagination: Query<Pagination>,
) -> impl IntoResponse {
    let conn = &state.database;
    let db_rewards = rewards::Entity::find()
        .filter(rewards::Column::Coinbase.eq(hex::decode(id.clone()).unwrap()))
        .order_by_desc(rewards::Column::Layer)
        .offset(pagination.skip)
        .limit(20)
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

    let template = RewardsTabTemplate {
        id,
        rewards,
        rewards_count: db_rewards.len() as u64,
        skip: pagination.skip.unwrap_or(0) + 20,
    };
    HtmlTemplate(template)
}

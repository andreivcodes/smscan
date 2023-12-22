use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};

use crate::HtmlTemplate;

pub fn home_routes() -> Router {
    Router::new()
        .route("/home/layers", get(layers_handler))
        .route("/home/blocks", get(blocks_handler))
        .route("/home/transactions", get(transactions_handler))
        .route("/", get(page))
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate;

pub async fn page() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Layer {
    id: u64,
    name: &'static str,
}
#[derive(Template)]
#[template(path = "components/home/layers.html")]
struct LayersListTemplate {
    layers: Vec<Layer>,
}
async fn layers_handler() -> impl IntoResponse {
    let layers = vec![
        Layer {
            id: 1,
            name: "Layer 1",
        },
        Layer {
            id: 2,
            name: "Layer 2",
        },
        Layer {
            id: 3,
            name: "Layer 3",
        },
    ];
    let template = LayersListTemplate { layers };
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    id: u64,
    name: &'static str,
}
#[derive(Template)]
#[template(path = "components/home/blocks.html")]
struct BlocksListTemplate {
    blocks: Vec<Block>,
}
async fn blocks_handler() -> impl IntoResponse {
    let blocks = vec![
        Block {
            id: 1,
            name: "Layer 1",
        },
        Block {
            id: 2,
            name: "Block 2",
        },
    ];
    let template = BlocksListTemplate { blocks };
    HtmlTemplate(template)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    id: u64,
    name: &'static str,
}
#[derive(Template)]
#[template(path = "components/home/transactions.html")]
struct TransactionsListTemplate {
    txs: Vec<Transaction>,
}
async fn transactions_handler() -> impl IntoResponse {
    let txs = vec![Transaction {
        id: 1,
        name: "Transaction 1",
    }];
    let template = TransactionsListTemplate { txs };
    HtmlTemplate(template)
}

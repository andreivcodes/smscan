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
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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

async fn page(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let conn = &state.database;
    let layer = layers::Entity::find_by_id(id).all(conn).await.unwrap();

    let blocks = blocks::Entity::find()
        .filter(blocks::Column::Layer.eq(id))
        .all(conn)
        .await
        .unwrap();

    let rewards = rewards::Entity::find()
        .filter(rewards::Column::Layer.eq(id))
        .all(conn)
        .await
        .unwrap();

    let transactions = transactions::Entity::find()
        .filter(transactions::Column::Layer.eq(id))
        .all(conn)
        .await
        .unwrap();

    println!("{:?}", layer);
    println!("{:?}", blocks);
    println!("{:?}", rewards);
    println!("{:?}", transactions);

    let template = LayerTemplate { id };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "components/layer/layer_tab.html")]
struct LayerTabTemplate;

async fn layer_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let template = LayerTabTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "components/layer/blocks_tab.html")]
struct BlocksTabTemplate;

async fn blocks_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let template = BlocksTabTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "components/layer/transactions_tab.html")]
struct TransactionsTabTemplate;

async fn transactions_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let template = TransactionsTabTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "components/layer/rewards_tab.html")]
struct RewardsTabTemplate;

async fn rewards_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let template = RewardsTabTemplate {};
    HtmlTemplate(template)
}

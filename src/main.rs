use crate::{
    db_entities::accounts,
    pages::{account::account_route, home::home_routes, layer::layer_routes},
};
use askama::Template;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use db_entities::layers;
use dotenv::dotenv;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub mod components;
mod db_entities;
mod pages;

#[derive(Clone)]
pub struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router and assets");

    let conn = Database::connect("sqlite://node-data/state.sql?mode=ro").await?;
    let state = AppState { database: conn };

    let assets_path = std::env::current_dir()?;

    let app = Router::new()
        .layer(tower_livereload::LiveReloadLayer::new())
        .nest("/", home_routes())
        .nest("/layer", layer_routes())
        .nest("/account", account_route())
        .route("/search", get(search_handler))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(state);

    // run it, make sure you handle parsing your environment variables properly!
    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Deserialize)]
struct Search {
    input: String,
}

#[axum::debug_handler]
async fn search_handler(query: Query<Search>, State(state): State<AppState>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let conn = &state.database;

    match query.input.parse::<i32>() {
        Ok(input) => {
            let layer = layers::Entity::find_by_id(input).one(conn).await.unwrap();

            if layer.is_some() {
                headers.insert(
                    "HX-Redirect",
                    format!("/layer/{}", query.input).parse().unwrap(),
                );
                return headers;
            }
        }
        Err(_) => {}
    }

    let account = accounts::Entity::find()
        .filter(accounts::Column::Address.eq(hex::decode(query.input.clone()).unwrap()))
        .one(conn)
        .await
        .unwrap();

    if account.is_some() {
        headers.insert(
            "HX-Redirect",
            format!("/account/{}", query.input).parse().unwrap(),
        );
        return headers;
    }

    headers.insert("HX-Redirect", "/".parse().unwrap());
    return headers;
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

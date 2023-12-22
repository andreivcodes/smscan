use crate::pages::{address::address_routes, home::home_routes, layer::layer_routes};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod db_entities;
mod pages;

#[derive(Clone)]
struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router and assets");

    let conn = Database::connect("sqlite://state.sql?mode=ro").await?;
    let state = AppState { database: conn };

    let assets_path = std::env::current_dir()?;

    let app = Router::new()
        .layer(tower_livereload::LiveReloadLayer::new())
        .nest("/", home_routes())
        .nest("/layer", layer_routes())
        .nest("/address", address_routes())
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(state);

    // run it, make sure you handle parsing your environment variables properly!
    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("router initialized, not listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
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

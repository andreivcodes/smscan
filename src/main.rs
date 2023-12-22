use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use dotenv::dotenv;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod pages;
use crate::pages::home::home_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_static_web_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router and assets");

    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new().route("/hello", get(say_hello));

    let app = Router::new()
        .nest("/", home_routes())
        .layer(tower_livereload::LiveReloadLayer::new())
        .nest("/api", api_router)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    // run it, make sure you handle parsing your environment variables properly!
    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("router initialized, not listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .context("error while starting server")?;

    Ok(())
}

async fn say_hello() -> &'static str {
    "Hello!"
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

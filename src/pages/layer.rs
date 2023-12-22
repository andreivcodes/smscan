use crate::{AppState, HtmlTemplate};
use askama::Template;
use axum::{extract::Path, response::IntoResponse, routing::get, Router};

pub fn layer_routes() -> Router<AppState> {
    Router::new().route("/:id", get(page))
}

#[derive(Template)]
#[template(path = "pages/layer.html")]
struct LayerTemplate;

async fn page(Path(id): Path<u64>) -> impl IntoResponse {
    println!("id: {}", id);
    let template = LayerTemplate {};
    HtmlTemplate(template)
}

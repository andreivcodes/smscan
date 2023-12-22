use crate::{AppState, HtmlTemplate};
use askama::Template;
use axum::{extract::Path, response::IntoResponse, routing::get, Router};

pub fn address_routes() -> Router<AppState> {
    Router::new().route("/:coinbase", get(page))
}

#[derive(Template)]
#[template(path = "pages/address.html")]
struct AddressTemplate;

async fn page(Path(coinbase): Path<String>) -> impl IntoResponse {
    let template = AddressTemplate {};
    HtmlTemplate(template)
}

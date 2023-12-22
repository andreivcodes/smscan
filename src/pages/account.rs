use crate::{AppState, HtmlTemplate};
use askama::Template;
use axum::{extract::Path, response::IntoResponse, routing::get, Router};

pub fn account_route() -> Router<AppState> {
    Router::new().route("/:coinbase", get(page))
}

#[derive(Template)]
#[template(path = "pages/account.html")]
struct AddressTemplate;

async fn page(Path(_coinbase): Path<String>) -> impl IntoResponse {
    let template = AddressTemplate {};
    HtmlTemplate(template)
}

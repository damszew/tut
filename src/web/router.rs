use axum::{routing::get, Router};
use tower::ServiceBuilder;

use super::controllers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(controllers::home::page))
        .layer(ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()))
        .with_state(())
}

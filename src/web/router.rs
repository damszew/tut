use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;

use super::controllers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(controllers::home::page))
        .route("/daily/new", get(controllers::daily::create_form))
        .route("/daily", post(controllers::daily::create))
        .route("/daily/:id", get(controllers::daily::room))
        // TODO: websocket
        .layer(ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()))
        .with_state(())
}

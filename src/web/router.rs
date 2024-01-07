use axum::{routing, Router};
use tower::ServiceBuilder;

use super::controllers::{daily, home};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(home::page))
        .route("/daily/new", routing::get(daily::create_form))
        .route("/daily", routing::post(daily::create))
        .route("/daily/:id", routing::get(daily::room))
        .route("/daily/:id/ws", routing::get(daily::websocket))
        .layer(ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()))
        .with_state(())
}

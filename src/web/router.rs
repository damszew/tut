use axum::{routing, Router};
use tower::ServiceBuilder;

use crate::daily_router::DailyRouter;

use super::controllers::{daily, home};

#[derive(Debug, Clone)]
pub struct AppState {
    pub daily_router: DailyRouter,
}

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(home::page))
        .route("/daily/new", routing::get(daily::create_form))
        .route("/daily", routing::post(daily::create))
        .route("/daily/:id", routing::get(daily::room))
        .route("/daily/:id/ws", routing::get(daily::websocket))
        .layer(ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()))
        .with_state(app_state)
}

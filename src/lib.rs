use daily_router::DailyRouter;
use tokio::net::TcpListener;
use web::router::AppState;

mod daily;
mod daily_router;
mod participant;

mod web;

pub struct Config {
    pub tcp: TcpListener,
}

pub async fn run(config: Config) -> Result<(), anyhow::Error> {
    let app_state = AppState {
        daily_router: DailyRouter::new(),
    };

    let routes = web::router::router(app_state);

    axum::serve(config.tcp, routes).await?;

    Ok(())
}

use tokio::net::TcpListener;
use tower::ServiceBuilder;

pub struct Config {
    pub tcp: TcpListener,
}

pub async fn run(config: Config) -> Result<(), anyhow::Error> {
    let routes = axum::Router::new()
        .layer(ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()))
        .with_state(());

    axum::serve(config.tcp, routes).await?;

    Ok(())
}

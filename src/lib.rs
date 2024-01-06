use tokio::net::TcpListener;

mod daily;
mod web;

pub struct Config {
    pub tcp: TcpListener,
}

pub async fn run(config: Config) -> Result<(), anyhow::Error> {
    let routes = web::router::router();

    axum::serve(config.tcp, routes).await?;

    Ok(())
}

use clap::Parser;
use tokio::net::TcpListener;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Options {
    #[clap(long, env, default_value = "debug")]
    log_level: String,

    #[clap(long, env, default_value = "8000")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    set_panic_hook();

    let options = Options::parse();

    init_tracing(&options.log_level);
    tracing::info!(?options);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], options.port));
    let tcp = TcpListener::bind(addr).await?;

    tracing::info!(%addr, "Starting server");
    tut::run(tut::Config { tcp }).await
}

fn init_tracing(log_level: &str) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Tracing initialized");
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic| {
        // If the panic has a source location, record it as structured fields.
        if let Some(location) = panic.location() {
            tracing::error!(
                panic.file = location.file(),
                panic.line = location.line(),
                panic.column = location.column(),
                %panic
            );
        } else {
            tracing::error!(%panic);
        }
    }));
}

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use proserve::build_router;

#[tracing::instrument]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Starting");

    // build routes
    let router = build_router();
    
    let listener = TcpListener::bind("0.0.0.0:6612").await?;
    println!("Starting {} on http://{}", env!("CARGO_CRATE_NAME"), listener.local_addr()?);
    axum::serve(
        listener, 
        router.into_make_service_with_connect_info::<SocketAddr>()).await?;

    anyhow::Ok(())
} 


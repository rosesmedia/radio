use miette::{Context, IntoDiagnostic};
use stream_controller::{client::ApiClient, routes, services::ServiceManager, AppState};
use tracing_subscriber::prelude::*;

#[cfg(debug_assertions)]
const DEFAULT_LOG_LEVEL: &str = "debug";
#[cfg(not(debug_assertions))]
const DEFAULT_LOG_LEVEL: &str = "info";

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.into()),
        ))
        .init();

    let api_base = std::env::var("STREAM_API_BASE")
        .expect("environment variable `STREAM_API_BASE` must be set");
    let client = ApiClient::new(&api_base);

    let service_manager = ServiceManager::new().await?;

    let ingest_points = client
        .get_ingest_points()
        .await
        .into_diagnostic()
        .with_context(|| "fetching initial ingest point list")?;
    for ingest in ingest_points.ingest_points {
        service_manager.start_ingest(&ingest.id).await?;
    }

    let host = std::env::var("HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "1350".to_string())
        .parse()
        .expect("`PORT` environment variable to be an integer");

    let bind_addr = format!("{host}:{port}");

    let app = routes::routes().with_state(AppState { service_manager });

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .into_diagnostic()
        .with_context(|| "binding listener")?;
    axum::serve(listener, app)
        .await
        .into_diagnostic()
        .with_context(|| "serving app")?;

    Ok(())
}

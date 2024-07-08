mod appstate;
mod handlers;
mod serde_structs;

use anyhow::Ok;
use appstate::AppState;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const SOCKET: &str = "0.0.0.0:7331";

const DEFAULT_LOGLEVEL: &str = if cfg!(debug_assertions) {
    "debug,tower_http=debug,ort=info"
} else {
    "info"
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or(DEFAULT_LOGLEVEL.into()))
        .with(fmt::layer())
        .init();

    tracing::info!("Will listen on {0}", SOCKET);

    let state = AppState::new(
        "models/knn_boys.onnx",
        "models/knn_girls.onnx",
        "models/knn_women.onnx",
        "models/knn_men.onnx",
    )?;

    let static_files_srv = ServeDir::new("dist/").fallback(ServeFile::new("dist/404.html"));

    let router = Router::new()
        .route("/test", get(handlers::test))
        .route("/infer", get(handlers::infer))
        .nest_service("/", static_files_srv)
        .with_state(state);

    let listener = TcpListener::bind(SOCKET).await?;

    axum::serve(listener, router.layer(TraceLayer::new_for_http())).await?;

    Ok(())
}

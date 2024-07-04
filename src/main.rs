mod appstate;
mod handlers;
mod url_queries;

use anyhow::Ok;
use appstate::AppState;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const SOCKET: &str = "127.0.0.1:6969";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from(if cfg!(debug_assertions) {
            "debug,tower_http=debug,ort=info"
        } else {
            "info,tower_http=warn"
        }))
        .with(fmt::layer())
        .init();

    tracing::info!("Will listen on {0}", SOCKET);

    let state = AppState::new("models/sample_model.onnx")?;

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

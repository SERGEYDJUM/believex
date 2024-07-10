mod appstate;
mod handlers;
mod queries;

use anyhow::Ok;
use appstate::AppState;
use axum::{routing::get, Router};
use believex_backend::MMConfig;
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

    let mm_config = MMConfig {
        mlf2h: "models/knn_male_adults_lf_2h.onnx",
        mlf5d: "models/knn_male_adults_lf_5d.onnx",
        mhf2h: "models/knn_male_adults_hf_5d.onnx",
        mhf5d: "models/knn_male_adults_hf_5d.onnx",
        flf2h: "models/knn_female_adults_lf_2h.onnx",
        flf5d: "models/knn_female_adults_lf_5d.onnx",
        fhf2h: "models/knn_female_adults_hf_2h.onnx",
        fhf5d: "models/knn_female_adults_hf_5d.onnx",
    };

    let state = AppState::new(mm_config)?;

    let static_files_srv = ServeDir::new("dist/").fallback(ServeFile::new("dist/404.html"));

    let router = Router::new()
        .route("/infer", get(handlers::infer))
        .route("/infer_htmx", get(handlers::infer))
        .nest_service("/", static_files_srv)
        .with_state(state);

    let listener = TcpListener::bind(SOCKET).await?;

    axum::serve(listener, router.layer(TraceLayer::new_for_http())).await?;

    Ok(())
}

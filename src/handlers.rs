use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{appstate::AppState, url_queries::TestQuery};

pub async fn test() -> impl IntoResponse {
    "sample response text"
}

pub async fn infer(
    Query(query): Query<TestQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Showcase the query mechanism
    let query = query.testval;
    tracing::debug!("got number {0}", query);

    let lock = state.ort_model.lock().await;

    match lock.infer() {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

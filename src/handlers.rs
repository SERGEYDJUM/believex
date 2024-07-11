use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{
    appstate::AppState,
    queries::{InferenceQuery, ObservationTime, Sex},
};

#[derive(Debug, Serialize)]
struct Forecast {
    lf: f32,
    hf: f32,
}

pub async fn infer(
    Query(query): Query<InferenceQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let is_late = matches!(query.otime, ObservationTime::After5d);
    let is_male = matches!(query.sex, Sex::Male);

    let (lf, hf) = match state
        .model_manager
        .infer(query.lf_b, query.hf_b, is_male, is_late)
        .await
    {
        Ok(v) => v,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Inference failed with error: {0}", err),
            ))
        }
    };

    Ok(Json(Forecast { lf, hf }))
}

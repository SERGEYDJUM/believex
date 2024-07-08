use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{
    appstate::AppState,
    serde_structs::{AgeGroup, InferenceQuery, Sex},
};

pub async fn test() -> impl IntoResponse {
    "sample response text"
}

pub async fn infer(
    Query(query): Query<InferenceQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let is_child = matches!(query.age, AgeGroup::Child);
    let is_male = matches!(query.sex, Sex::Male);

    match state
        .model_manager
        .infer(query.lf, query.hf, is_male, is_child)
        .await
    {
        Ok(value) => value.to_string(),
        Err(err) => err.to_string(),
    }
}

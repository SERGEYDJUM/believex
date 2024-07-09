use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{
    appstate::AppState,
    serde_structs::{InferenceQuery, Sex},
};

pub async fn test() -> impl IntoResponse {
    "sample response text"
}

pub async fn infer(
    Query(query): Query<InferenceQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let is_child = query.age <= 12;
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

pub async fn infer_htmx(
    Query(query): Query<InferenceQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let is_child = query.age <= 12;
    let is_male = matches!(query.sex, Sex::Male);

    let res = state
        .model_manager
        .infer(query.lf, query.hf, is_male, is_child)
        .await;

    format!(
        "<tr>
            <td>{0}</td>
            <td>{1}</td>
            <td>{2}</td>
            <td>{3}</td>
            <td>{4}</td>
        </tr>",
        query.age,
        if is_male { "Male" } else { "Female" },
        query.lf,
        query.hf,
        match res {
            Ok(v) => v.to_string(),
            Err(_) => "ERR".to_string(),
        }
    )
}

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    appstate::AppState,
    queries::{InferenceQuery, ObservationTime, Sex},
};

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

    let new_row = format!(
        "<tr>
            <td>{0}</td>
            <td>{1}</td>
            <td>{2}</td>
            <td>{3}</td>
            <td>{4}</td>
            <td>{5}</td>
        </tr>",
        if is_late { "5 days" } else { "2 hours" },
        if is_male { "M" } else { "F" },
        query.lf_b,
        query.hf_b,
        lf,
        hf,
    );

    Ok(new_row)
}

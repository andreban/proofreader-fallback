use axum::{Json, extract::State};

use crate::{AppState, proofreader};

#[derive(serde::Deserialize)]
pub struct ProofreadParams {
    pub input: String,
}

pub async fn proofread(
    State(app_state): State<AppState>,
    Json(params): Json<ProofreadParams>,
) -> Json<proofreader::Proofreading> {
    let response = proofreader::proofread(&app_state.vertex_client, &params.input)
        .await
        .unwrap();
    Json(response)
}

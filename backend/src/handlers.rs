use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::*;
use crate::AppState;

fn verify_write_key(headers: &HeaderMap, state: &AppState) -> Result<(), AppError> {
    let key = headers
        .get("X-Write-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if key != state.write_key {
        return Err(AppError::Unauthorized);
    }
    Ok(())
}

fn verify_read_key(headers: &HeaderMap, state: &AppState) -> Result<(), AppError> {
    let key = headers
        .get("X-Read-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if key != state.read_key {
        return Err(AppError::Unauthorized);
    }
    Ok(())
}

// Write APIs

pub async fn track_event(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<TrackEventRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    verify_write_key(&headers, &state)?;

    state
        .db
        .insert_event(&payload.app_id, &payload.event, &payload.user_id, &payload.properties)
        .await?;

    Ok(Json(SuccessResponse { success: true }))
}

pub async fn submit_feedback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<FeedbackRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    verify_write_key(&headers, &state)?;

    state
        .db
        .insert_feedback(
            &payload.app_id,
            &payload.content,
            payload.user_id.as_deref(),
            payload.contact.as_deref(),
            &payload.properties,
        )
        .await?;

    Ok(Json(SuccessResponse { success: true }))
}

// Read APIs

pub async fn get_apps(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<AppsResponse>, AppError> {
    verify_read_key(&headers, &state)?;

    let apps = state.db.get_apps().await?;
    Ok(Json(AppsResponse { apps }))
}

pub async fn get_dau(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<DauQuery>,
) -> Result<Json<DauResponse>, AppError> {
    verify_read_key(&headers, &state)?;

    let data = state.db.get_dau(&query.app_id, query.days).await?;
    Ok(Json(DauResponse { data }))
}

pub async fn get_installs(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<InstallsQuery>,
) -> Result<Json<InstallsResponse>, AppError> {
    verify_read_key(&headers, &state)?;

    let (total, data) = state.db.get_installs(&query.app_id, query.days).await?;
    Ok(Json(InstallsResponse { total, data }))
}

pub async fn get_retention(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<RetentionQuery>,
) -> Result<Json<RetentionResponse>, AppError> {
    verify_read_key(&headers, &state)?;

    let data = state
        .db
        .get_retention(&query.app_id, query.start_date.as_deref())
        .await?;
    Ok(Json(RetentionResponse { data }))
}

pub async fn get_feedbacks(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<FeedbackQuery>,
) -> Result<Json<FeedbacksResponse>, AppError> {
    verify_read_key(&headers, &state)?;

    let data = state.db.get_feedbacks(&query.app_id, query.limit).await?;
    Ok(Json(FeedbacksResponse { data }))
}

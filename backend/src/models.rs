use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Request models
#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub app_id: String,
    pub event: String,
    pub user_id: String,
    #[serde(default)]
    pub properties: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct FeedbackRequest {
    pub app_id: String,
    pub content: String,
    pub user_id: Option<String>,
    pub contact: Option<String>,
    #[serde(default)]
    pub properties: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DauQuery {
    pub app_id: String,
    #[serde(default = "default_days")]
    pub days: i32,
}

fn default_days() -> i32 {
    30
}

#[derive(Debug, Deserialize)]
pub struct RetentionQuery {
    pub app_id: String,
    pub start_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InstallsQuery {
    pub app_id: String,
    #[serde(default = "default_days")]
    pub days: i32,
}

#[derive(Debug, Deserialize)]
pub struct FeedbackQuery {
    pub app_id: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_limit() -> i32 {
    50
}

// Response models
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct AppInfo {
    pub app_id: String,
    pub dau_today: i64,
    pub total_installs: i64,
}

#[derive(Debug, Serialize)]
pub struct AppsResponse {
    pub apps: Vec<AppInfo>,
}

#[derive(Debug, Serialize)]
pub struct DauData {
    pub date: String,
    pub dau: i64,
}

#[derive(Debug, Serialize)]
pub struct DauResponse {
    pub data: Vec<DauData>,
}

#[derive(Debug, Serialize)]
pub struct RetentionData {
    pub cohort_date: String,
    pub day0: i64,
    pub day1: Option<f64>,
    pub day7: Option<f64>,
    pub day30: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct RetentionResponse {
    pub data: Vec<RetentionData>,
}

#[derive(Debug, Serialize)]
pub struct InstallData {
    pub date: String,
    pub installs: i64,
}

#[derive(Debug, Serialize)]
pub struct InstallsResponse {
    pub total: i64,
    pub data: Vec<InstallData>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct FeedbackData {
    pub id: i64,
    pub content: String,
    pub user_id: Option<String>,
    pub contact: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct FeedbacksResponse {
    pub data: Vec<FeedbackData>,
}

// Database row models
#[derive(Debug, FromRow)]
pub struct DauRow {
    pub event_date: String,
    pub dau: i64,
}

#[derive(Debug, FromRow)]
pub struct InstallRow {
    pub event_date: String,
    pub installs: i64,
}

#[derive(Debug, FromRow)]
pub struct CountRow {
    pub count: i64,
}

#[derive(Debug, FromRow)]
pub struct AppIdRow {
    pub app_id: String,
}

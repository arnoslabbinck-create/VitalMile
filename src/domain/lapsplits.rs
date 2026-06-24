use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, sqlx::FromRow, Deserialize)]
pub struct LapSplits {
    pub id: String,
    pub run_session_id: String,
    pub lap_number: i32,
    pub pace_seconds_per_km: f64,
    pub heart_rate: Option<i32>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>
}
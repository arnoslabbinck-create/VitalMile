
use chrono::{DateTime, Utc, Local};
use serde::Deserialize;

#[derive(Debug, Default, sqlx::FromRow, Deserialize)]
pub struct RunSession {
    pub id: String,
    pub date: DateTime<Utc>,
    pub distance: f64,
    pub duration_minutes: i32,
    pub avg_heart_rate: Option<i32>,
    pub rpe: Option<f64>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>
}   
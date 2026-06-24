use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Default, sqlx::FromRow, Deserialize)]
pub struct DailyWellness {
    pub id: String,
    pub sleep_hours: f64,
    pub stress_level: i32,
    pub energy_level: i32,
    pub mood: i32,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

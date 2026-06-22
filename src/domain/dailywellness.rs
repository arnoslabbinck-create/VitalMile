
use sqlx::{postgres::PgRow, FromRow, QueryBuilder, Row};
use chrono::{DateTime, Local};


#[derive(Debug, Default, sqlx::FromRow)]
pub struct DailyWellness {
    pub id: String,
    pub sleep_hours: f64,
    pub stress_level: i32,
    pub energy_level: i32,
    pub mood: i32,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}


use chrono::{DateTime, Utc, Local};
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RunSession {
    pub id: String,
    pub date: DateTime<Utc>,
    pub distance: f64,
    pub duration_minutes: i32,
    pub avg_heart_rate: Option<i32>,
    pub rpe: Option<f64>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedÒAt: Option<DateTime<Local>>
}   
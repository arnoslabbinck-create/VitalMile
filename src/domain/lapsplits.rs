use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LapSplits {
    pub id: String,
    pub run_session_id: String,
    pub lap_number: i32,
    pub pace_seconds_per_km: f64,
    pub heart_rate: Option<i32>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>
}
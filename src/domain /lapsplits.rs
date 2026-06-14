use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct lapSplits {
    pub id: uuid::Uuid,
    pub run_session_id: Uuid,
    pub lap_number: i32,
    pub pace_seconds_per_km: f64,
    pub heart_rate: Option<i32>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>
}
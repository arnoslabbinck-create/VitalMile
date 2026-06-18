use chrono::{DateTime, Utc, Local  };
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrainingMarker {
    pub id: String,
    pub date: DateTime<Utc>,
    pub vo2_max: f64,
    pub lactate_threshold: f64,
    pub max_heart_rate: i32, 
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>
}
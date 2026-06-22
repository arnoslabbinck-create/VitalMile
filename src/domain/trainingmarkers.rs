use chrono::{DateTime, Utc, Local  };
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, sqlx::FromRow)]
pub struct TrainingMarker {
    pub id: String,
    pub date: DateTime<Utc>,
    pub vo2_max: f64,
    pub lactate_threshold: f64,
    pub max_heart_rate: i32, 
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>
}
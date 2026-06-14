use chrono::{DateTime, Utc, Local};
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct dailyWellness {
    pub id: uuid::Uuid,
    pub sleep_hours: f64,
    pub stress_level: i32,
    pub energy_level: i32,
    pub mood: i32,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>,
}
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};



#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DailyWellness {
    pub id: String,
    pub sleep_hours: f64,
    pub stress_level: i32,
    pub energy_level: i32,
    pub mood: i32,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>,
}

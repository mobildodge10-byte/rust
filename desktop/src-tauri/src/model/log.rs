use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ====================================
// Response DTOs
// ====================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventLog {
    pub id: i32,
    pub description: String,
    pub event_type_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActivityLog {
    pub id: i32,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

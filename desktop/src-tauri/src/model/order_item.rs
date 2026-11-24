use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// status dikirim sebagai nama: PENDING -> IN_PRODUCTION -> READY_FOR_PICKUP -> ASSIGNED -> IN_TRANSIT -> DELIVERED

// ====================================
// Response DTOs
// ====================================

// Sama dengan Location di server (pickup = manufacturer, destination = warehouse)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub name: String,
    pub x_position: f32,
    pub y_position: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub medicine_id: i32,
    pub medicine_name: String,
    pub quantity: i32,
    pub status: String,
    pub courier_id: Option<i32>,
    pub courier_codename: Option<String>,
    pub pickup: Location,
    pub destination: Location,
    pub pickup_time: Option<DateTime<Utc>>,
    pub delivery_time: Option<DateTime<Utc>>,
}

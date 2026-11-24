use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;

// ====================================
// Response DTOs
// ====================================

#[derive(Serialize)]
pub struct Location {
    pub name: String,
    pub x_position: f32,
    pub y_position: f32,
}

#[derive(Serialize)]
pub struct OrderItemResponse {
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
    pub pickup_time: Option<DateTimeUtc>,
    pub delivery_time: Option<DateTimeUtc>,
}
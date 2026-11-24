use serde::Serialize;

use crate::core::domain::dto::medicine::Medicine;

// ====================================
// Response DTOs
// ====================================

#[derive(Serialize)]
pub struct ManufacturerResponse {
    pub id: i32,
    pub name: String,
    pub region: String,
    pub x_position: f32,
    pub y_position: f32,
    pub medicine: Vec<Medicine>,
}
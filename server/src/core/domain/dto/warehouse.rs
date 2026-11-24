use serde::{Deserialize, Serialize};

// ====================================
// Request DTOs
// ====================================

#[derive(Deserialize)]
pub struct GetWarehouseInventoryRequest {
    pub warehouse_id: i32,
}


// ====================================
// Response DTOs
// ====================================

#[derive(Serialize)]
pub struct WarehouseResponse {
    pub id: i32,
    pub name: String,
    pub region: String,
    pub capacity: i32,
    pub x_position: f32,
    pub y_position: f32,
    pub inventory: Vec<WarehouseInventoryItem>,
}


#[derive(Serialize)]
pub struct WarehouseInventoryItem {
    pub medicine_id: i32,
    pub medicine_name: String,
    pub quantity: i32,
    pub reorder_threshold: Option<i32>,
}
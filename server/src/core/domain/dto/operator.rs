use serde::{Deserialize, Serialize};


// ====================================
// Request DTOs
// ====================================

#[derive(Deserialize)]
pub struct GetTaskListRequest {
    pub operator_id: i32,
}

#[derive(Deserialize)]
pub struct CompleteTaskRequest {
    pub operator_id: i32,
    pub order_item_id: i32,
}

#[derive(Deserialize)]
pub struct PickupMedicineRequest {
    pub operator_id: i32,
    pub order_item_id: i32,
}

#[derive(Deserialize)]
pub struct ViewJobStatsRequest {
    pub operator_id: i32,
}

#[derive(Deserialize)]
pub struct DispatchOperatorRequest {
    pub operator_id: i32,
    pub order_item_id: i32,
}

// ====================================
// Response DTOs
// ====================================

#[derive(Serialize)]
pub struct OperatorDetailsResponse {
    pub id: i32,
    pub fullname: String,
    pub codename: String,
    pub age: i32,
    pub gender: String,
    pub race: String,
    pub role: String,
    pub place_of_birth: String,
    pub date_of_birth: chrono::NaiveDate,
    pub region_specialty: String,
    pub completed_job_count: i32,
    pub x_position: f32,
    pub y_position: f32,
    pub availability: i8,
}
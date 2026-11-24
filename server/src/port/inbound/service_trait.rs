use axum::http::{StatusCode};
use crate::core::domain::dto::operator::OperatorDetailsResponse;
use crate::core::domain::dto::order::OrderItemResponse;
use crate::core::domain::dto::manufacturer::ManufacturerResponse;
use crate::core::domain::dto::warehouse::WarehouseResponse;
use crate::core::domain::entity::event_log::Model as EventLogModel;
use crate::core::domain::entity::activity_log::Model as ActivityLogModel;
use crate::core::domain::entity::event_type::Model as EventTypeModel;
use crate::core::domain::entity::region::Model as RegionModel;
use crate::core::domain::entity::medicine::Model as MedicineModel;
use crate::core::domain::entity::warehouse::ModelEx as WarehouseModelEx;
use crate::core::domain::entity::order_item::ModelEx as OrderItemModelEx;
use crate::core::domain::entity::operator::Model as OperatorModel;
use crate::core::domain::entity::manufacturer::ModelEx as ManufacturerModelEx;

pub trait ServiceTrait: Send + Sync {

    // Dispatcher
    fn view_operator_list(&self) -> impl Future<Output = Result<Vec<OperatorDetailsResponse>, StatusCode>> + Send;
    fn view_order_item(&self) -> impl Future<Output = Result<Vec<OrderItemResponse>, StatusCode>> + Send;

    // Courier
    fn view_task_list(&self, operator_id: i32) -> impl Future<Output = Result<Vec<OrderItemResponse>, StatusCode>> + Send;
    fn complete_task(&self, operator_id: i32, order_item_id: i32) -> impl Future<Output = Result<(), StatusCode>> + Send;
    fn pickup_medicine(&self, operator_id: i32, order_item_id: i32) -> impl Future<Output = Result<(), StatusCode>> + Send;
    fn view_job_stats(&self, operator_id: i32) -> impl Future<Output = Result<OperatorDetailsResponse, StatusCode>> + Send;

    // Manufacturer
    fn view_manufacturer_list(&self) -> impl Future<Output = Result<Vec<ManufacturerResponse>, StatusCode>> + Send;

    // Warehouse
    fn view_warehouse_list(&self) -> impl Future<Output = Result<Vec<WarehouseResponse>, StatusCode>> + Send;
    fn view_warehouse_inventory(&self, warehouse_id: i32) -> impl Future<Output = Result<WarehouseResponse, StatusCode>> + Send;

    // Simulation
    fn view_event_log(&self) -> impl Future<Output = Result<Vec<EventLogModel>, StatusCode>> + Send;
    fn view_activity_log(&self) -> impl Future<Output = Result<Vec<ActivityLogModel>, StatusCode>> + Send;
    fn record_event(&self, event_type_id: i32, description: String) -> impl Future<Output = Result<(), StatusCode>> + Send;
    fn decrease_stock(&self, warehouse_id: i32, medicine_id: i32, quantity: i32) -> impl Future<Output = Result<(), StatusCode>> + Send;
    fn produce_medicine_batch(&self) -> impl Future<Output = Result<u64, StatusCode>> + Send;
    fn restock_warehouse(&self, warehouse_id: i32) -> impl Future<Output = Result<(), StatusCode>> + Send;
    fn list_event_types(&self) -> impl Future<Output = Result<Vec<EventTypeModel>, StatusCode>> + Send;
    fn list_regions(&self) -> impl Future<Output = Result<Vec<RegionModel>, StatusCode>> + Send;
    fn list_medicines(&self) -> impl Future<Output = Result<Vec<MedicineModel>, StatusCode>> + Send;
    fn list_warehouses(&self) -> impl Future<Output = Result<Vec<WarehouseModelEx>, StatusCode>> + Send;

    // Courier Movement
    fn list_active_order_items(&self) -> impl Future<Output = Result<Vec<OrderItemModelEx>, StatusCode>> + Send;
    fn list_available_operators(&self) -> impl Future<Output = Result<Vec<OperatorModel>, StatusCode>> + Send;
    fn list_manufacturers(&self) -> impl Future<Output = Result<Vec<ManufacturerModelEx>, StatusCode>> + Send;
    fn move_operators(&self, positions: Vec<(i32, f32, f32)>) -> impl Future<Output = Result<(), StatusCode>> + Send;
    fn dispatch_operator(&self, operator_id: i32, order_item_id: i32) -> impl Future<Output = Result<(), StatusCode>> + Send;
}

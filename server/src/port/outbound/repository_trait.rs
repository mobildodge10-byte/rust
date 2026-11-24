use sea_orm::DbErr;

use crate::core::domain::entity::order_item::{ActiveModel as OrderItemActiveModel, ModelEx as OrderItemModelEx, Model as OrderItemModel};
use crate::core::domain::entity::medicine_type::Model as MedicineTypeModel;
use crate::core::domain::entity::medicine::Model as MedicineModel;
use crate::core::domain::entity::warehouse::{ModelEx as WarehouseModelEx};
use crate::core::domain::entity::warehouse_inventory::{Model as WarehouseInventoryModel, ModelEx as WarehouseInventoryModelEx, ActiveModel as WarehouseInventoryActiveModel};
use crate::core::domain::entity::manufacturer::ModelEx as ManufacturerModelEx;
use crate::core::domain::entity::operator::{ModelEx as OperatorModelEx, ActiveModel as OperatorActiveModel, Model as OperatorModel};
use crate::core::domain::entity::{region::Model as RegionModel, race::Model as RaceModel, role::Model as RoleModel, event_type::Model as EventTypeModel};
use crate::core::domain::entity::order::ActiveModel as OrderActiveModel;
use crate::core::domain::entity::event_log::{ActiveModel as EventLogActiveModel, Model as EventLogModel};
use crate::core::domain::entity::activity_log::Model as ActivityLogModel;
use crate::core::domain::entity::activity_log::ActiveModel as ActivityLogActiveModel;

pub trait RepositoryTrait: Send + Sync {

    // Operator
    fn get_operator_by_id(&self, operator_id: i32) -> impl Future<Output = Result<OperatorModelEx, DbErr>> + Send;
    fn get_operator_list(&self) -> impl Future<Output = Result<Vec<OperatorModelEx>, DbErr>> + Send;
    fn find_operator(&self, operator_id: i32) -> impl Future<Output = Result<OperatorModel, DbErr>> + Send;
    fn dispatch_operator(&self, operator: OperatorActiveModel, order_item: OrderItemActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn get_operator_task_list(&self, operator_id: i32) -> impl Future<Output = Result<Vec<OrderItemModelEx>, DbErr>> + Send;
    
    // Order
    fn pickup_medicine(&self, order_item: OrderItemActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn finish_order_item(&self, operator: OperatorActiveModel, order_item: OrderItemActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn get_order_items(&self) -> impl Future<Output = Result<Vec<OrderItemModelEx>, DbErr>> + Send;
    fn get_order_item(&self, order_item_id: i32) -> impl Future<Output = Result<OrderItemModelEx, DbErr>> + Send;
    fn get_active_order_items(&self) -> impl Future<Output = Result<Vec<OrderItemModelEx>, DbErr>> + Send;

    // Warehouse
    fn load_warehouses(&self) -> impl Future<Output = Result<Vec<WarehouseModelEx>, DbErr>> + Send;
    fn load_warehouse(&self, warehouse_id: i32) -> impl Future<Output = Result<WarehouseModelEx, DbErr>> + Send;
    fn load_warehouse_inventory(&self, warehouse_id: i32) -> impl Future<Output = Result<Vec<WarehouseInventoryModelEx>, DbErr>> + Send;
    fn load_all_warehouse_inventory(&self) -> impl Future<Output = Result<Vec<WarehouseInventoryModelEx>, DbErr>> + Send;
    fn add_inventory_quantity(&self, warehouse_id: i32, medicine_id: i32, quantity: i32) -> impl Future<Output = Result<(), DbErr>> + Send;

    // Manufacturer
    fn load_manufacturers(&self) -> impl Future<Output = Result<Vec<ManufacturerModelEx>, DbErr>> + Send;

    // ==================================
    // Di bawah ini ngga ada bug
    // ==================================

    // Other
    fn load_region(&self) -> impl Future<Output = Result<Vec<RegionModel>, DbErr>> + Send;
    fn load_race(&self) -> impl Future<Output = Result<Vec<RaceModel>, DbErr>> + Send;
    fn load_role(&self) -> impl Future<Output = Result<Vec<RoleModel>, DbErr>> + Send;

    // Medicine
    fn load_medicine_types(&self) -> impl Future<Output = Result<Vec<MedicineTypeModel>, DbErr>> + Send;
    fn load_medicines(&self) -> impl Future<Output = Result<Vec<MedicineModel>, DbErr>> + Send;

    // Event
    fn load_event_types(&self) -> impl Future<Output = Result<Vec<EventTypeModel>, DbErr>> + Send;
    fn get_evet_log(&self) -> impl Future<Output = Result<Vec<EventLogModel>, DbErr>> + Send;
    fn get_activity_log(&self) -> impl Future<Output = Result<Vec<ActivityLogModel>, DbErr>> + Send;

    // Simulation
    fn create_order(&self, order: OrderActiveModel, order_items: Vec<OrderItemActiveModel>) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn get_open_order_items(&self, warehouse_id: i32) -> impl Future<Output = Result<Vec<OrderItemModel>, DbErr>> + Send;
    fn get_order_items_by_status(&self, status: i32) -> impl Future<Output = Result<Vec<OrderItemModel>, DbErr>> + Send;
    fn update_order_item(&self, order_item: OrderItemActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn update_operator_positions(&self, positions: Vec<(i32, f32, f32)>) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn get_available_operators(&self) -> impl Future<Output = Result<Vec<OperatorModel>, DbErr>> + Send;
    fn insert_event_log(&self, event_log: EventLogActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn get_inventory_item(&self, warehouse_id: i32, medicine_id: i32) -> impl Future<Output = Result<WarehouseInventoryModel, DbErr>> + Send;
    fn update_inventory(&self, inventory: WarehouseInventoryActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;
    fn insert_activity_log(&self, activity_log: ActivityLogActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;
}

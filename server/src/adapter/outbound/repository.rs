use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, TransactionTrait};
use sea_orm::sea_query::{CaseStatement, Expr, ExprTrait};
use crate::core::domain::entity::prelude::*;
use crate::core::domain::entity::order_item::{self, ActiveModel as OrderItemActiveModel, ModelEx as OrderItemModelEx, Model as OrderItemModel};
use crate::core::domain::entity::medicine_type::Model as MedicineTypeModel;
use crate::core::domain::entity::medicine::Model as MedicineModel;
use crate::core::domain::entity::warehouse::{self, ModelEx as WarehouseModelEx};
use crate::core::domain::entity::warehouse_inventory::{self, Model as WarehouseInventoryModel, ModelEx as WarehouseInventoryModelEx, ActiveModel as WarehouseInventoryActiveModel};
use crate::core::domain::entity::manufacturer::ModelEx as ManufacturerModelEx;
use crate::core::domain::entity::operator::{self, ModelEx as OperatorModelEx, ActiveModel as OperatorActiveModel, Model as OperatorModel};
use crate::core::domain::entity::{region::Model as RegionModel, race::Model as RaceModel, role::Model as RoleModel, event_type::Model as EventTypeModel};
use crate::core::domain::entity::order::{self, ActiveModel as OrderActiveModel};
use crate::core::domain::entity::event_log::{ActiveModel as EventLogActiveModel, Model as EventLogModel};
use crate::core::domain::entity::activity_log::Model as ActivityLogModel;
use crate::core::domain::entity::activity_log::ActiveModel as ActivityLogActiveModel;


use crate::port::outbound::repository_trait::RepositoryTrait;

pub struct Repository {
    database_connection: DatabaseConnection,
}

pub fn new_repository(database_connection: DatabaseConnection) -> Repository {
    Repository { database_connection }
}

impl RepositoryTrait for Repository {

    // ==============================================================
    // Operator Related Queries
    // ==============================================================

    async fn get_operator_by_id(&self, operator_id: i32) -> Result<OperatorModelEx, DbErr> {
        let operator = Operator::load()
        .filter(operator::Column::Id.eq(operator_id))
        .with(Race)
        .with(Role)
        .with(operator::Relation::Region2) // Place of Birth
        .with(operator::Relation::Region1) // Region Specialty
        .one(&self.database_connection)
        .await;

        match operator {
            Ok(Some(operator)) => Ok(operator),
            Ok(None) => Err(DbErr::RecordNotFound(format!("Operator with ID {} not found", operator_id))),
            Err(err) => Err(err),  
        }
    }

    async fn get_operator_list(&self) -> Result<Vec<OperatorModelEx>, DbErr> {
        let operators = Operator::load()
        .with(Race)
        .with(Role)
        .with(operator::Relation::Region2) // Place of Birth
        .with(operator::Relation::Region1) // Region Specialty
        .all(&self.database_connection)
        .await;

        match operators {
            Ok(operators) => Ok(operators),
            Err(err) => Err(err),
        }  
    }

    async fn find_operator(&self, operator_id: i32) -> Result<OperatorModel, DbErr> {
        let operator = Operator::find_by_id(operator_id).one(&self.database_connection).await;

        match operator {
            Ok(Some(operator)) => Ok(operator),
            Ok(None) => Err(DbErr::RecordNotFound(format!("Operator with ID {} not found", operator_id))),
            Err(err) => Err(err),
        }
    }

    async fn dispatch_operator(&self, operator: OperatorActiveModel, order_item: OrderItemActiveModel) -> Result<(), DbErr> {

        let tx = self.database_connection.begin().await?;

        let operator_result = operator.update(&tx).await;
        if let Err(err) = operator_result {
            return Err(err);
        }

        let order_item_result = order_item.update(&tx).await;
        if let Err(err) = order_item_result {
            return Err(err);
        }

        tx.commit().await?;

        Ok(())
    }

    async fn get_operator_task_list(&self, operator_id: i32) -> Result<Vec<OrderItemModelEx>, DbErr> {
        let order_items = OrderItem::load()
            .filter(order_item::Column::CourierId.eq(operator_id))
            .with((Medicine, Manufacturer))   
            .with((Order, Warehouse))         
            .with(OrderStatus)
            .with(Operator)
            .all(&self.database_connection)
            .await;

        match order_items {
            Ok(order_items) => Ok(order_items),
            Err(err) => Err(err),
        }
    }

    // ==============================================================
    // Order Related Queries
    // ==============================================================

    async fn pickup_medicine(&self, order_item: OrderItemActiveModel) -> Result<(), DbErr> {
    
        let order_item_result = order_item.update(&self.database_connection).await;
        if let Err(err) = order_item_result {
            return Err(err);
        }

        Ok(())
    }

    async fn finish_order_item(&self, operator: OperatorActiveModel, order_item: OrderItemActiveModel) -> Result<(), DbErr> {

        let tx = self.database_connection.begin().await?;

        let operator_result = operator.update(&tx).await;
        if let Err(err) = operator_result {
            return Err(err);
        }

        let order_item_result = order_item.update(&tx).await;
        if let Err(err) = order_item_result {
            return Err(err);
        }

        tx.commit().await?;

        Ok(())
    }

    async fn get_order_items(&self) -> Result<Vec<OrderItemModelEx>, DbErr> {
        let order_items = OrderItem::load()
        .with((Medicine, Manufacturer))
        .with((Order, Warehouse))
        .with(OrderStatus)
        .with(Operator)
        .all(&self.database_connection)
        .await;

        match order_items {
            Ok(order_items) => Ok(order_items),
            Err(err) => Err(err),
        }
    }

    async fn get_order_item(&self, order_item_id: i32) -> Result<OrderItemModelEx, DbErr> {
        let order_items = self.get_order_items().await;

        match order_items {
            Ok(order_items) => {
                for order_item in order_items {
                    if order_item.id == order_item_id {
                        return Ok(order_item);
                    }
                }
                Err(DbErr::RecordNotFound(format!("Order item with ID {} not found", order_item_id)))
            }
            Err(err) => Err(err),
        }
    }

    async fn get_active_order_items(&self) -> Result<Vec<OrderItemModelEx>, DbErr> {
        let order_items = OrderItem::load()
            .filter(order_item::Column::Status.is_in([4, 5]))
            .with(Order)
            .with(Operator)
            .all(&self.database_connection)
            .await;

        match order_items {
            Ok(order_items) => Ok(order_items),
            Err(err) => Err(err),
        }
    }

    // ==============================================================
    // Warehouse Related Queries (Ga ada bug di sini, jangan diganti)
    // ==============================================================

    async fn load_warehouses(&self) -> Result<Vec<WarehouseModelEx>, DbErr> {
        let warehouses = Warehouse::load()
        .with(Region)
        .all(&self.database_connection).await;
        match warehouses {
            Ok(warehouses) => Ok(warehouses),
            Err(err) => Err(err),
        }
    }

    async fn load_warehouse(&self, warehouse_id: i32) -> Result<WarehouseModelEx, DbErr> {
        let warehouse = Warehouse::load()
        .filter(warehouse::Column::Id.eq(warehouse_id))
        .with(Region)
        .one(&self.database_connection).await;
        
        match warehouse {
            Ok(Some(warehouse)) => Ok(warehouse),
            Ok(None) => Err(DbErr::RecordNotFound(format!("Warehouse with ID {} not found", warehouse_id))),
            Err(err) => Err(err),
        }
    }

    async fn load_warehouse_inventory(&self, warehouse_id: i32) -> Result<Vec<WarehouseInventoryModelEx>, DbErr> {
        let warehouse_inventory = WarehouseInventory::load()
        .filter(warehouse_inventory::Column::WarehouseId.eq(warehouse_id))
        .with(Medicine)
        .all(&self.database_connection).await;
        match warehouse_inventory {
            Ok(warehouse_inventory) => Ok(warehouse_inventory),
            Err(err) => Err(err),
        }
    }

    async fn load_all_warehouse_inventory(&self) -> Result<Vec<WarehouseInventoryModelEx>, DbErr> {
        let warehouse_inventory = WarehouseInventory::load()
        .with(Medicine)
        .all(&self.database_connection).await;

        match warehouse_inventory {
            Ok(warehouse_inventory) => Ok(warehouse_inventory),
            Err(err) => Err(err),
        }
    }

    async fn add_inventory_quantity(&self, warehouse_id: i32, medicine_id: i32, quantity: i32) -> Result<(), DbErr> {
        let result = WarehouseInventory::update_many()
        .col_expr(warehouse_inventory::Column::Quantity, Expr::col(warehouse_inventory::Column::Quantity).add(quantity))
        .filter(warehouse_inventory::Column::WarehouseId.eq(warehouse_id))
        .filter(warehouse_inventory::Column::MedicineId.eq(medicine_id))
        .exec(&self.database_connection)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected == 0 {
                    return Err(DbErr::RecordNotFound(format!("Medicine {} not found in warehouse {}", medicine_id, warehouse_id)));
                }
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    // ==============================================================
    // Manufacturer Related Queries (Ga ada bug di sini, jangan diganti)
    // ==============================================================

    async fn load_manufacturers(&self) -> Result<Vec<ManufacturerModelEx>, DbErr> {
        let manufacturers = Manufacturer::load()
        .with(Region)
        .with(Medicine)
        .all(&self.database_connection).await;
        match manufacturers {
            Ok(manufacturers) => Ok(manufacturers),
            Err(err) => Err(err),
        }
    }

    // ==============================================================
    // Load Reference Data (Ga ada bug di sini, jangan diganti)
    // ==============================================================

    async fn load_region(&self) -> Result<Vec<RegionModel>, DbErr> {
        let regions = Region::find().all(&self.database_connection).await;

        match regions {
            Ok(regions) => Ok(regions),
            Err(err) => Err(err),
        }
    }

    async fn load_race(&self) -> Result<Vec<RaceModel>, DbErr> {
        let races = Race::find().all(&self.database_connection).await;
        match races {
            Ok(races) => Ok(races),
            Err(err) => Err(err),
        }
    }

    async fn load_role(&self) -> Result<Vec<RoleModel>, DbErr> {
        let roles = Role::find().all(&self.database_connection).await;
        match roles {
            Ok(roles) => Ok(roles),
            Err(err) => Err(err),
        }
    }

    async fn load_medicine_types(&self) -> Result<Vec<MedicineTypeModel>, DbErr> {
        let medicine_types = MedicineType::find().all(&self.database_connection).await;
        match medicine_types {
            Ok(medicine_types) => Ok(medicine_types),
            Err(err) => Err(err),
        }
    }

    async fn load_medicines(&self) -> Result<Vec<MedicineModel>, DbErr> {
        let medicines = Medicine::find().all(&self.database_connection).await;
        match medicines {
            Ok(medicines) => Ok(medicines),
            Err(err) => Err(err),
        }
    }

    async fn load_event_types(&self) -> Result<Vec<EventTypeModel>, DbErr> {
        let event_types = EventType::find().all(&self.database_connection).await;
        match event_types {
            Ok(event_types) => Ok(event_types),
            Err(err) => Err(err),
        }
    }

    async fn get_evet_log(&self) -> Result<Vec<EventLogModel>, DbErr> {
        let event_logs = EventLog::find().all(&self.database_connection).await;
        match event_logs {
            Ok(event_logs) => Ok(event_logs),
            Err(err) => Err(err),
        }
    }

    async fn get_activity_log(&self) -> Result<Vec<ActivityLogModel>, DbErr> {
        let activity_logs = ActivityLog::find().all(&self.database_connection).await;
        match activity_logs {
            Ok(activity_logs) => Ok(activity_logs),
            Err(err) => Err(err),
        }
    }

    // ==============================================================
    // Simulation Related Queries (Ga ada bug di sini, jangan diganti)
    // ==============================================================

    async fn create_order(&self, order: OrderActiveModel, order_item: Vec<OrderItemActiveModel>) -> Result<(), DbErr> {

        let tx = self.database_connection.begin().await?;

        let order = order.insert(&tx).await?;

        let mut items = Vec::new();
        for item in order_item {
            let mut item = item;
            item.order_id = sea_orm::ActiveValue::Set(order.id);
            items.push(item);
        }

        OrderItem::insert_many(items).exec(&tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn get_open_order_items(&self, warehouse_id: i32) -> Result<Vec<OrderItemModel>, DbErr> {
        let order_items = OrderItem::find()
            .inner_join(Order)
            .filter(order::Column::WarehouseId.eq(warehouse_id))
            .filter(order_item::Column::Status.lt(6))
            .all(&self.database_connection)
            .await;

        match order_items {
            Ok(order_items) => Ok(order_items),
            Err(err) => Err(err),
        }
    }

    async fn get_order_items_by_status(&self, status: i32) -> Result<Vec<OrderItemModel>, DbErr> {
        let order_items = OrderItem::find()
            .filter(order_item::Column::Status.eq(status))
            .all(&self.database_connection)
            .await;

        match order_items {
            Ok(order_items) => Ok(order_items),
            Err(err) => Err(err),
        }
    }

    async fn update_order_item(&self, order_item: OrderItemActiveModel) -> Result<(), DbErr> {
        let result = order_item.update(&self.database_connection).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn update_operator_positions(&self, positions: Vec<(i32, f32, f32)>) -> Result<(), DbErr> {
        if positions.is_empty() {
            return Ok(());
        }

        let mut operator_ids = Vec::new();
        let mut x_case = CaseStatement::new();
        let mut y_case = CaseStatement::new();

        for (operator_id, x_position, y_position) in positions {
            operator_ids.push(operator_id);
            x_case = x_case.case(operator::Column::Id.eq(operator_id), x_position);
            y_case = y_case.case(operator::Column::Id.eq(operator_id), y_position);
        }

        let result = Operator::update_many()
            .col_expr(operator::Column::XPosition, x_case.finally(Expr::col(operator::Column::XPosition)).into())
            .col_expr(operator::Column::YPosition, y_case.finally(Expr::col(operator::Column::YPosition)).into())
            .filter(operator::Column::Id.is_in(operator_ids))
            .exec(&self.database_connection)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn get_available_operators(&self) -> Result<Vec<OperatorModel>, DbErr> {
        let operators = Operator::find()
            .filter(operator::Column::Availability.eq(1))
            .all(&self.database_connection)
            .await;

        match operators {
            Ok(operators) => Ok(operators),
            Err(err) => Err(err),
        }
    }

    async fn insert_event_log(&self, event_log: EventLogActiveModel) -> Result<(), DbErr> {
        let result = event_log.insert(&self.database_connection).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn get_inventory_item(&self, warehouse_id: i32, medicine_id: i32) -> Result<WarehouseInventoryModel, DbErr> {
        let inventory = WarehouseInventory::find_by_id((warehouse_id, medicine_id)).one(&self.database_connection).await?;

        match inventory {
            Some(inventory) => Ok(inventory),
            None => Err(DbErr::RecordNotFound(format!("Medicine {} not found in warehouse {}", medicine_id, warehouse_id))),
        }
    }

    async fn update_inventory(&self, inventory: WarehouseInventoryActiveModel) -> Result<(), DbErr> {
        let result = inventory.update(&self.database_connection).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn insert_activity_log(&self, activity_log: ActivityLogActiveModel) -> Result<(), DbErr> {
        activity_log.insert(&self.database_connection).await.map(|_| ())
    }
}


use sea_orm::{ActiveValue, DbErr, IntoActiveModel};
use axum::http::{StatusCode};
use crate::port::outbound::repository_trait::RepositoryTrait;
use crate::port::inbound::service_trait::ServiceTrait;
use crate::core::domain::dto::operator::OperatorDetailsResponse;
use crate::core::domain::dto::order::OrderItemResponse;
use crate::core::domain::dto::order::Location;
use crate::core::domain::dto::manufacturer::ManufacturerResponse;
use crate::core::domain::dto::warehouse::WarehouseResponse;
use crate::core::domain::dto::medicine::Medicine;
use crate::core::domain::dto::warehouse::WarehouseInventoryItem;
use crate::core::domain::entity::event_log::Model as EventLogModel;
use crate::core::domain::entity::event_log::ActiveModel as EventLogActiveModel;
use crate::core::domain::entity::activity_log::Model as ActivityLogModel;
use crate::core::domain::entity::order_item::ActiveModel as OrderItemActiveModel;
use crate::core::domain::entity::order::ActiveModel as OrderActiveModel;
use crate::core::domain::entity::event_type::Model as EventTypeModel;
use crate::core::domain::entity::region::Model as RegionModel;
use crate::core::domain::entity::medicine::Model as MedicineModel;
use crate::core::domain::entity::warehouse::ModelEx as WarehouseModelEx;
use crate::core::domain::entity::order_item::ModelEx as OrderItemModelEx;
use crate::core::domain::entity::operator::{ActiveModel as OperatorActiveModel, Model as OperatorModel};
use crate::core::domain::entity::manufacturer::ModelEx as ManufacturerModelEx;
use crate::core::domain::entity::activity_log::ActiveModel as ActivityLogActiveModel;


const RESTOCK_MULTIPLIER: i32 = 3;

// Courier dianggap sudah sampai kalau jaraknya ke manufacturer / warehouse <= ARRIVAL_RADIUS unit peta
const ARRIVAL_RADIUS: f32 = 20.0;

pub struct Service<R: RepositoryTrait> {
    repository: R,
}

pub fn new_service<R: RepositoryTrait>(repository: R) -> Service<R> {
    Service { repository }
}

impl<R: RepositoryTrait + Send + Sync> Service<R> {
    async fn log_activity(&self, description: String) {
        let activity = ActivityLogActiveModel {
            description: ActiveValue::Set(description),
            ..Default::default()
        };
        // Gagal mencatat log tidak boleh membatalkan aksi utama, cukup dicetak ke console
        if let Err(err) = self.repository.insert_activity_log(activity).await {
            eprintln!("Failed to write activity log: {err}");
        }
    }
}

impl<R: RepositoryTrait + Send + Sync> ServiceTrait for Service<R> {

    // ==============================================================
    // Dispatcher
    // ==============================================================
    
    async fn view_operator_list(&self) -> Result<Vec<OperatorDetailsResponse>, StatusCode> {
        let operators = self.repository.get_operator_list().await;
        if operators.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let operators = operators.unwrap();
        let mut operator_data = Vec::new();

        for operator in operators {

            let race = match operator.race.into_option() {
                Some(race) => race.name,
                None => String::new(),
            };

            let place_of_birth = match operator.region_2.into_option() {
                Some(place) => place.name,
                None => String::new(),
            };

            let role = match operator.role.into_option() {
                Some(role) => role.name,
                None => String::new(),
            };

            let region_specialty = match operator.region_1.into_option() {
                Some(region) => region.name,
                None => "No specialty".to_string(),
            };

            let operator_details = OperatorDetailsResponse {
                id: operator.id,
                fullname: operator.fullname,
                codename: operator.codename,
                age: operator.age,
                gender: operator.gender,
                race: race,
                role: role,
                place_of_birth: place_of_birth,
                date_of_birth: operator.date_of_birth,
                region_specialty: region_specialty,
                completed_job_count: operator.completed_job_count,
                x_position: operator.x_position,
                y_position: operator.y_position,
                availability: operator.availability,
            };
            operator_data.push(operator_details);
        }

        Ok(operator_data)
    }

    async fn view_order_item(&self) -> Result<Vec<OrderItemResponse>, StatusCode>  {

        let order_items = self.repository.get_order_items().await;
        if order_items.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let order_items = order_items.unwrap();
        let mut order_item_data = Vec::new();

        for order_item in order_items {
            let status = match order_item.order_status.into_option() {
                Some(order_status) => order_status.status,
                None => String::new(),
            };

            let courier_codename = match order_item.operator.into_option() {
                Some(courier) => Some(courier.codename),
                None => None,
            };

            let mut medicine_name = String::new();
            let mut pickup = Location { name: String::new(), x_position: 0.0, y_position: 0.0 };

            if let Some(medicine) = order_item.medicine.into_option() {
                medicine_name = medicine.name;

                if let Some(manufacturer) = medicine.manufacturer.into_option() {
                    pickup = Location { name: manufacturer.name, x_position: manufacturer.x_position, y_position: manufacturer.y_position };
                }
            }

            let mut destination = Location { name: String::new(), x_position: 0.0, y_position: 0.0 };

            if let Some(order) = order_item.order.into_option() {
                if let Some(warehouse) = order.warehouse.into_option() {
                    destination = Location { name: warehouse.name, x_position: warehouse.x_position, y_position: warehouse.y_position };
                }
            }

            let pickup_time = if order_item.status >= 5 { Some(order_item.pickup_time) } else { None };
            let delivery_time = if order_item.status == 6 { Some(order_item.delivery_time) } else { None };

            order_item_data.push(OrderItemResponse {
                id: order_item.id,
                order_id: order_item.order_id,
                medicine_id: order_item.medicine_id,
                medicine_name,
                quantity: order_item.quantity,
                status,
                courier_id: order_item.courier_id,
                courier_codename,
                pickup,
                destination,
                pickup_time,
                delivery_time,
            });
        }

        Ok(order_item_data)
    }    

    async fn dispatch_operator(&self, operator_id: i32, order_item_id: i32) -> Result<(), StatusCode> {
        if operator_id <= 0 || order_item_id <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let operator = self.repository.find_operator(operator_id).await;
        if let Err(DbErr::RecordNotFound(_)) = operator {
            return Err(StatusCode::NOT_FOUND);
        }
        if operator.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let order_item = self.repository.get_order_item(order_item_id).await;
        if let Err(DbErr::RecordNotFound(_)) = order_item {
            return Err(StatusCode::NOT_FOUND);
        }
        if order_item.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let order_item = order_item.unwrap();
        let operator = operator.unwrap();

        if operator.availability != 1 {
            return Err(StatusCode::BAD_REQUEST);
        }

        if operator.role_id != 2 {
            return Err(StatusCode::BAD_REQUEST);
        }

        if order_item.status != 3 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let operator_active = OperatorActiveModel {
            id: ActiveValue::Unchanged(operator_id),
            availability: ActiveValue::Set(0),
            ..Default::default()
        };

        let order_item_active = OrderItemActiveModel {
            id: ActiveValue::Unchanged(order_item_id),
            status: ActiveValue::Set(4), // 4 itu Assigned
            courier_id: ActiveValue::Set(Some(operator_id)),
            ..Default::default()
        };

        let dispatch_result = self.repository.dispatch_operator(operator_active, order_item_active).await;
        if dispatch_result.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        self.log_activity(format!("Operator #{} dispatched to order item #{}", operator_id, order_item_id)).await;
        Ok(())
    }



    // ==============================================================
    // Courier
    // ==============================================================

    async fn view_task_list(&self, operator_id: i32) -> Result<Vec<OrderItemResponse>, StatusCode> {
        if operator_id <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let order_items = self.repository.get_operator_task_list(operator_id).await;
        if order_items.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let order_items = order_items.unwrap();
        let mut order_item_data = Vec::new();

        for order_item in order_items {
            let status = match order_item.order_status.into_option() {
                Some(order_status) => order_status.status,
                None => String::new(),
            };

            let courier_codename = match order_item.operator.into_option() {
                Some(courier) => Some(courier.codename),
                None => None,
            };

            let mut medicine_name = String::new();
            let mut pickup = Location { name: String::new(), x_position: 0.0, y_position: 0.0 };

            if let Some(medicine) = order_item.medicine.into_option() {
                medicine_name = medicine.name;

                if let Some(manufacturer) = medicine.manufacturer.into_option() {
                    pickup = Location { name: manufacturer.name, x_position: manufacturer.x_position, y_position: manufacturer.y_position };
                }
            }

            let mut destination = Location { name: String::new(), x_position: 0.0, y_position: 0.0 };

            if let Some(order) = order_item.order.into_option() {
                if let Some(warehouse) = order.warehouse.into_option() {
                    destination = Location { name: warehouse.name, x_position: warehouse.x_position, y_position: warehouse.y_position };
                }
            }

            let pickup_time = if order_item.status >= 5 { Some(order_item.pickup_time) } else { None };
            let delivery_time = if order_item.status == 6 { Some(order_item.delivery_time) } else { None };

            order_item_data.push(OrderItemResponse {
                id: order_item.id,
                order_id: order_item.order_id,
                medicine_id: order_item.medicine_id,
                medicine_name,
                quantity: order_item.quantity,
                status,
                courier_id: order_item.courier_id,
                courier_codename,
                pickup,
                destination,
                pickup_time,
                delivery_time,
            });
        }

        Ok(order_item_data)
    }
    
    async fn complete_task(&self, operator_id: i32, order_item_id: i32) -> Result<(), StatusCode> {
        if operator_id <= 0 || order_item_id <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let operator = self.repository.find_operator(operator_id).await;
        if let Err(DbErr::RecordNotFound(_)) = operator {
            return Err(StatusCode::NOT_FOUND);
        }
        if operator.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let order_item = self.repository.get_order_item(order_item_id).await;
        if let Err(DbErr::RecordNotFound(_)) = order_item {
            return Err(StatusCode::NOT_FOUND);
        }
        if order_item.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let order_item = order_item.unwrap();

        if order_item.courier_id != Some(operator_id) {
            return Err(StatusCode::FORBIDDEN);
        }

        if order_item.status != 5 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let operator = operator.unwrap();

        let mut destination = None;
        if let Some(order) = order_item.order.into_option() {
            if let Some(warehouse) = order.warehouse.into_option() {
                destination = Some((warehouse.id, warehouse.x_position, warehouse.y_position));
            }
        }

        let (warehouse_id, warehouse_x, warehouse_y) = match destination {
            Some(destination) => destination,
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let dx = warehouse_x - operator.x_position;
        let dy = warehouse_y - operator.y_position;
        if (dx * dx + dy * dy).sqrt() > ARRIVAL_RADIUS {
            return Err(StatusCode::CONFLICT);
        }

        let operator_active = OperatorActiveModel {
            id: ActiveValue::Unchanged(operator_id),
            availability: ActiveValue::Set(1), // 1 itu Available
            completed_job_count: ActiveValue::Set(operator.completed_job_count + 1),
            ..Default::default()
        };

        let order_item_active = OrderItemActiveModel {
            id: ActiveValue::Unchanged(order_item_id),
            status: ActiveValue::Set(6), // 6 itu Delivered
            delivery_time: ActiveValue::Set(chrono::Utc::now()),
            ..Default::default()
        };

        let finish_result = self.repository.finish_order_item(operator_active, order_item_active).await;
        if finish_result.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let stock_result = self.repository.add_inventory_quantity(warehouse_id, order_item.medicine_id, order_item.quantity).await;
        if stock_result.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        self.log_activity(format!("Operator #{} delivered order item #{}", operator_id, order_item_id)).await;
        Ok(())
    }
    
    async fn pickup_medicine(&self, operator_id: i32, order_item_id: i32) -> Result<(), StatusCode> {
        if operator_id <= 0 || order_item_id <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let order_item = self.repository.get_order_item(order_item_id).await;
        if let Err(DbErr::RecordNotFound(_)) = order_item {
            return Err(StatusCode::NOT_FOUND);
        }
        if order_item.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let order_item = order_item.unwrap();

        if order_item.courier_id != Some(operator_id) {
            return Err(StatusCode::FORBIDDEN);
        }

        if order_item.status != 4 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let operator = self.repository.find_operator(operator_id).await;
        if let Err(DbErr::RecordNotFound(_)) = operator {
            return Err(StatusCode::NOT_FOUND);
        }
        if operator.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let operator = operator.unwrap();

        let mut pickup = None;
        if let Some(medicine) = order_item.medicine.into_option() {
            if let Some(manufacturer) = medicine.manufacturer.into_option() {
                pickup = Some((manufacturer.x_position, manufacturer.y_position));
            }
        }

        let (pickup_x, pickup_y) = match pickup {
            Some(pickup) => pickup,
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let dx = pickup_x - operator.x_position;
        let dy = pickup_y - operator.y_position;
        if (dx * dx + dy * dy).sqrt() > ARRIVAL_RADIUS {
            return Err(StatusCode::CONFLICT);
        }

        let order_item_active = OrderItemActiveModel {
            id: ActiveValue::Unchanged(order_item_id),
            status: ActiveValue::Set(5), // 5 itu In Transit
            pickup_time: ActiveValue::Set(chrono::Utc::now()),
            ..Default::default()
        };

        let pickup_result = self.repository.pickup_medicine(order_item_active).await;
        
        match pickup_result {
            Ok(_) => {
                self.log_activity(format!("Operator #{} picked up order item #{}", operator_id, order_item_id)).await;
                Ok(())
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    
    async fn view_job_stats(&self, operator_id: i32) -> Result<OperatorDetailsResponse, StatusCode> {
        if operator_id <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let stats = self.repository.get_operator_by_id(operator_id).await;
        match stats {
            Ok(operator) => {
                let race = match operator.race.into_option() {
                    Some(race) => race.name,
                    None => String::new(),
                };

                let place_of_birth = match operator.region_2.into_option() {
                    Some(region) => region.name,
                    None => String::new(),
                };

                let role = match operator.role.into_option() {
                    Some(role) => role.name,
                    None => String::new(),
                };

                let region_specialty = match operator.region_1.into_option() {
                    Some(region) => region.name,
                    None => "No specialty".to_string(),
                };

                let operator_details = OperatorDetailsResponse {
                    id: operator.id,
                    fullname: operator.fullname,
                    codename: operator.codename,
                    age: operator.age,
                    gender: operator.gender,
                    race: race,
                    role: role,
                    place_of_birth: place_of_birth,
                    date_of_birth: operator.date_of_birth,
                    region_specialty: region_specialty,
                    completed_job_count: operator.completed_job_count,
                    x_position: operator.x_position,
                    y_position: operator.y_position,
                    availability: operator.availability,
                };

                Ok(operator_details)
            },
            Err(DbErr::RecordNotFound(_)) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    // ==============================================================
    // Manufacturer
    // ==============================================================

    async fn view_manufacturer_list(&self) -> Result<Vec<ManufacturerResponse>, StatusCode> {
        let manufacturers = self.repository.load_manufacturers().await;
        if manufacturers.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let manufacturers = manufacturers.unwrap();

        let mut manufacturer_data = Vec::new();

        for manufacturer in manufacturers {
            let mut medicines = Vec::new();
            for medicine in manufacturer.medicines {
                medicines.push(Medicine {
                    id: medicine.id,
                    name: medicine.name,
                    description: medicine.description.unwrap_or_default(),
                    medicine_type_id: medicine.medicine_type_id,
                    dosage_form: medicine.dosage_form,
                });
            }

            let region = match manufacturer.region.into_option() {
                Some(region) => region.name,
                None => String::new(),
            };

            manufacturer_data.push(ManufacturerResponse {
                id: manufacturer.id,
                name: manufacturer.name,
                region,
                x_position: manufacturer.x_position,
                y_position: manufacturer.y_position,
                medicine: medicines,
            });
        }
        
        Ok(manufacturer_data)
    }

    // ==============================================================
    // Warehouse
    // ==============================================================

    async fn view_warehouse_list(&self) -> Result<Vec<WarehouseResponse>, StatusCode> {

        let warehouses = self.repository.load_warehouses().await;
        if warehouses.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let warehouses = warehouses.unwrap();
        let mut warehouse_data = Vec::new();

        for warehouse in warehouses {
            let inventory = self.repository.load_warehouse_inventory(warehouse.id).await;
            if inventory.is_err() {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }

            let mut items = Vec::new();

            for item in inventory.unwrap() {
                let medicine_name = match item.medicine.into_option() {
                    Some(medicine) => medicine.name,
                    None => "Unnamed Medicine".to_string(),
                };

                items.push(WarehouseInventoryItem {
                    medicine_id: item.medicine_id,
                    medicine_name,
                    quantity: item.quantity,
                    reorder_threshold: Some(item.reorder_threshold),
                });
            }

            let region = match warehouse.region.into_option() {
                Some(region) => region.name,
                None => String::new(),
            };

            warehouse_data.push(WarehouseResponse {
                id: warehouse.id,
                name: warehouse.name,
                region,
                capacity: warehouse.capacity,
                x_position: warehouse.x_position,
                y_position: warehouse.y_position,
                inventory: items,
            });
        }

        Ok(warehouse_data)
    }

    async fn view_warehouse_inventory(&self, warehouse_id: i32) -> Result<WarehouseResponse, StatusCode> {
        if warehouse_id <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let warehouse = self.repository.load_warehouse(warehouse_id).await;
        if let Err(DbErr::RecordNotFound(_)) = warehouse {
            return Err(StatusCode::NOT_FOUND);
        }
        if warehouse.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let inventory = self.repository.load_warehouse_inventory(warehouse_id).await;
        if inventory.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let warehouse = warehouse.unwrap();
        let mut items = Vec::new();

        for item in inventory.unwrap() {
            let medicine_name = match item.medicine.into_option() {
                Some(medicine) => medicine.name,
                None => "Unnamed Medicine".to_string(),
            };

            items.push(WarehouseInventoryItem {
                medicine_id: item.medicine_id,
                medicine_name,
                quantity: item.quantity,
                reorder_threshold: Some(item.reorder_threshold),
            });
        }

        let region = match warehouse.region.into_option() {
            Some(region) => region.name,
            None => String::new(),
        };

        Ok(WarehouseResponse {
            id: warehouse.id,
            name: warehouse.name,
            region,
            capacity: warehouse.capacity,
            x_position: warehouse.x_position,
            y_position: warehouse.y_position,
            inventory: items,
        })
    }









































    

    // ==============================================================
    // Simulation (Ga ada bug di sini, sumpah, jangan diganti-ganti)
    // ==============================================================

    async fn view_event_log(&self) -> Result<Vec<EventLogModel>, StatusCode> {
        let event_logs = self.repository.get_evet_log().await;

        match event_logs {
            Ok(event_logs) => Ok(event_logs),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn view_activity_log(&self) -> Result<Vec<ActivityLogModel>, StatusCode> {
        let activity_logs = self.repository.get_activity_log().await;

        match activity_logs {
            Ok(activity_logs) => Ok(activity_logs),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn record_event(&self, event_type_id: i32, description: String) -> Result<(), StatusCode> {
        if event_type_id <= 0 || description.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let event_log = EventLogActiveModel {
            event_type_id: ActiveValue::Set(event_type_id),
            description: ActiveValue::Set(description),
            ..Default::default()
        };

        let record_result = self.repository.insert_event_log(event_log).await;

        match record_result {
            Ok(_) => Ok(()),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn decrease_stock(&self, warehouse_id: i32, medicine_id: i32, quantity: i32) -> Result<(), StatusCode> {
        if warehouse_id <= 0 || medicine_id <= 0 || quantity <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let inventory = self.repository.get_inventory_item(warehouse_id, medicine_id).await;
        if inventory.is_err() {
            return Err(StatusCode::NOT_FOUND);
        }

        let inventory = inventory.unwrap();

        let new_quantity = if inventory.quantity > quantity { inventory.quantity - quantity } else { 0 };

        let mut active = inventory.into_active_model();
        active.quantity = ActiveValue::Set(new_quantity);

        let decrease_result = self.repository.update_inventory(active).await;

        match decrease_result {
            Ok(_) => Ok(()),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn produce_medicine_batch(&self) -> Result<u64, StatusCode> {
        let in_production = self.repository.get_order_items_by_status(2).await;
        if in_production.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let pending = self.repository.get_order_items_by_status(1).await;
        if pending.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let in_production = in_production.unwrap();
        let pending = pending.unwrap();
        let mut moved_count = 0;

        for order_item in in_production {
            let mut active = order_item.into_active_model();
            active.status = ActiveValue::Set(3);

            let update_result = self.repository.update_order_item(active).await;
            if update_result.is_err() {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            moved_count += 1;
        }

        for order_item in pending {
            let mut active = order_item.into_active_model();
            active.status = ActiveValue::Set(2);

            let update_result = self.repository.update_order_item(active).await;
            if update_result.is_err() {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            moved_count += 1;
        }

        Ok(moved_count)
    }

    async fn restock_warehouse(&self, warehouse_id: i32) -> Result<(), StatusCode> {
        if warehouse_id <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let inventory = self.repository.load_warehouse_inventory(warehouse_id).await;
        if inventory.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let open_order_items = self.repository.get_open_order_items(warehouse_id).await;
        if open_order_items.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }


        let inventory = inventory.unwrap();
        let open_order_items = open_order_items.unwrap();
        let mut order_items: Vec<OrderItemActiveModel> = Vec::new();

        for item in inventory {
            let already_ordered = open_order_items.iter().any(|order_item| order_item.medicine_id == item.medicine_id);
            // Skip kalau stock masih cukup atau medicine ini sudah dipesan
            if item.quantity > item.reorder_threshold || already_ordered {
                continue;
            }

            order_items.push(OrderItemActiveModel {
                medicine_id: ActiveValue::Set(item.medicine_id),
                quantity: ActiveValue::Set(item.reorder_threshold * RESTOCK_MULTIPLIER - item.quantity),
                status: ActiveValue::Set(1), // 1 itu Pending
                ..Default::default()
            });
        }

        if order_items.is_empty() {
            return Ok(());
        }

        let order = OrderActiveModel {
            warehouse_id: ActiveValue::Set(warehouse_id),
            ..Default::default()
        };

        let create_result = self.repository.create_order(order, order_items).await;

        match create_result {
            Ok(_) => Ok(()),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn list_event_types(&self) -> Result<Vec<EventTypeModel>, StatusCode> {
        let event_types = self.repository.load_event_types().await;

        match event_types {
            Ok(event_types) => Ok(event_types),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn list_regions(&self) -> Result<Vec<RegionModel>, StatusCode> {
        let regions = self.repository.load_region().await;

        match regions {
            Ok(regions) => Ok(regions),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn list_medicines(&self) -> Result<Vec<MedicineModel>, StatusCode> {
        let medicines = self.repository.load_medicines().await;

        match medicines {
            Ok(medicines) => Ok(medicines),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn list_warehouses(&self) -> Result<Vec<WarehouseModelEx>, StatusCode> {
        let warehouses = self.repository.load_warehouses().await;

        match warehouses {
            Ok(warehouses) => Ok(warehouses),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn list_active_order_items(&self) -> Result<Vec<OrderItemModelEx>, StatusCode> {
        let order_items = self.repository.get_active_order_items().await;

        match order_items {
            Ok(order_items) => Ok(order_items),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn list_available_operators(&self) -> Result<Vec<OperatorModel>, StatusCode> {
        let operators = self.repository.get_available_operators().await;

        match operators {
            Ok(operators) => Ok(operators),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn list_manufacturers(&self) -> Result<Vec<ManufacturerModelEx>, StatusCode> {
        let manufacturers = self.repository.load_manufacturers().await;

        match manufacturers {
            Ok(manufacturers) => Ok(manufacturers),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    // positions: (operator_id, x_position, y_position), disimpan sekaligus dalam 1 query
    async fn move_operators(&self, positions: Vec<(i32, f32, f32)>) -> Result<(), StatusCode> {
        for (operator_id, _, _) in &positions {
            if *operator_id <= 0 {
                return Err(StatusCode::BAD_REQUEST);
            }
        }

        if positions.is_empty() {
            return Ok(());
        }

        let move_result = self.repository.update_operator_positions(positions).await;

        match move_result {
            Ok(_) => Ok(()),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

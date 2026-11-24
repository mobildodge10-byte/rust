use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::http::StatusCode;
use rand::{Rng, seq::IndexedRandom};

use crate::{
    core::domain::entity::{
        event_type::Model as EventTypeModel, medicine::Model as MedicineModel,
        region::Model as RegionModel, warehouse::ModelEx as WarehouseModelEx,
    },
    port::inbound::service_trait::ServiceTrait,
};

const TICK_INTERVAL: Duration = Duration::from_secs(80);

// Courier movement: satu langkah tiap MOVE_INTERVAL, sejauh COURIER_SPEED unit peta
const MOVE_INTERVAL: Duration = Duration::from_secs(1);
const COURIER_SPEED: f32 = 10.0;

// Wandering: operator yang available jalan pelan di sekitar anchor (posisi saat mulai available)
const WANDER_SPEED: f32 = 3.0;
const WANDER_RADIUS: f32 = 40.0;

// Batas peta untuk wandering (peta 1000 x 800, margin 50 seperti data seed)
const MAP_MIN_X: f32 = 50.0;
const MAP_MAX_X: f32 = 950.0;
const MAP_MIN_Y: f32 = 50.0;
const MAP_MAX_Y: f32 = 750.0;

// Fixed range of the demand surge per medicine, multiplied by the event's risk_level
const DEMAND_MIN: i32 = 20;
const DEMAND_MAX: i32 = 120;

pub struct SimulationContext {
    pub event_types: Vec<EventTypeModel>,
    pub regions: Vec<RegionModel>,
    pub warehouses: Vec<WarehouseModelEx>,
    pub medicines: Vec<MedicineModel>,
}

// Manufacturer dan warehouse tidak pernah pindah, jadi posisinya di-cache sekali di awal
pub struct MovementContext {
    pub pickup_by_medicine: HashMap<i32, (f32, f32)>,  // medicine_id -> posisi manufacturer
    pub warehouse_positions: HashMap<i32, (f32, f32)>, // warehouse_id -> posisi warehouse
}

// State wandering satu operator (disimpan di memory, bukan di database)
pub struct Wander {
    pub anchor: (f32, f32),
    pub target: (f32, f32),
}

// ===============================================================================
// Event Data
// ===============================================================================

// One simulated incident produced by generate_event (not saved yet)
pub struct GeneratedEvent {
    pub event_type_id: i32,
    pub region_id: i32,
    pub risk_level: i32,
    pub description: String,
}

// Stock one warehouse loses to the demand surge of one event
pub struct DemandItem {
    pub warehouse_id: i32,
    pub medicine_id: i32,
    pub quantity: i32,
}

// ===============================================================================
// Generator Loop
// ===============================================================================

// Spawned from main next to the HTTP server, stopped with abort() on shutdown
// Every tick: one event, restock of the hit warehouses, one production step
pub async fn run_event_generator<S: ServiceTrait + 'static>(service: Arc<S>) {
    let context = load_context(service.as_ref()).await;

    let ctx = match context {
        Ok(ctx) => ctx,
        Err(status_code) => {
            eprintln!("Event generator stopped, failed to load data: {status_code}");
            return;
        }
    };

    let mut interval = tokio::time::interval(TICK_INTERVAL);

    loop {
        interval.tick().await;

        let tick_result = event_tick(service.as_ref(), &ctx).await;

        match tick_result {
            Ok(warehouse_ids) => {
                let restock_result = restock_sweep(service.as_ref(), warehouse_ids).await;

                match restock_result {
                    Ok(_) => {}
                    Err(status_code) => eprintln!("Restock failed: {status_code}"),
                }
            }
            Err(status_code) => eprintln!("Event tick failed: {status_code}"),
        }

        let production_result = service.produce_medicine_batch().await;

        match production_result {
            Ok(_) => {}
            Err(status_code) => eprintln!("Production failed: {status_code}"),
        }
    }
}

async fn load_context<S: ServiceTrait>(service: &S) -> Result<SimulationContext, StatusCode> {
    let event_types = match service.list_event_types().await {
        Ok(event_types) => event_types,
        Err(status_code) => return Err(status_code),
    };

    let regions = match service.list_regions().await {
        Ok(regions) => regions,
        Err(status_code) => return Err(status_code),
    };

    let warehouses = match service.list_warehouses().await {
        Ok(warehouses) => warehouses,
        Err(status_code) => return Err(status_code),
    };

    let medicines = match service.list_medicines().await {
        Ok(medicines) => medicines,
        Err(status_code) => return Err(status_code),
    };

    Ok(SimulationContext { event_types, regions, warehouses, medicines })
}

// ===============================================================================
// Event Tick
// ===============================================================================

// Returns the ids of the warehouses hit by the event (input for restock_sweep)
async fn event_tick<S: ServiceTrait>(
    service: &S,
    ctx: &SimulationContext,
) -> Result<Vec<i32>, StatusCode> {
    let event = match generate_event(ctx) {
        Some(event) => event,
        None => return Ok(Vec::new()),
    };

    let demand = generate_demand(&event, ctx);

    println!("[event] {}", event.description);

    let record_result = service.record_event(event.event_type_id, event.description).await;

    match record_result {
        Ok(_) => {}
        Err(status_code) => return Err(status_code),
    }

    for item in &demand {
        let decrease_result = service.decrease_stock(item.warehouse_id, item.medicine_id, item.quantity).await;

        match decrease_result {
            Ok(_) => {}
            Err(status_code) => return Err(status_code),
        }
    }

    // demand is grouped by warehouse, so dedup leaves one id per warehouse
    let mut warehouse_ids: Vec<i32> = demand.iter().map(|item| item.warehouse_id).collect();
    warehouse_ids.dedup();
    Ok(warehouse_ids)
}

// ===============================================================================
// Restock Sweep
// ===============================================================================

async fn restock_sweep<S: ServiceTrait>(
    service: &S,
    warehouse_ids: Vec<i32>,
) -> Result<(), StatusCode> {
    for warehouse_id in warehouse_ids {
        let restock_result = service.restock_warehouse(warehouse_id).await;

        match restock_result {
            Ok(_) => {}
            Err(status_code) => return Err(status_code),
        }
    }

    Ok(())
}

// ===============================================================================
// Courier Movement
// ===============================================================================

// Spawned from main next to the event generator, stopped with abort() on shutdown
// Every MOVE_INTERVAL: couriers on duty move one step towards their target,
// available operators wander around their anchor
pub async fn run_courier_movement<S: ServiceTrait + 'static>(service: Arc<S>) {
    let context = load_movement_context(service.as_ref()).await;

    let ctx = match context {
        Ok(ctx) => ctx,
        Err(status_code) => {
            eprintln!("Courier movement stopped, failed to load data: {status_code}");
            return;
        }
    };

    let mut wandering: HashMap<i32, Wander> = HashMap::new();
    let mut interval = tokio::time::interval(MOVE_INTERVAL);

    loop {
        interval.tick().await;

        let move_result = movement_tick(service.as_ref(), &ctx, &mut wandering).await;

        match move_result {
            Ok(_) => {}
            Err(status_code) => eprintln!("Courier movement failed: {status_code}"),
        }
    }
}

async fn load_movement_context<S: ServiceTrait>(service: &S) -> Result<MovementContext, StatusCode> {
    let manufacturers = match service.list_manufacturers().await {
        Ok(manufacturers) => manufacturers,
        Err(status_code) => return Err(status_code),
    };

    let warehouses = match service.list_warehouses().await {
        Ok(warehouses) => warehouses,
        Err(status_code) => return Err(status_code),
    };

    // Medicine cuma dibuat oleh 1 manufacturer, jadi pickup bisa dicari dari medicine_id
    let mut pickup_by_medicine = HashMap::new();
    for manufacturer in manufacturers {
        for medicine in manufacturer.medicines {
            pickup_by_medicine.insert(medicine.id, (manufacturer.x_position, manufacturer.y_position));
        }
    }

    let mut warehouse_positions = HashMap::new();
    for warehouse in warehouses {
        warehouse_positions.insert(warehouse.id, (warehouse.x_position, warehouse.y_position));
    }

    Ok(MovementContext { pickup_by_medicine, warehouse_positions })
}

// Per tick: 2 query baca (order item aktif, operator available) + 1 query tulis semua posisi
async fn movement_tick<S: ServiceTrait>(
    service: &S,
    ctx: &MovementContext,
    wandering: &mut HashMap<i32, Wander>,
) -> Result<(), StatusCode> {
    let order_items = match service.list_active_order_items().await {
        Ok(order_items) => order_items,
        Err(status_code) => return Err(status_code),
    };

    let available_operators = match service.list_available_operators().await {
        Ok(operators) => operators,
        Err(status_code) => return Err(status_code),
    };

    let mut positions: Vec<(i32, f32, f32)> = Vec::new();

    // 1. Courier yang bertugas
    // 4 itu Assigned -> jalan ke manufacturer (pickup)
    // 5 itu In Transit -> jalan ke warehouse (destination)
    for order_item in order_items {
        let courier = match order_item.operator.into_option() {
            Some(courier) => courier,
            None => continue,
        };

        let target = if order_item.status == 4 {
            ctx.pickup_by_medicine.get(&order_item.medicine_id)
        } else {
            match order_item.order.into_option() {
                Some(order) => ctx.warehouse_positions.get(&order.warehouse_id),
                None => None,
            }
        };

        let (target_x, target_y) = match target {
            Some(target) => *target,
            None => continue,
        };

        // Sudah sampai, tunggu courier pickup / complete
        if courier.x_position == target_x && courier.y_position == target_y {
            continue;
        }

        let (x_position, y_position) = step_towards(courier.x_position, courier.y_position, target_x, target_y, COURIER_SPEED);
        positions.push((courier.id, x_position, y_position));
    }

    // 2. Operator yang available jalan-jalan random
    // Buang state wandering operator yang sudah tidak available (baru di-dispatch)
    wandering.retain(|operator_id, _| available_operators.iter().any(|operator| operator.id == *operator_id));

    for operator in available_operators {
        let position = (operator.x_position, operator.y_position);

        // Baru available: anchor di posisi sekarang (misalnya warehouse tempat terakhir deliver)
        let wander = wandering.entry(operator.id).or_insert(Wander { anchor: position, target: position });

        // Target sudah dicapai, pilih titik random baru di sekitar anchor
        if position == wander.target {
            wander.target = random_wander_target(wander.anchor);
        }

        let (x_position, y_position) = step_towards(position.0, position.1, wander.target.0, wander.target.1, WANDER_SPEED);
        positions.push((operator.id, x_position, y_position));
    }

    service.move_operators(positions).await
}

// Satu langkah lurus ke target. Kalau sisa jarak <= step, langsung pas di target
// supaya posisi courier sama persis dengan posisi manufacturer / warehouse
fn step_towards(x: f32, y: f32, target_x: f32, target_y: f32, step: f32) -> (f32, f32) {
    let dx = target_x - x;
    let dy = target_y - y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance <= step {
        return (target_x, target_y);
    }

    (x + dx / distance * step, y + dy / distance * step)
}

// ===============================================================================
// Randomized Helpers (sync, so the thread-local rng never lives across an .await)
// ===============================================================================

fn generate_event(ctx: &SimulationContext) -> Option<GeneratedEvent> {
    let event_type = match pick_event_type(&ctx.event_types) {
        Some(event_type) => event_type,
        None => return None,
    };

    let region_id = match pick_region(&ctx.warehouses) {
        Some(region_id) => region_id,
        None => return None,
    };

    let region = match ctx.regions.iter().find(|region| region.id == region_id) {
        Some(region) => region,
        None => return None,
    };

    Some(GeneratedEvent {
        event_type_id: event_type.id,
        region_id,
        risk_level: event_type.risk_level,
        description: build_description(event_type, region),
    })
}

fn generate_demand(event: &GeneratedEvent, ctx: &SimulationContext) -> Vec<DemandItem> {
    let medicine_types = affected_medicine_types(event.event_type_id);
    let mut demand = Vec::new();

    for warehouse in ctx.warehouses.iter().filter(|w| w.region_id == event.region_id) {
        for medicine in ctx.medicines.iter().filter(|m| medicine_types.contains(&m.medicine_type_id)) {
            demand.push(DemandItem {
                warehouse_id: warehouse.id,
                medicine_id: medicine.id,
                quantity: demand_quantity(event.risk_level),
            });
        }
    }

    return demand
}

// Weighted by risk_level: dangerous events happen more often
fn pick_event_type(event_types: &[EventTypeModel]) -> Option<&EventTypeModel> {
    let event_type = event_types.choose_weighted(&mut rand::rng(), |event_type| event_type.risk_level);

    match event_type {
        Ok(event_type) => Some(event_type),
        Err(_) => None,
    }
}

// Picking a random warehouse makes regions with more warehouses more likely
fn pick_region(warehouses: &[WarehouseModelEx]) -> Option<i32> {
    let warehouse = warehouses.choose(&mut rand::rng());

    match warehouse {
        Some(warehouse) => Some(warehouse.region_id),
        None => None,
    }
}

// Event type id -> medicine type ids it creates demand for (ids from create+insert.sql)
fn affected_medicine_types(event_type_id: i32) -> &'static [i32] {
    match event_type_id {
        1 => &[9, 2, 5, 6, 10], // Catastrophe
        2 => &[1, 8],           // Originium Exposure Incident
        3 => &[1, 2],           // Oripathy Case Surge
        4 => &[4, 3, 6],        // Disease Outbreak
        5 => &[9, 2, 5],        // Armed Conflict
        6 => &[2, 5],           // Civil Unrest
        7 => &[10, 6, 4],       // Population Displacement
        8 => &[9, 1, 8],        // Mining Accident
        9 => &[7, 9],           // Industrial Accident
        10 => &[9, 2],          // Transport Accident
        11 => &[7, 8, 9],       // Fire and Explosion
        12 => &[6, 4],          // Food and Water Contamination
        _ => &[],
    }
}

// Titik random di sekitar anchor (maksimal WANDER_RADIUS), tetap di dalam peta
fn random_wander_target(anchor: (f32, f32)) -> (f32, f32) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..std::f32::consts::TAU);
    let distance = rng.random_range(0.0..=WANDER_RADIUS);

    let x = (anchor.0 + angle.cos() * distance).clamp(MAP_MIN_X, MAP_MAX_X);
    let y = (anchor.1 + angle.sin() * distance).clamp(MAP_MIN_Y, MAP_MAX_Y);

    (x, y)
}

fn demand_quantity(risk_level: i32) -> i32 {
    rand::rng().random_range(DEMAND_MIN..=DEMAND_MAX) * risk_level
}

fn build_description(event_type: &EventTypeModel, region: &RegionModel) -> String {
    format!("{} happened in {}", event_type.name, region.name)
}

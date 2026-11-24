use std::{future::Future, io, sync::Arc};

use axum::{Json, Router, extract::State, http::{Method, StatusCode, header}, response::IntoResponse, routing::{get, post}};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::{core::domain::dto::{operator::{CompleteTaskRequest, DispatchOperatorRequest, GetTaskListRequest, PickupMedicineRequest, ViewJobStatsRequest}, warehouse::GetWarehouseInventoryRequest}, port::inbound::service_trait::ServiceTrait};
pub struct Server<S: ServiceTrait> {
    service: Arc<S>
}

pub fn new_server<S: ServiceTrait + Send + Sync + 'static>(service: Arc<S>) -> Server<S> {
    Server { service }
}

impl<S: ServiceTrait + Send + Sync + 'static> Server<S> {

    pub fn router(service: Arc<S>) -> Router {

        let dispatcher_router = Router::new()
            .route("/get-order-item", get(Self::get_order_item))
            .route("/get-operator-list", get(Self::get_operator_list))
            .route("/dispatch-operator", post(Self::dispatch_operator))
            .route("/get-event-log", get(Self::view_event_log))
            .route("/get-activity-log", get(Self::view_activity_log));

        let courier_router = Router::new()
            .route("/get-task-list", get(Self::get_task_list))
            .route("/complete-task", post(Self::complete_task))
            .route("/pickup-medicine", post(Self::pickup_medicine))
            .route("/get-job-stats", get(Self::view_job_stats));

        let warehouse_router = Router::new()
            .route("/get-warehouse-list", get(Self::get_warehouse_list))
            .route("/get-warehouse-inventory", post(Self::get_warehouse_inventory));

        let manufacturer_router = Router::new()
            .route("/get-manufacturer-list", get(Self::get_manufacturer_list));

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE]);

        let app = Router::new()
            .nest("/dispatcher", dispatcher_router)
            .nest("/courier", courier_router)
            .nest("/warehouse", warehouse_router)
            .nest("/manufacturer", manufacturer_router)
            .with_state(service)
            .layer(cors);
        
        return app;
    }

    pub async fn serve(
        self,
        shutdown_signal: impl Future<Output = ()> + Send + 'static,
    ) -> io::Result<()> {
        let app = Self::router(self.service);
        let listener = TcpListener::bind("127.0.0.1:3000").await?;

        println!("Web server listening on http://127.0.0.1:3000");

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal)
            .await
    }

    // ===============================================================================
    // Dispatcher Endpoints
    // ===============================================================================

    async fn get_order_item(
        State(service): State<Arc<S>>
    ) -> impl IntoResponse {

        let order_items = service.view_order_item().await;

        match order_items {
            Ok(order_item) => (StatusCode::OK, Json(order_item)).into_response(),
            Err(status_code) => (status_code, "Error retrieving order item").into_response(),
        }
    }

    async fn get_operator_list(
        State(service): State<Arc<S>>
    ) -> impl IntoResponse {
        let operators = service.view_operator_list().await;

        match operators {
            Ok(operators) => (StatusCode::OK, Json(operators)).into_response(),
            Err(status_code) => (status_code, "Error retrieving operator list").into_response(),
        }
    }

    async fn dispatch_operator(
        State(service): State<Arc<S>>,
        Json(payload): Json<DispatchOperatorRequest>
    ) -> impl IntoResponse {
        let operator_id = payload.operator_id;
        let order_item_id = payload.order_item_id;

        let result = service.dispatch_operator(operator_id, order_item_id).await;

        match result {
            Ok(_) => (StatusCode::OK, "Operator dispatched successfully").into_response(),
            Err(status_code) => (status_code, "Error dispatching operator").into_response(),
        }
    }

    async fn view_event_log(
        State(service): State<Arc<S>>
    ) -> impl IntoResponse {
        let event_logs = service.view_event_log().await;

        match event_logs {
            Ok(event_logs) => (StatusCode::OK, Json(event_logs)).into_response(),
            Err(status_code) => (status_code, "Error retrieving event log").into_response(),
        }
    }

    async fn view_activity_log(
        State(service): State<Arc<S>>
    ) -> impl IntoResponse {
        let activity_logs = service.view_activity_log().await;

        match activity_logs {
            Ok(activity_logs) => (StatusCode::OK, Json(activity_logs)).into_response(),
            Err(status_code) => (status_code, "Error retrieving activity log").into_response(),
        }
    }

    // ===============================================================================
    // Courier Endpoints
    // ===============================================================================

    async fn get_task_list(
        State(service): State<Arc<S>>,
        Json(payload): Json<GetTaskListRequest>,
    ) -> impl IntoResponse {

        let result = service.view_task_list(payload.operator_id).await;

        match result {
            Ok(task_list) => (StatusCode::OK, Json(task_list)).into_response(),
            Err(status_code) => (status_code, "Error retrieving task list").into_response(),
        }
    }

    async fn complete_task(
        State(service): State<Arc<S>>,
        Json(payload): Json<CompleteTaskRequest>,
    ) -> impl IntoResponse {
        let result = service.complete_task(payload.operator_id, payload.order_item_id).await;

        match result {
            Ok(_) => (StatusCode::OK, "Task completed successfully").into_response(),
            Err(status_code) => (status_code, "Error completing task").into_response(),
        }
    }

    async fn pickup_medicine(
        State(service): State<Arc<S>>,
        Json(payload): Json<PickupMedicineRequest>,
    ) -> impl IntoResponse {
        let result = service.pickup_medicine(payload.operator_id, payload.order_item_id).await;

        match result {
            Ok(_) => (StatusCode::OK, "Medicine picked up successfully").into_response(),
            Err(status_code) => (status_code, "Error picking up medicine").into_response(),
        }
    }

    async fn view_job_stats(
        State(service): State<Arc<S>>,
        Json(payload): Json<ViewJobStatsRequest>,
    ) -> impl IntoResponse {
        let result = service.view_job_stats(payload.operator_id).await;

        match result {
            Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
            Err(status_code) => (status_code, "Error retrieving job stats").into_response(),
        }
    }

    // ===============================================================================
    // Warehouse Endpoints
    // ===============================================================================

    async fn get_warehouse_list(
        State(service): State<Arc<S>>
    ) -> impl IntoResponse {
        let warehouses = service.view_warehouse_list().await;

        match warehouses {
            Ok(warehouses) => (StatusCode::OK, Json(warehouses)).into_response(),
            Err(status_code) => (status_code, "Error retrieving warehouse list").into_response(),
        }
    }

    async fn get_warehouse_inventory(
        State(service): State<Arc<S>>,
        Json(payload): Json<GetWarehouseInventoryRequest>,
    ) -> impl IntoResponse {
        let result = service.view_warehouse_inventory(payload.warehouse_id).await;

        match result {
            Ok(inventory) => (StatusCode::OK, Json(inventory)).into_response(),
            Err(status_code) => (status_code, "Error retrieving warehouse inventory").into_response(),
        }
    }

    // ==============================================================================
    // Manufacturer Endpoints
    // ==============================================================================

    async fn get_manufacturer_list(
        State(service): State<Arc<S>>
    ) -> impl IntoResponse {
        let manufacturers = service.view_manufacturer_list().await;

        match manufacturers {
            Ok(manufacturers) => (StatusCode::OK, Json(manufacturers)).into_response(),
            Err(status_code) => (status_code, "Error retrieving manufacturer list").into_response(),
        }
    }
}

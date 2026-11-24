use reqwest::{Client, StatusCode};

use crate::model::{
    log::{ActivityLog, EventLog},
    operator::{
        CompleteTaskRequest,
        DispatchOperatorRequest,
        GetTaskListRequest,
        OperatorDetailsResponse,
        PickupMedicineRequest,
        ViewJobStatsRequest
    },
    order_item::OrderItem,
};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

pub fn new_api_client(base_url: &str) -> ApiClient {
    ApiClient {
        client: Client::new(),
        base_url: base_url.to_string(),
    }
}

// ============================================================
// Bro ga ada bug di sini
// ============================================================

impl ApiClient {

    // ==============================================================
    // Dispatcher
    // ==============================================================

    pub async fn get_order_item_list(&self) -> Result<Vec<OrderItem>, StatusCode> {
        let response = self.client
            .get(format!("{}/dispatcher/get-order-item", self.base_url))
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        let order_items = response.json::<Vec<OrderItem>>().await;

        match order_items {
            Ok(order_items) => Ok(order_items),
            Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        }
    }

    pub async fn get_operator_list(&self) -> Result<Vec<OperatorDetailsResponse>, StatusCode> {
        let response = self.client
            .get(format!("{}/dispatcher/get-operator-list", self.base_url))
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        let operators = response.json::<Vec<OperatorDetailsResponse>>().await;

        match operators {
            Ok(operators) => Ok(operators),
            Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        }
    }

    pub async fn dispatch_operator(&self, operator_id: i32, order_item_id: i32) -> Result<(), StatusCode> {
        let request = DispatchOperatorRequest {
            operator_id: operator_id,
            order_item_id: order_item_id,
        };

        let response = self.client
            .post(format!("{}/dispatcher/dispatch-operator", self.base_url))
            .json(&request)
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        Ok(())
    }

    pub async fn get_event_log(&self) -> Result<Vec<EventLog>, StatusCode> {
        let response = self.client
            .get(format!("{}/dispatcher/get-event-log", self.base_url))
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        let event_logs = response.json::<Vec<EventLog>>().await;

        match event_logs {
            Ok(event_logs) => Ok(event_logs),
            Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        }
    }

    pub async fn get_activity_log(&self) -> Result<Vec<ActivityLog>, StatusCode> {
        let response = self.client
            .get(format!("{}/dispatcher/get-activity-log", self.base_url))
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        let activity_logs = response.json::<Vec<ActivityLog>>().await;

        match activity_logs {
            Ok(activity_logs) => Ok(activity_logs),
            Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        }
    }

    // ==============================================================
    // Courier
    // ==============================================================

    pub async fn get_task_list(&self, operator_id: i32) -> Result<Vec<OrderItem>, StatusCode> {
        let request = GetTaskListRequest {
            operator_id: operator_id,
        };

        let response = self.client
            .get(format!("{}/courier/get-task-list", self.base_url))
            .json(&request)
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        let task_list = response.json::<Vec<OrderItem>>().await;

        match task_list {
            Ok(task_list) => Ok(task_list),
            Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        }
    }

    pub async fn complete_task(&self, operator_id: i32, order_item_id: i32) -> Result<(), StatusCode> {
        let request = CompleteTaskRequest {
            operator_id: operator_id,
            order_item_id: order_item_id,
        };

        let response = self.client
            .post(format!("{}/courier/complete-task", self.base_url))
            .json(&request)
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        Ok(())
    }

    pub async fn pickup_medicine(&self, operator_id: i32, order_item_id: i32) -> Result<(), StatusCode> {
        let request = PickupMedicineRequest {
            operator_id: operator_id,
            order_item_id: order_item_id,
        };

        let response = self.client
            .post(format!("{}/courier/pickup-medicine", self.base_url))
            .json(&request)
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        Ok(())
    }

    pub async fn get_job_stats(&self, operator_id: i32) -> Result<OperatorDetailsResponse, StatusCode> {
        let request = ViewJobStatsRequest {
            operator_id: operator_id,
        };

        let response = self.client
            .get(format!("{}/courier/get-job-stats", self.base_url))
            .json(&request)
            .send()
            .await;
        if response.is_err() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        let stats = response.json::<OperatorDetailsResponse>().await;

        match stats {
            Ok(stats) => Ok(stats),
            Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        }
    }
}

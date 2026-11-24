use tauri::State;

use crate::{
    AppState,
    model::{
        log::{ActivityLog, EventLog},
        operator::OperatorDetailsResponse,
        order_item::OrderItem,
    },
};

// ===============================================================================
// Dispatcher Commands
// ===============================================================================

#[tauri::command(rename_all = "snake_case")]
pub async fn get_order_item_list(
    state: State<'_, AppState>
) -> Result<Vec<OrderItem>, String> {
    let order_items = state.api_client.get_order_item_list().await;

    match order_items {
        Ok(order_items) => Ok(order_items),
        Err(status_code) => Err(format!("Error retrieving order item list ({})", status_code)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_operator_list(
    state: State<'_, AppState>
) -> Result<Vec<OperatorDetailsResponse>, String> {
    let operators = state.api_client.get_operator_list().await;

    match operators {
        Ok(operators) => Ok(operators),
        Err(status_code) => Err(format!("Error retrieving operator list ({})", status_code)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn dispatch_operator(
    state: State<'_, AppState>,
    operator_id: i32,
    order_item_id: i32,
) -> Result<(), String> {
    if operator_id <= 0 || order_item_id <= 0 {
        return Err("Invalid operator or order item".to_string());
    }
    let result = state.api_client.dispatch_operator(operator_id, order_item_id).await;

    match result {
        Ok(_) => {
            state.refresh.notify_one();
            Ok(())
        }
        Err(status_code) => Err(format!("Error dispatching operator ({})", status_code)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_event_log(
    state: State<'_, AppState>
) -> Result<Vec<EventLog>, String> {
    let event_logs = state.api_client.get_event_log().await;

    match event_logs {
        Ok(event_logs) => Ok(event_logs),
        Err(status_code) => Err(format!("Error retrieving event log ({})", status_code)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_activity_log(
    state: State<'_, AppState>
) -> Result<Vec<ActivityLog>, String> {
    let activity_logs = state.api_client.get_activity_log().await;

    match activity_logs {
        Ok(activity_logs) => Ok(activity_logs),
        Err(status_code) => Err(format!("Error retrieving activity log ({})", status_code)),
    }
}

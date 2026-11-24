use tauri::State;

use crate::{
    AppState,
    model::{
        operator::OperatorDetailsResponse,
        order_item::OrderItem,
    },
};

// ===============================================================================
// Courier Commands
// ===============================================================================

#[tauri::command(rename_all = "snake_case")]
pub async fn get_task_list(
    state: State<'_, AppState>,
    operator_id: i32,
) -> Result<Vec<OrderItem>, String> {
    if operator_id <= 0 {
        return Err("Invalid operator".to_string());
    }

    let result = state.api_client.get_task_list(operator_id).await;

    match result {
        Ok(task_list) => Ok(task_list),
        Err(status_code) => Err(format!("Error retrieving task list ({})", status_code)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn pickup_medicine(
    state: State<'_, AppState>,
    operator_id: i32,
    order_item_id: i32,
) -> Result<(), String> {

    let result = state.api_client.pickup_medicine(operator_id, order_item_id).await;
    if operator_id <= 0 || order_item_id <= 0 {
        return Err("Invalid operator or order item".to_string());
    }
    match result {
        Ok(_) => {
            state.refresh.notify_one();
            Ok(())
        }
        Err(status_code) => Err(format!("Error picking up medicine ({})", status_code)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn complete_task(
    state: State<'_, AppState>,
    operator_id: i32,
    order_item_id: i32,
) -> Result<(), String> {
    if operator_id <= 0 || order_item_id <= 0 {
        return Err("Invalid operator or order item".to_string());
    }

    let result = state.api_client.complete_task(operator_id, order_item_id).await;

    match result {
        Ok(_) => {
            // Langsung refresh data ke React
            state.refresh.notify_one();
            Ok(())
        }
        Err(status_code) => Err(format!("Error completing task ({})", status_code)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_job_stats(
    state: State<'_, AppState>,
    operator_id: i32,
) -> Result<OperatorDetailsResponse, String> {
    if operator_id <= 0 {
        return Err("Invalid operator".to_string());
    }

    let result = state.api_client.get_job_stats(operator_id).await;

    match result {
        Ok(stats) => Ok(stats),
        Err(status_code) => Err(format!("Error retrieving job stats ({})", status_code)),
    }
}

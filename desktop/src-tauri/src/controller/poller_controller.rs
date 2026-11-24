use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, State};

use crate::AppState;

const POLL_INTERVAL: Duration = Duration::from_secs(3);

// ===============================================================================
// Ini buat polling data, ga ada bug di sini, jangan diubah pls
// ===============================================================================

pub async fn run_poller(app: AppHandle) {
    let state = app.state::<AppState>();

    loop {
        let operators = state.api_client.get_operator_list().await;
        let order_items = state.api_client.get_order_item_list().await;

        match (operators, order_items) {
            (Ok(operators), Ok(order_items)) => {
                let _ = app.emit("operator-list", operators);
                let _ = app.emit("order-item-list", order_items);
            }
            (Err(status_code), _) | (_, Err(status_code)) => {
                let _ = app.emit("server-error", format!("Error retrieving data ({})", status_code));
            }
        }

        tokio::select! {
            _ = tokio::time::sleep(POLL_INTERVAL) => {}
            _ = state.refresh.notified() => {}
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn refresh_data(state: State<'_, AppState>) {
    state.refresh.notify_one();
}

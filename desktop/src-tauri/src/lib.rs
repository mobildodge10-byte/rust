mod controller;
mod model;

use tokio::sync::Notify;

use crate::{
    controller::{courier_controller, dispatcher_controller, poller_controller},
    model::api_client::{ApiClient, new_api_client},
};

const SERVER_URL: &str = "http://127.0.0.1:3000";

pub struct AppState {
    pub api_client: ApiClient,
    pub refresh: Notify,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let api_client = new_api_client(SERVER_URL);

    let state = AppState {
        api_client: api_client,
        refresh: Notify::new(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .setup(|app| {
            tauri::async_runtime::spawn(poller_controller::run_poller(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Dispatcher
            dispatcher_controller::get_order_item_list,
            dispatcher_controller::get_operator_list,
            dispatcher_controller::dispatch_operator,
            dispatcher_controller::get_event_log,
            dispatcher_controller::get_activity_log,

            // Courier
            courier_controller::get_task_list,
            courier_controller::pickup_medicine,
            courier_controller::complete_task,
            courier_controller::get_job_stats,

            // Poller
            poller_controller::refresh_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod config;
mod listener;
mod tracker;

use crate::config::{get_config, update_config, Config};
use crate::listener::{start_listener, Data};
use crate::tracker::run_tracker;
use std::sync::{Arc, Mutex};
use tauri::async_runtime::spawn;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data = Arc::new(Mutex::new(Data::default()));
    let data_clone = data.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let store = app.store("tracker.json")?;

            let config = store
                .get("config")
                .and_then(|value| serde_json::from_value(value).ok())
                .unwrap_or_else(|| {
                    println!("No valid config found, using default");
                    Config::default()
                });

            store.close_resource();

            app.manage(RwLock::new(config));

            spawn(run_tracker(app_handle));
            start_listener(data_clone);

            Ok(())
        })
        .manage(data)
        .invoke_handler(tauri::generate_handler![get_config, update_config])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { .. } => {
                println!("Exit requested. Sending shutdown signal...");
            }
            _ => {}
        });
}

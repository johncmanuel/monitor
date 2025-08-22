mod config;
mod listener;
mod tracker;

use crate::config::{get_config, update_config, Config};
use crate::listener::{start_listener, Data};
use crate::tracker::run_tracker;
use std::sync::{Arc, Mutex};
use tauri::async_runtime::spawn;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data = Arc::new(Mutex::new(Data::default()));
    let data_clone = data.clone();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            spawn(run_tracker(app_handle));
            start_listener(data_clone);
            Ok(())
        })
        .manage(RwLock::new(Config {
            api_url: "http://localhost:8000/tracker".to_string(),
            interval_secs: 10,
        }))
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

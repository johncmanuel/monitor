mod config;
mod listener;
mod tracker;

use rdev::listen;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::async_runtime::spawn;
use tokio::sync::RwLock;
use crate::config::{Config, get_config, update_config};
use crate::listener::{Data, run_listener};
use crate::tracker::run_tracker;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data = Arc::new(Mutex::new(Data::default()));
    let data_clone = data.clone();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            spawn(run_tracker(app_handle));

            thread::spawn(move || {
                listen(move |event| run_listener(event, &data_clone))
                    .expect("Could not listen to events");
            });
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

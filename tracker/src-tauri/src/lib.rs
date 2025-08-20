use rdev::{listen, Button, Event, EventType};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{mem, thread};
use tauri::async_runtime::spawn;
use tauri::{Manager, State};
use tokio::sync::RwLock;
use ts_rs::TS;

// in the future, i'll include more types of events
#[derive(Serialize, Clone, Debug, Default, TS)]
#[ts(export, export_to = "../../../api/types/data.d.ts")]
struct Data {
    timestamp: u64,
    keypresses: u64,
    left_clicks: u64,
    right_clicks: u64,
    middle_clicks: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/types/config.d.ts")]
struct Config {
    api_url: String,
    interval_secs: u64,
}

#[tauri::command]
async fn get_config(config: State<'_, RwLock<Config>>) -> Result<Config, ()> {
    let config_guard = config.read().await;
    Ok(config_guard.clone())
}

#[tauri::command]
async fn update_config(new_config: Config, config: State<'_, RwLock<Config>>) -> Result<(), ()> {
    let mut config_guard = config.write().await;
    *config_guard = new_config;
    println!("Configuration updated!");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data = Arc::new(Mutex::new(Data::default()));
    let data_clone = data.clone();

    // create new thread for event listening
    thread::spawn(move || {
        listen(move |event| callback(event, &data_clone)).expect("Could not listen to events");
    });

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            spawn(run_tracker(app_handle));
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
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

async fn run_tracker(app_handle: tauri::AppHandle) {
    let client = reqwest::Client::new();
    let config_state: State<RwLock<Config>> = app_handle.state();
    let counter_state: State<Arc<Mutex<Data>>> = app_handle.state();

    loop {
        let interval = {
            let config = config_state.read().await;
            config.interval_secs
        };

        tokio::time::sleep(Duration::from_secs(interval)).await;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut data_snapshot = {
            let mut data_guard = counter_state.lock().unwrap();
            mem::replace(&mut *data_guard, Data::default())
        };
        data_snapshot.timestamp = now;

        let total_events = data_snapshot.keypresses
            + data_snapshot.left_clicks
            + data_snapshot.right_clicks
            + data_snapshot.middle_clicks;

        if total_events > 0 {
            let url = config_state.read().await.api_url.clone();
            println!("Sending {} events to {}", total_events, url);
            send_to_api(&client, data_snapshot, &url).await;
        } else {
            println!("No events in the last {} seconds.", interval);
        }
    }
}

fn callback(event: Event, data_tmp: &Mutex<Data>) {
    let mut data = data_tmp.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(_) => data.keypresses += 1,
        EventType::ButtonPress(button) => match button {
            Button::Left => data.left_clicks += 1,
            Button::Right => data.right_clicks += 1,
            Button::Middle => data.middle_clicks += 1,
            _ => (),
        },
        _ => (),
    }
}

async fn send_to_api(client: &reqwest::Client, data: Data, api_url: &str) {
    match client.post(api_url).json(&data).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Data sent successfully.");
            } else {
                println!("Failed to send data: {}", resp.status());
            }
        }
        Err(e) => println!("Error sending data: {}", e),
    }
}

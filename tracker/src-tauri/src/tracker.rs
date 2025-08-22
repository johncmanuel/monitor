use crate::config::Config;
use crate::listener::Data;
use std::mem;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{Manager, State};
use tokio::sync::RwLock;

pub async fn run_tracker(app_handle: tauri::AppHandle) {
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

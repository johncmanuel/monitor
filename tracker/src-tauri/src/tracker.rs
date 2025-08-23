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
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");
    let mut failed_data_cache: Option<Data> = None;

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
        data_snapshot.ts = now;

        let total_events =
            data_snapshot.kp + data_snapshot.lc + data_snapshot.rc + data_snapshot.mc;

        if let Some(previous_data) = failed_data_cache.take() {
            println!("Merging cached data from a previous failed request.");
            data_snapshot = data_snapshot + previous_data;
        }

        if total_events > 0 {
            let url = config_state.read().await.api_url.clone();
            println!(
                "Sending {} events to {} in the last {} seconds",
                total_events, url, interval
            );

            match send_to_api(&client, &data_snapshot, &url, &api_key).await {
                Ok(_) => {
                    println!("Data sent successfully.");
                }
                Err(e) => {
                    eprintln!("Error sending data: {}", e);
                    failed_data_cache = Some(data_snapshot);
                }
            }
        } else {
            println!("No events in the last {} seconds.", interval);
        }
    }
}

async fn send_to_api(
    client: &reqwest::Client,
    data: &Data,
    api_url: &str,
    api_key: &str,
) -> Result<(), reqwest::Error> {
    client
        .post(api_url)
        .json(data)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

use tauri::State;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/types/config.d.ts")]
pub struct Config {
    pub api_url: String,
    pub interval_secs: u64,
}

#[tauri::command]
pub async fn get_config(config: State<'_, RwLock<Config>>) -> Result<Config, ()> {
    let config_guard = config.read().await;
    Ok(config_guard.clone())
}

#[tauri::command]
pub async fn update_config(new_config: Config, config: State<'_, RwLock<Config>>) -> Result<(), ()> {
    let mut config_guard = config.write().await;
    *config_guard = new_config;
    println!("Configuration updated!");
    Ok(())
}
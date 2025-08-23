use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::RwLock;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/types/config.d.ts")]
pub struct Config {
    pub api_url: String,
    pub interval_secs: u64,
}

impl Config {
    pub fn set_default_config() -> Self {
        if cfg!(debug_assertions) {
            Self {
                api_url: "http://localhost:8000/data".to_string(),
                interval_secs: 10,
            }
        } else {
            Self {
                // honestly it'll be just me using this app, so set this as default
                // but can change this if needed
                api_url: "https://monitor.johncarlomanuel.com/data".to_string(),
                interval_secs: 60,
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::set_default_config()
    }
}

#[tauri::command]
pub async fn get_config(config: State<'_, RwLock<Config>>) -> Result<Config, ()> {
    let config_guard = config.read().await;
    Ok(config_guard.clone())
}

#[tauri::command]
pub async fn update_config(
    new_config: Config,
    config: State<'_, RwLock<Config>>,
) -> Result<(), ()> {
    let mut config_guard = config.write().await;
    *config_guard = new_config;
    println!("Configuration updated!");
    Ok(())
}

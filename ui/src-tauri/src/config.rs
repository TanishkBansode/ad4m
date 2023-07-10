use std::path::PathBuf;

use tauri::api::path::home_dir;

pub fn data_path() -> PathBuf {
    home_dir().expect("Could not get home dir").join(".ad4m")
}

pub fn log_path() -> PathBuf {
    data_path().join("ad4m.log")
}

pub fn binary_path() -> PathBuf {
    data_path().join("binary")
}

pub fn holochain_binary_path() -> PathBuf {
    if cfg!(windows) {
        binary_path().join("holochain.exe")
    } else {
        binary_path().join("holochain")
    }
}

#[cfg(feature = "custom-protocol")]
pub fn app_url() -> String {
    "index.html".to_string()
}

#[cfg(not(feature = "custom-protocol"))]
pub fn app_url() -> String {
    "http://127.0.0.1:3000".to_string()
}

#[cfg(feature = "custom-protocol")]
pub fn app_tray_message_url() -> String {
    "index.html/tray_message".to_string()
}

#[cfg(not(feature = "custom-protocol"))]
pub fn app_tray_message_url() -> String {
    "http://127.0.0.1:3000/tray_message".to_string()
}

pub fn executor_port_path() -> PathBuf {
    data_path().join("executor-port")
}
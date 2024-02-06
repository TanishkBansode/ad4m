extern crate remove_dir_all;
use std::time::{Duration, SystemTime};

use crate::Payload;
use crate::util::create_tray_message_windows;
use crate::{config::data_path, get_main_window};

use remove_dir_all::*;

use tauri::{LogicalSize, Manager, window};
use tauri::Size;
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
pub fn close_application(app_handle: tauri::AppHandle) {
    app_handle.exit(0)
}

#[tauri::command]
pub fn close_main_window(app_handle: tauri::AppHandle) {
    let window = get_main_window(&app_handle);
    if let Ok(true) = window.is_visible() {
        let _ = window.hide();
    }
}

#[tauri::command]
pub fn open_tray(app_handle: tauri::AppHandle) {
    let window = get_main_window(&app_handle);
    if let Ok(true) = window.is_visible() {
        let _ = window.hide();
    } else {
        let _ = window.show();
        let _ = window.move_window(Position::TopRight);
        let _ = window.set_size(Size::Logical(LogicalSize {
            width: 400.0,
            height: 700.0,
        }));
        let _ = window.set_decorations(false);
        let _ = window.set_always_on_top(true);
    }
}

#[tauri::command]
#[cfg(feature = "custom-protocol")]
pub fn open_tray_message(app_handle: tauri::AppHandle) {
    match app_handle.get_window("TrayMessage") {
        Some(tray_window) => {
            if let Ok(true) = tray_window.is_visible() {
                let _ = tray_window.hide();
            } else {
                let _ = tray_window.show();
            }
        },
        None => {
            create_tray_message_windows(&app_handle);
        }
    }
}

#[tauri::command]
#[cfg(not(feature = "custom-protocol"))]
pub fn open_tray_message(app_handle: tauri::AppHandle) {
    println!("In debug mode won't open tray message");
}

#[tauri::command]
pub fn clear_state(app_handle: tauri::AppHandle) {
    let _ = remove_dir_all(data_path());

    app_handle.restart();
}

#[tauri::command]
pub fn open_dapp() {
    if webbrowser::open("http://localhost:8080/").is_err() {
        println!("Failed to open URL");
    }
}
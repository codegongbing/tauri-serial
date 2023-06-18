// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod serial;
mod tauri_command;
use tauri_command::command::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_serial_process,
            set_serial_settings,
            choose_serial,
            close_or_reconnect_serial,
            change_write
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

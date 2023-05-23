// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serial;
use tauri::Window;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
struct SerialSettingsData {
    baud_rate: u32,
    data_bits: u8,
    check_bit: String,
    stop_bits: u8,
}

// 向前端发送串口信息
#[tauri::command]
fn get_serial_process(window: Window) {
    std::thread::spawn(move || loop {
        window.emit("serial-port", serial::get_serial()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
    });
}

// 获取前端串口设置
#[tauri::command]
fn set_serial_settings(data: SerialSettingsData) {
    println!("data: {:?}",&data);

}

#[tauri::command]
fn choose_serial(serial: String) {
    println!("serial: {}", serial);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_serial_process,
            set_serial_settings,
            choose_serial
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

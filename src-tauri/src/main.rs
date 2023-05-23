// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serial;
use serialport::{self, DataBits, FlowControl, Parity, SerialPortBuilder, StopBits};
use std::sync::Mutex;
use tauri::Window;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SERIAL_PORT: Mutex<String> = Mutex::new(String::from("port"));
    static ref BAUD_RATE: Mutex<u32> = Mutex::new(115200);
    static ref DATA_BITS: Mutex<DataBits> = Mutex::new(DataBits::Eight);
    static ref FLOW_CONTROL: Mutex<FlowControl> = Mutex::new(FlowControl::None);
    static ref PARITY: Mutex<Parity> = Mutex::new(Parity::None);
    static ref STOP_BITS: Mutex<StopBits> = Mutex::new(StopBits::One);
    static ref TIMEOUT: Mutex<u64> = Mutex::new(1000);
    static ref SERIAL_PORT_BUILDER: Mutex<SerialPortBuilder> = Mutex::new(
        serialport::new(&*SERIAL_PORT.lock().unwrap(), *BAUD_RATE.lock().unwrap())
            .data_bits(*DATA_BITS.lock().unwrap())
            .flow_control(*FLOW_CONTROL.lock().unwrap())
            .parity(*PARITY.lock().unwrap())
            .stop_bits(*STOP_BITS.lock().unwrap())
            .timeout(std::time::Duration::from_millis(*TIMEOUT.lock().unwrap()))
    );
}

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
    *BAUD_RATE.lock().unwrap() = data.baud_rate;
    *DATA_BITS.lock().unwrap() = match data.data_bits {
        5 => DataBits::Five,
        6 => DataBits::Six,
        7 => DataBits::Seven,
        8 => DataBits::Eight,
        _ => DataBits::Eight,
    };
    *PARITY.lock().unwrap() = match data.check_bit.as_str() {
        "none" => Parity::None,
        "odd" => Parity::Odd,
        "even" => Parity::Even,
        _ => Parity::None,
    };
    *STOP_BITS.lock().unwrap() = match data.stop_bits {
        1 => StopBits::One,
        2 => StopBits::Two,
        _ => StopBits::One,
    };
}

// 选择串口
#[tauri::command]
fn choose_serial(serial: String) {
    *SERIAL_PORT.lock().unwrap() = serial;
    *SERIAL_PORT_BUILDER.lock().unwrap() =
        serialport::new(&*SERIAL_PORT.lock().unwrap(), *BAUD_RATE.lock().unwrap())
            .data_bits(*DATA_BITS.lock().unwrap())
            .flow_control(*FLOW_CONTROL.lock().unwrap())
            .parity(*PARITY.lock().unwrap())
            .stop_bits(*STOP_BITS.lock().unwrap())
            .timeout(std::time::Duration::from_millis(*TIMEOUT.lock().unwrap()));
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

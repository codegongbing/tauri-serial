// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serial;
use serialport::{self, DataBits, FlowControl, Parity, SerialPortBuilder, StopBits};
use std::{sync::Mutex, thread::spawn, time::Duration};
use tauri::Window;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SERIAL_PORT: Mutex<String> = Mutex::new(String::from(""));
    pub static ref BAUD_RATE: Mutex<u32> = Mutex::new(115200);
    pub static ref DATA_BITS: Mutex<DataBits> = Mutex::new(DataBits::Eight);
    pub static ref FLOW_CONTROL: Mutex<FlowControl> = Mutex::new(FlowControl::None);
    pub static ref PARITY: Mutex<Parity> = Mutex::new(Parity::None);
    pub static ref STOP_BITS: Mutex<StopBits> = Mutex::new(StopBits::One);
    // static ref TIMEOUT: Mutex<u64> = Mutex::new(1000);
    pub static ref SERIAL_PORT_BUILDER: Mutex<SerialPortBuilder> = Mutex::new(
        serialport::new(&*SERIAL_PORT.lock().unwrap(), *BAUD_RATE.lock().unwrap())
            .data_bits(*DATA_BITS.lock().unwrap())
            .flow_control(*FLOW_CONTROL.lock().unwrap())
            .parity(*PARITY.lock().unwrap())
            .stop_bits(*STOP_BITS.lock().unwrap())
        );
    pub static ref IS_CLOSE: Mutex<bool> = Mutex::new(false);
    pub static ref IS_WRITE: Mutex<bool> = Mutex::new(false);
    pub static ref WRITE_DATA: Mutex<String> = Mutex::new(String::from(""));
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
struct SerialSettingsData {
    baud_rate: u32,
    data_bits: u8,
    check_bit: String,
    stop_bits: u8,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
struct OutputData {
    data: String,
    is_close: bool,
}

// 向前端发送串口信息
#[tauri::command]
fn get_serial_process(window: Window) {
    std::thread::spawn(move || {
        window.emit("serial-port", serial::get_serial()).unwrap();
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
fn choose_serial(serial: String, window: Window) {
    *SERIAL_PORT.lock().unwrap() = serial;

    let opened_port = SERIAL_PORT_BUILDER.lock().unwrap().clone().open();

    match opened_port {
        Ok(mut port) => {
            spawn(move || loop {
                match &port.data_bits() {
                    Ok(_) => {}
                    Err(_) => {
                        let data = OutputData {
                            data: "".to_string(),
                            is_close: true,
                        };
                        window.emit("output-data", data).unwrap();
                        return;
                    }
                }
                // 判断是否写数据
                if *IS_WRITE.lock().unwrap() == true {
                    // 写入串口
                    // String转&[u8]
                    let write_result = port.write(WRITE_DATA.lock().unwrap().as_bytes());
                    match write_result {
                        Ok(r) => {
                            println!("{}", r);
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                    *IS_WRITE.lock().unwrap() = false;
                    *WRITE_DATA.lock().unwrap() = "".into();
                } else {
                    // 读取串口
                    let mut serial_buf: Vec<u8> = vec![0; 10000];
                    let read_result = port.read(serial_buf.as_mut_slice());
                    match read_result {
                        Ok(t) => {
                            let serial_str = String::from_utf8(serial_buf[..t].to_vec());
                            match serial_str {
                                Ok(s) => {
                                    let data = OutputData {
                                        data: s,
                                        is_close: false,
                                    };
                                    window.emit("output-data", data).unwrap();
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
            });
        }
        Err(_) => {
            return;
        }
    }
}

#[tauri::command]
fn close_serial() {}

#[tauri::command]
fn change_write(data: String) {
    *WRITE_DATA.lock().unwrap() = data;
    println!("{:?}",&*WRITE_DATA.lock().unwrap());
    *IS_WRITE.lock().unwrap() = true;
    println!("{:?}",&*IS_WRITE.lock().unwrap());

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_serial_process,
            set_serial_settings,
            choose_serial,
            close_serial,
            change_write
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

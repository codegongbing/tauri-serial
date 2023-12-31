use super::super::serial::{self, static_value::*, types::*};
use hex;
use serialport::{self, DataBits, Parity, StopBits};
use std::thread::spawn;
use tauri::Window;

// 向前端发送串口信息
#[tauri::command]
pub fn get_serial_process(window: Window) {
    std::thread::spawn(move || {
        window.emit("serial-port", serial::get_serial()).unwrap();
    });
}

// 获取前端串口设置
#[tauri::command]
pub fn set_serial_settings(data: SerialSettingsData) {
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
pub fn choose_serial(serial: String, window: Window) {
    *SERIAL_PORT.lock().unwrap() = serial;
    *IS_SUSPENDED.lock().unwrap() = false;

    let opened_port = SERIAL_PORT_BUILDER.lock().unwrap().clone().open();

    match opened_port {
        Ok(mut port) => {
            spawn(move || loop {
                match &port.data_bits() {
                    Ok(_) => {}
                    Err(_) => {
                        let data = OutputData {
                            data: "".to_string(),
                            is_suspended: true,
                        };
                        window.emit("output-data", data).unwrap();
                        return;
                    }
                }
                // 判断是否写数据
                if *IS_WRITE.lock().unwrap() == true {
                    if *IS_SUSPENDED.lock().unwrap() == false {
                        // 写入串口
                        // let write_result = port.write(WRITE_DATA.lock().unwrap().as_bytes());
                        let write_result = match *IS_HEX.lock().unwrap() {
                            true => {
                                let write_data =
                                    hex::decode(WRITE_DATA.lock().unwrap().clone()).unwrap();
                                port.write(write_data.as_slice())
                            }
                            false => port.write(WRITE_DATA.lock().unwrap().as_bytes()),
                        };
                        match write_result {
                            Ok(_) => {}
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                        *IS_WRITE.lock().unwrap() = false;
                        *WRITE_DATA.lock().unwrap() = "".into();
                    }
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
                                        is_suspended: false,
                                    };
                                    if *IS_SUSPENDED.lock().unwrap() == false {
                                        window.emit("output-data", data).unwrap();
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            });
        }
        Err(_) => {
            return;
        }
    }
}

#[tauri::command]
pub fn close_or_reconnect_serial(state: bool) {
    *IS_SUSPENDED.lock().unwrap() = state;
}

#[tauri::command]
pub fn change_write(data: WriteData) {
    *IS_HEX.lock().unwrap() = data.is_hex;
    *WRITE_DATA.lock().unwrap() = data.data;
    *IS_WRITE.lock().unwrap() = true;
}

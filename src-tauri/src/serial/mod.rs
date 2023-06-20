pub mod init;
pub mod static_value;
pub mod types;

use types::*;

pub fn get_serial() -> Vec<SerialInfo> {
    let old_serial_ports = init::get();
    let mut new_serial_ports: Vec<SerialInfo> = vec![];
    let mut i = 0;
    if cfg!(target_os = "macos") {
        for item in &old_serial_ports {
            if item.port_name.contains("cu.") {
                let mut new_serial_port = item.clone();
                new_serial_port.id = i;
                i += 1;
                new_serial_ports.push(new_serial_port);
            }
        }
    } else if cfg!(target_os = "windows") {
        for item in &old_serial_ports {
            let mut new_serial_port = item.clone();
            new_serial_port.id = i;
            i += 1;
            new_serial_ports.push(new_serial_port);
        }
    } else if cfg!(target_os = "linux") {
    } else {
    }
    return new_serial_ports;
}

use super::types::SerialInfo;
use serialport::SerialPortType;

pub fn get() -> Vec<SerialInfo> {
    let mut serials: Vec<SerialInfo> = vec![];
    let ports_result = serialport::available_ports();
    match ports_result {
        Ok(ports) => {
            match ports.len() {
                0 => {
                    let empty_serial: Vec<SerialInfo> = vec![];
                    return empty_serial;
                }
                _ => (),
            };
            for port in ports {
                let mut serial_info = SerialInfo::new();
                serial_info.port_name = port.port_name;
                match port.port_type {
                    SerialPortType::UsbPort(info) => {
                        serial_info.port_type = "USB".to_string();
                        serial_info.vid = info.vid;
                        serial_info.pid = info.pid;
                        serial_info.product = info.product.unwrap_or("".to_string());
                    }
                    SerialPortType::BluetoothPort => {
                        serial_info.port_type = "Bluetooth".to_string();
                    }
                    SerialPortType::PciPort => {
                        serial_info.port_type = "PCI".to_string();
                    }
                    SerialPortType::Unknown => {
                        serial_info.port_type = "Unknown".to_string();
                    }
                }
                serials.push(serial_info);
            }
            return serials;
        }
        Err(e) => {
            eprintln!("{:?}", e);
            let empty_serial: Vec<SerialInfo> = vec![];
            return empty_serial;
        }
    }
}

pub mod init;

use serde;

#[derive(Clone, serde::Serialize)]
pub struct SerialInfo {
    id: u16,
    port_name: String,
    port_type: String,
    vid: u16,
    pid: u16,
    product: String,
}

// 实现new，返回空的SerialInfo
impl SerialInfo {
    fn new() -> Self {
        Self {
            id: 0,
            port_name: "".to_string(),
            port_type: "".to_string(),
            vid: 0,
            pid: 0,
            product: "".to_string(),
        }
    }
}

// 实现Debug
impl std::fmt::Debug for SerialInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SerialInfo")
            .field("id", &self.id)
            .field("port_name", &self.port_name)
            .field("port_type", &self.port_type)
            .field("vid", &self.vid)
            .field("pid", &self.pid)
            .field("product", &self.product)
            .finish()
    }
}

// 实现Serialize
// impl serde::Serialize for SerialInfo {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         // 通过元组实现序列化
//         let tuple = (
//             &self.id,
//             &self.port_name,
//             &self.port_type,
//             &self.vid,
//             &self.pid,
//             &self.product,
//         );
//         tuple.serialize(serializer)
//     }
// }

// 实现Clone
// impl Clone for SerialInfo {
//     fn clone(&self) -> Self {
//         Self {
//             id: self.id,
//             port_name: self.port_name.clone(),
//             port_type: self.port_type.clone(),
//             vid: self.vid,
//             pid: self.pid,
//             product: self.product.clone(),
//         }
//     }
// }

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
    } else if cfg!(target_os = "linux") {
    } else {
    }
    return new_serial_ports;
}

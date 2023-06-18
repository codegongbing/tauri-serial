use serde;

#[derive(Clone, serde::Serialize, Debug)]
pub struct SerialInfo {
    pub id: u16,
    pub port_name: String,
    pub port_type: String,
    pub vid: u16,
    pub pid: u16,
    pub product: String,
}

// 实现new，返回空的SerialInfo
impl SerialInfo {
    pub fn new() -> Self {
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

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct SerialSettingsData {
    pub baud_rate: u32,
    pub data_bits: u8,
    pub check_bit: String,
    pub stop_bits: u8,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct OutputData {
    pub data: String,
    pub is_suspended: bool,
}

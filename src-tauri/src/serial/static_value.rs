use serialport::{self, DataBits, FlowControl, Parity, SerialPortBuilder, StopBits};
use std::sync::Mutex;
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
    pub static ref IS_SUSPENDED: Mutex<bool> = Mutex::new(false);
    pub static ref IS_WRITE: Mutex<bool> = Mutex::new(false);
    pub static ref IS_HEX: Mutex<bool> = Mutex::new(false);
    pub static ref WRITE_DATA: Mutex<String> = Mutex::new(String::from(""));
}

use serialport::SerialPort;
use std::thread;
use std::time::Duration;

use crate::logger::log_serial;

pub struct SerialportProps {
  pub port_name: String,
  pub baud_rate: u32,
}

pub fn setup_serialport(props: SerialportProps) -> Box<dyn SerialPort> {
  loop {
    match serialport::new(&props.port_name, props.baud_rate)
      .timeout(Duration::from_secs(1))
      .open()
    {
      Ok(port) => {
        log_serial(&format!(
          "✅ Serial port {} opened successfully!",
          props.port_name
        ));
        return port;
      }
      Err(e) => {
        log_serial(&format!(
          "⚠️ Failed to open serial port {}: {}. Retrying in 10 seconds...",
          props.port_name, e
        ));
        thread::sleep(Duration::from_secs(10));
      }
    }
  }
}

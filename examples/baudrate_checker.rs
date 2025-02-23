use std::{io::Read, time::Duration};

fn try_baudrate(port_name: &str, baudrate: u32) -> bool {
  match serialport::new(port_name, baudrate)
    .data_bits(serialport::DataBits::Eight)
    .stop_bits(serialport::StopBits::One)
    .parity(serialport::Parity::None)
    .flow_control(serialport::FlowControl::None)
    .timeout(Duration::from_millis(500))
    .open()
  {
    Ok(mut port) => {
      let mut buf = [0; 128];
      match port.read(&mut buf) {
        Ok(bytes_read) if bytes_read > 0 => {
          println!("Baud rate {} detected on {}", baudrate, port_name);
          true
        }
        _ => false,
      }
    }
    Err(_) => false,
  }
}

fn main() {
  let baudrates = [9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600];
  let ports = serialport::available_ports().expect("Failed to list serial ports");

  for port in ports {
    println!("Checking port: {}", port.port_name);
    for &baud in &baudrates {
      if try_baudrate(&port.port_name, baud) {
        println!("Detected baud rate: {} on port {}", baud, port.port_name);
        break;
      }
    }
  }
}

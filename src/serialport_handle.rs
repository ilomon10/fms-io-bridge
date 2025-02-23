pub struct SerialportProps {
  pub port_name: String,
  pub baud_rate: u32,
  // pub data_bits: serialport::DataBits,
  // pub flow_control: serialport::FlowControl,
  // pub parity: serialport::Parity,
  // pub stop_bits: serialport::StopBits,

  // pub timeout: std::time::Duration,
  // pub write_timeout: std::time::Duration,
  // pub read_timeout: std::time::Duration,
}

pub fn setup_serialport(config: SerialportProps) -> Box<dyn serialport::SerialPort> {
  println!("Opening port: {}", config.port_name);

  let port = serialport::new(config.port_name, config.baud_rate)
    .timeout(std::time::Duration::from_secs(1))
    .open()
    .unwrap_or_else(|err| {
      eprintln!("Failed to open port: {}", err);
      std::process::exit(1);
    });

  return port;
}

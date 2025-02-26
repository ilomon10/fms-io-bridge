use crate::logger::{log_error, log_gps, log_warn};
use crate::serialport_handle::setup_serialport;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
// use std::time::Duration;

pub struct SerialConfig {
  pub port: String,
  pub baud_rate: u32,
}

pub fn start_serial_reader(
  serial_config: SerialConfig,
  nmea_parser: Arc<Mutex<Option<nmea::Nmea>>>,
) {
  thread::spawn(move || loop {
    let port = setup_serialport(crate::serialport_handle::SerialportProps {
      port_name: serial_config.port.to_string(),
      baud_rate: serial_config.baud_rate,
    });

    let mut reader = io::BufReader::new(port);
    let mut buffer = String::new();

    loop {
      buffer.clear();
      match reader.read_line(&mut buffer) {
        Ok(0) => {
          log_warn("[SERIAL]", "⚠️ Serial port disconnected. Reconnecting...");
          break;
        }
        Ok(_) => {
          let mut new_parser = nmea::Nmea::create_for_navigation(&[
            nmea::SentenceType::GGA,
            nmea::SentenceType::GLL,
            nmea::SentenceType::GSV,
          ])
          .unwrap();

          if let Err(e) = new_parser.parse(&buffer) {
            log_gps(&format!("Failed to parse NMEA sentence: {}", e));
            continue;
          }

          if let Some(fix) = new_parser.fix_type() {
            if fix != nmea::sentences::FixType::Invalid {
              let latitude = new_parser.latitude.unwrap_or(0.0);
              let longitude = new_parser.longitude.unwrap_or(0.0);
              let speed = new_parser.speed_over_ground.unwrap_or(0.0);
              let altitude = new_parser.altitude.unwrap_or(0.0);
              let timestamp = new_parser.fix_timestamp();
              let date = new_parser.fix_date;

              log_gps(
                  &format!("Fix: {:?}, Lat: {}, Lon: {}, Speed: {} knots, Altitude: {} m, Date: {:?}, Time: {:?}",
                  fix, latitude, longitude, speed, altitude, date, timestamp)
                );

              let mut shared_parser = nmea_parser.lock().unwrap();
              *shared_parser = Some(new_parser);
            }
          }
        }
        Err(e) => match e.kind() {
          io::ErrorKind::TimedOut => {
            log_warn("SERIAL", " Serial port read timeout");
            continue;
          }
          _ => {
            log_error(
              "SERIAL",
              &format!("Failed to read from serial port: {:?}", e),
            );
            break;
          }
        },
      }
    }

    thread::sleep(std::time::Duration::from_millis(5000));
  });
}

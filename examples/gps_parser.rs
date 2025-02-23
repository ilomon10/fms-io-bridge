use std::io::{self, BufRead};
use std::time::Duration;

fn main() {
  let port_name = "COM6"; // Change this based on your system (e.g., COM3 on Windows)
  let baud_rate = 9600;

  let port = serialport::new(port_name, baud_rate)
    .timeout(Duration::from_secs(1))
    .open()
    .expect("Failed to open serial port");

  let mut reader = io::BufReader::new(port);
  let mut buffer = String::new();
  // let mut nmea_sentence = nmea::Nmea::default();
  let mut nmea_parser = nmea::Nmea::create_for_navigation(&[
    nmea::SentenceType::GGA,
    nmea::SentenceType::GLL,
    nmea::SentenceType::GSV,
  ])
  .unwrap();

  println!("Reading GPS data...");
  loop {
    buffer.clear();
    if reader.read_line(&mut buffer).is_ok() {
      if let Err(e) = nmea_parser.parse(&buffer) {
        eprintln!("Failed to parse NMEA sentence: {}", e);
        continue;
      }

      if let Some(fix) = nmea_parser.fix_type() {
        // if fix != nmea::sentences::FixType::Invalid {
        let latitude = nmea_parser.latitude.unwrap_or(0.0);
        let longitude = nmea_parser.longitude.unwrap_or(0.0);
        let speed = nmea_parser.speed_over_ground.unwrap_or(0.0);
        let altitude = nmea_parser.altitude.unwrap_or(0.0);
        let timestamp = nmea_parser.fix_timestamp();
        let date = nmea_parser.fix_satellites().unwrap();

        println!(
          "Fix: {:?}, Lat: {}, Lon: {}, Speed: {} knots, Altitude: {} m, Date: {:?}, Time: {:?}",
          fix, latitude, longitude, speed, altitude, date, timestamp
        );
        // } else {
        //     println!("No valid GPS fix.");
        // }
      }
    }
  }
}

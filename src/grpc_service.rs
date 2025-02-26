use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Result, Status};

use crate::proto;

pub struct GpsService {
  pub nmea_parser: Arc<Mutex<Option<nmea::Nmea>>>,
}

#[tonic::async_trait]
impl proto::gps_server::Gps for GpsService {
  async fn get_lat_lng(
    &self,
    request: Request<proto::LatLngRequest>,
  ) -> Result<Response<proto::LatLngResponse>, Status> {
    println!("Received gRPC request: {:?}", request);

    let parser = self.nmea_parser.lock().unwrap();
    if let Some(ref nmea) = *parser {
      let response = proto::LatLngResponse {
        lat: nmea.latitude().unwrap_or(0.0),
        lng: nmea.longitude().unwrap_or(0.0),
      };
      Ok(Response::new(response))
    } else {
      Err(Status::not_found("No GPS data available"))
    }
  }

  async fn get_nmea(
    &self,
    request: Request<proto::NmeaRequest>,
  ) -> Result<Response<proto::NmeaResponse>, Status> {
    println!("Received gRPC request: {:?}", request);

    let parser = self.nmea_parser.lock().unwrap();
    if let Some(ref nmea) = *parser {
      let response = proto::NmeaResponse {
        latitude: nmea.latitude().unwrap_or(0.0),
        longitude: nmea.longitude().unwrap_or(0.0),
        altitude: nmea.altitude().unwrap_or(0.0),
        speed: nmea.speed_over_ground.unwrap_or(0.0),
        hdop: nmea.hdop().unwrap_or(0.0),
      };
      Ok(Response::new(response))
    } else {
      Err(Status::not_found("No GPS data available"))
    }
  }

  async fn get_fix_nmea(
    &self,
    request: Request<proto::NmeaRequest>,
  ) -> Result<Response<proto::FixNmeaResponse>, Status> {
    println!("Received gRPC request: {:?}", request);

    let parser = self.nmea_parser.lock().unwrap();
    if let Some(ref nmea) = *parser {
      let response = proto::FixNmeaResponse {
        latitude: nmea.latitude().unwrap_or(0.0),
        longitude: nmea.longitude().unwrap_or(0.0),
        altitude: nmea.altitude().unwrap_or(0.0),
        speed: nmea.speed_over_ground.unwrap_or(0.0),
        hdop: nmea.hdop().unwrap_or(0.0),
        fix_date: nmea
          .fix_date
          .map(|d| d.format("%Y-%m-%d").to_string())
          .unwrap_or_default(),
        fix_time: nmea
          .fix_time
          .map(|t| t.format("%H:%M:%S").to_string())
          .unwrap_or_default(),
        fix_satellites: nmea.fix_satellites().expect("REASON"),
      };
      Ok(Response::new(response))
    } else {
      Err(Status::not_found("No GPS data available"))
    }
  }
}

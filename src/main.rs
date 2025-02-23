use std::io::{self, BufRead};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serialport_handle::setup_serialport;
use tonic::{transport::Server, Request, Response, Result, Status};

mod proto {
    tonic::include_proto!("fms.gps.v1");

    pub(crate) const FILE_DESCRIPTION_SET: &[u8] =
        tonic::include_file_descriptor_set!("fms_descriptor");
}

mod serialport_handle;

// Shared NMEA state
type SharedNmea = Arc<Mutex<Option<nmea::Nmea>>>;

#[derive(Debug)]
struct GpsService {
    nmea_parser: SharedNmea,
}

#[tonic::async_trait]
impl proto::gps_server::Gps for GpsService {
    async fn get_lat_lng(
        &self,
        request: Request<proto::LatLngRequest>,
    ) -> Result<Response<proto::LatLngResponse>, Status> {
        println!("Received gRPC request: {:?}", request);

        // Read the latest parsed NMEA data
        let parser = self.nmea_parser.lock().unwrap();
        if let Some(ref nmea) = *parser {
            let response = proto::LatLngResponse {
                lat: nmea.latitude.unwrap_or(0.0),
                lng: nmea.longitude.unwrap_or(0.0),
            };
            Ok(Response::new(response))
        } else {
            Err(Status::not_found("No GPS data available"))
        }
    }
}

async fn wait() {
    println!("Server listening...");
    tokio::signal::ctrl_c().await.ok();
    println!("Shutdown complete");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nmea_parser = Arc::new(Mutex::new(None)); // No initial parser state

    // Clone shared state for the thread
    let parser_clone = Arc::clone(&nmea_parser);
    thread::spawn(move || {
        let port = setup_serialport(serialport_handle::SerialportProps {
            port_name: "COM6".to_string(),
            baud_rate: 9600,
        });

        let mut reader = io::BufReader::new(port);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            if reader.read_line(&mut buffer).is_ok() {
                let mut new_parser = nmea::Nmea::create_for_navigation(&[
                    nmea::SentenceType::GGA,
                    nmea::SentenceType::GLL,
                    nmea::SentenceType::GSV,
                ])
                .unwrap();

                if let Err(e) = new_parser.parse(&buffer) {
                    eprintln!("Failed to parse NMEA sentence: {}", e);
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

                        println!(
                            "Fix: {:?}, Lat: {}, Lon: {}, Speed: {} knots, Altitude: {} m, Date: {:?}, Time: {:?}",
                            fix, latitude, longitude, speed, altitude, date, timestamp
                        );

                        // Replace the old parser with the new one
                        let mut shared_parser = parser_clone.lock().unwrap();
                        *shared_parser = Some(new_parser);
                    }
                }
            }

            thread::sleep(Duration::from_millis(100)); // Avoid CPU overload
        }
    });

    let protobuf_addr: SocketAddr = "[::1]:50001".parse()?;

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTION_SET)
        .build_v1alpha()
        .unwrap();

    let gps_service = GpsService {
        nmea_parser: Arc::clone(&nmea_parser),
    };

    println!("Starting gRPC server on port 50001...");

    let grpc_server = Server::builder()
        .add_service(service)
        .add_service(proto::gps_server::GpsServer::new(gps_service))
        .serve_with_shutdown(protobuf_addr, wait());

    let grpc_handle = tokio::spawn(grpc_server);

    let _ = tokio::try_join!(grpc_handle);

    Ok(())
}

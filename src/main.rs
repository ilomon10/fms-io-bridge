mod args_match;
mod grpc_service;
mod logger;
mod serial_reader;
mod serialport_handle;

use grpc_service::GpsService;
use logger::{init_logger, log_server};
use serial_reader::start_serial_reader;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;

mod proto {
  tonic::include_proto!("fms.gps.v1");
  pub(crate) const FILE_DESCRIPTION_SET: &[u8] =
    tonic::include_file_descriptor_set!("fms_descriptor");
}

async fn wait() {
  println!("Server listening...");
  tokio::signal::ctrl_c().await.ok();
  println!("Shutdown complete");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let matches = args_match::setup_args();

  init_logger();

  let nmea_parser = Arc::new(Mutex::new(None));

  // Start serial reader in a separate thread
  start_serial_reader(
    serial_reader::SerialConfig {
      port: matches
        .get_one::<String>("port")
        .expect("Expected required 'port' cli argument")
        .to_string(),
      baud_rate: matches.get_one::<u32>("baud_rate").cloned().unwrap_or(9600),
    },
    Arc::clone(&nmea_parser),
  );

  log_server("✅ GPS Service is starting...");

  let protobuf_addr: SocketAddr = format!(
    "[::1]:{}",
    matches
      .get_one::<String>("grpc_port")
      .cloned()
      .unwrap_or("50001".to_string())
  )
  .parse()?;

  let service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTION_SET)
    .build_v1alpha()
    .unwrap();

  let grpc_service = GpsService {
    nmea_parser: Arc::clone(&nmea_parser),
  };

  log_server(&format!("Starting gRPC server on {}...", protobuf_addr));

  let grpc_server = Server::builder()
    .add_service(service)
    .add_service(proto::gps_server::GpsServer::new(grpc_service))
    .serve_with_shutdown(protobuf_addr, wait());

  let grpc_handle = tokio::spawn(grpc_server);

  let _ = tokio::try_join!(grpc_handle);

  Ok(())
}

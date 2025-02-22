use std::net::SocketAddr;

use tonic::{transport::Server, Request, Response, Result, Status};

mod proto {
  tonic::include_proto!("fms.gps.v1");

  pub(crate) const FILE_DESCRIPTION_SET: &[u8] =
    tonic::include_file_descriptor_set!("fms_descriptor");
}

#[derive(Debug, Default)]
struct GpsService {}

#[tonic::async_trait]
impl proto::gps_server::Gps for GpsService {
  async fn get_lat_lng(
    &self,
    request: Request<proto::LatLngRequest>,
  ) -> Result<Response<proto::LatLngResponse>, Status> {
    println!("Got a request {:?}", request);

    // let input = request.get_ref();

    let response = proto::LatLngResponse { 
      lat: 127.22234234, 
      lng: 0.238191 
    };

    Ok(Response::new(response))
  }
}

async fn wait() {
  println!("server listening");
  tokio::signal::ctrl_c().await.ok();
  println!("shutdown complete");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let protobuf_addr: SocketAddr = "[::1]:50001".parse()?;

  let service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTION_SET)
    .build_v1alpha()
    .unwrap();

  let loc: GpsService = GpsService::default();

  println!("50001");

  let grpc_server = Server::builder()
    .add_service(service)
    .add_service(proto::gps_server::GpsServer::new(loc))
    .serve_with_shutdown(protobuf_addr, wait());

  let grpc_handle = tokio::spawn(grpc_server);

  let _ = tokio::try_join!(grpc_handle);

  Ok(())
}

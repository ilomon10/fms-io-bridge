use clap::{value_parser, Arg, ArgMatches, Command};

pub fn setup_args() -> ArgMatches {
  Command::new("fms-gps")
    .version("0.1.0")
    .author("ilomon10")
    .arg_required_else_help(true)
    .arg(
      Arg::new("port")
        .value_name("port")
        .short('p')
        .long("port")
        .required(true)
        .help("Serial port to open"),
    )
    .arg(
      Arg::new("baud_rate")
        .value_name("baud_rate")
        .short('b')
        .long("baud")
        .required(false)
        .default_value("9600")
        .value_parser(value_parser!(u32))
        .help("Baud rate to the port to open"),
    )
    .arg(
      Arg::new("grpc_port")
        .value_name("grpc_port")
        .long("grpc_port")
        .required(false)
        .default_value("50001")
        .help("gRPC server port"),
    )
    .get_matches()
}

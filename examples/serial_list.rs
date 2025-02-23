fn main() {
  println!("Listing");
  let ports = serialport::available_ports().expect("no ports found!");

  for p in &ports {
    println!("Port Type\t: {:?}", p.port_type);
    println!("Port Path\t {:?}", p.port_name);
  }

  println!("done {:?}", ports.len());
}

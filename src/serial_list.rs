fn main() {
    println!("Listing");
    let ports = serialport::available_ports().expect("no ports found!");
    
    for p in &ports {
        println!("Port 1 {:?}", p.port_type);
    } 

    println!("done {:?}", ports.len());
}
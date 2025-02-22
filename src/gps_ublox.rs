
fn main() {
    // let port = <String>"COM5";
    // let builder = serialport::new()
    // let port = 
    // let mut device = Device::new(port); 
}

struct Device {
    port: Box<dyn serialport::SerialPort>,
    parser: ublox::Parser<Vec<u8>>,
}

impl Device {
    pub fn new(port: Box<dyn serialport::SerialPort>) -> Device {
        let parser = ublox::Parser::default();
        Device {port, parser}
    }

    fn read_port(&mut self, output: &mut [u8])-> std::io::Result<usize> {
        match self.port.read(output){
            Ok(b)=> Ok(b),
            Err(e)=>{
                if e.kind() == std::io::ErrorKind::TimedOut {
                    Ok(0)
                } else {
                    Err(e)
                }
            }
        }
    }
}

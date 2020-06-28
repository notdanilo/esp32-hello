use std::net::UdpSocket;

pub enum BeaconError {}

pub struct Beacon {
    socket : UdpSocket
}

impl Beacon {
    pub fn new(port : u16) -> Result<Self,std::io::Error> {
        let address = format!("0.0.0.0:{}", port);
        let socket  = UdpSocket::bind(address)?;
        socket.set_broadcast(true)?;
        Ok(Self {socket})
    }

    pub fn run(&self) {
        let mut message : [u8;8] = [0;8];
        let (_size, address) = self.socket.recv_from(&mut message).expect("Couldn't peek");
        println!("Received: {:?}", message);
        self.socket.send_to(&message, address).expect("Couldn't send to");
    }
}

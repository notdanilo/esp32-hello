#![feature(never_type)]
#![no_main]

extern crate alloc;

use std::thread::sleep;
use std::time::Duration;

use esp32_hal::EspError;
use crate::wifi::WifiClient;
use futures::executor::block_on;

mod blink;
mod wifi;

#[no_mangle]
pub fn app_main() {
  if let Err(err) = main() {
    println!("{}", err);
  }
}

fn main() -> Result<!, EspError> {
  block_on(async_main())?;
  loop {}
}

async fn async_main() -> Result<!, EspError> {
  let ssid     = "AP Amarelo".to_string();
  let password = "demogroni".to_string();
  let wifi = WifiClient::new(&ssid, &password).await.expect("Couldn't connect");
  println!("Connected to {} with {}", ssid, wifi.address());

  sleep(Duration::from_secs(5));

  let socket = std::net::UdpSocket::bind("0.0.0.0:34254").expect("Couldn't bind socket");
  socket.set_broadcast(true).expect("Couldn't enable broadcast receiving");

  loop {
    match socket.send_to("Hello".as_bytes(), "192.168.15.60:34254") {
      Ok(_)  => {
        let mut message : [u8;8] = [0; 8];
        match socket.recv_from(&mut message) {
          Ok((size,address)) => println!("Received {} bytes from {}", size, address),
          Err(e)             => println!("Error: {}", e)
        }
      },
      Err(e) => println!("{:#?}", e)
    }
    sleep(Duration::from_secs(1));
  }
}
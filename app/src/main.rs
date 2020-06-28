#![feature(never_type)]
#![no_main]

extern crate alloc;

mod blink;
// mod wifi;
// mod beacon;
// mod gpio;

use std::thread::sleep;
use std::time::Duration;

// use esp32_hal::EspError;
use futures::executor::block_on;

// use wifi::WifiClient;
// use beacon::Beacon;

#[no_mangle]
pub fn app_main() {
  if let Err(err) = main() {
    println!("{}", err);
  }
}

type EspError = u32;

fn main() -> Result<!, EspError> {
  block_on(async_main())?;
  loop {}
}

async fn async_main() -> Result<!, EspError> {
  // let ssid     = "AP Amarelo".to_string();
  // let password = "demogroni".to_string();
  // let wifi = WifiClient::new(&ssid, &password).await.expect("Couldn't connect");
  // println!("Connected to {} with {}", ssid, wifi.address());
  //
  // let beacon = Beacon::new(34254).expect("Couldn't create beacon");

  // std::thread::spawn(move || {
  //   gpio::start();
  // });
  println!("Starting blinker");
//  use std::fmt::Write;
  blink::start_blinker();

  loop {
    // beacon.run();
    sleep(Duration::from_millis(1000))
  }
}

#![feature(never_type)]
#![no_main]

extern crate alloc;

use std::thread::sleep;
use std::time::Duration;

use esp32_hal::EspError;

mod blink;
mod wifi;

#[no_mangle]
pub fn app_main() {
  if let Err(err) = rust_blink_and_write() {
    println!("{}", err);
  }
}

fn rust_blink_and_write() -> Result<!, EspError> {
//  blink::start_blinker();
  wifi::start_wifi()?;

  loop {
    sleep(Duration::from_secs(5))
  }
}

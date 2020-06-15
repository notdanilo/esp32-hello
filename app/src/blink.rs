use std::thread::{self, sleep};
use std::time::Duration;
use esp32_hal::gpio::*;
use embedded_hal::digital::v2::OutputPin;

pub fn start_blinker() {
    let mut gpio = GPIO22::into_input_output();

    thread::Builder::new()
        .name("blink_thread".into())
        .spawn(move || {
            loop {
                gpio.set_low().unwrap();
                sleep(Duration::from_secs(1));
                gpio.set_high().unwrap();
                sleep(Duration::from_secs(1));
            }
        })
        .unwrap();
}

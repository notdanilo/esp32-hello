use std::thread::{self, sleep};
use std::time::Duration;
// use esp32_hal::gpio::*;
// use embedded_hal::digital::v2::OutputPin;

pub struct GPIO {
    pin : u32
}

pub type EspErr = u32;

impl GPIO {
    pub fn new(pin:u32) -> Result<Self,EspErr> {
        let error = unsafe {
            let intr_type    = esp_idf_sys::GPIO_INT_TYPE_GPIO_PIN_INTR_DISABLE;
            let mode         = esp_idf_sys::GPIO_MODE_DEF_OUTPUT;
            let pin_bit_mask = 1 << pin as u64;
            let pull_down_en = 0;
            let pull_up_en   = 0;

            let config = esp_idf_sys::gpio_config_t{intr_type,mode,pin_bit_mask,pull_down_en,pull_up_en};
            esp_idf_sys::gpio_config(&config)
        };
        Ok(Self {pin})
    }

    pub fn set_high(&mut self) {
        unsafe {
            esp_idf_sys::gpio_set_level(self.pin,1);
        }
    }

    pub fn set_low(&mut self) {
        unsafe {
            esp_idf_sys::gpio_set_level(self.pin,0);
        }
    }
}

pub fn start_blinker(millis : u64) {
    let mut gpio = GPIO::new(22).expect("Couldn't create GPIO");

    thread::Builder::new()
        .name("blink_thread".into())
        .spawn(move || {
            loop {
                gpio.set_high();
                sleep(Duration::from_millis(millis));
                gpio.set_low();
                sleep(Duration::from_millis(millis));
            }
        })
        .unwrap();
}

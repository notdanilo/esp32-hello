use super::*;

use alloc::string::String;

use std::thread;
use std::sync::{Mutex, RwLock};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::TcpListener;

use esp32_hal::{*, nvs::*, wifi::*};

use futures::executor::block_on;
mod wifi_manager;
use wifi_manager::*;
mod dns;

pub struct WifiClient {
    station : StaRunning
}

impl WifiClient {
    pub async fn new(ssid : &str, password : &str) -> Result<Self,WifiError> {
        let mut nvs = NonVolatileStorage::default()?;
        let wifi = Wifi::init(&mut nvs);

        let sta_config = StaConfig::builder()
            .ssid(&ssid)
            .password(&password)
            .build();

        let station = wifi.into_sta(&sta_config);
        let station = station.connect().await?;

        Ok(Self{station})
    }

    pub fn address(&self) -> Ipv4Addr {
        let StaRunning(ip) = self.station;
        Ipv4Addr::from(ip)
    }
}

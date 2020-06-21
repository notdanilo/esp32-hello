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

pub fn start_wifi() -> Result<(), EspError> {
    let mut nvs = NonVolatileStorage::default()?;

    let wifi = Wifi::init(&mut nvs);

    println!("AP started.");

    let namespace = nvs.open("wifi")?;
    println!("namespace: {:?}", namespace);

    thread::Builder::new()
        .name("dns_thread".into())
        .stack_size(8192)
        .spawn(dns::server)
        .unwrap();

    thread::Builder::new()
        .name("server_thread".into())
        .stack_size(8192)
        .spawn(move || block_on(async {
            let mac = mac_address(MacAddressType::Ap);
            let ap_ssid = format!("ESP {}", mac);

            let ap_config = ApConfig::builder()
                .ssid(&ap_ssid)
                .build();

            let mut wifi_storage = namespace;

            let ssid     = Some(String::from("YOUR SSID"));
            let password = Some(String::from("YOUR PASSWORD"));

            let mut ap_running = None;
            let mut sta_running = None;

            if let (Some(ssid), Some(password)) = (ssid, password) {
                let sta_config = StaConfig::builder()
                    .ssid(&ssid)
                    .password(&password)
                    .build();

                let sta = wifi.into_sta(&sta_config);

                println!("Attempting to connect.");

                match sta.connect().await {
                    Ok(sta) => {
                        let StaRunning(ip) = sta;
                        println!("Connected to '{}' with IP '{}'.", ssid, Ipv4Addr::from(ip));
                        sta_running = Some(sta);
                    },
                    Err(err) => {
                        println!("Failed to connect.");
                        let ap = err.wifi().into_ap(&ap_config);
                        ap_running = Some(ap.start());
                    }
                }
            } else {
                println!("Starting Access Point '{}' …", ap_ssid);
                let ap = wifi.into_ap(&ap_config);
                ap_running = Some(ap.start());
            }

            let stream = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 80)).expect("failed starting TCP listener");

            loop {
                match stream.accept() {
                    Ok((client, addr)) => {
                        match handle_request(client, addr, &ap_config, &mut wifi_storage, ap_running.take(), sta_running.take()).await {
                            Ok((ap, sta)) => {
                                ap_running = ap;
                                sta_running = sta;
                            },
                            Err(err) => {
                                eprintln!("Failed to handle request: {:?}", err);
                            },
                        }
                    },
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::WouldBlock {
                            eprintln!("couldn't get client: {:?}", e);
                        }
                    },
                }

                thread::yield_now();
            }
        }))
        .unwrap();
    Ok(())
}

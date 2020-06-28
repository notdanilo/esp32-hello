
use esp32_hal::gpio::*;
use embedded_hal::digital::v2::OutputPin;
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;

pub fn start() -> Result<(),std::io::Error> {
    let mut listener = TcpListener::bind("0.0.0.0:12312").expect("Couldn't create listener.");

    let mut gpio = GPIO22::into_input_output();

    println!("GPIO server started.");
    loop {
        if let Ok((mut client,_addr)) = listener.accept() {
            println!("New connection.");
            let mut buf: [u8; 1024] = [0; 1024];
            let len = client.read(&mut buf).unwrap();

            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut req = httparse::Request::new(&mut headers);

            let status = req.parse(&buf);

            println!("Status: {:?}", status);
            println!("Request: {:?}", req);

            if let Ok(httparse::Status::Complete(res)) = status {
                println!("Path: {:#?}", req.path);
                match req.path {
                    Some("/on") => {
                        gpio.set_high().unwrap();
                        writeln!(client, "HTTP/1.1 200 OK")?;
                        writeln!(client, "Content-Type: text/plain")?;
                        writeln!(client)?;
                        writeln!(client, "on")?;
                    },
                    Some("/off") => {
                        gpio.set_low().unwrap();
                        writeln!(client, "HTTP/1.1 200 OK")?;
                        writeln!(client, "Content-Type: text/plain")?;
                        writeln!(client)?;
                        writeln!(client, "off")?;
                    },
                    _ => {
                        writeln!(client, "HTTP/1.1 404 Not Found")?;
                        writeln!(client)?;
                    },
                }
            } else {
                writeln!(client, "HTTP/1.1 500 INTERNAL SERVER ERROR")?;
                writeln!(client)?;
            }
        }
    }
}

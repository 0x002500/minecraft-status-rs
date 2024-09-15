use std::net::TcpStream;
use std::time::Duration;

pub fn status(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);

    match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(5)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

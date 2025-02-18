use std::net::IpAddr;

use shared::constants::API_PORT;

pub fn run(address: IpAddr, effect: &str) {
    let socket_address: std::net::SocketAddr = (address, API_PORT).into();
    reqwest::blocking::Client::new()
        .post(&format!("http://{}/api/temporary_effect/{}", socket_address.to_string(), effect))
        .send()
        .expect("Failed to send request to server");
}
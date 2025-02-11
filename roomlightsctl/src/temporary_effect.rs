use std::net::IpAddr;

pub fn run(address: IpAddr, effect: &str) {
    let socket_address: std::net::SocketAddr = (address, shared::constants::API_PORT).into();
    reqwest::blocking::Client::new()
        .post(&format!("http://{}/api/temporary-effect/{}", socket_address.to_string(), effect))
        .send()
        .expect("Failed to send request to server");
}
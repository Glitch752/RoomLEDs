pub fn run(address: &str, effect: &str) {
    reqwest::blocking::Client::new()
        .post(&format!("http://{}/api/temporary-effect/{}", address, effect))
        .send()
        .expect("Failed to send request to server");
}
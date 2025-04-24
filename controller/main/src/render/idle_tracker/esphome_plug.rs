use super::power_device::{PowerDevice, PowerStats};

pub struct ESPHomePlug {
    pub ip: String,
    pub switch_id: String,
    pub power_sensor_id: String
}

impl ESPHomePlug {
    pub fn new(ip: String, switch_id: String, power_sensor_id: String) -> ESPHomePlug {
        ESPHomePlug {
            ip,
            switch_id,
            power_sensor_id
        }
    }
}

impl PowerDevice for ESPHomePlug {
    fn get_stats(&self) -> Option<PowerStats> {
        // TODO: Use async here
        
        let power_usage_result =
            reqwest::blocking::get(&format!("http://{}/sensor/{}", self.ip, self.power_sensor_id))
            .ok()?
            .json::<serde_json::Value>()
            .ok()?;

        Some(PowerStats {
            current_power_usage: power_usage_result["value"].as_f64().unwrap_or(0.0) as f32
        })
    }

    fn set_power(&mut self, power: bool) {
        // TODO: Use async here

        println!("Setting power to: {}", power);

        let url = format!("http://{}/switch/{}/turn_{}", self.ip, self.switch_id, if power { "on" } else { "off" });
        // Send a post request to the switch to turn it on or off
        match reqwest::blocking::Client::new().post(&url).send() {
            Ok(_) => println!("Successfully set power to: {}", power),
            Err(e) => eprintln!("Failed to set power to {}: {:?}", power, e)
        }
    }
}
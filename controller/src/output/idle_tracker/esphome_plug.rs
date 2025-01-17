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
        // It's okay to use the blocking client here, since we should
        // only change the power state infrequently and when there isn't any
        // other important data to send anyway.
        // Maybe this should be in the render thread instead, though.
        let power_usage_result =
            reqwest::blocking::get(&format!("http://{}/sensor/{}", self.ip, self.power_sensor_id))
            .ok()?
            .json::<serde_json::Value>()
            .ok()?;

        Some(PowerStats {
            current_power_usage: power_usage_result["value"].as_f64().unwrap_or(0.0) as f32
        })
    }

    fn set_power(&self, power: bool) {
        // It's okay to use the blocking client here, since we should
        // only change the power state infrequently and when there isn't any
        // other important data to send anyway.
        // Maybe this should be in the render thread instead, though.

        match reqwest::blocking::get(
            &format!("http://{}/switch/{}/turn_{}", self.ip, self.switch_id, if power { "on" } else { "off" })
        ) {
            Ok(_) => (),
            Err(e) => eprintln!("Error setting power: {:?}", e)
        }
    }
}
#[derive(Clone, Debug)]
pub struct PowerStats {
    /**
     * The current power usage in watts.
     * TODO: Graphing/logging this information would be useful.
     */
    pub current_power_usage: f32
}

pub trait PowerDevice {
    fn get_stats(&self) -> Option<PowerStats>;
    fn set_power(&mut self, power: bool);
}

pub struct LoggingPowerDevice {
    pub power: bool,
    pub stats: PowerStats
}

impl LoggingPowerDevice {
    pub fn new() -> Self {
        Self {
            power: false,
            stats: PowerStats {
                current_power_usage: 0.0
            }
        }
    }
}

impl PowerDevice for LoggingPowerDevice {
    fn get_stats(&self) -> Option<PowerStats> {
        Some(self.stats.clone())
    }
    
    fn set_power(&mut self, power: bool) {
        self.power = power;
        println!("Power set to {}", power);
    }
}
pub struct PowerStats {
    /**
     * The current power usage in watts.
     * TODO: Graphing/logging this information would be useful.
     */
    pub current_power_usage: f32
}

pub trait PowerDevice {
    fn get_stats(&self) -> Option<PowerStats>;
    fn set_power(&self, power: bool);
}
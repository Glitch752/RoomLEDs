use std::time::{Duration, Instant};

mod power_device;
pub mod esphome_plug;

/**
 * We always send an update at this interval, so we can be sure we don't
 * accidentally leave the lights in the wrong state somehow.
 */
static POWER_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/**
 * Tracks when the lights are idle and disables power to them.
 */
pub struct IdleTracker {
    pub last_idle: Instant,
    pub last_power_update: Instant,
    
    pub idle_threshold: Duration,
    pub idle: bool,

    pub get_idle: fn() -> bool,
    pub power_device: Box<dyn power_device::PowerDevice>    
}

impl IdleTracker {
    pub fn new(idle_threshold: Duration, get_idle: fn() -> bool, power_device: Box<dyn power_device::PowerDevice>) -> IdleTracker {
        IdleTracker {
            last_idle: Instant::now(),
            last_power_update: Instant::now(),
            idle_threshold,
            idle: false,
            get_idle,
            power_device
        }
    }

    pub fn update(&mut self) {
        let idle = (Instant::now() - self.last_idle) > self.idle_threshold || (self.get_idle)();
        
        if idle {
            self.last_idle = Instant::now();
        }

        if idle != self.idle {
            self.idle = idle;
            self.last_power_update = Instant::now();

            self.power_device.set_power(!self.idle);
        }

        if self.last_power_update.elapsed() > POWER_UPDATE_INTERVAL {
            self.last_power_update = Instant::now();
            self.power_device.set_power(!self.idle);
        }
    }
}
use std::time::{Duration, Instant};

use crate::render::frame::PresentedFrame;

pub mod power_device;
pub mod esphome_plug;

/// We always send an update at this interval, so we can be sure we don't
/// accidentally leave the lights in the wrong state somehow.
static POWER_UPDATE_INTERVAL: Duration = Duration::from_secs(60 * 10);

/// Tracks when the lights are idle and disables power to them.
pub struct IdleTracker {
    last_power_update: Instant,
    
    /// The debounce when transitioning from non-idle to idle.
    rising_debounce_time: Duration,
    /// The debounce when transitioning from idle to non-idle.
    falling_debounce_time: Duration,

    idle: Option<bool>,

    last_idle_switch: Instant,
    last_idle_target: bool,
    debounce: bool,

    power_device: Box<dyn power_device::PowerDevice>    
}

impl IdleTracker {
    pub fn new(
        rising_debounce_time: Duration,
        falling_debounce_time: Duration,
        power_device: Box<dyn power_device::PowerDevice>
    ) -> IdleTracker {
        IdleTracker {
            last_power_update: Instant::now(),
            power_device,
            
            rising_debounce_time,
            falling_debounce_time,
            
            idle: None,
            
            last_idle_switch: Instant::now(),
            last_idle_target: false,
            debounce: false
        }
    }

    fn get_idle(&self, lights: &PresentedFrame) -> bool {
        return lights.pixel_data.iter().all(|v| *v == 0);
    }

    pub fn is_idle(&self) -> bool {
        self.idle.unwrap_or(false)
    }

    pub fn update(&mut self, lights: &PresentedFrame) {
        let now = Instant::now();
        let idle_target = self.get_idle(lights);

        // If we've been targetting idle for over rising_debounce, switch to idle
        if idle_target != self.last_idle_target {
            self.last_idle_target = idle_target;
            self.last_idle_switch = now;
            self.debounce = true;
        }

        let debounce_time = if idle_target { self.rising_debounce_time } else { self.falling_debounce_time };
        if self.idle.is_none() || (
            self.debounce &&
            now - self.last_idle_switch > debounce_time &&
            idle_target != self.idle.unwrap()
        ) {
            self.debounce = false;
            self.last_power_update = now;
            self.idle = Some(idle_target);
            self.power_device.set_power(!idle_target);
        }

        // If we haven't updated the power in a while, do it now
        if now - self.last_power_update > POWER_UPDATE_INTERVAL {
            self.last_power_update = now;
            self.power_device.set_power(!self.idle.unwrap_or(true));
        }
    }
}
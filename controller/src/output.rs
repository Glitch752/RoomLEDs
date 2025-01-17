use ringbuf::traits::Consumer;
use serial_driver::SerialDriver;
use thread_priority::{ThreadBuilderExt, ThreadPriority};

use std::{thread::Thread, time::Duration};

use crate::render::RenderRingBufConsumer;

mod serial_driver;
mod idle_tracker;

fn run_output_thread(render_thread: Thread, mut render_consumer: RenderRingBufConsumer) {
    let mut drivers = SerialDriver::get_all_connected_drivers();

    if drivers.len() == 0 {
        eprintln!("Output: no drivers connected!");
    } else if drivers.len() == 1 {
        eprintln!("Output: only one driver connected! Only one driver will be used.");
    } else if drivers.len() == 2 {
        println!("Output: found two drivers connected!");
    } else {
        eprintln!("Output: too many drivers connected! Only the first two will be used.");
        drivers.truncate(2);
    }

    let idle_tracker = idle_tracker::IdleTracker::new(
        Duration::from_secs(60),
        || false,
        Box::new(idle_tracker::esphome_plug::ESPHomePlug::new(
            "192.168.68.131".to_string(),
            "kauf_plug".to_string(),
            "kauf_plug_power".to_string(),
        ))
    ); // TODO

    loop {
        // Since the controller only requests frames periodically, we expect them to "self-synchronize" if we sequentially send the data.
        // This... kind of works, but it's not perfect. I want to send the data in parallel with multiple threads and synchronize frame
        // presentation with a signal between the drivers, but that's more complicated and this is good enough for now.
        // TODO: Implement a better synchronization mechanism

        match render_consumer.try_pop() {
            Some(frame) => {
                for driver in &mut drivers {
                    driver.send_frame(&frame);
                }
            }
            None => {
                // If we don't have a frame, we don't update the output
                eprintln!("Output: no frame available from render thread. This caused a dropped frame.");
            }
        }

        // If no devices are connected, sleep for a bit to avoid a busy loop.
        // We still render frames (which can be shown in the web interface), but we don't output them.
        if drivers.len() == 0 {
            std::thread::sleep(Duration::from_secs(1) / 20);
        }
        
        // Wake up the render thread to render the next frame
        render_thread.unpark();
    }
}

pub fn start_output_thread(render_thread: Thread, render_consumer: RenderRingBufConsumer) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("lightingOutputThread".to_string())
        .spawn_with_priority(ThreadPriority::Max, |result| {
            match result {
                Ok(_) => println!("Successfully started output thread!"),
                Err(e) => {
                    eprintln!("Failed to start output thread with maximum priority: {:?}", e);
                }
            };
            
            run_output_thread(render_thread, render_consumer);
        })
        .expect("Failed to create output thread")
}
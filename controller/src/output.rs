use assert_no_alloc::assert_no_alloc;
use parking_lot::Mutex;
use render::render_frame;
use thread_priority::{ThreadBuilderExt, ThreadPriority};

use std::{io::Write, sync::Arc, time::{Duration, Instant}};

use serialport::SerialPortType;

use crate::{RenderState, FRAME_TIMES_STORED};

mod render;

// 1M baud is the absolute highest speed we can push the ESP8266 to.
// Sadly, with 407 pixels, this limits us to around 40 FPS.
static DRIVER_BAUD_RATE: u32 = 1_000_000;

static TOTAL_PIXELS: u32 = 814;

static ARDUINO_VID: u16 = 0x10C4;
static ARDUINO_PID: u16 = 0xEA60;

fn configure_driver_serial(path: &str) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    let mut driver_serial_port = serialport::new(path, DRIVER_BAUD_RATE)
        .timeout(Duration::from_millis(10))
        .open()?;

    driver_serial_port.set_data_bits(serialport::DataBits::Eight)?;
    driver_serial_port.set_parity(serialport::Parity::None)?;
    driver_serial_port.set_stop_bits(serialport::StopBits::One)?;
    driver_serial_port.set_flow_control(serialport::FlowControl::None)?;
    driver_serial_port.set_timeout(Duration::from_millis(10))?;

    Ok(driver_serial_port)
}

fn attempt_send_frame(port: &mut Result<Box<dyn serialport::SerialPort>, serialport::Error>, serial_buf: &mut [u8; 1], data: &[u8]) {
    match port {
        Ok(ref mut port) => {
            // Wait for a '>' character indicating we can start sending data
            // We basically just use a spinlock here since we need the most precise timing possible
            loop {
                match port.read(serial_buf.as_mut()) {
                    Ok(_) => {
                        if serial_buf[0] == '>' as u8 {
                            break;
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            serial_buf[0] = 0;

            // Send a '<' character to indicate the start of the pixel data
            match port.write("<".as_bytes()) {
                Ok(_) => (),
                Err(e) => eprintln!("{:?}", e),
            }

            // Send the pixel data
            match port.write(data) {
                Ok(_) => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Err(_) => (), // TODO: Try to reconnect to the serial port
    }
}

fn run_output_thread(render_state: Arc<Mutex<RenderState>>) {
    let mut ports = serialport::available_ports().expect("Unable to list serial ports");

    // Find the two drivers connected over USB with the correct VID/PID
    ports.sort_by_key(|i| i.port_name.clone());

    // TODO: Some kind of identification handshake to make sure we don't mix up the order of the drivers
    let mut driver_paths: Vec<&str> = ports
        .iter()
        .filter_map(|p| {
            match p.port_type.clone() {
                SerialPortType::UsbPort(info) => {
                    if info.vid == ARDUINO_VID && info.pid == ARDUINO_PID {
                        Some(p.port_name.as_str())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect();

    if driver_paths.len() == 0 {
        // TODO: Retry until we find the drivers
        eprintln!("No drivers found with VID 0x{:04X} and PID 0x{:04X}", ARDUINO_VID, ARDUINO_PID);
        return;
    }
    if driver_paths.len() < 2 {
        // Add fake drivers so we can test with only one driver
        // TODO: Retry until we find the drivers
        driver_paths.push("fake_driver");
    }

    // TODO: Better error handling when the serial port can't be opened
    let mut driver_1_serial_port = configure_driver_serial(driver_paths[0]);
    let mut driver_2_serial_port = configure_driver_serial(driver_paths[1]);

    match &driver_1_serial_port {
        Ok(port) => {
            println!("Opened driver 1 serial port at {}", port.name().unwrap());
        }
        Err(e) => eprintln!("Failed to open driver 1 serial port: {:?}", e),
    }
    match &driver_2_serial_port {
        Ok(port) => {
            println!("Opened driver 2 serial port at {}", port.name().unwrap());
        }
        Err(e) => eprintln!("Failed to open driver 2 serial port: {:?}", e),
    }

    // If we want to move rendering to another thread, we could use a ring buffer like https://crates.io/crates/ringbuf provides.
    // Rendering currently takes a minimal amount of time, though, so it's not necessary.
    let mut pixel_data: Vec<u8> = vec![0; (TOTAL_PIXELS * 3) as usize];
    render_frame(&mut pixel_data, Duration::from_millis(0), &render_state);

    let mut serial_buf = [0; 1];

    let mut last_frame_time: Instant = Instant::now();
    
    assert_no_alloc(|| {
        loop {
            let start_time: Instant = Instant::now();
            let delta = start_time - last_frame_time;
            last_frame_time = start_time;

            // We should never hold a lock on the render state for a significant amount of time in other threads
            match render_state.try_lock_for(Duration::from_millis(1)) {
                Some(mut state) => {
                    state.frames += 1;
                    
                    let frames = state.frames;
                    state.frame_times[frames % FRAME_TIMES_STORED] = delta.as_secs_f64();
                }
                None => {
                    eprintln!("Warning: failed to lock render state after 1ms. This caused a dropped frame.");
                }
            }

            // TODO: This should probably be on another thread so we can guarentee we'll send pixel data at the right time
            render_frame(&mut pixel_data, delta, &render_state);

            // TODO: A simple checksum?

            // Since the controller only requests frames periodically, they should "self-synchronize" to the same frame if we sequentially send the data

            // Clear the serial buffers
            if let Ok(ref mut driver_1_serial_port) = driver_1_serial_port {
                match driver_1_serial_port.clear(serialport::ClearBuffer::All) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            if let Ok(ref mut driver_2_serial_port) = driver_2_serial_port {
                match driver_2_serial_port.clear(serialport::ClearBuffer::All) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            // Driver 1 gets the first half of the pixel data
            // Since we move clockwise around the room and driver 1 faces in the counterclockwise direction,
            // we need to reverse the data. This means reversing 3-byte chunks, since we're sending RGB data.
            let driver_1_data = &pixel_data[0..(TOTAL_PIXELS / 2 * 3) as usize];
            let driver_1_data: Vec<u8> = driver_1_data.chunks(3).rev().flatten().copied().collect();
            attempt_send_frame(&mut driver_1_serial_port, &mut serial_buf, driver_1_data.as_slice());

            // Driver 2 gets the second half of the pixel data
            let driver_2_data = &pixel_data[(TOTAL_PIXELS / 2 * 3) as usize..(TOTAL_PIXELS * 3) as usize];
            attempt_send_frame(&mut driver_2_serial_port, &mut serial_buf, driver_2_data);
        }
    });
}

pub fn start_output_thread(render_state: Arc<Mutex<RenderState>>) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("lightingOutputThread".to_string())
        .spawn_with_priority(ThreadPriority::Max, |result| {
            match result {
                Ok(_) => println!("Successfully started output thread!"),
                Err(e) => {
                    eprintln!("Failed to start output thread with maximum priority: {:?}", e);
                }
            };
            
            run_output_thread(render_state);
        })
        .expect("Failed to create output thread")
}
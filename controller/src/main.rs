use std::{io::Write, time::{Duration, Instant}};

use assert_no_alloc::*;
use color_space::ToRgb;
use serialport::SerialPortType;

#[cfg(debug_assertions)] // required when disable_release is set (default)
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

// 1M baud is the absolute highest speed we can push the ESP8266 to.
// Sadly, with 407 pixels, this limits us to around 40 FPS.
static DRIVER_BAUD_RATE: u32 = 1_000_000;

static TOTAL_PIXELS: u32 = 814;

static ARDUINO_VID: u16 = 0x10C4;
static ARDUINO_PID: u16 = 0xEA60;

fn render_frame(pixel_data: &mut Vec<u8>, delta: Duration, start_hue: &mut f64) {
    *start_hue += delta.as_secs_f64() * 240.;
    
    for i in 0..TOTAL_PIXELS {
        let hue = (*start_hue + (i as f64) * 6.) % 360.;
        let color = color_space::Hsl::new(hue, 1., 0.5);
        let rgb = color.to_rgb();

        pixel_data[(i * 3 + 0) as usize] = rgb.r as u8;
        pixel_data[(i * 3 + 1) as usize] = rgb.g as u8;
        pixel_data[(i * 3 + 2) as usize] = rgb.b as u8;
    }

    // print_frame_to_stdout(pixel_data);
}

fn print_frame_to_stdout(pixel_data: &Vec<u8>) {
    // For testing: print the pixel data to the console using ANSI full-color escape codes and a full block character
    for i in 0..TOTAL_PIXELS {
        let r = pixel_data[(i * 3 + 0) as usize];
        let g = pixel_data[(i * 3 + 1) as usize];
        let b = pixel_data[(i * 3 + 2) as usize];
        print!("\x1b[38;2;{};{};{}m█", r, g, b);
    }
    print!("\x1b[0m\n");
}

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
            let start = Instant::now();
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
            println!("Waited for '>' for {}ms", start.elapsed().as_millis());
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

fn main() {
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

    let mut pixel_data: Vec<u8> = vec![0; (TOTAL_PIXELS * 3) as usize];
    let mut start_hue: f64 = 0.;
    render_frame(&mut pixel_data, Duration::from_millis(0), &mut start_hue);

    let mut serial_buf = [0; 1];

    let mut last_frame_time: Instant = Instant::now();
    
    assert_no_alloc(|| {
        // TODO: This should probably be on another thread so we can guarentee we'll send pixel data at the right time
        
        loop {
            let start_time: Instant = Instant::now();
            let delta = start_time - last_frame_time;
            last_frame_time = start_time;

            render_frame(&mut pixel_data, delta, &mut start_hue);

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
            let driver_1_data = &pixel_data[0..(TOTAL_PIXELS / 2 * 3) as usize];
            attempt_send_frame(&mut driver_1_serial_port, &mut serial_buf, driver_1_data);

            // Driver 2 gets the second half of the pixel data
            let driver_2_data = &pixel_data[(TOTAL_PIXELS / 2 * 3) as usize..(TOTAL_PIXELS * 3) as usize];
            attempt_send_frame(&mut driver_2_serial_port, &mut serial_buf, driver_2_data);
            
            // let elapsed = start_time.elapsed();
            // println!("Wrote frame to driver 1 in {}ms", elapsed.as_millis());
        }
    });
}

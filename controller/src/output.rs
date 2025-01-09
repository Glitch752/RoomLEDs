use ringbuf::traits::Consumer;
use thread_priority::{ThreadBuilderExt, ThreadPriority};

use std::{io::Write, thread::Thread, time::Duration};

use serialport::SerialPortType;

use crate::{render::RenderRingBufConsumer, TOTAL_PIXELS};

// 1M baud is the absolute highest speed we can push the ESP8266 to.
// Sadly, with 407 pixels, this limits us to around 40 FPS.
static DRIVER_BAUD_RATE: u32 = 1_000_000;

static ARDUINO_VID: u16 = 0x10C4;
static ARDUINO_PID: u16 = 0xEA60;

fn configure_driver_serial(paths: &Vec<&str>, index: usize) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    if paths.len() <= index {
        return Err(serialport::Error { kind: serialport::ErrorKind::NoDevice, description: "".to_string() });
    }

    let path = paths[index];

    let driver_serial_port = serialport::new(path, DRIVER_BAUD_RATE)
        .timeout(Duration::from_millis(10))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .open()?;

    Ok(driver_serial_port)
}

fn send_command(port: &mut Result<Box<dyn serialport::SerialPort>, serialport::Error>, serial_buf: &mut [u8; 1], command: u8, data: &[u8]) {
    match port {
        Ok(ref mut port) => {
            // Wait for a '>' character indicating we can start sending data
            // We basically just use a spinlock here since we need the most precise timing possible
            serial_buf[0] = 0;
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

            match port.write(&[command]) {
                Ok(_) => (),
                Err(e) => eprintln!("Writing command failed: {:?}", e),
            }

            match port.write(data) {
                Ok(_) => (),
                Err(e) => eprintln!("Writing data failed: {:?}", e),
            }

            // Clear the serial buffers
            match port.clear(serialport::ClearBuffer::All) {
                Ok(_) => (),
                Err(e) => eprintln!("Clearing buffer failed: {:?}", e),
            }
        }
        Err(_) => {
            // TODO: Try to reconnect to the serial port
        }
    }
}

fn attempt_set_brightness(port: &mut Result<Box<dyn serialport::SerialPort>, serialport::Error>, serial_buf: &mut [u8; 1], brightness: u8) {
    send_command(port, serial_buf, b'b', &[brightness])
}

fn attempt_send_frame(port: &mut Result<Box<dyn serialport::SerialPort>, serialport::Error>, serial_buf: &mut [u8; 1], data: &[u8]) {
    send_command(port, serial_buf, b'<' as u8, data)
}

fn run_output_thread(render_thread: Thread, mut render_consumer: RenderRingBufConsumer) {
    let mut ports = serialport::available_ports().expect("Unable to list serial ports");

    // Find the two drivers connected over USB with the correct VID/PID
    ports.sort_by_key(|i| i.port_name.clone());

    // TODO: Some kind of identification handshake to make sure we don't mix up the order of the drivers
    let driver_paths: Vec<&str> = ports
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

    if driver_paths.len() < 2 {
        eprintln!("Only {} devices found with VID 0x{:04X} and PID 0x{:04X}. Will retry until found.", driver_paths.len(), ARDUINO_VID, ARDUINO_PID);
    }

    // TODO: Better error handling when the serial port can't be opened
    let mut driver_1_serial_port = configure_driver_serial(&driver_paths, 0);
    let mut driver_2_serial_port = configure_driver_serial(&driver_paths, 1);

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

    // The render thread adds pixel data to the ring buffer
    let mut pixel_data: Vec<u8> = vec![0; (TOTAL_PIXELS * 3) as usize];

    let mut serial_buf = [0; 1];
    let mut reverse_scratch_buffer = [0; TOTAL_PIXELS as usize / 2 * 3];

    // TEMPORARY
    attempt_set_brightness(&mut driver_1_serial_port, &mut serial_buf, 255);
    attempt_set_brightness(&mut driver_2_serial_port, &mut serial_buf, 255);
    
    loop {
        // Since the controller only requests frames periodically, we expect them to "self-synchronize" if we sequentially send the data.
        // This... kind of works, but it's not perfect. I want to send the data in parallel with multiple threads and synchronize frame
        // presentation with a signal between the drivers, but that's more complicated and this is good enough for now.
        // TODO: Implement a better synchronization mechanism

        match render_consumer.try_pop() {
            Some(frame) => {
                // Copy the pixel data from the frame into the output buffer
                pixel_data.copy_from_slice(&frame.pixel_data);
            }
            None => {
                // If we don't have a frame, we don't update the output
                eprintln!("Output: no frame available from render thread. This caused a dropped frame.");
            }
        }

        // Driver 1 gets the first half of the pixel data
        // Since we move clockwise around the room and driver 1 faces in the counterclockwise direction,
        // we need to reverse the data. This means reversing 3-byte chunks, since we're sending RGB data.
        let driver_1_data = &pixel_data[0..(TOTAL_PIXELS / 2 * 3) as usize];
        // Since we want to minimize allocations in the output thread, we use a scratch buffer
        for i in 0..(TOTAL_PIXELS / 2) {
            reverse_scratch_buffer[i as usize * 3 + 0] = driver_1_data[(TOTAL_PIXELS / 2 - i - 1) as usize * 3 + 0];
            reverse_scratch_buffer[i as usize * 3 + 1] = driver_1_data[(TOTAL_PIXELS / 2 - i - 1) as usize * 3 + 1];
            reverse_scratch_buffer[i as usize * 3 + 2] = driver_1_data[(TOTAL_PIXELS / 2 - i - 1) as usize * 3 + 2];
        }
        attempt_send_frame(&mut driver_1_serial_port, &mut serial_buf, &reverse_scratch_buffer);
        
        // Driver 2 gets the second half of the pixel data
        let driver_2_data = &pixel_data[(TOTAL_PIXELS / 2 * 3) as usize..(TOTAL_PIXELS * 3) as usize];
        attempt_send_frame(&mut driver_2_serial_port, &mut serial_buf, driver_2_data);
        
        // If no devices are connected, sleep for a bit to avoid a busy loop.
        // We still render frames (which can be shown in the web interface), but we don't output them.
        if driver_1_serial_port.is_err() && driver_2_serial_port.is_err() {
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
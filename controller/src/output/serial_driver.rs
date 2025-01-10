use std::time::{Duration, Instant};

use serialport::SerialPortType;

use crate::render::frame::PresentedFrame;

// 1M baud is the absolute highest speed we can push the ESP8266 to.
// Sadly, with 407 pixels, this limits us to around 40 FPS.
static DRIVER_BAUD_RATE: u32 = 1_000_000;

static ARDUINO_VID: u16 = 0x10C4;
static ARDUINO_PID: u16 = 0xEA60;

static IDENTIFY_COMMAND: u8 = b'i';
static SET_BRIGHTNESS_COMMAND: u8 = b'b';
static SEND_FRAME_COMMAND: u8 = b'<';

static RESPONSE_READY: u8 = b'r';
static RESPONSE_HANDSHAKE: u8 = b'i';
static RESPONSE_DEBUG: u8 = b'd';

static DRIVERS: usize = 2;
static DRIVER_LOCATIONS: [DriverStrandLocation; DRIVERS] = [
    DriverStrandLocation {
        start: 406,
        end: 811
    },
    DriverStrandLocation {
        start: 405,
        end: 0
    },
];

#[derive(Debug, Clone)]
pub struct DriverStrandLocation {
    /// The first pixel in the strand, inclusive
    pub start: u32,
    /// The last pixel in the strand, inclusive. If `end` is less than `start`, the strand is reversed.
    pub end: u32
}

pub struct SerialDriver {
    port: Box<dyn serialport::SerialPort>,
    id: Option<u8>
}

impl SerialDriver {
    pub fn get_all_connected_drivers() -> Vec<SerialDriver> {
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

        let mut drivers = Vec::new();

        for path in driver_paths {
            let port = SerialDriver::open_serial_port(path).expect("Failed to open serial port");
            let mut driver = SerialDriver {
                port,
                id: None
            };
            driver.identify();

            if let Some(id) = driver.id {
                println!("Successfully opened driver at {} and identified as ID {}", path, id);
            } else {
                eprintln!("Failed to identify driver at {}", path);
            }
            
            drivers.push(driver);
        }

        drivers
    }


    fn open_serial_port(path: &str) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
        let driver_serial_port = serialport::new(path, DRIVER_BAUD_RATE)
            .timeout(Duration::from_millis(10))
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .open()?;

        Ok(driver_serial_port)
    }

    fn send_command(self: &mut SerialDriver, command: u8, data: &[u8]) {
        let mut message = vec![command];
        message.extend(data);

        let mut encoded_message = vec![0; corncobs::max_encoded_len(message.len()) + 1];
        let length = corncobs::encode_buf(&message, &mut encoded_message);
        encoded_message.truncate(length + 1); // The last byte is always 0x00

        loop {
            match self.port.write_all(&encoded_message) {
                Ok(_) => {},
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(e) => {
                    eprintln!("Failed to write to serial port: {:?}", e);
                    break;
                }
            }
            break;
        }
    }

    fn packet_available(self: &mut SerialDriver) -> bool {
        self.port.bytes_to_read().unwrap() > 0
    }

    fn read_packet(self: &mut SerialDriver) -> Option<Vec<u8>> {
        let mut packet = Vec::new();
        // Read until we encounter a 0x00 byte
        loop {
            let mut buf = [0; 1];
            match self.port.read_exact(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(e) => {
                    eprintln!("Failed to read from serial port: {:?}", e);
                    return None;
                }
            }

            if buf[0] == 0x00 && packet.len() == 0 && self.port.bytes_to_read().unwrap() > 0 {
                continue;
            }

            packet.push(buf[0]);

            if buf[0] == 0x00 {
                break;
            }
        }

        let mut decoded_packet = vec![0; corncobs::max_encoded_len(packet.len())];
        match corncobs::decode_buf(&packet, &mut decoded_packet) {
            Ok(size) => {
                decoded_packet.truncate(size);
            },
            Err(e) => {
                eprintln!("Failed to decode packet: {:?}", e);
                return None;
            }
        }

        if decoded_packet.len() == 0 {
            eprintln!("Received empty packet from driver {}", self.id.unwrap_or(0));
            return None;
        }

        Some(decoded_packet)
    }
    fn wait_for_packet(self: &mut SerialDriver, timeout: Duration) -> Option<Vec<u8>> {
        let start = Instant::now();
        while !self.packet_available() {
            if start.elapsed() > timeout {
                eprintln!("Timeout waiting for packet from driver {}", self.id.unwrap_or(0));
                return None;
            }
            // We use a busy loop here because we need the most precise timing possible
        }
        self.read_packet()
    }
    fn wait_for_packet_discard_others(self: &mut SerialDriver, packet_type: u8, timeout: Duration) -> Option<Vec<u8>> {
        loop {
            let packet = self.wait_for_packet(timeout)?;
            if packet[0] == packet_type {
                return Some(packet[1..].to_vec());
            } else if packet[0] == RESPONSE_DEBUG {
                eprintln!("Debug message from driver {}: {:?}", self.id.unwrap_or(0), packet);
            }
        }
    }
    fn discard_waiting_packets(self: &mut SerialDriver) {
        while self.packet_available() {
            let packet = self.read_packet();
            if let Some(packet) = packet {
                if packet[0] == RESPONSE_DEBUG {
                    eprintln!("Debug message from driver {}: {:?}", self.id.unwrap_or(0), packet);
                }
            }
        }
    }

    fn send_frame_data(self: &mut SerialDriver, data: &[u8]) {
        self.send_command(SEND_FRAME_COMMAND, data)
    }

    fn identify(self: &mut SerialDriver) {
        for attempt in 1..=5 {
            self.discard_waiting_packets();

            self.send_command(IDENTIFY_COMMAND, &[]);
            
            if let Some(response) = self.wait_for_packet_discard_others(RESPONSE_HANDSHAKE, Duration::from_millis(100)) {
                self.id = Some(response[0]);
        
                if self.id.unwrap() >= DRIVERS as u8 {
                    eprintln!("Driver ID {} is out of bounds; setting to 0", self.id.unwrap());
                    self.id = None;
                }

                break;
            } else {
                eprintln!("Failed to identify driver on attempt {}", attempt);
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }

    fn get_strand_location(self: &SerialDriver) -> DriverStrandLocation {
        if let Some(id) = self.id {
            return DRIVER_LOCATIONS[id as usize].clone();
        } else {
            return DRIVER_LOCATIONS[0].clone();
        }
    }
    
    pub fn set_brightness(self: &mut SerialDriver, brightness: u8) {
        self.send_command(SET_BRIGHTNESS_COMMAND, &[brightness])
    }

    pub fn send_frame(self: &mut SerialDriver, frame: &PresentedFrame) {
        let location = self.get_strand_location();

        // We intentionally ignore the case where we don't recieve a response here,
        // because that will be the case on the first frame sent to the driver.
        _ = self.wait_for_packet_discard_others(RESPONSE_READY, Duration::from_millis(100));

        let mut start = location.start as usize;
        let mut end = location.end as usize;
        let reverse = location.end < location.start;
        if reverse {
            (start, end) = (end, start);
        }
        end += 1; // Inclusive range

        let length = end - start;
        let mut data = vec![0; length * 3];
        for i in 0..length {
            let pixel = if reverse {
                frame.get_pixel((end - i) as u32)
            } else {
                frame.get_pixel((start + i) as u32)
            };
            data[i * 3] = pixel.0;
            data[i * 3 + 1] = pixel.1;
            data[i * 3 + 2] = pixel.2;
        }

        self.send_frame_data(&data);
    }
}
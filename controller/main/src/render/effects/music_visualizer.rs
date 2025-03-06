use std::{cmp::min, net::{Ipv4Addr, SocketAddr, UdpSocket}, time::Duration};

use color_space::Hsl;
use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::{Frame, Pixel}, RenderInfo, TOTAL_PIXELS};

use super::{AnyEffect, Effect};

static PACKET_FROP_FRAMES: usize = 500;

/// The music visualizer effect runs a TCP socket server that listens for
/// audio data from the music visualizer client. Then, it renders the audio
/// data as a visualizer.
#[derive(Reflect, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicVisualizerEffect {
    /// The UDP listener that listens for audio data from the music visualizer client
    #[serde(rename = "port", deserialize_with = "deserialize_udp_socket", serialize_with = "serialize_udp_socket")]
    #[reflect(as_type = "u16")]
    listener: UdpSocket,

    /// The buffer that stores the audio data
    #[serde(skip)]
    audio_buffer: Vec<f32>,

    /// The time at which the last audio data was received
    /// If no audio data has been received in a while, the visualizer will
    /// display a pulsing red color
    #[serde(skip)]
    data_last_received: Option<std::time::Instant>,

    /// Used to calculate last frames' packet drop rate
    #[cfg(debug_assertions)]
    #[serde(skip, default="default_packet_receive_frames")]
    packet_receive_frames: [bool; PACKET_FROP_FRAMES],
}

impl Clone for MusicVisualizerEffect {
    fn clone(&self) -> Self {
        Self {
            listener: self.listener.try_clone().unwrap(),
            audio_buffer: self.audio_buffer.clone(),
            data_last_received: self.data_last_received,
            #[cfg(debug_assertions)]
            packet_receive_frames: self.packet_receive_frames.clone(),
        }
    }
}

fn default_packet_receive_frames() -> [bool; PACKET_FROP_FRAMES] {
    [false; PACKET_FROP_FRAMES]
}

/// Deserialize a UDP socket from a port number
fn deserialize_udp_socket<'de, D>(deserializer: D) -> Result<UdpSocket, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let port = u16::deserialize(deserializer)?;
    let listener = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))).unwrap();
    listener.set_nonblocking(true).unwrap();
    Ok(listener)
}

fn serialize_udp_socket<S>(listener: &UdpSocket, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    listener.local_addr().unwrap().port().serialize(serializer)
}

impl MusicVisualizerEffect {
    /// Creates a new music visualizer effect that listens on the specified port.
    #[allow(unused)]
    pub fn new(port: u16) -> AnyEffect {
        let listener = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))).unwrap();
        listener.set_nonblocking(true).unwrap();

        println!("Music visualizer effect listening on port {}", port);
        
        Self {
            listener,
            audio_buffer: vec![],
            data_last_received: None,

            #[cfg(debug_assertions)]
            packet_receive_frames: [false; PACKET_FROP_FRAMES],
        }.into()
    }
}

impl Effect for MusicVisualizerEffect {
    fn render(&mut self, delta: Duration, info: &mut RenderInfo) -> Frame {
        static BLOCK_SIZE: usize = 4;

        // Read audio data from the client
        let mut looped = false;
        let mut audio_data = vec![0; TOTAL_PIXELS as usize / BLOCK_SIZE];
        while self.listener.peek_from(&mut [0; 1]).is_ok() {
            looped = true;
            self.listener.recv(&mut audio_data).unwrap();
        }
        
        if looped {
            self.audio_buffer = audio_data.iter().map(|&x| x as f32).collect();
            self.data_last_received = Some(std::time::Instant::now());

            #[cfg(debug_assertions)] {
                self.packet_receive_frames[info.frames % PACKET_FROP_FRAMES] = true;
            }
        } else {
            // No audio data is available, so slowly fade out the audio data to make it feel slightly more responsive
            for i in 0..self.audio_buffer.len() {
                self.audio_buffer[i] *= 0.5_f32.powf(delta.as_secs_f32());
            }

            #[cfg(debug_assertions)] {
                self.packet_receive_frames[info.frames % PACKET_FROP_FRAMES] = false;
            }
        }

        if self.data_last_received.is_none() || self.data_last_received.unwrap().elapsed().as_secs() > 2 {
            // If there are no incoming connections, return pulsing red
            let mut frame = Frame::empty();
            let color = Pixel::new(
                255, 0, 0,
                (info.time * 2.).sin() * 0.4 + 0.4
            );

            static PULSE_SECTION_WIDTH: i32 = 3;
            for i in -PULSE_SECTION_WIDTH..PULSE_SECTION_WIDTH {
                frame.set_pixel(i.rem_euclid(TOTAL_PIXELS as i32) as u32, color.clone());
            }
            for i in (TOTAL_PIXELS / 2 - PULSE_SECTION_WIDTH as u32)..(TOTAL_PIXELS / 2 + PULSE_SECTION_WIDTH as u32) {
                frame.set_pixel(i, color.clone());
            }
            return frame;
        }


        // Linearly interpolate the audio data
        let audio_data = &self.audio_buffer;

        let mut audio_data_interpolated = vec![0; TOTAL_PIXELS as usize];
        
        for i in 0..audio_data.len() {
            let start = i * BLOCK_SIZE;
            let end = min((i + 1) * BLOCK_SIZE, TOTAL_PIXELS as usize);

            for j in start..end {
                let t = (j - start) as f64 / BLOCK_SIZE as f64;
                audio_data_interpolated[j] = (audio_data[i] as f64 * (1. - t) + audio_data[min(i + 1, audio_data.len() - 1)] as f64 * t) as u8;
            }
        }

        // Render the visualizer
        let mut frame = Frame::empty();
        for i in 0..TOTAL_PIXELS as usize {
            let hue = i as f64 / TOTAL_PIXELS as f64 * 360.;
            let lightness = audio_data_interpolated[i] as f64 / 255.;
            let color = Hsl::new(hue, 0.5, lightness as f64).into();
            
            frame.set_pixel(i as u32, color);
        }

        #[cfg(debug_assertions)] {
            // Calculate packet drop rate
            let frames = min(info.frames, PACKET_FROP_FRAMES);
            let packets_received = self.packet_receive_frames.iter().take(frames).filter(|&&x| x).count();
            let packets_dropped = PACKET_FROP_FRAMES - packets_received;
            let packet_drop_rate = packets_dropped as f64 / (packets_received + packets_dropped) as f64;
            info.debug_text = format!("Packet drop rate over {} frames: {:.1}% ({} dropped)", frames, packet_drop_rate * 100., packets_dropped);
        }

        return frame;
    }
}
use std::{cmp::min, net::{Ipv4Addr, SocketAddr, UdpSocket}};

use color_space::Hsl;

use crate::{render::frame::{Frame, Pixel}, RenderState, TOTAL_PIXELS};

use super::Layer;

/// The music visualizer layer runs a TCP socket server that listens for
/// audio data from the music visualizer client. Then, it renders the audio
/// data as a visualizer.
pub struct MusicVisualizerLayer {
    /// The UDP listener that listens for audio data from the music visualizer client
    listener: UdpSocket,

    /// The buffer that stores the audio data
    audio_buffer: Vec<f32>,

    /// The time at which the last audio data was received
    /// If no audio data has been received in a while, the visualizer will
    /// display a pulsing red color
    data_last_received: Option<std::time::Instant>
}

impl MusicVisualizerLayer {
    /// Creates a new music visualizer layer
    pub fn new(port: u16) -> Self {
        let listener = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))).unwrap();
        listener.set_nonblocking(true).unwrap();

        println!("Music visualizer layer listening on port {}", port);
        
        Self {
            listener,
            audio_buffer: vec![],
            data_last_received: None
        }
    }
}

impl Layer for MusicVisualizerLayer {
    fn render(&mut self, state: &RenderState) -> Frame {
        static BLOCK_SIZE: usize = 4;

        // Read audio data from the client
        if self.listener.peek_from(&mut [0; 1]).is_ok() {
            self.data_last_received = Some(std::time::Instant::now());

            let mut audio_data = vec![0; TOTAL_PIXELS as usize / BLOCK_SIZE];
            self.listener.recv(&mut audio_data).unwrap();
            
            self.audio_buffer = audio_data.iter().map(|&x| x as f32).collect();
        }

        if self.data_last_received.is_none() || self.data_last_received.unwrap().elapsed().as_secs() > 1 {
            // If there are no incoming connections, return pulsing red
            let mut frame = Frame::empty();
            let color = Pixel::new(
                255, 0, 0,
                (state.time * 2.).sin() * 0.4 + 0.4
            );
            for i in (TOTAL_PIXELS / 2 - 5)..(TOTAL_PIXELS / 2 + 5) {
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

        return frame;
    }
}
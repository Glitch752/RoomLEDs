use std::{io::Read, net::IpAddr, process::Stdio};

use crate::TOTAL_PIXELS;

static BLOCK_SIZE: usize = 4;
static FRAMERATE: u32 = 80;

pub fn run(address: IpAddr) {
    let socket_address: std::net::SocketAddr = (address, shared::constants::MUSIC_VISUALIZER_PORT).into();

    // Ensure the `cava` command is installed
    let cava_installed = std::process::Command::new("which")
        .arg("cava")
        .output()
        .expect("Failed to check if `cava` is installed")
        .status
        .success();

    if !cava_installed {
        eprintln!("`cava` is not installed. Please install it before running the server.");
        std::process::exit(1);
    }

    // Connect to the server
    let udp_socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to UDP socket");
    udp_socket.connect(socket_address).expect("Failed to connect to server");

    println!("Connected to server at {} (probably... because UDP)", socket_address);

    // Create a configuration file for `cava` so it outputs raw data
    let cava_config = format!(r#"
## Configuration file for CAVA.
# Remove the ; to change parameters.


[general]

# Accepts only non-negative values.
framerate = {}

# 'autosens' will attempt to decrease sensitivity if the bars peak. 1 = on, 0 = off
# new as of 0.6.0 autosens of low values (dynamic range)
# 'overshoot' allows bars to overshoot (in % of terminal height) without initiating autosens. DEPRECATED as of 0.6.0
; autosens = 1
; overshoot = 20

# Manual sensitivity in %. If autosens is enabled, this will only be the initial value.
# 200 means double height. Accepts only non-negative values.
; sensitivity = 100

# The number of bars (0-512). 0 sets it to auto (fill up console).
# Bars' width and space between bars in number of characters.
bars = {}

# Lower and higher cutoff frequencies for lowest and highest bars
# the bandwidth of the visualizer.
# Note: there is a minimum total bandwidth of 43Mhz x number of bars.
# Cava will automatically increase the higher cutoff if a too low band is specified.
; lower_cutoff_freq = 50
; higher_cutoff_freq = 10000

# Seconds with no input before cava goes to sleep mode. Cava will not perform FFT or drawing and
# only check for input once per second. Cava will wake up once input is detected. 0 = disable.
; sleep_timer = 0

[input]
# Audio capturing method. Possible methods are: 'fifo', 'portaudio', 'pipewire', 'alsa', 'pulse', 'sndio', 'oss', 'jack' or 'shmem'
# Defaults to 'oss', 'pipewire', 'sndio', 'jack', 'pulse', 'alsa', 'portaudio' or 'fifo', in that order, dependent on what support cava was built with.
# On Mac it defaults to 'portaudio' or 'fifo'
# On windows this is automatic and no input settings are needed.
#
# All input methods uses the same config variable 'source'
# to define where it should get the audio.
#
# For pulseaudio and pipewire 'source' will be the source. Default: 'auto', which uses the monitor source of the default sink
# (all pulseaudio sinks(outputs) have 'monitor' sources(inputs) associated with them).
#
# For pipewire 'source' will be the object name or object.serial of the device to capture from.
# Both input and output devices are supported.
#
# For alsa 'source' will be the capture device.
# For fifo 'source' will be the path to fifo-file.
# For shmem 'source' will be /squeezelite-AA:BB:CC:DD:EE:FF where 'AA:BB:CC:DD:EE:FF' will be squeezelite's MAC address
#
# For sndio 'source' will be a raw recording audio descriptor or a monitoring sub-device, e.g. 'rsnd/2' or 'snd/1'. Default: 'default'.
# README.md contains further information on how to setup CAVA for sndio.
#
# For oss 'source' will be the path to a audio device, e.g. '/dev/dsp2'. Default: '/dev/dsp', i.e. the default audio device.
# README.md contains further information on how to setup CAVA for OSS on FreeBSD.
#
# For jack 'source' will be the name of the JACK server to connect to, e.g. 'foobar'. Default: 'default'.
# README.md contains further information on how to setup CAVA for JACK.
#
; method = pulse
; source = auto

; method = pipewire
; source = auto

; method = alsa
; source = hw:Loopback,1

; method = fifo
; source = /tmp/mpd.fifo

; method = shmem
; source = /squeezelite-AA:BB:CC:DD:EE:FF

; method = portaudio
; source = auto

; method = sndio
; source = default

; method = oss
; source = /dev/dsp

; method = jack
; source = default

# The options 'sample_rate', 'sample_bits', 'channels' and 'autoconnect' can be configured for some input methods:
#   sample_rate: fifo, pipewire, sndio, oss
#   sample_bits: fifo, pipewire, sndio, oss
#   channels:    sndio, oss, jack
#   autoconnect: jack
# Other methods ignore these settings.
#
# For 'sndio' and 'oss' they are only preferred values, i.e. if the values are not supported
# by the chosen audio device, the device will use other supported values instead.
# Example: 48000, 32 and 2, but the device only supports 44100, 16 and 1, then it
# will use 44100, 16 and 1.
#
; sample_rate = 44100
; sample_bits = 16
; channels = 2
; autoconnect = 2

[output]
# Output method. Can be 'ncurses', 'noncurses', 'raw', 'noritake', 'sdl'
# or 'sdl_glsl'.
# 'noncurses' (default) uses a buffer and cursor movements to only print
# changes from frame to frame in the terminal. Uses less resources and is less
# prone to tearing (vsync issues) than 'ncurses'.
#
# 'raw' is an 8 or 16 bit (configurable via the 'bit_format' option) data
# stream of the bar heights that can be used to send to other applications.
# 'raw' defaults to 200 bars, which can be adjusted in the 'bars' option above.
#
# 'noritake' outputs a bitmap in the format expected by a Noritake VFD display
#  in graphic mode. It only support the 3000 series graphical VFDs for now.
#
# 'sdl' uses the Simple DirectMedia Layer to render in a graphical context.
# 'sdl_glsl' uses SDL to create an OpenGL context. Write your own shaders or
# use one of the predefined ones.
method = raw

# Visual channels. Can be 'stereo' or 'mono'.
# 'stereo' mirrors both channels with low frequencies in center.
# 'mono' outputs left to right lowest to highest frequencies.
# 'mono_option' set mono to either take input from 'left', 'right' or 'average'.
# set 'reverse' to 1 to display frequencies the other way around.
; channels = stereo
; mono_option = average
; reverse = 0

# Raw output target. A fifo will be created if target does not exist.
raw_target = /dev/stdout

# Raw data format. Can be 'binary' or 'ascii'.
data_format = binary

# Binary bit format, can be '8bit' (0-255) or '16bit' (0-65530).
bit_format = 8bit

# Ascii max value. In 'ascii' mode range will run from 0 to value specified here
; ascii_max_range = 1000

# Ascii delimiters. In ascii format each bar and frame is separated by a delimiters.
# Use decimal value in ascii table (i.e. 59 = ';' and 10 = '\n' (line feed)).
; bar_delimiter = 59
; frame_delimiter = 10

[smoothing]
# Disables or enables the so-called "Monstercat smoothing" with or without "waves". Set to 0 to disable.
; monstercat = 0
; waves = 0

# Noise reduction, int 0 - 100. default 77
# the raw visualization is very noisy, this factor adjusts the integral and gravity filters to keep the signal smooth
# 100 will be very slow and smooth, 0 will be fast but noisy.
noise_reduction = 70


[eq]
# This one is tricky. You can have as much keys as you want.
# Remember to uncomment more than one key! More keys = more precision.
# Look at readme.md on github for further explanations and examples.
1 = 1 # bass
2 = 1
3 = 1 # midtone
4 = 1
5 = 1 # treble
"#, FRAMERATE, TOTAL_PIXELS / BLOCK_SIZE as u32 + 1); // I'm unsure why we need to add 1 to get the correct number of blocks

    std::fs::write("/tmp/cava.conf", cava_config).expect("Failed to write cava configuration file");

    // Start the `cava` process
    let mut cava_process = std::process::Command::new("cava")
        .arg("-p")
        .arg("/tmp/cava.conf")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cava process");

    // Send the output of `cava` to the server
    let mut stdout = cava_process.stdout.take().unwrap();

    // There's probably a better way to do this, but I'm not sure what it would be.
    let mut buffer: [u8; TOTAL_PIXELS as usize / BLOCK_SIZE + 1] = [0; TOTAL_PIXELS as usize / BLOCK_SIZE + 1];
    
    let mut exponential_backoff = std::time::Duration::from_secs(1);

    // Used to measure the average data transfer rate
    let mut total_bytes_sent = 0;
    let mut total_packets_sent = 0;
    let mut last_transfer_rate_update = std::time::Instant::now();

    loop {
        // Read the audio data from `cava`
        stdout.read_exact(&mut buffer).expect("Failed to read audio data from cava");
        std::thread::sleep(std::time::Duration::from_millis(1000 / FRAMERATE as u64));
        if let Err(e) = udp_socket.send(&buffer) {
            eprintln!("Failed to send audio data to server: {}", e);
            std::thread::sleep(exponential_backoff);
            exponential_backoff *= 2;
            exponential_backoff = std::cmp::min(exponential_backoff, std::time::Duration::from_secs(120));
        } else {
            exponential_backoff = std::time::Duration::from_secs(1);

            total_bytes_sent += buffer.len();
            total_packets_sent += 1;

            if total_packets_sent % 500 == 0 {
                let elapsed = last_transfer_rate_update.elapsed().as_secs_f64();
                let bytes_per_second = total_bytes_sent as f64 / elapsed;
                let packets_per_second = 500. / elapsed;
                println!("Average data transfer rate: {:.1} bytes/s ({:.1} packets/s - {} packets total)", bytes_per_second, packets_per_second, total_packets_sent);

                total_bytes_sent = 0;
                last_transfer_rate_update = std::time::Instant::now();
            }
        }
    }
}
# RoomLEDs

The most over-engineered solution to drive a strip of NeoPixels (WS2812B LEDs) mounted around the edge of a room. The strips are controlled by two ESP8266 microcontrollers, and the microcontrollers are sent data over serial from an SBC (currently the Libre Computer Le Potato) running a Rust application. Other hardware components, including a custom power system, make the system more robust and powerful.  

Oh, and it's in Rust (BTW)... because why not have complete memory safety for your room lights, I guess?

## Features
- Custom ESP8266 firmware to control a strip of LEDs over serial
- Integration with ESPHome to control a Kauf Plug for automatic idle power management
- Rust controller server and deploy scripts
  - Web interface with a preview, statistics, and preset controls (with more to come)
  - System to render complex animations with spatial data
  - Precise real-time system to send data to the LEDs at the correct time
  - Momentary effects triggered by an API call
- Control client with features to integrate devices with the LEDs
  - Music visualizer using Cava

## TODO
- [ ] Make custom ESPHome replacement firmware for the Kauf Plug to make the power system more robust and allow for more control
- [ ] Create an Alexa integration
- [ ] Add a door sensor system to turn on the lights when someone enters the room
- [ ] Allow composing animations and effects with the web interface
- [ ] Add an alarm system that flashes the lights
- [ ] Document the hardware required
- [ ] Graphing/logging power usage data

## Using
I don't have a proper deploy system set up, but running the script `install.sh` under `controller` should install the necessary dependencies and set up a systemd service to run the Python script at boot. Maybe Nix would be appropriate for this project, but I'm honestly a bit tired of Nix after setting up my whole system with it for now haha.

## Developing
To make PlatformIO work properly, you need to open the `arduino-driver` folder in a separate VSCode window. The arduino driver is set up with a minimal Wokwi diagram and configuration to allow testing without the hardware; you just need to uncomment `#define WOKWI` in `main.cpp`.

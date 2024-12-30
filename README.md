# RoomLEDs

A project I'm currently working on to drive a strip of NeoPixels (WS2812B LEDs) mounted around the edge of my room. The strips are controlled by two ESP8266 microcontrollers, and the microcontrollers are sent data over serial from a SBC (currently the Libre Computer Le Potato) running a Python script.  

The goal is to eventually make the LEDs respond to events like opening doors, notifications on any of my devices, and music.

To make PlatformIO work properly, you need to open the `arduino-driver` folder in a separate VSCode window. The arduino driver is set up with a minimal Wokwi diagram and configuration to allow testing without the hardware; you just need to uncomment `#define WOKWI` in `main.cpp`.

I don't have a proper deploy system set up, but running the script `install.sh` under `controller` should install the necessary dependencies and set up a systemd service to run the Python script at boot. Maybe Nix would be appropriate for this project, but I'm honestly a bit tired of Nix after setting up my whole system with it for now haha.
# RoomLEDs

A project I'm currently working on to drive a strip of NeoPixels (WS2812B LEDs) mounted around the edge of my room. The strips are controlled by two ESP8266 microcontrollers, and the microcontrollers are sent data over serial from a SBC (currently the Libre Computer Le Potato) running a Python script.  

The goal is to eventually make the LEDs respond to events like opening doors, notifications on any of my devices, and music.

To make PlatformIO work properly, you need to open the `arduino-driver` folder in a separate VSCode window. Maybe I could fix this with a multi-root workspace, but I haven't tried yet. The arduino driver is set up with a minimal Wokwi diagram and configuration to allow testing without the hardware; you just need to uncomment `#define WOKWI` in `main.cpp`.
#define FASTLED_ESP8266_NODEMCU_PIN_ORDER
// 70% overclock
#define FASTLED_LED_OVERCLOCK 1.7
#include <FastLED.h>
#include <PacketSerial.h>

#include <stdint.h>

// The ID of this device; used for identification in the handshake.
// IDs are 0-indexed.
#define DEVICE_ID 0x01

// Debug mode; if enabled; the built-in LED is used to indicate status
// #define DEBUG

// Uncomment if using Wokwi simulator
// #define WOKWI

#ifdef WOKWI
  #define LED_PIN     25
  #define NUM_LEDS    48
  #define LED_BUILTIN 13
#else
  #define LED_PIN     5
  #define NUM_LEDS    406
#endif

// LED settings
// Note that this is a preprocessing layer, so anything lower than 255 will reduce the LED brightness range.
#define DEFAULT_BRIGHTNESS 255
#define LED_TYPE           WS2812B
#define COLOR_ORDER        GRB

// Only used for determining if we're hitting the target framerate;
// the actual framerate is based on the maximum speed we can send
// serial updates and update the LEDs.
#define TARGET_FPS  40

CRGB leds[NUM_LEDS];

static uint32_t lastUpdate = 0;

enum Command {
  COMMAND_INITIAL_HANDSHAKE = 'i', // Initial handshake; identify ourself and reset the LED strip
  COMMAND_SET_BRIGHTNESS = 'b', // Set the brightness
  COMMAND_SEND_FRAME = '<' // Send a frame
};
enum Response {
  RESPONSE_READY = 'r', // Ready to receive a frame
  RESPONSE_HANDSHAKE = 'i', // Handshake response
  RESPONSE_DEBUG = 'd' // Debug response
};

// Technically, 4608000 is the maximum supported baud rate, but it's unreliable in my experience
#define SERIAL_BAUD 1000000
// 10 bytes of extra space is arbitrary
#define PACKET_SERIAL_BUFFER_SIZE (NUM_LEDS * 3 + 10)
#define COBS_PACKET_BOUNDARY 0x00

// By default, PacketSerial automatically wraps the built-in `Serial` object.
// While it is still possible to use the Serial object directly, it is
// recommended that the user let the PacketSerial object manage all serial
// communication.
PacketSerial_<COBS, COBS_PACKET_BOUNDARY, PACKET_SERIAL_BUFFER_SIZE> packetSerial;

void onPacketReceived(const uint8_t* buffer, size_t size);

void setup() {
  packetSerial.begin(1000000);
  packetSerial.setPacketHandler(&onPacketReceived);

  FastLED.addLeds<LED_TYPE, LED_PIN, COLOR_ORDER>(leds, NUM_LEDS).setCorrection(TypicalLEDStrip);
  FastLED.setBrightness(DEFAULT_BRIGHTNESS);

  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH); // High is off for the built-in LED

  lastUpdate = micros();

  Serial.setTimeout(500);

  #ifdef DEBUG
    sendDebugResponse("Device ID: " + String(DEVICE_ID));
  #endif
}

void loop() {
  packetSerial.update();

  if(packetSerial.overflow()) {
    #ifdef DEBUG
      digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
      sendDebugResponse("Receive buffer overflowed");
    #endif
  }
}

void sendDebugResponse(String message) {
  uint8_t response[message.length() + 1];
  response[0] = RESPONSE_DEBUG;
  message.getBytes(response + 1, message.length() + 1);
  packetSerial.send(response, message.length() + 1);
}

void handle_frame(const uint8_t* buffer, size_t size) {
  // Copy the frame data to the LED buffer
  for(int i = 0; i < NUM_LEDS; i++) {
    leds[i] = CRGB(buffer[i * 3], buffer[i * 3 + 1], buffer[i * 3 + 2]);
  }

  // Show the frame
  FastLED.show();

  #ifdef DEBUG
    if(micros() - lastUpdate > 1000000 / TARGET_FPS) {
      // We aren't hitting the target framerate; blink the LED if debug mode is on
      digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));

      sendDebugResponse("Frame dropped; took " + String(micros() - lastUpdate) + " µs");
    }
  #endif
  lastUpdate = micros();

  uint8_t response[1] = {RESPONSE_READY};
  packetSerial.send(response, 1);
}

void handle_handshake(const uint8_t* buffer, size_t size) {
  uint8_t response[2] = {RESPONSE_HANDSHAKE, DEVICE_ID};
  packetSerial.send(response, 2);
  
  FastLED.clear();
  FastLED.setBrightness(DEFAULT_BRIGHTNESS);
  FastLED.show();
}

// When an encoded packet is received and decoded, it will be delivered here.
void onPacketReceived(const uint8_t* buffer, size_t size) {
  #ifdef DEBUG
    digitalWrite(LED_BUILTIN, HIGH);
  #endif

  int command = buffer[0];
  switch(command) {
    case COMMAND_INITIAL_HANDSHAKE:
      handle_handshake(buffer + 1, size - 1);
      break;
    
    case COMMAND_SET_BRIGHTNESS:
      FastLED.setBrightness(buffer[1]);
      break;
    
    case COMMAND_SEND_FRAME:
      handle_frame(buffer + 1, size - 1);
      break;
    
    default:
      // Unknown command; ignore it
      #ifdef DEBUG
        for(int i = 0; i < 5; i++) {
          digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
          delay(100);
        }
        delay(100);

        sendDebugResponse("Unknown command: " + String(command));
      #endif
      break;
  }
}
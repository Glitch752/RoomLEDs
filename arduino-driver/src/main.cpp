#define FASTLED_ESP8266_NODEMCU_PIN_ORDER
// 70% overclock
#define FASTLED_LED_OVERCLOCK 1.7
#include <FastLED.h>

#include <stdint.h>

#define DEVICE_ID 0x02 // The ID of this device; used for addressing

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
  INITIAL_HANDSHAKE = 'i', // Initial handshake; identify ourself and reset the LED strip
  SET_BRIGHTNESS = 'b', // Set the brightness
  SEND_FRAME = '<' // Send a frame
};

void setup() {
  Serial.begin(1000000);

  FastLED.addLeds<LED_TYPE, LED_PIN, COLOR_ORDER>(leds, NUM_LEDS).setCorrection(TypicalLEDStrip);
  FastLED.setBrightness(DEFAULT_BRIGHTNESS);

  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH); // High is off for the built-in LED

  lastUpdate = micros();

  Serial.setTimeout(500);
}

void loop() {
  // Clear the serial buffer and wait for data to stop coming in
  while(Serial.available() > 0) {
    Serial.read();
  }
  
  Serial.print('>'); // Indicate we're ready for the next command

  uint32_t start_time = millis();
  while(Serial.available() == 0) {
    yield(); // Allow other tasks to run
    wdt_reset(); // Reset the watchdog timer
    if(millis() - start_time > 250) {
      // We've been waiting for a frame for over a second; blink the LED and restart the loop
      #ifdef DEBUG
        digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
      #endif
      return;
    }
  }

  #ifdef DEBUG
    digitalWrite(LED_BUILTIN, HIGH);
  #endif

  int command = Serial.read();
  switch(command) {
    case INITIAL_HANDSHAKE:
      Serial.write(DEVICE_ID);
      FastLED.clear();
      FastLED.setBrightness(DEFAULT_BRIGHTNESS);
      FastLED.show();
      break;
    case SET_BRIGHTNESS:
      FastLED.setBrightness(Serial.read());
      break;
    case SEND_FRAME:
      // Read the frame
      Serial.readBytes((char*)leds, NUM_LEDS * 3);

      // Show the frame
      FastLED.show();

      #ifdef DEBUG
        if(micros() - lastUpdate > 1000000 / TARGET_FPS) {
          // We aren't hitting the target framerate; blink the LED if debug mode is on
          digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
        }
      #endif
      lastUpdate = micros();
      break;
    default:
      // Unknown command; ignore it
      #ifdef DEBUG
        for(int i = 0; i < 5; i++) {
          digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
          delay(100);
        }
        delay(100);
      #endif
      break;
  }
}
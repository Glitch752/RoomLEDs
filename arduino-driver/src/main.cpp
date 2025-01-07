#define FASTLED_ESP8266_NODEMCU_PIN_ORDER
// 70% overclock
#define FASTLED_LED_OVERCLOCK 1.7
#include <FastLED.h>

#include <stdint.h>

#define DEVICE_ID 0x01 // The ID of this device; used for addressing

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
#define DEFAULT_BRIGHTNESS 100
#define LED_TYPE           WS2812B
#define COLOR_ORDER        GRB

// Only used for determining if we're hitting the target framerate;
// the actual framerate is based on the maximum speed we can send
// serial updates and update the LEDs.
#define TARGET_FPS  40

CRGB leds[NUM_LEDS];

void wait_for_serial_connection() {
  uint32_t timeout_end = millis() + 2000;
  Serial.begin(1000000);
  while(!Serial && timeout_end > millis()) {}
}
static uint32_t lastUpdate = 0;

void setup() {
  wait_for_serial_connection(); // Optional, but seems to help Teensy out a lot.
  
  FastLED.addLeds<LED_TYPE, LED_PIN, COLOR_ORDER>(leds, NUM_LEDS).setCorrection(TypicalLEDStrip);
  FastLED.setBrightness(DEFAULT_BRIGHTNESS);

  pinMode(LED_BUILTIN, OUTPUT);
  lastUpdate = micros();

  Serial.setTimeout(200);
}

void loop() {
  Serial.print('>'); // Indicate we're ready for the next command

  uint32_t start_time = millis();
  while(!Serial.available()) {
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
    digitalWrite(LED_BUILTIN, false);
  #endif

  int command = Serial.read();
  switch(command) {
    case 'i': // Initial handshake; identify ourself and reset the LED strip
      Serial.write(DEVICE_ID);
      FastLED.clear();
      FastLED.setBrightness(DEFAULT_BRIGHTNESS);
      FastLED.show();
      break;
    case 'b': // Set the brightness
      FastLED.setBrightness(Serial.read());
      break;
    case '<': // Send a new frame
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
          delay(10);
        }
      #endif
      break;
  }
}
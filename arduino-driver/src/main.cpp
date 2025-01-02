#define FASTLED_ESP8266_NODEMCU_PIN_ORDER
// 70% overclock
#define FASTLED_LED_OVERCLOCK 1.7
// #define WOKWI

#include <FastLED.h>
#include <stdint.h>

#ifdef WOKWI
  #define LED_PIN     25
  #define NUM_LEDS    48
  #define LED_BUILTIN 13
#else
  #define LED_PIN     5
  #define NUM_LEDS    406
#endif

#define BRIGHTNESS  100
#define LED_TYPE    WS2812B
#define COLOR_ORDER GRB

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
  FastLED.setBrightness(BRIGHTNESS);

  pinMode(LED_BUILTIN, OUTPUT);
  lastUpdate = micros();

  Serial.setTimeout(200);
}

void loop() {
  // Read our next frame from the serial port
  Serial.print('>'); // Indicate we're ready for the next frame

  // Wait until we receive a '<' as an acknowledgement of an incoming frame
  uint32_t start_time = millis();
  while(!Serial.available() || Serial.read() != '<') {
    yield(); // Allow other tasks to run
    wdt_reset(); // Reset the watchdog timer
    if(millis() - start_time > 250) {
      // We've been waiting for a frame for over a second; blink the LED and restart the loop
      // TODO: Only keep on while waiting for a frame
      digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
      return;
    }
  }

  // Read the frame
  Serial.readBytes((char*)leds, NUM_LEDS * 3);

  // Show the frame
  FastLED.show();

  if(micros() - lastUpdate > 1000000 / TARGET_FPS) {
    // We aren't hitting the target framerate; blink the LED
    digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
    // Serial.println("Missed target framerate; update took " + String(micros() - lastUpdate) + "us");
  }
  lastUpdate = micros();
}
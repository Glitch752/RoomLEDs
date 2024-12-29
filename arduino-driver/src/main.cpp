#define FASTLED_ESP8266_NODEMCU_PIN_ORDER
// 50% overclock
#define FASTLED_LED_OVERCLOCK 1.0
// #define WOKWI

#include <FastLED.h>
#include <stdint.h>

#ifdef WOKWI
  #define LED_PIN     25
  #define NUM_LEDS    48
  #define LED_BUILTIN 13
#else
  #define LED_PIN     5
  #define NUM_LEDS    407
#endif

#define BRIGHTNESS  30
#define LED_TYPE    WS2812B
#define COLOR_ORDER GRB

#define TARGET_FPS  60

CRGB leds[NUM_LEDS];

void wait_for_serial_connection() {
  uint32_t timeout_end = millis() + 2000;
  Serial.begin(115200);
  while(!Serial && timeout_end > millis()) {}
}
static uint32_t lastUpdate = 0;

void setup() {
  wait_for_serial_connection(); // Optional, but seems to help Teensy out a lot.
  
  FastLED.addLeds<LED_TYPE, LED_PIN, COLOR_ORDER>(leds, NUM_LEDS).setCorrection(TypicalLEDStrip);
  FastLED.setBrightness(BRIGHTNESS);

  pinMode(LED_BUILTIN, OUTPUT);
  lastUpdate = millis();
}

void loop() {
  static uint8_t hue = 0;
  fill_rainbow(leds, NUM_LEDS, hue, 7);

  FastLED.show();
  if(millis() - lastUpdate > 1000 / TARGET_FPS) {
    // We aren't hitting the target framerate; blink the LED
    digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
    Serial.println("Missed target framerate; loop took " + String(millis() - lastUpdate) + "ms");
  } else {
    // We are hitting the target framerate; wait until the next frame
    delay(1000 / TARGET_FPS - (millis() - lastUpdate));
  }
  lastUpdate = millis();

  hue += 5;
}
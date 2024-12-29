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
  #define NUM_LEDS    407
#endif

#define BRIGHTNESS  30
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
}

void loop() {
  // Read our next frame from the serial port
  Serial.print('>'); // Indicate we're ready for the next frame
  // Wait until we receive a '<' as an acknowledgement of an incoming frame
  while(!Serial.available() || Serial.read() != '<') {}
  // Read the frame
  Serial.readBytes((char*)leds, NUM_LEDS * 3);

  // Show the frame
  FastLED.show();

  if(micros() - lastUpdate > 1000000 / TARGET_FPS) {
    // We aren't hitting the target framerate; blink the LED
    digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
    // Serial.println("Missed target framerate; update took " + String(micros() - lastUpdate) + "us");
  } else {
    // We are hitting the target framerate; wait until the next frame
    // Serial.println("Update took " + String(micros() - lastUpdate) + "us, leaving " + String(1000000 / TARGET_FPS - (micros() - lastUpdate)) + "us to wait");
    delayMicroseconds(1000000 / TARGET_FPS - (micros() - lastUpdate));
  }
  lastUpdate = micros();
}
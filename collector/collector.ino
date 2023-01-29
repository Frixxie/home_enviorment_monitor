#include <Adafruit_Sensor.h>
#include <DHT.h>
#include <DHT_U.h>
#include <WiFi.h>
#include <HTTPClient.h>
#include "credentials.h"

#define DHTPIN 22
#define DHTTYPE DHT11

const char* ssid = SSID;
const char* password = PASSWORD;
const char* url = URL;
const char* room = "room";

DHT dht(DHTPIN, DHTTYPE);
WiFiClient client;
HTTPClient http;

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  dht.begin();
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    blink_led(LED_BUILTIN, 2000);
  }
  Serial.begin(115200);
}

void blink_led(int pin, int delay_ms) {
  digitalWrite(pin, HIGH);
  delay(delay_ms);
  digitalWrite(pin, LOW);
  delay(delay_ms);
}

void loop() {
  Serial.println("Reading sensor...");
  float temp = dht.readTemperature();
  float humi = dht.readHumidity();
  while (isnan(temp) || isnan(humi)) {
    temp = dht.readTemperature();
    humi = dht.readHumidity();
    delay(500);
  }
  blink_led(LED_BUILTIN, 1000);

  http.begin(url);
  String payload = "{\"room\": \"mobile\", \"temp\": " + String(temp) + ", \"hum\": " + String(humi) + "}";
  http.addHeader("Content-Type", "application/json");
  int res = http.POST(payload);
  http.end();

  Serial.println("Response code: " + String(res));
  delay(60000);
}

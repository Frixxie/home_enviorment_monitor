#include <Adafruit_Sensor.h>
#include <DHT.h>
#include <DHT_U.h>
#include <WiFi.h>
#include "credentials.h"

#define DHTPIN 22
#define DHTTYPE DHT11

const char* ssid = SSID;
const char* password = PASSWORD;
const char* hostname = "192.168.0.166";
const uint  port = 65534;
const char* method = "POST";
const char* path = "/";


char digits[16];
DHT dht(DHTPIN, DHTTYPE);
WiFiClient client;


void setup() {
    pinMode(LED_BUILTIN, OUTPUT);
    dht.begin();
    WiFi.begin(ssid, password);
    while (WiFi.status() != WL_CONNECTED) {
        blink_led(LED_BUILTIN, 2000);
    }
}

void blink_led(int pin, int delay_ms) {
    digitalWrite(pin, HIGH);
    delay(delay_ms);
    digitalWrite(pin, LOW);
    delay(delay_ms);
}

void led_print(int led_gpio, int output) {
    int count = 0;
    while (output > 0) {
        digits[count] = output % 10;
        output /= 10;
        count++;
    }
    for (int i = count - 1; i >= 0; i--) {
        for (int j = 0; j < digits[i]; j++) {
            blink_led(led_gpio, 500);
        }
        delay(500);
    }
}

void loop() {
    float temp = dht.readTemperature();
    float humi = dht.readHumidity();
    while (isnan(temp) || isnan(humi)) {
        temp = dht.readTemperature();
        humi = dht.readHumidity();
        delay(1000);
    }
    blink_led(LED_BUILTIN, 1000);
    if (client.connect(hostname, port)) {
        String payload = "{\"room\": \"mobile\", \"temperature\": " + String(temp) + ", \"humidity\": " + String(humi) + "}";
        client.println(String(method) + " " + String(path) + " HTTP/1.1");
        client.println("Content-Type: application/json");
        client.println("Content-Length: " + String(payload.length()));
        client.println("Host: " + String(hostname));
        client.println("Connection: close");
        client.println("");

        client.println(payload);
        client.println("");

        if (!client.connected()) {
            client.stop();
        }
    }
    delay(60000);
}

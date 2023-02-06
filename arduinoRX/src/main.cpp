#include <Arduino.h>
#include <stdint.h>

unsigned long int counter = 0;

void setup() 
	{
	Serial.begin(9600);
	pinMode(LED_BUILTIN, OUTPUT);
	}

void loop() 
	{
	digitalWrite(LED_BUILTIN, HIGH);
	delay(250);
	digitalWrite(LED_BUILTIN, LOW);
	delay(250);
	Serial.println(String(counter));
	counter++;
	}
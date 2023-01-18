#include <Arduino.h>
#include <stdint.h>

unsigned long int counter = 0;

void setup() 
	{
	Serial.begin(9600);
	}

void loop() 
	{
	Serial.println(String(counter));
	counter++;
	delay(500);
	}
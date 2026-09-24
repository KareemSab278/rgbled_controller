#!/usr/bin/env python3
"""
Test script for WS2812B RGB LED strip on Raspberry Pi 4B.
Uses the official rpi_ws281x Python library.

Install on the Pi:
    sudo pip3 install rpi_ws281x

Run with sudo (root is required for GPIO/DMA access):
    sudo python3 python_test.py
"""

import time
from rpi_ws281x import PixelStrip, Color

# Configuration
LED_COUNT = 64          # Number of LEDs in the strip
GPIO_PIN = 10           # GPIO10 = SPI0 MOSI (physical pin 19)
LED_FREQ_HZ = 800000    # 800kHz for WS2812B
LED_DMA = 10            # DMA channel to use (try 5 or 10)
LED_BRIGHTNESS = 255    # 0-255
LED_INVERT = False


def color_wipe(strip, color, wait_ms=20):
    """Wipe color across display a pixel at a time."""
    for i in range(strip.numPixels()):
        strip.setPixelColor(i, color)
        strip.show()
        time.sleep(wait_ms / 1000.0)


def solid_color(strip, color, hold_s=2):
    """Set the entire strip to one color."""
    for i in range(strip.numPixels()):
        strip.setPixelColor(i, color)
    strip.show()
    time.sleep(hold_s)


if __name__ == "__main__":
    strip = PixelStrip(
        LED_COUNT,
        GPIO_PIN,
        LED_FREQ_HZ,
        LED_DMA,
        LED_INVERT,
        LED_BRIGHTNESS,
    )
    strip.begin()

    print("Clearing LEDs...")
    solid_color(strip, Color(0, 0, 0), 0.5)

    try:
        while True:
            print("Red")
            solid_color(strip, Color(255, 0, 0))

            print("Green")
            solid_color(strip, Color(0, 255, 0))

            print("Blue")
            solid_color(strip, Color(0, 0, 255))

            print("White")
            solid_color(strip, Color(255, 255, 255))

            print("Yellow")
            solid_color(strip, Color(255, 255, 0))

            print("Color wipe test")
            color_wipe(strip, Color(255, 0, 0))
            color_wipe(strip, Color(0, 255, 0))
            color_wipe(strip, Color(0, 0, 255))

    except KeyboardInterrupt:
        print("Exiting, clearing LEDs...")
        solid_color(strip, Color(0, 0, 0), 0.5)

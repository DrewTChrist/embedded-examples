import board
import displayio
import digitalio
import terminalio
import time
from adafruit_display_text import label, bitmap_label, wrap_text_to_pixels
from adafruit_st7735r import ST7735R
from adafruit_ahtx0 import AHTx0


# LCD Wiring
# --------------------
# VCC      -> 3.3v out
# CS      -> RX
# RST     -> 9
# A0 (DC) -> 6
# SDA     -> MOSI
# SCK     -> SCK
# LED     -> 5

# Initialize peripherals
i2c = board.I2C()
spi = board.SPI()

# LCD LED pin
led_pin = digitalio.DigitalInOut(board.D5)
led_pin.direction = digitalio.Direction.OUTPUT

# Initialize Sensor
sensor = AHTx0(i2c)

# Release resources and set up display protocol
displayio.release_displays()
display_bus = displayio.FourWire(spi, command=board.D6, chip_select=board.RX, reset=board.D9)

# Create display object
display = ST7735R(display_bus, rotation=90, width=160, height=128, bgr=True)

# Turn on the screen
led_pin.value = True


while True:
    text = f"Temperature: {sensor.temperature} C\nHumidity: {sensor.relative_humidity} %"
    text_area = label.Label(terminalio.FONT, text=text)
    text_area.x = 10
    text_area.y = 10
    display.show(text_area)
    time.sleep(0.1)

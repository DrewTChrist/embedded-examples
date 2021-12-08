import board
import displayio
import digitalio
import terminalio
import time
from adafruit_display_text import label
from adafruit_st7735r import ST7735R


# LCD Wiring
# --------------------
# VCC     -> 3.3v out
# CS      -> RX
# RST     -> 9
# A0 (DC) -> 6
# SDA     -> MOSI
# SCK     -> SCK
# LED     -> 5
# Button  -> 10

# Initialize peripherals
spi = board.SPI()

# LCD LED pin
led_pin = digitalio.DigitalInOut(board.D5)
led_pin.direction = digitalio.Direction.OUTPUT

# Button
button = digitalio.DigitalInOut(board.D10)
button.direction = digitalio.Direction.INPUT

# Release resources and set up display protocol
displayio.release_displays()
display_bus = displayio.FourWire(spi, command=board.D6, chip_select=board.RX, reset=board.D9)

# Create display object
display = ST7735R(display_bus, rotation=90, width=160, height=128, bgr=True)


# Turn on the screen
led_pin.value = True

def drive_display():
    text = "Hello World!"
    text_area = label.Label(terminalio.FONT, text=text)
    text_area.x = 10
    text_area.y = 10
    display.show(text_area)

while True:
    if button.value:
        led_pin.value = not led_pin.value
        
        # Prevents "user" (me) from holding the button
        # to repeatedly turn the screen on/off
        while button.value:
            pass
    
    # This check prevents us from wasting those precious cycles
    # in case we were logging data or something as well
    if led_pin.value:
        drive_display()
        
    time.sleep(0.1)





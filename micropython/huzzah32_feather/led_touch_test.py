import machine
import time
import neopixel

led = machine.Pin(13, machine.Pin.OUT)
touch = machine.TouchPad(machine.Pin(12))
touch.config(50)

blink = True

while True:
    if touch.read() < 700:
        blink = not blink
        time.sleep(0.25)
        
    if blink:
        led.on()
        time.sleep(0.25)
        led.off()
        time.sleep(0.25)


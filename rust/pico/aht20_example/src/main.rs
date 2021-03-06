//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use aht20::*;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::blocking::{
    delay::DelayMs,
    i2c::{Write, WriteRead},
};
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use embedded_time::rate::Extensions;
use panic_probe as _;
use rp2040_hal as hal;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();
    let sda_pin = pins.gpio18.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio19.into_mode::<hal::gpio::FunctionI2C>();
    // let not_an_scl_pin = pins.gpio20.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::i2c::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    let mut aht = Aht20::new(i2c, delay).unwrap();

    loop {
        //read_aht(&mut aht, &mut led_pin);
        led_pin.set_high().unwrap();
        let (h, t) = aht.read().unwrap();
        println!(
            "relative humidity={0}%; temperature={1}C",
            h.rh(),
            t.celsius()
        );
        aht.delay(500).unwrap();
        led_pin.set_low().unwrap();
        aht.delay(500).unwrap();
    }
}

/*fn read_aht<I2C, D, I, M>(aht: &Aht20<I2C, D>, led: &rp2040_hal::gpio::Pin<I, M>)
    where I2C: WriteRead<Error = E> + Write<Error = E>,
          D: DelayMs<u16>
          I: rp2040_hal::gpio::PinId,
          M: rp2040_hal::gpio::PinMode + rp2040_hal::gpio::ValidPinMode<I> {
    led.set_high().unwrap();
    let (h, t) = aht.read().unwrap();
    println!(
        "relative humidity={0}%; temperature={1}C",
        h.rh(),
        t.celsius()
    );
}

fn read_aht(aht: &Aht20<hal::i2c::I2C, cortex_m::delay::Delay>, led: &rp2040_hal::gpio::Pin<rp2040_hal::gpio::bank0::Gpio25, rp2040_hal::gpio::PushPullOutput>) {
    led.set_high().unwrap();
    let (h, t) = aht.read().unwrap();
    println!(
        "relative humidity={0}%; temperature={1}C",
        h.rh(),
        t.celsius()
    );
}*/

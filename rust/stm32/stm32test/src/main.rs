#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};
use stm32f3xx_hal::gpio::{Gpioa, U, Input};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut leds = [
        gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
    ];

    let mut button = gpioa.pa0.into_input(&mut gpioa.moder);
    let mut index = 0;
    let mut prev_index = index;

    loop {
      /*for led in leds.iter_mut() {
            led.toggle().unwrap();
            asm::delay(250_000);
            led.toggle().unwrap();
        }*/
        if button.is_high().unwrap() {
            prev_index = index;
            if index == leds.len() - 1 {
                index = 0;
            } else {
                index += 1;
            }
            leds[index].toggle().unwrap();
            asm::delay(1_000_000);
            while button.is_high().unwrap() {
            }
        }
    }
}

fn on_button_hold(button: &stm32f3xx_hal::gpio::Pin<Gpioa, U<0_u8>, Input>) {
    while button.is_high().unwrap() {
    }
}

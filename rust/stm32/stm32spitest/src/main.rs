#![no_std]
#![no_main]

use cortex_m::asm;
//use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use panic_semihosting as _;

use stm32f3xx_hal as hal;

use hal::prelude::*;

use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use hal::delay::Delay;
use hal::pac;
use hal::spi::Spi;
use st7735_lcd::Orientation;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let sck = gpioc
        .pc10
        .into_af6_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);
    let miso = gpioc
        .pc11
        .into_af6_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);
    let mosi = gpioc
        .pc12
        .into_af6_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);

    let rst = gpioc
        .pc9
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let dc = gpioc
        .pc8
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let mut button = gpioa.pa0.into_input(&mut gpioa.moder);
    
    let mut led = gpioc.pc7.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let spi: Spi<_, _> = Spi::new(dp.SPI3, (sck, miso, mosi), 10.MHz(), clocks, &mut rcc.apb1);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK);

    disp.set_offset(0, 25);

    // draw ferris
    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../../assets/ferris.raw"), 86);
    let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));
    image.draw(&mut disp).unwrap();
    led.toggle();

    loop {
        if button.is_high().unwrap() {
            led.toggle();
            while button.is_high().unwrap() {
            }
        }
    }
}

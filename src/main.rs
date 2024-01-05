#![no_std]
#![no_main]

use arduino_hal::prelude::_void_ResultVoidExt;

extern crate panic_halt;
extern crate arduino_hal;
extern crate embedded_hal_v0;

use embedded_hal_v0::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    ufmt::uwriteln!(&mut serial, "Hello World!\r").void_unwrap();

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
        led.toggle();
        arduino_hal::delay_ms(1000);
        ufmt::uwriteln!(&mut serial, "Lede dede dada\r").void_unwrap();
    }
}

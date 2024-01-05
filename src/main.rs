#![no_std]
#![no_main]

use arduino_hal::{prelude::_void_ResultVoidExt, hal::adc, adc::channel::Gnd};

extern crate panic_halt;
extern crate arduino_hal;
extern crate embedded_hal_v0;

use embedded_hal_v0::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );

    ufmt::uwriteln!(&mut serial, "Vbandgap: {}\r", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}\r", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature: {}\r", tmp).void_unwrap();

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();

    // Set pins for analog inputs
    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);
    let a3 = pins.a3.into_analog_input(&mut adc);
    let a4 = pins.a4.into_analog_input(&mut adc);
    let a5 = pins.a5.into_analog_input(&mut adc);

    led.set_high();

    loop {
        // led.toggle();
        // arduino_hal::delay_ms(1000);
        // led.toggle();
        // arduino_hal::delay_ms(1000);
        // ufmt::uwriteln!(&mut serial, "Lede dede dada\r").void_unwrap();
        let values = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
            a2.analog_read(&mut adc),
            a3.analog_read(&mut adc),
            a4.analog_read(&mut adc),
            a5.analog_read(&mut adc),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
        }

        if values[0] > 200 {
            ufmt::uwrite!(&mut serial, "Nothing there!").void_unwrap();
        }
        else {
            ufmt::uwrite!(&mut serial, "Something is there!").void_unwrap();
        }

        ufmt::uwriteln!(&mut serial, "\r").void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}

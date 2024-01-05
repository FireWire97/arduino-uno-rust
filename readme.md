arduino-uno-rust
===========
Foundation for Rust AVR projects, based on ['avr-hal'](https://github.com/Rahix/avr-hal) work.
===========
# Build

Install all required dependencies, and build using:

'''bash
cargo build
'''

In case the build fails, you may need to lock the Rust release for this workspace:

'''bash
rustup override set nightly
'''

# Flash

When build is completed, flash the board using 'flash.sh' script. Path to the binary must be specified as a first argument, in my case:

'''bash
./flash.sh /home/gucho/Documents/Projects/arduino-uno-rust/target/avr-atmega328p/debug/arduino-uno-rust.elf
'''
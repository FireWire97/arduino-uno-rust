# First argument should be
# /home/gucho/Documents/Projects/arduino-uno-rust/target/avr-atmega328p/debug/arduino-uno-rust.elf

set -e

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <path-to-binary.elf>" >&2
    exit 1
fi

if [ "$#" -lt 1 ]; then
    echo "$0: Expecting a .elf file" >&2
    exit 1
fi

# sudo -u creativcoder cargo build
avrdude -q -C/etc/avrdude.conf -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:$1:e"


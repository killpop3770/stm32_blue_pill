st-flash erase
cargo build --release
cargo objcopy --bin stm32_blue_pill --target thumbv7m-none-eabi --release -- -O binary stm32_blue_pill.bin
st-flash write stm32_blue_pill.bin 0x8000000
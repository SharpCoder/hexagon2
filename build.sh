#!/bin/sh
mkdir -p out/

rm -rf out/*.asm
rm -rf out/*.map
rm -rf out/*.lst

# Compile rust
rustc --target thumbv7em-none-eabihf -o out/kernel.o -O --emit=obj src/kernel.rs


# Compile assembly
arm-none-eabi-gcc \
    -O3 \
    -Wall \
    -Werror \
    -mcpu=cortex-m7 \
    -mthumb \
    -mfloat-abi=hard \
    -c src/teensy.c -o out/teensy.o

# Generate elf
arm-none-eabi-ld \
    -Map=out/kernel.map \
    -T src/linker.ld \
    out/kernel.o out/teensy.o -o out/kernel.elf

# Dump a bunch of debug stuff
arm-none-eabi-objdump -S out/kernel.elf > out/kern.asm
arm-none-eabi-objdump -d -S -C out/kernel.elf > out/kern.lst

# Final hex output
arm-none-eabi-objcopy -O ihex -R .eeprom out/kernel.elf out/kern.hex

# Cleanup
rm -rf out/*.elf
rm -rf out/*.o
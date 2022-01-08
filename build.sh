#!/bin/sh
mkdir -p out/

rm -rf out/*

echo Compiling rust...
# Compile rust
rustc --target thumbv7em-none-eabihf \
    -C panic=abort \
    --crate-type staticlib \
    -O --emit=link \
    -o out/kernel.o \
    -C opt-level=3 \ # This can't be 0 or 1 otherewise things don't work
    src/main.rs \


echo Compiling c...
# Compile c code which is vaguely used
# to setup and copy some memory around.
# This could actually be migrated to rust someday.
arm-none-eabi-gcc \
    -O3 \
    -Wall \
    -Werror \
    -mcpu=cortex-m7 \
    -mthumb \
    -mfloat-abi=hard \
    -c src/teensy.c -o out/teensy.o


echo Generating .elf...
# Generate the elf
arm-none-eabi-ld \
    -Map=out/kernel.map \
    -T src/linker.ld \
    -strip-all \
    --gc-sections \
    out/teensy.o \
    out/kernel.o \
    -o out/kernel.elf


# Dump a bunch of debug stuff
# Note: This actually takes a significant amount
# of time, so it's commented out. Include if you
# are debugging assembly-level optimizations.
if false;
then
    echo Generating debug content...
    arm-none-eabi-objdump -S out/teensy.o > out/teensy.asm
    arm-none-eabi-objdump -S out/kernel.o > out/kernel.asm
    arm-none-eabi-objdump -S out/kernel.elf > out/hex.asm
fi

# Final hex output
echo Generating .hex...
arm-none-eabi-objcopy -O ihex -R .eeprom out/kernel.elf out/kern.hex

# Cleanup
rm -rf out/*.elf
rm -rf out/*.o

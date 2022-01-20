#!/bin/sh
mkdir -p out/
rm -rf out/*.hex
rm -rf out/*.elf

# Download the linker file if it is not already present
if [ ! -f out/linker.ld ]:
then
    curl https://raw.githubusercontent.com/SharpCoder/teensycore/main/src/linker.ld > out/linker.ld
fi

./hotswap.sh 2

# Build with cargo
RUSTFLAGS="-C panic=abort -C opt-level=2 -C no-redzone" cargo build --target thumbv7em-none-eabihf

./hotswap.sh 1

# Extract all projects in the workspace
# and then build them into individual hex files
DIR=$(cargo metadata | jq '.target_directory' | tr -d '"')/thumbv7em-none-eabihf/debug
arr=$(ls $DIR/*.a)
for elf in "${arr}"
do :

    # Link the built file
    arm-none-eabi-ld \
        -T out/linker.ld \
        -strip-all \
        --gc-sections \
        --stats \
        $elf \
        -o out/kernel.elf

    # Extract the name, so the hex can have a pleasant file name
    lib=$(basename $elf .a)
    proj=$(echo $lib | sed 's/lib//g')

    arm-none-eabi-objdump -S out/kernel.elf > out/$proj.asm

    # Use objcopy to generate the hex output
    arm-none-eabi-objcopy -O ihex out/kernel.elf out/$proj.hex
done

# Remove artifacts
rm -rf out/kernel.elf

# #!/bin/sh
# mkdir -p out/

# rm -rf out/*

# # Hot-swap the wifi password so I don't accidentally check that in to source :P
# ./hotswap.sh 2

# echo Compiling rust...
# # Compile rust
# rustc --target thumbv7em-none-eabihf \
#     -C panic=abort \
#     --crate-type staticlib \
#     -O --emit=link \
#     -o out/kernel.o \
#     -C opt-level=3 \
#     -C no-redzone \
#     src/main.rs \


# echo Compiling c...
# # Compile c code which is vaguely used
# # to setup and copy some memory around.
# # This could actually be migrated to rust someday.
# arm-none-eabi-gcc \
#     -O3 \
#     -Wall \
#     -Werror \
#     -mcpu=cortex-m7 \
#     -mthumb \
#     -mfloat-abi=hard \
#     -c src/teensy.c -o out/teensy.o


# echo Generating .elf...
# # Generate the elf
# arm-none-eabi-ld \
#     -Map=out/kernel.map \
#     -T src/linker.ld \
#     -strip-all \
#     --gc-sections \
#     out/teensy.o \
#     out/kernel.o \
#     -o out/kernel.elf


# # Dump a bunch of debug stuff
# # Note: This actually takes a significant amount
# # of time, so it's commented out. Include if you
# # are debugging assembly-level optimizations.
# if false;
# then
#     echo Generating debug content...
#     arm-none-eabi-objdump -S out/teensy.o > out/teensy.asm
#     arm-none-eabi-objdump -S out/kernel.o > out/kernel.asm
#     arm-none-eabi-objdump -S out/kernel.elf > out/hex.asm
# fi

# # Final hex output
# echo Generating .hex...
# arm-none-eabi-objcopy -O ihex -R .eeprom out/kernel.elf out/kern.hex

# # Cleanup
# rm -rf out/*.elf
# rm -rf out/*.o

# # Hot-swap the wifi password back so I don't accidentally check that in to source :P
# ./hotswap.sh 1
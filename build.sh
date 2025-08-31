#!/bin/bash

set -e

DEBUG=${DEBUG:-0}
TARGET=loongarch64-unknown-none-softfloat
# TARGET=loongarch64-unknown-none

echo "building baremetal LoongArch64 application..."

# Check if LoongArch64 target is available
if ! rustup target list | grep -q "$TARGET"; then
    echo "Adding LoongArch64 target..."
    rustup target add $TARGET
fi

echo "building with cargo..."
cargo build --target $TARGET -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem

TARGET_FILE=target/$TARGET/debug/baremetal-loongarch64-test
OBJDUMP=loongarch64-unknown-linux-gnu-objdump
READELF=loongarch64-unknown-linux-gnu-readelf
OBJCOPY=loongarch64-unknown-linux-gnu-objcopy

echo "build completed successfully!"
file $TARGET_FILE

if [ $DEBUG -eq 1 ]; then
    echo "dumping debug info..."
    $OBJDUMP -d -S $TARGET_FILE > $TARGET_FILE.asm
    $READELF -wf $TARGET_FILE > $TARGET_FILE.eh_frame_hdr
    $READELF -a $TARGET_FILE > $TARGET_FILE.readelf
    $OBJCOPY -O binary $TARGET_FILE $TARGET_FILE.bin
    echo "done"
fi

# THEN build another target
TARGET2=loongarch64-unknown-none

if ! rustup target list | grep -q "$TARGET2"; then
    echo "Adding LoongArch64 target..."
    rustup target add $TARGET2
fi

echo "building with cargo..."
cargo build --target $TARGET2 -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem

TARGET_FILE2=target/$TARGET2/debug/baremetal-loongarch64-test

echo "build completed successfully!"
file $TARGET_FILE2

if [ $DEBUG -eq 1 ]; then
    echo "dumping debug info..."
    $OBJDUMP -d -S $TARGET_FILE2 > $TARGET_FILE2.asm
    $READELF -wf $TARGET_FILE2 > $TARGET_FILE2.eh_frame_hdr
    $READELF -a $TARGET_FILE2 > $TARGET_FILE2.readelf
    $OBJCOPY -O binary $TARGET_FILE2 $TARGET_FILE2.bin
    echo "done"
fi

# print a table of summary of the two targets
echo "summary of the two targets:"
echo "--------------------------------"
echo "target: $TARGET"
echo $(ls -l $TARGET_FILE)
echo $(file $TARGET_FILE)
echo "--------------------------------"
echo "target: $TARGET2"
echo $(ls -l $TARGET_FILE2)
echo $(file $TARGET_FILE2)
echo "--------------------------------"
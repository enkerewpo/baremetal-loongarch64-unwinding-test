#!/bin/bash

set -e

DEBUG=${DEBUG:-0}

echo "building baremetal LoongArch64 application..."

# Check if LoongArch64 target is available
if ! rustup target list | grep -q "loongarch64-unknown-none"; then
    echo "Adding LoongArch64 target..."
    rustup target add loongarch64-unknown-none
fi

echo "building with cargo..."
cargo build --target loongarch64-unknown-none -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem

TARGET_FILE=target/loongarch64-unknown-none/debug/baremetal-loongarch64-test
OBJDUMP=loongarch64-unknown-linux-gnu-objdump
READELF=loongarch64-unknown-linux-gnu-readelf
OBJCOPY=loongarch64-unknown-linux-gnu-objcopy

echo "build completed successfully!"
file $TARGET_FILE

if [ $DEBUG -eq 1 ]; then
    echo "dumping debug info..."
    $OBJDUMP -d -S $TARGET_FILE > $TARGET_FILE.asm
    $READELF -a $TARGET_FILE > $TARGET_FILE.readelf
    $READELF -wf $TARGET_FILE > $TARGET_FILE.eh_frame_hdr
    echo "done"
fi
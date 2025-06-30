#!/bin/bash

set -e

echo "Building baremetal LoongArch64 application..."

# Check if LoongArch64 target is available
if ! rustup target list | grep -q "loongarch64-unknown-none"; then
    echo "Adding LoongArch64 target..."
    rustup target add loongarch64-unknown-none
fi

# Build project
echo "Building with cargo..."
cargo build --target loongarch64-unknown-none -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem

TARGET_FILE=target/loongarch64-unknown-none/debug/baremetal-loongarch64-test
OBJDUMP=loongarch64-unknown-linux-gnu-objdump
READELF=loongarch64-unknown-linux-gnu-readelf
OBJCOPY=loongarch64-unknown-linux-gnu-objcopy

echo "Build completed successfully!"
file $TARGET_FILE
$OBJDUMP -d -S $TARGET_FILE > $TARGET_FILE.asm
$READELF -a $TARGET_FILE > $TARGET_FILE.readelf
$READELF -wf $TARGET_FILE > $TARGET_FILE.eh_frame_hdr
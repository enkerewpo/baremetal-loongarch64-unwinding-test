#!/bin/bash

set -e

ELF_FILE="target/loongarch64-unknown-none/debug/baremetal-loongarch64-test"

if [ ! -f "$ELF_FILE" ]; then
    echo "ELF file not found. Please build the project first:"
    echo "./build.sh"
    exit 1
fi

echo "Starting QEMU with LoongArch64 baremetal application..."
echo "ELF file: $ELF_FILE"
echo ""
echo "Press Ctrl+A, then press X to stop QEMU"
echo ""

# Start QEMU
qemu-system-loongarch64 \
    -M virt \
    -cpu la464 \
    -m 4G \
    -nographic \
    -serial mon:stdio \
    -kernel "$ELF_FILE"
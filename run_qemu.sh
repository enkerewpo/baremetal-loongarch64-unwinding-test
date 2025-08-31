#!/bin/bash

set -e

DEBUG=${DEBUG:-0}
# TARGET=loongarch64-unknown-none-softfloat
TARGET=loongarch64-unknown-none

ELF_FILE="target/$TARGET/debug/baremetal-loongarch64-test"

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

ARGS="-M virt -cpu la464 -m 4G -nographic -serial mon:stdio -kernel $ELF_FILE"

if [ $DEBUG -eq 1 ]; then
    ARGS="$ARGS -s -S"
fi

# Start QEMU
qemu-system-loongarch64 $ARGS
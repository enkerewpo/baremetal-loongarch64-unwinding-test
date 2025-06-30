#!/bin/bash

set -e

ELF_FILE="target/loongarch64-unknown-none/debug/baremetal-loongarch64-test"

if [ ! -f "$ELF_FILE" ]; then
    echo "ELF file not found. Please build the project first:"
    echo "./build.sh"
    exit 1
fi

GDB=loongarch64-unknown-linux-gnu-gdb
$GDB $ELF_FILE -ex "target remote localhost:1234"
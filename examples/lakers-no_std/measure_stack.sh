#!/bin/bash

# first, check it compiles
if ! cargo build --target=thumbv7em-none-eabihf 2>/dev/null; then
    echo "Compilation failed"
    exit 1
fi
echo "Compilation successful"

# flash and get the amount of bytes painted
echo "Flashing..."
cargo_output=$(cargo run --target=thumbv7em-none-eabihf 2>/dev/null)
# parse "INFO  __sheap: 0x20000440"
heap_start=$(echo $cargo_output | grep -o "__sheap: 0x[0-9a-f]\+" | awk '{print $2}')
# parse "INFO  _stack_start: 0x20001000"
stack_start=$(echo $cargo_output | grep -o "_stack_start: 0x[0-9a-f]\+" | awk '{print $2}')
printf "Heap starts at $heap_start, stack starts at $stack_start\n"

painted_bytes=$((stack_start - heap_start))
printf "Painted $painted_bytes bytes\n"

# divide by 4 to get the number of words
words=$((painted_bytes / 4))
printf "Painted $words words\n"

# get the amount of memory used
# example concrete command: probe-rs read b32 0x20000080 434 --chip nRF52840_xxAA | tr ' ' '\n' | grep deadbeef | wc -l | awk '{print $1*4}'
remaining=$(
    probe-rs read b32 $heap_start $words --chip nRF52840_xxAA | \
    tr ' ' '\n' | \
    grep deadbeef | \
    wc -l | \
    awk '{print $1*4}'
)
used=$((painted_bytes - remaining))
printf "Used $used bytes\n"

#!/usr/bin/env bash

chip="${1}"
target="xtensa-${chip}-none-elf"
profile=${2}
serial_port=$($(pwd)/detect_serial_port.sh ${3})

bootloader_offset=0x0000

if [[ "${chip}" = 'esp32' ]]; then
  bootloader_offset=0x1000
fi

esptool.py --chip "${chip}" --port "${serial_port}" --baud 115200 --before default_reset --after hard_reset write_flash \
  -z --flash_mode dio \
  --flash_freq 80m \
  --flash_size detect \
  "${bootloader_offset}" "target/${target}/esp-build/bootloader/bootloader.bin" \
  0x8000 "target/${target}/esp-build/partitions.bin" \
  0x10000 "target/${target}/${profile:-debug}/esp32-hello.bin"

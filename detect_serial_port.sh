#!/usr/bin/env bash

serial_port=${1}

if test -f "$serial_port"; then
  echo $serial_port
  exit
fi

serial_port="$(find /dev -name 'tty.usbserial-*' 2>/dev/null | head -n 1 || true)"

if [[ -z "${serial_port}" ]]; then
  serial_port="$(find /dev -name 'ttyUSB*' 2>/dev/null | head -n 1 || true)"
fi

echo $serial_port
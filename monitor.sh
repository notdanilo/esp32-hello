#!/usr/bin/env bash

serial_port=$($(pwd)/detect_serial_port.sh)

python3 -m serial.tools.miniterm --raw --exit-char=3 --rts=0 --dtr=0 $serial_port 115200

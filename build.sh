#!/usr/bin/env bash

set -euo pipefail

chip="${1:-esp32}"

serial_port=$($(pwd)/detect_serial_port.sh)

set -euo pipefail

target="xtensa-${chip}-none-elf"

profile=release

IDF_PATH="$(pwd)/esp-idf"
export IDF_PATH

IDF_TOOLS_PATH="$(pwd)/target/esp-idf-tools"
export IDF_TOOLS_PATH

mkdir -p "${IDF_TOOLS_PATH}"

cross build ${profile:+--${profile}} --target "${target}" -vv

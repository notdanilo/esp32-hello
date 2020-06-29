#!/usr/bin/env bash

chip=${1}
profile=${2}

if [[ -z "${chip}" ]]; then
  echo "Chip argument missing."
  echo "Try ./build.sh esp32 release"
  exit
fi

if [[ -z "${profile}" ]]; then
  echo "Profile argument missing."
  echo "Try ./build.sh esp32 release"
  exit
fi

serial_port=$($(pwd)/detect_serial_port.sh)

set -euo pipefail

target="xtensa-${chip}-none-elf"

IDF_PATH="$(pwd)/esp-idf"
export IDF_PATH

IDF_TOOLS_PATH="$(pwd)/target/esp-idf-tools"
export IDF_TOOLS_PATH

mkdir -p "${IDF_TOOLS_PATH}"

cross build ${profile:+--${profile}} --target "${target}" -vv

#!/usr/bin/env bash

#set -euo pipefail

if [[ -z ]]

chip=${1}
profile=${2}

if [[ -z "${chip}" ]]; then
  echo "Chip argument missing."
  echo "Try ./build.sh esp32 release"
fi

if [[ -z "${profile}" ]]; then
  echo "Chip argument missing."
  echo "Try ./build.sh esp32 release"
fi

echo $chip

exit
serial_port=$($(pwd)/detect_serial_port.sh)

set -euo pipefail

target="xtensa-${chip}-none-elf"

IDF_PATH="$(pwd)/esp-idf"
export IDF_PATH

IDF_TOOLS_PATH="$(pwd)/target/esp-idf-tools"
export IDF_TOOLS_PATH

mkdir -p "${IDF_TOOLS_PATH}"

cross build ${profile:+--${profile}} --target "${target}" -vv

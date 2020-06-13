# Dependencies

- pip install pyserial
- Docker
- [`cross`](https://github.com/rust-embedded/cross) with images
  from https://github.com/reitermarkus/cross/tree/xtensa:
  
  ```
  git clone -b xtensa https://github.com/reitermarkus/cross
  cd cross
  cargo install --path . --force
  ./build-docker-image.sh xtensa-esp32-none-elf
  ```
- [`esptool.py`](https://github.com/espressif/esptool)

# Cloning

`git clone --recursive https://github.com/chocol4te/esp32-hello/`

# Building

```
./build.sh
```

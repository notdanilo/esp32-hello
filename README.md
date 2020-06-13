# ESP32-HELLO

This is a project started by [chocol4te](https://github.com/chocol4te) and it was forked from [esp32-hello](https://github.com/chocol4te/esp32-hello). It's the best quickstart for esp32 development with Rust I have found.

This fork introduces:

* Improved documentation (with information I thought it was missing)
* Better build system with separated `build`, `deploy` and `monitor`ing commands
* TODO: Better project organization so it can be used as a template for esp32 development with Rust

# Notice

This repository was only tested on `Ubuntu 20.04 LTS`. I am also planning to test it with `esp8266` at some point but I suspect it isn't a straightfoward process.

# Dependencies

- `sudo apt install jq`
- Docker
- [`cross`](https://github.com/rust-embedded/cross) with images
  from https://github.com/reitermarkus/cross/tree/xtensa:
  
  ```
  git clone -b xtensa https://github.com/reitermarkus/cross
  cd cross
  cargo install --path . --force
  ./build-docker-image.sh xtensa-esp32-none-elf
  ```
- Optional: [`esptool.py`](https://github.com/espressif/esptool) for deploying with `./deploy.sh`
- Optional: `pyserial` for monitoring with `./monitor.sh`: `pip install pyserial`

# Build & Deploy

First of all, be sure to clone this repository recursively to initialize all its submodules with:
```
git clone --recursive https://github.com/notdanilo/esp32-hello/
```

or if you already have this repository cloned, you can initialize it with:

```bash
git submodule update --init --recursive
```

Now you are ready to build the project:

```bash
./build.sh esp32 release
```

To deploy it:

```bash
./deploy.sh esp32 release [optional:serial_port]
```

And monitor its serial messages with:

```bash
./monitor.sh [optional:serial_port]
```

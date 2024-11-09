# Rust Embedded Workshop

The Embedded track aims to serve as a first step in working with [**Embassy-rs**](https://embassy.dev/).

## What is Embassy-rs?

It is a full-fledged embedded framework for **Rust** embedded development. Besides the implementation of the [`embedded-hal`](https://docs.rs/embedded-hal/latest/embedded_hal/) for different **MCUs** (STM32, nRF52, nRF53 and nRF91, ESP32 and RP2040), embassy-rs provides several functions like timers, BLE and network communication.

## What microcontroller are we using?

For this workshop we will use the **Raspberry Pi Pico W**, which is a flexible, low-cost board. The MCU powering it is a **RP2040**, which is an **Arm Cortex M0+**.

## Prerequisites

### Rust Toolchain

In order to install the tools needed to compile Rust code, follow the next steps, depending on your operating system.

#### Linux/MacOS

Run the this command in terminal:

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This downloads and runs `rustup-init.sh`, which in turn downloads and runs the correct version of the `rustup-init` executable for your platform.

```shell
$ sudo apt-get install libudev-dev
```

#### Windows

Download the respective executable:

* [RUSTUP-INIT.exe - 64bit](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
* [RUSTUP-INIT.exe - 32bit](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)

You may be prompted to install [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). If so, follow the instructions from the previous link.

The last step is to run `rustup --version` in terminal. If everything went well, you should see an output similar to this:

```shell
rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.76.0 (07dca489a 2024-02-04)`
```

### Probe-rs

For actually flashing and running our applications on the Raspberry Pi Pico, we need to install the `probe-rs` tool: [Installation Guide](https://probe.rs/docs/getting-started/installation/#cargo-binstall).

## Tasks 

You will find the tasks in the `src` directory. The order is: `hello`, `color`, `smart_clock`. Each module will have the
task written in the top comment. Example for running the `hello` task: 

```shell
$ cargo run --release --bin hello 
```


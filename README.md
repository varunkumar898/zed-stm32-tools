# STM32 Tools for Zed Editor

[![CI](https://github.com/cherry/zed-stm32-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/cherry/zed-stm32-tools/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

A production-quality **STM32 embedded development environment for the Zed editor**, inspired by the workflow of **STM32CubeIDE**, but implemented natively using Zed's extension architecture, WebAssembly component runtime (`wasm32-wasip2`), Zed Tasks, Debug Adapter Protocol (DAP), and standard open-source embedded toolchains.

---

## Features

- **MCU & Device Database**: Built-in specifications, memory maps, ARM Cortex-M cores, and OpenOCD/probe-rs targets for STM32 families: **F0, F1, F2, F3, F4, F7, G0, G4, H5, H7, L0, L1, L4, L5, U5, WB, and WL**.
- **STM32CubeMX Workflow**: Seamless detection of `.ioc` project files, GUI launching, headless code generation, and strict preservation of `USER CODE BEGIN` / `USER CODE END` regions.
- **Language Intelligence (clangd)**: Automatic clangd orchestration with `--query-driver=/**/*arm-none-eabi*` ensuring proper indexing of CMSIS, HAL, LL, and FreeRTOS headers and macros without breaking existing C/C++ projects.
- **Multi-Build System Support**: Automatic detection and execution for **GNU Make**, **CMake**, and **Ninja**.
- **Post-Build Memory Analytics**: Automatically parses `arm-none-eabi-size` output and calculates exact Flash and RAM usage with percentage progress bars.
- **Multi-Backend Flashing**: Supports **STM32CubeProgrammer** (`STM32_Programmer_CLI`), **probe-rs**, and **OpenOCD** with ST-LINK auto-detection, flash-and-run, target reset, and safe confirmation-guarded erase.
- **Hardware Debugging (DAP)**: Native Zed Debug Adapter Protocol integration supporting **probe-rs** and **GDB + OpenOCD** with CMSIS-SVD register inspection and FreeRTOS thread awareness.
- **Toolchain Doctor**: Slash command (`/stm32-doctor`) and CLI script verifying GCC, GDB, CubeMX, CubeProgrammer, OpenOCD, probe-rs, clangd, and build tools.
- **Extensive Snippets**: Production snippets for HAL (GPIO, UART, SPI, I2C, TIM, ADC, DMA, EXTI), CMSIS registers/NVIC, and FreeRTOS tasks/queues/semaphores.

---

## Architecture Overview

```
Zed Editor
│
├── STM32 Zed Extension (Rust on wasm32-wasip2)
│   ├── Device Database (F0-WL specifications & memory maps)
│   ├── .ioc Project & Linker Script (.ld) Parsers
│   ├── Clangd Language Server Orchestrator (--query-driver)
│   ├── Slash Commands (/stm32-doctor, /stm32-info, /stm32-device)
│   └── DAP Scenario Generator (probe-rs & GDB)
│
├── Zed Tasks Engine (.zed/tasks.json)
│   ├── STM32: Build, Clean, Rebuild, Build Release, Build Debug
│   ├── STM32: Flash, Flash and Run, Reset, Erase
│   ├── STM32: Memory Usage Analyzer
│   └── STM32: Open CubeMX & Generate Code
│
├── Zed DAP Debugger (.zed/debug.json)
│   ├── probe-rs Embedded Debugger
│   ├── GDB + OpenOCD Server
│   └── CMSIS-SVD Register View
│
└── External Native Toolchain (User's System)
    ├── arm-none-eabi-gcc, gdb, objcopy, size
    ├── clangd
    ├── STM32CubeProgrammer CLI / STM32CubeMX
    └── OpenOCD / probe-rs
```

---

## Requirements & Toolchain Setup

The extension coordinates tools installed on your host machine.

Run the advisor script to inspect missing packages:
```bash
./scripts/install-dependencies.sh
```

### 1. ARM GNU Toolchain
- **Debian / Ubuntu**: `sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi gdb-multiarch`
- **Arch Linux**: `sudo pacman -S arm-none-eabi-gcc arm-none-eabi-binutils arm-none-eabi-gdb`
- **Fedora**: `sudo dnf install arm-none-eabi-gcc-cs arm-none-eabi-binutils-cs arm-none-eabi-gdb`
- **macOS**: `brew install arm-none-eabi-gcc`

### 2. Language Server (clangd)
- **Debian / Ubuntu**: `sudo apt install clangd`
- **Arch Linux**: `sudo pacman -S clang`
- **macOS**: `brew install llvm`

### 3. Build Systems
- Install `make`, `cmake`, and `ninja`.

### 4. Flashing & Debugging Tools (Pick one or both)
- **probe-rs** (Recommended for high speed):
  ```bash
  cargo install probe-rs-tools
  ```
- **OpenOCD**:
  ```bash
  sudo apt install openocd # or pacman -S openocd
  ```
- **STM32CubeProgrammer**:
  Download installer from [st.com/stm32cubeprog](https://www.st.com/en/development-tools/stm32cubeprog.html). Add `STM32_Programmer_CLI` to your `$PATH`.

### 5. STM32CubeMX
Download from [st.com/stm32cubemx](https://www.st.com/en/development-tools/stm32cubemx.html).

---

## Installing the Extension into Zed

1. Clone this repository:
   ```bash
   git clone https://github.com/cherry/zed-stm32-tools.git
   cd zed-stm32-tools
   ```
2. Build the WebAssembly extension module:
   ```bash
   rustup target add wasm32-wasip2
   cargo build --target wasm32-wasip2
   ```
3. In Zed, open the command palette (`Ctrl+Shift+P` / `Cmd+Shift+P`), type:
   ```text
   zed: extensions
   ```
4. Click **Install Dev Extension** and select this directory (`zed-stm32-tools`).

---

## Quickstart: Creating an STM32 Project

You can use the project templates located in `templates/`:
- `templates/bare-metal/`: Minimal register-level blinky with linker script and startup code.
- `templates/hal/`: STM32 HAL structure with clock configuration.
- `templates/freertos/`: FreeRTOS kernel integration with `FreeRTOSConfig.h`.
- `templates/cmake/`: Modern CMake build system with cross-compilation toolchain file.

Copy a template into your new project directory, or use STM32CubeMX to generate a project.

---

## STM32CubeMX Workflow

1. Open your STM32 project containing `firmware.ioc` in Zed.
2. In the Zed Command Palette (`Ctrl+Shift+P`), run:
   ```text
   task: spawn -> STM32: Open CubeMX
   ```
3. Configure pins, clocks, peripherals, and middleware in CubeMX.
4. Set **Project Manager -> Toolchain / IDE** to **Makefile** or **CMake**.
5. Click **GENERATE CODE** in CubeMX and switch back to Zed.
6. Write your application logic inside user code guards:
   ```c
   /* USER CODE BEGIN WHILE */
   HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_5);
   HAL_Delay(500);
   /* USER CODE END WHILE */
   ```
7. Re-running code generation will never overwrite code in `USER CODE` sections.

---

## Building, Flashing & Debugging

Open the command palette (`Ctrl+Shift+P` or `Cmd+Shift+P`) and choose `task: spawn`:

| Task Name | Description |
|:---|:---|
| **STM32: Build** | Compiles project using detected build tool (Make, CMake, Ninja) |
| **STM32: Clean** | Cleans build directory |
| **STM32: Rebuild** | Performs clean followed by full build |
| **STM32: Memory Usage** | Displays `.text`, `.data`, `.bss`, Flash & RAM usage with progress bars |
| **STM32: Flash and Run** | Programs MCU via ST-LINK (CubeProgrammer/probe-rs/OpenOCD) and starts execution |
| **STM32: Reset Target** | Hardware/core reset without reflashing |
| **STM32: Erase (Full Chip)** | Explicit mass erase of target flash memory |
| **STM32: Detect ST-LINK** | Lists connected ST-LINK probes, serial numbers, and voltages |
| **STM32: Toolchain Doctor** | Runs comprehensive environment check |

### Debugging with Zed DAP

1. Ensure `.zed/debug.json` is in your project root (sample provided in `debug/stm32-debug.json`).
2. Open the debug panel in Zed (`Ctrl+Shift+D`).
3. Select **STM32: Debug with probe-rs** or **STM32: Debug with GDB & OpenOCD**.
4. Set breakpoints, step over/into, inspect registers via CMSIS-SVD, and watch memory variables!

---

## Project Configuration (`.stm32/config.toml`)

Optional configuration file placed in your project root:

```toml
cubemx_path = "/opt/ST/STM32CubeMX/STM32CubeMX"
cubeprogrammer_path = "/usr/local/bin/STM32_Programmer_CLI"
openocd_path = "/usr/bin/openocd"
probe_rs_path = "/home/user/.cargo/bin/probe-rs"
svd_path = "./svd/STM32F401.svd"

programmer_interface = "SWD"
programmer_frequency_khz = 4000
debug_adapter = "probe-rs"
```

---

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

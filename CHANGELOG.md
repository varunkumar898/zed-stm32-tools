# Changelog

All notable changes to the `stm32-tools` extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-27

### Added
- Complete STM32 development extension architecture for Zed built with Rust and WebAssembly (`wasm32-wasip2`).
- **Device Database**: Built-in specifications, memory boundaries, ARM Cortex-M cores, and OpenOCD/probe-rs target configs covering STM32F0, F1, F2, F3, F4, F7, G0, G4, H5, H7, L0, L1, L4, L5, U5, WB, and WL families.
- **Toolchain Discovery & Doctor**: Diagnostics system detecting ARM GCC (`arm-none-eabi-gcc`), GDB (`arm-none-eabi-gdb` and `gdb-multiarch`), clangd, STM32CubeMX, STM32CubeProgrammer, OpenOCD, probe-rs, Make, CMake, and Ninja across Linux, macOS, and Windows.
- **Language Intelligence Integration**: Tailored clangd integration passing `--query-driver=/**/*arm-none-eabi*` for automatic CMSIS and HAL header and macro resolution.
- **Slash Commands**:
  - `/stm32-doctor`: Generates diagnostic report inside the Assistant panel.
  - `/stm32-info`: Inspects the active project for `.ioc`, linker scripts, FreeRTOS, and build systems.
  - `/stm32-device`: Queries the STM32 device database with auto-completion for part numbers.
- **STM32CubeMX Integration**:
  - Automatic `.ioc` project detection.
  - GUI and headless code generation scripts (`scripts/cubemx.sh`).
  - `USER CODE BEGIN` / `USER CODE END` preservation engine to prevent code overwrites.
- **Build System Support**:
  - Automatic detection for GNU Make, CMake, and Ninja.
  - Standardized Zed tasks for Build, Clean, Rebuild, Build Release, and Build Debug.
  - Post-build firmware memory usage analysis parsing `arm-none-eabi-size` with visual ASCII progress bars.
- **Flashing & Device Management**:
  - Support for STM32CubeProgrammer CLI (`STM32_Programmer_CLI`), OpenOCD, and probe-rs.
  - Commands for Probe Detection, Flash, Flash and Run, Target Reset, and Safe Full-Chip Erase.
- **Debugging & DAP Integration**:
  - Pre-configured debug adapters for probe-rs and GDB/OpenOCD.
  - Automatic CMSIS-SVD discovery for peripheral register inspection.
  - FreeRTOS awareness and thread inspection.
- **Project Scaffolding & Templates**:
  - Starters for Bare-Metal, HAL, LL, FreeRTOS, and CMake.
  - Scaffold generator outputting linker scripts, vector table startup code, and build files.
- **Comprehensive Snippet Library**:
  - HAL GPIO, UART, SPI, I2C, TIM, ADC, DMA, and EXTI callbacks.
  - CMSIS register macros, NVIC configuration, and DWT cycle counter.
  - FreeRTOS tasks, delays, queues, and semaphores.

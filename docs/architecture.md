# STM32 Tools Architecture for Zed

## Design Philosophy

The `stm32-tools` extension bridges the high-performance modern editing experience of the **Zed editor** with the deep embedded-systems workflow traditionally found in **STM32CubeIDE**.

Instead of bundling proprietary binaries, device-packs, or trying to emulate an entire operating system inside WebAssembly, the extension operates as an **orchestration engine**:

```
+-------------------------------------------------------------+
|                         Zed Editor                          |
|  +--------------------+  +-------------------------------+  |
|  | C/C++ Buffer       |  | Assistant Panel & Slash Cmds  |  |
|  | (clangd LSP)       |  | (/stm32-doctor, -device)      |  |
|  +--------------------+  +-------------------------------+  |
|  +--------------------+  +-------------------------------+  |
|  | Zed Tasks UI       |  | Zed DAP Debugger Engine       |  |
|  +--------------------+  +-------------------------------+  |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                 STM32 Zed Extension (Wasm)                  |
|  - .ioc CubeMX parser & project inspector                   |
|  - Linker script (.ld) memory region analyzer               |
|  - STM32 device specifications database                     |
|  - Compiler diagnostics & arm-size memory reporter          |
|  - DAP scenario generation & SVD discovery                  |
|  - Multi-backend programmer & task orchestrator             |
+-------------------------------------------------------------+
                               |
             +-----------------+-----------------+
             |                                   |
             v                                   v
+---------------------------+       +---------------------------+
|    Host Toolchain         |       |   Hardware Probes         |
| - arm-none-eabi-gcc / gdb |       | - ST-LINK V2 / V3         |
| - clangd with ARM query   |       | - J-Link / CMSIS-DAP      |
| - Make / CMake / Ninja    |       | - Target STM32 Board      |
| - STM32CubeProgrammer CLI |       |                           |
| - OpenOCD / probe-rs      |       |                           |
| - STM32CubeMX             |       |                           |
+---------------------------+       +---------------------------+
```

## Layer Separation

### 1. Language Intelligence Layer
- Rather than reimplementing a C parser in WebAssembly, the extension configures **clangd** with `--query-driver=/**/*arm-none-eabi*`.
- This ensures clangd automatically queries `arm-none-eabi-gcc` for target system include paths (`arm-none-eabi/include`, standard CMSIS headers) and target architectures (e.g. `__ARM_ARCH_7EM__`, `__thumb__`, hardware floating point flags).

### 2. Task & Build Layer
- Build, clean, flash, and analysis tasks are exposed as standard Zed tasks (`.zed/tasks.json`).
- Automatically senses whether the project is driven by `Makefile`, `CMakeLists.txt`, or `build.ninja`.
- Invokes `arm-none-eabi-size` post-build to display exact Flash and RAM usage with ASCII visual indicators.

### 3. Debug Adapter Protocol (DAP) Layer
- Natively connects to `probe-rs` or `arm-none-eabi-gdb` (with OpenOCD).
- Automatic CMSIS-SVD discovery maps peripheral registers (e.g. `RCC`, `GPIOA`, `TIM2`) directly to the debugger session.
- FreeRTOS awareness detects `FreeRTOSConfig.h` and activates thread/task inspection.

### 4. Code Generation & CubeMX Workflow
- Detects `.ioc` configuration files.
- Provides headless and GUI triggers for STM32CubeMX.
- Respects and preserves user code enclosed within `/* USER CODE BEGIN */` and `/* USER CODE END */`.

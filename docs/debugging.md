# STM32 Debugging Guide

Zed features native Debug Adapter Protocol (DAP) integration. This extension provides configurations for both **probe-rs** and **GDB + OpenOCD**.

## Debug Configuration (`.zed/debug.json`)

To debug your STM32 project, ensure `.zed/debug.json` is present in your project root:

### Option 1: probe-rs (Recommended)

`probe-rs` is a modern, ultra-fast embedded toolkit that directly talks to ST-LINK, J-Link, and CMSIS-DAP over USB:

```json
[
  {
    "label": "STM32: Debug with probe-rs",
    "adapter": "probe-rs",
    "request": "launch",
    "cwd": "$ZED_WORKTREE_ROOT",
    "program": "$ZED_WORKTREE_ROOT/build/firmware.elf",
    "chip": "STM32F401RETx",
    "connectUnderReset": true,
    "svdFile": "$ZED_WORKTREE_ROOT/svd/STM32F401.svd"
  }
]
```

### Option 2: GDB + OpenOCD

Traditional GNU GDB workflow connecting to OpenOCD's GDB server:

```json
[
  {
    "label": "STM32: Debug with GDB & OpenOCD",
    "adapter": "gdb",
    "request": "launch",
    "cwd": "$ZED_WORKTREE_ROOT",
    "program": "$ZED_WORKTREE_ROOT/build/firmware.elf",
    "server": "localhost:3333",
    "gdbpath": "arm-none-eabi-gdb",
    "openocdConfig": [
      "-f", "interface/stlink.cfg",
      "-f", "target/stm32f4x.cfg"
    ]
  }
]
```

## CMSIS-SVD Register View

CMSIS-SVD files describe all MCU peripheral registers down to the bitfield level (e.g. `GPIOA->ODR`, `RCC->CR`, `TIM2->CCR1`).

1. Place the SVD file in your project:
   ```text
   your-project/
   ├── svd/
   │   └── STM32F401.svd
   ```
2. The extension automatically detects `svd/*.svd` or matches the target device name from the device database.
3. During a debug session, Zed inspects registers using the SVD definitions.

## FreeRTOS Thread Awareness

When `FreeRTOSConfig.h` is detected in your project:
- For OpenOCD, append `-rtos auto` or `-rtos FreeRTOS` in target configuration.
- GDB will display all FreeRTOS task stacks and states alongside hardware registers.

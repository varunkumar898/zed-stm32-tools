# STM32CubeMX Workflow Guide

STM32CubeMX is the official STMicroelectronics graphical tool for pinout, clock tree, and peripheral initialization code generation.

## Supported Workflow

```
Open .ioc in Zed
       |
       v
STM32: Open CubeMX  (or ./scripts/cubemx.sh open)
       |
       v
Configure in CubeMX:
- Pinout & Alternate Functions
- RCC Clock Tree & PLLs
- Peripheral Driver Mode (HAL or LL)
- Middleware (FreeRTOS, FatFS, USB)
       |
       v
Generate Code (Target Toolchain: Makefile or CMake)
       |
       v
Return to Zed
       |
       v
Edit C Code (Protected USER CODE blocks)
       |
       v
STM32: Build -> STM32: Flash and Run -> STM32: Debug
```

## User Code Protection

CubeMX generates comment guards:
```c
/* USER CODE BEGIN 1 */
volatile uint32_t my_counter = 0;
/* USER CODE END 1 */
```

**Rule:** Always write your application logic between `/* USER CODE BEGIN */` and `/* USER CODE END */`.

The extension's code regeneration scripts and headless generation commands (`./scripts/cubemx.sh generate`) preserve your modifications across re-generation passes.

## Headless Code Generation

To regenerate code without opening the GUI:
```bash
./scripts/cubemx.sh generate firmware.ioc
```
Or execute the Zed task: `STM32: Generate Code (CubeMX)`.

# Toolchain & Diagnostics Guide

## Diagnostic Doctor

The extension provides a built-in diagnostics tool to verify every component of your STM32 setup.

### Running Doctor via Zed Assistant:
Open the Assistant panel in Zed and type:
```text
/stm32-doctor
```

### Running Doctor via Zed Tasks:
Press `Ctrl+Shift+P` -> `task: spawn` -> Select:
```text
STM32: Toolchain Doctor
```

### Running Doctor from CLI:
```bash
./scripts/doctor.sh
```

## Sample Doctor Report

```markdown
# STM32 Toolchain Diagnostics (Doctor)

### Compilers & Assemblers
* [✓] ARM GNU GCC Compiler (Required)
  - Path: `/usr/bin/arm-none-eabi-gcc`
  - Version: `arm-none-eabi-gcc (Arch Repository) 14.2.0`
* [✓] ARM GNU Size (Required)
  - Path: `/usr/bin/arm-none-eabi-size`
* [✓] ARM GNU Objcopy (Required)
  - Path: `/usr/bin/arm-none-eabi-objcopy`

### Debuggers & Adapters
* [✓] ARM GDB Debugger (Required)
  - Path: `/usr/bin/gdb-multiarch`
* [✓] OpenOCD (Open On-Chip Debugger) (Optional)
  - Path: `/usr/bin/openocd`
* [✓] probe-rs (Optional)
  - Path: `/home/cherry/.cargo/bin/probe-rs`

### Flashing & Programmers
* [✓] STM32CubeProgrammer CLI (Optional)
  - Path: `/usr/local/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI`

### Language Intelligence
* [✓] clangd Language Server (Required)
  - Path: `/usr/bin/clangd`

### Build Systems
* [✓] GNU Make (Optional)
  - Path: `/usr/bin/make`
* [✓] CMake (Optional)
  - Path: `/usr/bin/cmake`
* [✓] Ninja Build (Optional)
  - Path: `/usr/bin/ninja`

---
✓ Core STM32 development toolchain is ready.
```

## Configuration (`.stm32/config.toml`)

You can create an optional `.stm32/config.toml` in your project root to override executable paths or settings:

```toml
# Override custom tool locations if not in PATH
cubemx_path = "/opt/ST/STM32CubeMX/STM32CubeMX"
cubeprogrammer_path = "/opt/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI"
openocd_path = "/usr/local/bin/openocd"
probe_rs_path = "/home/user/.cargo/bin/probe-rs"
svd_path = "./svd/STM32F401.svd"

# Programmer interface settings
programmer_interface = "SWD"
programmer_frequency_khz = 4000
debug_adapter = "probe-rs"
```

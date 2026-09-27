# Troubleshooting Guide

## 1. ST-LINK Not Detected / Permission Denied

### Symptom:
`STM32_Programmer_CLI -l` reports `Error: No ST-Link detected` or `libusb open failed: LIBUSB_ERROR_ACCESS`.

### Solution:
Install udev rules for ST-LINK probes:
```bash
sudo curl -fsSL https://raw.githubusercontent.com/stlink-org/stlink/develop/config/udev/rules.d/49-stlinkv2.rules -o /etc/udev/rules.d/49-stlinkv2.rules
sudo curl -fsSL https://raw.githubusercontent.com/stlink-org/stlink/develop/config/udev/rules.d/49-stlinkv2-1.rules -o /etc/udev/rules.d/49-stlinkv2-1.rules
sudo curl -fsSL https://raw.githubusercontent.com/stlink-org/stlink/develop/config/udev/rules.d/49-stlinkv3.rules -o /etc/udev/rules.d/49-stlinkv3.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```
Unplug and reconnect the ST-LINK USB cable.

## 2. clangd Missing ARM Headers or Macros

### Symptom:
clangd flags `__IO` or `uint32_t` or says `cannot find stdint.h`.

### Solution:
Ensure `arm-none-eabi-gcc` is installed. The extension configures clangd with:
```text
--query-driver=/**/*arm-none-eabi*
```
For Make-based projects, generate `compile_commands.json` using `bear`:
```bash
bear -- make
```
For CMake, add to your `CMakeLists.txt`:
```cmake
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
```

## 3. Target Locked or in Low Power Mode (Sleep/Stop/Standby)

### Symptom:
Flash programming fails with `Error: Cannot connect to target`.

### Solution:
Use Connect Under Reset:
```bash
STM32_Programmer_CLI -c port=SWD mode=UR -w build/firmware.bin 0x08000000 -v -rst
```
Or for OpenOCD:
```bash
openocd -f interface/stlink.cfg -c "reset_config srst_only srst_nogate" -f target/stm32f4x.cfg
```

## 4. GDB Version Mismatch

### Symptom:
`arm-none-eabi-gdb` is missing on Ubuntu 24.04+ or modern distributions.

### Solution:
Modern Debian/Ubuntu package multi-architecture GDB as `gdb-multiarch`:
```bash
sudo apt install gdb-multiarch
```
The extension automatically detects `gdb-multiarch` as a drop-in replacement.

# Installation Guide

## 1. Prerequisites

Before installing the extension, ensure you have:
1. **Zed Editor** (latest version with extension support).
2. **Rust & Cargo** (for building dev extensions via `wasm32-wasip2`).
   ```bash
   rustup target add wasm32-wasip2
   ```

## 2. Installing the Extension in Zed

### Method A: Install as Dev Extension (Local Development)
1. Clone or download this repository:
   ```bash
   git clone https://github.com/cherry/zed-stm32-tools.git
   cd zed-stm32-tools
   ```
2. Build the WebAssembly extension component:
   ```bash
   cargo build --target wasm32-wasip2
   ```
3. Open Zed.
4. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows) and run:
   ```text
   zed: extensions
   ```
5. Click **Install Dev Extension** at the top right of the Extensions panel.
6. Select this repository directory (`zed-stm32-tools`).

## 3. Host Dependencies

Run the dependency advisor script to see what packages are needed on your system:
```bash
./scripts/install-dependencies.sh
```

### Ubuntu / Debian:
```bash
sudo apt update
sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi gdb-multiarch openocd clangd make cmake ninja-build
```

### Arch Linux:
```bash
sudo pacman -S arm-none-eabi-gcc arm-none-eabi-binutils arm-none-eabi-gdb openocd clang make cmake ninja
```

### Fedora:
```bash
sudo dnf install arm-none-eabi-gcc-cs arm-none-eabi-binutils-cs arm-none-eabi-gdb openocd clang-tools-extra make cmake ninja-build
```

### Optional Modern Tooling:
- **probe-rs** (High-speed embedded debugger & flasher):
  ```bash
  cargo install probe-rs-tools
  ```
- **STM32CubeProgrammer**:
  Download from [STMicroelectronics](https://www.st.com/en/development-tools/stm32cubeprog.html).
- **STM32CubeMX**:
  Download from [STMicroelectronics](https://www.st.com/en/development-tools/stm32cubemx.html).

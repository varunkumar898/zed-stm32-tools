#!/usr/bin/env bash
# STM32 Dependencies Advisor
# In accordance with STM32 development guidelines:
# - Does NOT execute package-manager commands without explicit user approval.
# - Does NOT attempt automated installation of proprietary STM32 software.
# - Provides exact, copy-paste commands tailored to your Linux distribution.
set -euo pipefail

echo "=========================================================="
echo "          STM32 Development Dependency Advisor           "
echo "=========================================================="
echo ""

# Detect Linux Distribution
DISTRO="unknown"
if [ -f /etc/os-release ]; then
    # shellcheck source=/dev/null
    . /etc/os-release
    DISTRO="$ID"
fi

echo "Detected OS / Distribution: $DISTRO"
echo ""

MISSING_CORE=()
MISSING_DEBUG=()
MISSING_PROPRIETARY=()

# 1. ARM GCC
if ! command -v arm-none-eabi-gcc >/dev/null 2>&1; then
    MISSING_CORE+=("arm-none-eabi-gcc")
fi

# 2. Clangd
if ! command -v clangd >/dev/null 2>&1; then
    MISSING_CORE+=("clangd")
fi

# 3. Build tools
if ! command -v make >/dev/null 2>&1; then
    MISSING_CORE+=("make")
fi
if ! command -v cmake >/dev/null 2>&1; then
    MISSING_CORE+=("cmake")
fi
if ! command -v ninja >/dev/null 2>&1; then
    MISSING_CORE+=("ninja")
fi

# 4. GDB
if ! command -v arm-none-eabi-gdb >/dev/null 2>&1 && ! command -v gdb-multiarch >/dev/null 2>&1; then
    MISSING_DEBUG+=("gdb-multiarch / arm-none-eabi-gdb")
fi

# 5. OpenOCD
if ! command -v openocd >/dev/null 2>&1; then
    MISSING_DEBUG+=("openocd")
fi

# 6. probe-rs
if ! command -v probe-rs >/dev/null 2>&1; then
    MISSING_DEBUG+=("probe-rs")
fi

# 7. STM32CubeProgrammer
if ! command -v STM32_Programmer_CLI >/dev/null 2>&1; then
    MISSING_PROPRIETARY+=("STM32CubeProgrammer (STM32_Programmer_CLI)")
fi

# 8. STM32CubeMX
if ! command -v STM32CubeMX >/dev/null 2>&1; then
    MISSING_PROPRIETARY+=("STM32CubeMX")
fi

if [ ${#MISSING_CORE[@]} -eq 0 ] && [ ${#MISSING_DEBUG[@]} -eq 0 ] && [ ${#MISSING_PROPRIETARY[@]} -eq 0 ]; then
    echo "✓ All recommended STM32 development tools are present on your system!"
    exit 0
fi

echo "The following packages are recommended for your development environment:"
echo ""

if [ ${#MISSING_CORE[@]} -gt 0 ]; then
    echo "--- Open-Source Core Toolchain & Languages ---"
    for item in "${MISSING_CORE[@]}"; do
        echo "  • $item"
    done
    echo ""
    echo "Recommended installation command:"
    case "$DISTRO" in
        ubuntu|debian|pop|linuxmint)
            echo "  sudo apt update && sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi clangd make cmake ninja-build"
            ;;
        arch|manjaro|endeavouros)
            echo "  sudo pacman -S arm-none-eabi-gcc arm-none-eabi-binutils clang make cmake ninja"
            ;;
        fedora|rhel|centos)
            echo "  sudo dnf install arm-none-eabi-gcc-cs arm-none-eabi-binutils-cs clang-tools-extra make cmake ninja-build"
            ;;
        *)
            echo "  Install arm-none-eabi-gcc, clangd, make, cmake, and ninja via your system package manager."
            ;;
    esac
    echo ""
fi

if [ ${#MISSING_DEBUG[@]} -gt 0 ]; then
    echo "--- Debugging & Probe Tools ---"
    for item in "${MISSING_DEBUG[@]}"; do
        echo "  • $item"
    done
    echo ""
    echo "Recommended installation commands:"
    case "$DISTRO" in
        ubuntu|debian|pop|linuxmint)
            echo "  sudo apt install gdb-multiarch openocd"
            ;;
        arch|manjaro|endeavouros)
            echo "  sudo pacman -S arm-none-eabi-gdb openocd"
            ;;
        fedora|rhel|centos)
            echo "  sudo dnf install arm-none-eabi-gdb openocd"
            ;;
        *)
            echo "  Install gdb-multiarch and openocd via your package manager."
            ;;
    esac
    echo "  probe-rs (optional, high-speed Rust/C debugger):"
    echo "    cargo install probe-rs-tools"
    echo ""
fi

if [ ${#MISSING_PROPRIETARY[@]} -gt 0 ]; then
    echo "--- STMicroelectronics Official Tools (Optional / Vendor) ---"
    for item in "${MISSING_PROPRIETARY[@]}"; do
        echo "  • $item"
    done
    echo ""
    echo "Download from the official STMicroelectronics website:"
    echo "  • STM32CubeProgrammer: https://www.st.com/en/development-tools/stm32cubeprog.html"
    echo "  • STM32CubeMX:         https://www.st.com/en/development-tools/stm32cubemx.html"
    echo "After installation, ensure the binaries are added to PATH or configured in .stm32/config.toml."
    echo ""
fi

echo "=========================================================="
echo "Note: No system packages were modified by running this script."

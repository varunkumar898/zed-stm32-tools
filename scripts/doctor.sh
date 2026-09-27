#!/usr/bin/env bash
# STM32 Toolchain Diagnostics & Health Check Script
set -euo pipefail

echo "=========================================================="
echo "          STM32 Toolchain & Environment Diagnostics       "
echo "=========================================================="
echo ""

check_tool() {
    local name="$1"
    local binary="$2"
    local required="$3"
    local hint="$4"

    if command -v "$binary" >/dev/null 2>&1; then
        local path
        path=$(command -v "$binary")
        local version
        version=$("$binary" --version 2>&1 | head -n 1 || echo "unknown")
        echo -e "[✓] \033[1;32m$name\033[0m: found"
        echo "    Path:    $path"
        echo "    Version: $version"
    else
        if [ "$required" = "true" ]; then
            echo -e "[✗] \033[1;31m$name (REQUIRED)\033[0m: NOT FOUND"
            echo "    Hint:    $hint"
        else
            echo -e "[-] \033[1;33m$name (Optional)\033[0m: not found"
            echo "    Hint:    $hint"
        fi
    fi
    echo ""
}

echo "--- 1. Compilers & Assemblers ---"
check_tool "ARM GNU GCC" "arm-none-eabi-gcc" "true" "Install via package manager: apt install gcc-arm-none-eabi / pacman -S arm-none-eabi-gcc"
check_tool "ARM GNU G++" "arm-none-eabi-g++" "false" "Install via package manager: apt install g++-arm-none-eabi"
check_tool "ARM GNU Size" "arm-none-eabi-size" "true" "Part of arm-none-eabi binutils"
check_tool "ARM GNU Objcopy" "arm-none-eabi-objcopy" "true" "Part of arm-none-eabi binutils"

echo "--- 2. Debuggers ---"
if command -v arm-none-eabi-gdb >/dev/null 2>&1; then
    check_tool "ARM GDB" "arm-none-eabi-gdb" "true" ""
elif command -v gdb-multiarch >/dev/null 2>&1; then
    check_tool "GDB Multiarch" "gdb-multiarch" "true" ""
else
    check_tool "ARM GDB" "arm-none-eabi-gdb" "true" "Install arm-none-eabi-gdb or gdb-multiarch"
fi

check_tool "OpenOCD" "openocd" "false" "Install via package manager: apt install openocd / pacman -S openocd"
check_tool "probe-rs" "probe-rs" "false" "Install via: cargo install probe-rs-tools"

echo "--- 3. STMicroelectronics Tools ---"
CUBEMX_CANDIDATE=""
for p in /opt/ST/STM32CubeMX/STM32CubeMX /opt/st/stm32cubemx/STM32CubeMX "$HOME/STM32CubeMX/STM32CubeMX" /usr/local/bin/stm32cubemx; do
    if [ -x "$p" ]; then
        CUBEMX_CANDIDATE="$p"
        break
    fi
done

if command -v STM32CubeMX >/dev/null 2>&1; then
    check_tool "STM32CubeMX" "STM32CubeMX" "false" ""
elif [ -n "$CUBEMX_CANDIDATE" ]; then
    echo -e "[✓] \033[1;32mSTM32CubeMX\033[0m: found at standard location"
    echo "    Path:    $CUBEMX_CANDIDATE"
    echo ""
else
    echo -e "[-] \033[1;33mSTM32CubeMX (Optional)\033[0m: not found"
    echo "    Hint:    Download from st.com/stm32cubemx or set stm32.cubemx.path"
    echo ""
fi

CUBEPROG_CANDIDATE=""
for p in /usr/local/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI \
         /opt/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI \
         /opt/st/stm32cubeprogrammer/bin/STM32_Programmer_CLI; do
    if [ -x "$p" ]; then
        CUBEPROG_CANDIDATE="$p"
        break
    fi
done

if command -v STM32_Programmer_CLI >/dev/null 2>&1; then
    check_tool "STM32CubeProgrammer CLI" "STM32_Programmer_CLI" "false" ""
elif [ -n "$CUBEPROG_CANDIDATE" ]; then
    echo -e "[✓] \033[1;32mSTM32CubeProgrammer CLI\033[0m: found at standard location"
    echo "    Path:    $CUBEPROG_CANDIDATE"
    echo ""
else
    echo -e "[-] \033[1;33mSTM32CubeProgrammer CLI (Optional)\033[0m: not found"
    echo "    Hint:    Download from st.com/stm32cubeprog or set stm32.cubeprogrammer.path"
    echo ""
fi

echo "--- 4. Language Intelligence & Build Tools ---"
check_tool "clangd Language Server" "clangd" "true" "Install via: apt install clangd / pacman -S clang"
check_tool "GNU Make" "make" "false" "Install via package manager: apt install make / pacman -S make"
check_tool "CMake" "cmake" "false" "Install via package manager: apt install cmake / pacman -S cmake"
check_tool "Ninja" "ninja" "false" "Install via package manager: apt install ninja-build / pacman -S ninja"

echo "=========================================================="
echo "Diagnostics complete."

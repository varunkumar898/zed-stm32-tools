#!/usr/bin/env bash
# STM32 Intelligent Build and Memory Usage Script
set -euo pipefail

ACTION="${1:-build}"
PARAM="${2:-}"

find_elf() {
    if [ -n "$PARAM" ] && [ -f "$PARAM" ]; then
        echo "$PARAM"
        return
    fi
    # Search common build locations
    for candidate in build/*.elf *.elf Debug/*.elf Release/*.elf; do
        if [ -f "$candidate" ]; then
            echo "$candidate"
            return
        fi
    done
    echo ""
}

show_size() {
    local elf="$1"
    if [ -z "$elf" ] || [ ! -f "$elf" ]; then
        echo "No ELF firmware binary found to analyze."
        return
    fi

    echo ""
    echo "=========================================================="
    echo "               Firmware Memory Analysis                   "
    echo "=========================================================="
    echo "Binary: $elf"
    echo ""
    if command -v arm-none-eabi-size >/dev/null 2>&1; then
        arm-none-eabi-size "$elf"
    else
        echo "arm-none-eabi-size not found."
    fi
    echo "=========================================================="
}

case "$ACTION" in
    build)
        echo "==> Building STM32 Project..."
        if [ -f "CMakeLists.txt" ]; then
            echo "Detected CMake build system."
            cmake -S . -B build
            cmake --build build
        elif [ -f "build.ninja" ]; then
            echo "Detected Ninja build system."
            ninja -C build
        elif [ -f "Makefile" ]; then
            echo "Detected GNU Make build system."
            make -j"$(nproc 2>/dev/null || echo 2)"
        else
            echo "Error: No recognized build system found (CMakeLists.txt, build.ninja, or Makefile)."
            exit 1
        fi

        ELF_FILE=$(find_elf)
        if [ -n "$ELF_FILE" ]; then
            show_size "$ELF_FILE"
        fi
        ;;

    clean)
        echo "==> Cleaning STM32 Project..."
        if [ -f "CMakeLists.txt" ]; then
            if [ -d "build" ]; then
                cmake --build build --target clean || rm -rf build
            fi
        elif [ -f "build.ninja" ]; then
            ninja -C build -t clean
        elif [ -f "Makefile" ]; then
            make clean
        fi
        echo "Clean completed."
        ;;

    size)
        ELF_FILE=$(find_elf)
        show_size "$ELF_FILE"
        ;;

    *)
        echo "Usage: $0 {build|clean|size} [optional_path_to_elf]"
        exit 1
        ;;
esac

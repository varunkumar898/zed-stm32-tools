#!/usr/bin/env bash
# STM32CubeMX Automation Script
set -euo pipefail

ACTION="${1:-open}"
IOC_FILE="${2:-}"

# Find .ioc file if not specified
if [ -z "$IOC_FILE" ]; then
    for candidate in *.ioc firmware.ioc project.ioc; do
        if [ -f "$candidate" ]; then
            IOC_FILE="$candidate"
            break
        fi
    done
fi

# Locate CubeMX executable
CUBEMX_BIN=""
if command -v STM32CubeMX >/dev/null 2>&1; then
    CUBEMX_BIN=$(command -v STM32CubeMX)
else
    for candidate in /opt/ST/STM32CubeMX/STM32CubeMX \
                     /opt/st/stm32cubemx/STM32CubeMX \
                     "$HOME/STM32CubeMX/STM32CubeMX" \
                     /usr/local/bin/stm32cubemx; do
        if [ -x "$candidate" ]; then
            CUBEMX_BIN="$candidate"
            break
        fi
    done
fi

if [ -z "$CUBEMX_BIN" ]; then
    echo "Error: STM32CubeMX executable was not found."
    echo "Please download it from STMicroelectronics (st.com/stm32cubemx) or configure stm32.cubemx.path"
    exit 1
fi

case "$ACTION" in
    open)
        if [ -n "$IOC_FILE" ] && [ -f "$IOC_FILE" ]; then
            echo "==> Opening STM32CubeMX with $IOC_FILE..."
            "$CUBEMX_BIN" "$IOC_FILE" &
        else
            echo "==> Launching STM32CubeMX..."
            "$CUBEMX_BIN" &
        fi
        ;;

    generate)
        if [ -z "$IOC_FILE" ] || [ ! -f "$IOC_FILE" ]; then
            echo "Error: No .ioc file specified or found."
            exit 1
        fi
        echo "==> Triggering headless code generation for $IOC_FILE..."
        SCRIPT_TMP=$(mktemp /tmp/cubemx_script_XXXXXX.scr)
        echo "config load $IOC_FILE" > "$SCRIPT_TMP"
        echo "project generate" >> "$SCRIPT_TMP"
        echo "exit" >> "$SCRIPT_TMP"

        "$CUBEMX_BIN" -q "$SCRIPT_TMP"
        rm -f "$SCRIPT_TMP"
        echo "==> Code generation complete."
        ;;

    *)
        echo "Usage: $0 {open|generate} [path_to_ioc]"
        exit 1
        ;;
esac

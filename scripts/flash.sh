#!/usr/bin/env bash
# STM32 Flashing and Device Management Script
# Backends: STM32CubeProgrammer (default), OpenOCD, probe-rs
set -euo pipefail

ACTION="${1:-flash-run}"
FIRMWARE="${2:-}"

# Locate STM32_Programmer_CLI if not in PATH
if ! command -v STM32_Programmer_CLI >/dev/null 2>&1; then
    for candidate in /usr/local/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI \
                     /opt/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI \
                     /opt/st/stm32cubeprogrammer/bin/STM32_Programmer_CLI; do
        if [ -x "$candidate" ]; then
            export PATH="$(dirname "$candidate"):$PATH"
            break
        fi
    done
fi

find_firmware() {
    if [ -n "$FIRMWARE" ] && [ -f "$FIRMWARE" ]; then
        echo "$FIRMWARE"
        return
    fi
    for candidate in build/*.bin build/*.elf *.bin *.elf; do
        if [ -f "$candidate" ]; then
            echo "$candidate"
            return
        fi
    done
    echo ""
}

detect_backend() {
    if [ -n "${STM32_BACKEND:-}" ]; then
        echo "$STM32_BACKEND"
    elif command -v STM32_Programmer_CLI >/dev/null 2>&1; then
        echo "cubeprog"
    elif command -v probe-rs >/dev/null 2>&1; then
        echo "probe-rs"
    elif command -v openocd >/dev/null 2>&1; then
        echo "openocd"
    else
        echo "none"
    fi
}

BACKEND=$(detect_backend)
if [ "$BACKEND" = "none" ]; then
    echo "Error: No supported STM32 programmer found."
    echo "Install one of the following:"
    echo "  1. STM32CubeProgrammer (st.com/stm32cubeprog)"
    echo "  2. probe-rs (cargo install probe-rs-tools)"
    echo "  3. OpenOCD (apt install openocd / pacman -S openocd)"
    exit 1
fi

PORT="${STM32_PORT:-port=SWD}"
ADDRESS="${STM32_ADDR:-0x08000000}"
OPENOCD_IF="${OPENOCD_IF:-interface/stlink.cfg}"
OPENOCD_TARGET="${OPENOCD_TARGET:-target/stm32f4x.cfg}"

case "$ACTION" in
    detect)
        echo "==> Detecting connected ST-LINK / debug probes..."
        if [ "$BACKEND" = "cubeprog" ]; then
            STM32_Programmer_CLI -l
        elif [ "$BACKEND" = "probe-rs" ]; then
            probe-rs list
        elif [ "$BACKEND" = "openocd" ]; then
            openocd -f "$OPENOCD_IF" -c "init; shutdown"
        fi
        ;;

    flash|flash-run)
        FW=$(find_firmware)
        if [ -z "$FW" ]; then
            echo "Error: No firmware artifact found to flash."
            echo "Run 'build' task first or specify firmware path: $0 flash path/to/firmware.bin"
            exit 1
        fi

        echo "==> Flashing firmware: $FW via $BACKEND..."

        if [ "$BACKEND" = "cubeprog" ]; then
            RESET_FLAG=""
            if [ "$ACTION" = "flash-run" ]; then
                RESET_FLAG="-rst"
            fi

            if [[ "$FW" == *.bin ]]; then
                STM32_Programmer_CLI -c "$PORT" -w "$FW" "$ADDRESS" -v $RESET_FLAG
            else
                STM32_Programmer_CLI -c "$PORT" -w "$FW" -v $RESET_FLAG
            fi
        elif [ "$BACKEND" = "probe-rs" ]; then
            CHIP="${STM32_CHIP:-STM32F401RETx}"
            if [ "$ACTION" = "flash-run" ]; then
                probe-rs run --chip "$CHIP" "$FW"
            else
                probe-rs download --chip "$CHIP" "$FW"
            fi
        elif [ "$BACKEND" = "openocd" ]; then
            RESET_CMD=""
            if [ "$ACTION" = "flash-run" ]; then
                RESET_CMD="reset"
            fi
            openocd -f "$OPENOCD_IF" -f "$OPENOCD_TARGET" -c "program $FW verify $RESET_CMD exit $ADDRESS"
        fi
        echo "==> Programming succeeded."
        ;;

    erase)
        echo "⚠️ WARNING: Mass erase will completely wipe the target flash memory!"
        if [ "$BACKEND" = "cubeprog" ]; then
            STM32_Programmer_CLI -c "$PORT" -e all
        elif [ "$BACKEND" = "probe-rs" ]; then
            CHIP="${STM32_CHIP:-STM32F401RETx}"
            probe-rs erase --chip "$CHIP"
        elif [ "$BACKEND" = "openocd" ]; then
            openocd -f "$OPENOCD_IF" -f "$OPENOCD_TARGET" -c "init; reset halt; stm32x mass_erase 0; shutdown"
        fi
        echo "==> Device erased."
        ;;

    reset)
        echo "==> Resetting target MCU..."
        if [ "$BACKEND" = "cubeprog" ]; then
            STM32_Programmer_CLI -c "$PORT" -rst
        elif [ "$BACKEND" = "probe-rs" ]; then
            CHIP="${STM32_CHIP:-STM32F401RETx}"
            probe-rs reset --chip "$CHIP"
        elif [ "$BACKEND" = "openocd" ]; then
            openocd -f "$OPENOCD_IF" -f "$OPENOCD_TARGET" -c "init; reset run; shutdown"
        fi
        echo "==> Target reset."
        ;;

    *)
        echo "Usage: $0 {detect|flash|flash-run|erase|reset} [path_to_firmware]"
        exit 1
        ;;
esac

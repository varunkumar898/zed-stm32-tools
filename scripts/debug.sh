#!/usr/bin/env bash
# STM32 Debug Session Helper Script
set -euo pipefail

MODE="${1:-openocd}"
ELF_FILE="${2:-}"

find_elf() {
    if [ -n "$ELF_FILE" ] && [ -f "$ELF_FILE" ]; then
        echo "$ELF_FILE"
        return
    fi
    for candidate in build/*.elf *.elf; do
        if [ -f "$candidate" ]; then
            echo "$candidate"
            return
        fi
    done
    echo ""
}

case "$MODE" in
    openocd)
        IF_CFG="${OPENOCD_IF:-interface/stlink.cfg}"
        TARGET_CFG="${OPENOCD_TARGET:-target/stm32f4x.cfg}"
        echo "==> Starting OpenOCD GDB Server on localhost:3333..."
        openocd -f "$IF_CFG" -f "$TARGET_CFG"
        ;;

    probe-rs)
        CHIP="${STM32_CHIP:-STM32F401RETx}"
        echo "==> Starting probe-rs DAP server for chip: $CHIP..."
        probe-rs dap --chip "$CHIP"
        ;;

    gdb)
        FW=$(find_elf)
        GDB_BIN="arm-none-eabi-gdb"
        if ! command -v "$GDB_BIN" >/dev/null 2>&1; then
            GDB_BIN="gdb-multiarch"
        fi
        echo "==> Starting $GDB_BIN connecting to localhost:3333..."
        "$GDB_BIN" -ex "target extended-remote localhost:3333" "$FW"
        ;;

    *)
        echo "Usage: $0 {openocd|probe-rs|gdb} [path_to_elf]"
        exit 1
        ;;
esac

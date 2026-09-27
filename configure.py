#!/usr/bin/env python3
"""
Universal STM32 Target Auto-Configuration Script for Zed Editor.
Configures .zed/tasks.json and .zed/debug.json with target hardware parameters.

Supported STM32 Families (17 total):
  F0, F1, F2, F3, F4, F7, G0, G4, H5, H7, L0, L1, L4, L5, U5, WB, WL

Example Usage:
  python3 configure.py --family F4 --chip STM32F401RETx --define STM32F401xE --flash 512K --ram 96K --hal
  python3 configure.py --board nucleo-f401re
  python3 configure.py --list-families
  python3 configure.py --list-boards
"""

import argparse
import json
import os
import shutil
import sys
from pathlib import Path

# Terminal ANSI color formatting
BOLD = "\033[1m"
GREEN = "\033[32m"
CYAN = "\033[36m"
YELLOW = "\033[33m"
RED = "\033[31m"
RESET = "\033[0m"

# ------------------------------------------------------------------------------
# STM32 Family Hardware Matrix (All 17 Families)
# ------------------------------------------------------------------------------
STM32_FAMILIES = {
    "F0": {"core": "Cortex-M0",  "mcpu": "cortex-m0",     "fpu": None,            "float_abi": "soft", "default_ram_origin": "0x20000000"},
    "G0": {"core": "Cortex-M0+", "mcpu": "cortex-m0plus", "fpu": None,            "float_abi": "soft", "default_ram_origin": "0x20000000"},
    "L0": {"core": "Cortex-M0+", "mcpu": "cortex-m0plus", "fpu": None,            "float_abi": "soft", "default_ram_origin": "0x20000000"},
    "F1": {"core": "Cortex-M3",  "mcpu": "cortex-m3",     "fpu": None,            "float_abi": "soft", "default_ram_origin": "0x20000000"},
    "F2": {"core": "Cortex-M3",  "mcpu": "cortex-m3",     "fpu": None,            "float_abi": "soft", "default_ram_origin": "0x20000000"},
    "L1": {"core": "Cortex-M3",  "mcpu": "cortex-m3",     "fpu": None,            "float_abi": "soft", "default_ram_origin": "0x20000000"},
    "F3": {"core": "Cortex-M4F", "mcpu": "cortex-m4",     "fpu": "fpv4-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "F4": {"core": "Cortex-M4F", "mcpu": "cortex-m4",     "fpu": "fpv4-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "G4": {"core": "Cortex-M4F", "mcpu": "cortex-m4",     "fpu": "fpv4-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "L4": {"core": "Cortex-M4F", "mcpu": "cortex-m4",     "fpu": "fpv4-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "WB": {"core": "Cortex-M4F", "mcpu": "cortex-m4",     "fpu": "fpv4-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "WL": {"core": "Cortex-M4F", "mcpu": "cortex-m4",     "fpu": "fpv4-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "F7": {"core": "Cortex-M7F", "mcpu": "cortex-m7",     "fpu": "fpv5-d16",      "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "H7": {"core": "Cortex-M7F", "mcpu": "cortex-m7",     "fpu": "fpv5-d16",      "float_abi": "hard", "default_ram_origin": "0x24000000"},
    "H5": {"core": "Cortex-M33", "mcpu": "cortex-m33",    "fpu": "fpv5-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "L5": {"core": "Cortex-M33", "mcpu": "cortex-m33",    "fpu": "fpv5-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
    "U5": {"core": "Cortex-M33", "mcpu": "cortex-m33",    "fpu": "fpv5-sp-d16",   "float_abi": "hard", "default_ram_origin": "0x20000000"},
}

# ------------------------------------------------------------------------------
# Predefined Target Board Presets
# ------------------------------------------------------------------------------
BOARD_PRESETS = {
    "nucleo-f401re": {
        "family": "F4", "chip": "STM32F401RETx", "define": "STM32F401xE",
        "flash": "512K", "ram": "96K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32F401RE (84 MHz, 512KB Flash, 96KB RAM)"
    },
    "nucleo-f411re": {
        "family": "F4", "chip": "STM32F411RETx", "define": "STM32F411xE",
        "flash": "512K", "ram": "128K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32F411RE (100 MHz, 512KB Flash, 128KB RAM)"
    },
    "nucleo-f446re": {
        "family": "F4", "chip": "STM32F446RETx", "define": "STM32F446xx",
        "flash": "512K", "ram": "128K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32F446RE (180 MHz, 512KB Flash, 128KB RAM)"
    },
    "stm32f4-discovery": {
        "family": "F4", "chip": "STM32F407VGTx", "define": "STM32F407xx",
        "flash": "1024K", "ram": "192K", "ram_origin": "0x20000000",
        "description": "STM32F407G-DISC1 Discovery Board (168 MHz, 1MB Flash, 192KB RAM)"
    },
    "bluepill": {
        "family": "F1", "chip": "STM32F103C8Tx", "define": "STM32F103xB",
        "flash": "64K", "ram": "20K", "ram_origin": "0x20000000",
        "description": "STM32F103C8T6 BluePill Minimal Board (72 MHz, 64KB Flash, 20KB RAM)"
    },
    "blackpill-f401": {
        "family": "F4", "chip": "STM32F401CCUx", "define": "STM32F401xC",
        "flash": "256K", "ram": "64K", "ram_origin": "0x20000000",
        "description": "WeAct STM32F401CCU6 BlackPill (84 MHz, 256KB Flash, 64KB RAM)"
    },
    "blackpill-f411": {
        "family": "F4", "chip": "STM32F411CEUx", "define": "STM32F411xE",
        "flash": "512K", "ram": "128K", "ram_origin": "0x20000000",
        "description": "WeAct STM32F411CEU6 BlackPill (100 MHz, 512KB Flash, 128KB RAM)"
    },
    "nucleo-g071rb": {
        "family": "G0", "chip": "STM32G071RBTx", "define": "STM32G071xx",
        "flash": "128K", "ram": "36K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32G071RB (64 MHz, 128KB Flash, 36KB RAM)"
    },
    "nucleo-g474re": {
        "family": "G4", "chip": "STM32G474RETx", "define": "STM32G474xx",
        "flash": "512K", "ram": "128K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32G474RE (170 MHz, 512KB Flash, 128KB RAM)"
    },
    "nucleo-h743zi": {
        "family": "H7", "chip": "STM32H743ZITx", "define": "STM32H743xx",
        "flash": "2048K", "ram": "1024K", "ram_origin": "0x24000000",
        "description": "ST Nucleo-144 STM32H743ZI (480 MHz, 2MB Flash, 1MB RAM)"
    },
    "nucleo-l476rg": {
        "family": "L4", "chip": "STM32L476RGTx", "define": "STM32L476xx",
        "flash": "1024K", "ram": "128K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32L476RG (80 MHz, 1MB Flash, 128KB RAM)"
    },
    "nucleo-u575zi": {
        "family": "U5", "chip": "STM32U575ZITx", "define": "STM32U575xx",
        "flash": "2048K", "ram": "786K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-144 STM32U575ZI-Q (160 MHz Cortex-M33, 2MB Flash, 786KB RAM)"
    },
    "nucleo-wb55rg": {
        "family": "WB", "chip": "STM32WB55RGVx", "define": "STM32WB55xx",
        "flash": "1024K", "ram": "256K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32WB55RG Wireless BLE/Zigbee (64 MHz M4 + 32 MHz M0+)"
    },
    "nucleo-wl55jc": {
        "family": "WL", "chip": "STM32WL55JCIx", "define": "STM32WL55xx",
        "flash": "256K", "ram": "64K", "ram_origin": "0x20000000",
        "description": "ST Nucleo-64 STM32WL55JC Sub-GHz LoRa Wireless (48 MHz M4 + 48 MHz M0+)"
    },
}


def normalize_family(family_str: str) -> str:
    """Normalize family string to uppercase without 'STM32' prefix."""
    s = family_str.upper().strip()
    if s.startswith("STM32"):
        s = s[5:]
    return s


def detect_generator() -> str:
    """Detect whether Ninja or Unix Makefiles is preferred."""
    if shutil.which("ninja"):
        return "Ninja"
    return "Unix Makefiles"


def generate_zed_tasks(family: str, chip: str, define: str, flash: str, ram: str, ram_origin: str, default_hal: bool, generator: str):
    """Generate .zed/tasks.json content with the 5 required tasks."""
    return [
        {
            "label": "CMake Configure (Bare-Metal)",
            "command": "cmake",
            "args": [
                "-B", "build",
                "-G", generator,
                "-DCMAKE_TOOLCHAIN_FILE=cmake/stm32_gcc.cmake",
                f"-DSTM32_FAMILY={family}",
                f"-DSTM32_CHIP={chip}",
                f"-DFLASH_SIZE={flash}",
                f"-DRAM_SIZE={ram}",
                f"-DRAM_ORIGIN={ram_origin}",
                "-DUSE_HAL=OFF"
            ],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "CMake Configure (HAL)",
            "command": "cmake",
            "args": [
                "-B", "build",
                "-G", generator,
                "-DCMAKE_TOOLCHAIN_FILE=cmake/stm32_gcc.cmake",
                f"-DSTM32_FAMILY={family}",
                f"-DSTM32_CHIP={chip}",
                f"-DFLASH_SIZE={flash}",
                f"-DRAM_SIZE={ram}",
                f"-DRAM_ORIGIN={ram_origin}",
                "-DUSE_HAL=ON"
            ],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "Compile Firmware",
            "command": "cmake",
            "args": [
                "--build", "build"
            ],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "Flash Board (probe-rs)",
            "command": "probe-rs",
            "args": [
                "run",
                "--chip", chip,
                "build/stm32_app.elf"
            ],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "Clean Build",
            "command": "cmake",
            "args": [
                "--build", "build",
                "--target", "clean"
            ],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "Memory Usage (Size Report)",
            "command": "arm-none-eabi-size",
            "args": [
                "--format=berkeley",
                "build/stm32_app.elf"
            ],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "STM32: Toolchain Doctor",
            "command": "stm32-tools-cli",
            "args": ["doctor"],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "STM32: Project Info",
            "command": "stm32-tools-cli",
            "args": ["info"],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "STM32: Open STM32CubeMX GUI",
            "command": "stm32-tools-cli",
            "args": ["cubemx"],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        },
        {
            "label": "STM32: List Supported Devices",
            "command": "stm32-tools-cli",
            "args": ["devices"],
            "cwd": "$ZED_WORKTREE_ROOT",
            "reveal": "always",
            "use_new_terminal": False,
            "allow_concurrent_runs": False
        }
    ]


def generate_zed_debug(chip: str):
    """Generate .zed/debug.json probe-rs DAP configuration."""
    return [
        {
            "label": "Debug STM32 (probe-rs)",
            "adapter": "probe-rs",
            "request": "launch",
            "cwd": "${workspaceFolder}",
            "program": "${workspaceFolder}/build/stm32_app.elf",
            "chip": chip,
            "core": "main",
            "protocol": "swd",
            "speed": 4000,
            "connectUnderReset": True,
            "haltAfterReset": True,
            "flashing": {
                "enabled": True,
                "haltAfterReset": True
            }
        }
    ]


def print_families():
    print(f"\n{BOLD}{CYAN}=== Supported STM32 Microcontroller Families (17 Total) ==={RESET}")
    print(f"{'Family':<10} {'Core Architecture':<18} {'FPU Extension':<16} {'Float ABI':<10} {'Default RAM Origin'}")
    print("-" * 75)
    for fam, info in STM32_FAMILIES.items():
        fpu_str = info["fpu"] if info["fpu"] else "None (Soft FPU)"
        print(f"STM32{fam:<5} {info['core']:<18} {fpu_str:<16} {info['float_abi']:<10} {info['default_ram_origin']}")
    print()


def print_boards():
    print(f"\n{BOLD}{CYAN}=== Predefined Target Board Presets ==={RESET}")
    print(f"{'Preset Name':<20} {'Target Chip':<18} {'Flash':<8} {'RAM':<8} {'Description'}")
    print("-" * 80)
    for name, b in BOARD_PRESETS.items():
        print(f"{name:<20} {b['chip']:<18} {b['flash']:<8} {b['ram']:<8} {b['description']}")
    print()


def main():
    parser = argparse.ArgumentParser(
        description="Universal STM32 Target Configuration Tool for Zed Editor",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 configure.py --family F4 --chip STM32F401RETx --define STM32F401xE --flash 512K --ram 96K --hal
  python3 configure.py --board bluepill
  python3 configure.py --board nucleo-h743zi
  python3 configure.py --list-families
  python3 configure.py --list-boards
        """
    )

    parser.add_argument("--family", type=str, help="STM32 Family (e.g. F0, F1, F4, H7, G0, U5, WB, WL)")
    parser.add_argument("--chip", type=str, help="Target MCU part number / probe-rs chip ID (e.g. STM32F401RETx)")
    parser.add_argument("--define", type=str, help="CMSIS device define macro (e.g. STM32F401xE)")
    parser.add_argument("--flash", type=str, help="Flash size (e.g. 64K, 512K, 1024K, 2048K)")
    parser.add_argument("--ram", type=str, help="RAM size (e.g. 20K, 96K, 128K, 1024K)")
    parser.add_argument("--ram-origin", type=str, help="RAM start origin address (default: 0x20000000 or 0x24000000)")
    parser.add_argument("--hal", action="store_true", default=None, help="Set default configuration to STM32 HAL mode")
    parser.add_argument("--no-hal", action="store_false", dest="hal", help="Set default configuration to Bare-Metal mode")
    parser.add_argument("--board", type=str, help="Predefined board preset (e.g. nucleo-f401re, bluepill, nucleo-h743zi)")
    parser.add_argument("--generator", type=str, default=None, help="CMake build generator (Ninja or 'Unix Makefiles')")
    parser.add_argument("--list-families", action="store_true", help="List all 17 supported STM32 families")
    parser.add_argument("--list-boards", action="store_true", help="List available board presets")

    args = parser.parse_args()

    if args.list_families:
        print_families()
        return 0

    if args.list_boards:
        print_boards()
        return 0

    # Initialize configuration dictionary
    config = {
        "family": "F4",
        "chip": "STM32F401RETx",
        "define": "STM32F401xE",
        "flash": "512K",
        "ram": "96K",
        "ram_origin": "0x20000000",
        "hal": False
    }

    # Apply board preset if specified
    if args.board:
        preset_key = args.board.lower().strip()
        if preset_key not in BOARD_PRESETS:
            print(f"{RED}Error: Unknown board preset '{args.board}'. Available presets:{RESET}")
            for b in BOARD_PRESETS:
                print(f"  - {b}")
            return 1
        preset = BOARD_PRESETS[preset_key]
        config["family"] = preset["family"]
        config["chip"] = preset["chip"]
        config["define"] = preset["define"]
        config["flash"] = preset["flash"]
        config["ram"] = preset["ram"]
        config["ram_origin"] = preset["ram_origin"]
        print(f"{CYAN}Applied Board Preset:{RESET} {preset['description']}")

    # Command line arguments override preset / defaults
    if args.family:
        config["family"] = normalize_family(args.family)
    if args.chip:
        config["chip"] = args.chip.strip()
    if args.define:
        config["define"] = args.define.strip()
    if args.flash:
        config["flash"] = args.flash.strip()
    if args.ram:
        config["ram"] = args.ram.strip()
    if args.ram_origin:
        config["ram_origin"] = args.ram_origin.strip()
    elif config["family"] == "H7" and not args.ram_origin and not args.board:
        config["ram_origin"] = "0x24000000"
    if args.hal is not None:
        config["hal"] = args.hal

    # Validate family
    fam_norm = normalize_family(config["family"])
    if fam_norm not in STM32_FAMILIES:
        print(f"{RED}Error: Unsupported family '{config['family']}'. Supported:{RESET}")
        print(", ".join(STM32_FAMILIES.keys()))
        return 1
    config["family"] = fam_norm
    family_info = STM32_FAMILIES[fam_norm]

    # Generator selection
    generator = args.generator if args.generator else detect_generator()

    # Paths
    root_dir = Path(__file__).resolve().parent
    zed_dir = root_dir / ".zed"
    zed_dir.mkdir(parents=True, exist_ok=True)

    tasks_path = zed_dir / "tasks.json"
    debug_path = zed_dir / "debug.json"

    # Generate and write .zed/tasks.json
    tasks_content = generate_zed_tasks(
        family=config["family"],
        chip=config["chip"],
        define=config["define"],
        flash=config["flash"],
        ram=config["ram"],
        ram_origin=config["ram_origin"],
        default_hal=config["hal"],
        generator=generator
    )
    with open(tasks_path, "w", encoding="utf-8") as f:
        json.dump(tasks_content, f, indent=2)

    # Generate and write .zed/debug.json
    debug_content = generate_zed_debug(chip=config["chip"])
    with open(debug_path, "w", encoding="utf-8") as f:
        json.dump(debug_content, f, indent=2)

    # Display configuration summary
    print(f"\n{BOLD}{GREEN}✓ Zed STM32 Workspace Configured Successfully!{RESET}\n")
    print(f"{BOLD}Configuration Parameters:{RESET}")
    print(f"  • Family:       STM32{config['family']} ({family_info['core']}, -mcpu={family_info['mcpu']})")
    print(f"  • FPU:          {family_info['fpu'] if family_info['fpu'] else 'None'} (-mfloat-abi={family_info['float_abi']})")
    print(f"  • Target Chip:  {config['chip']}")
    print(f"  • Define:       {config['define']}")
    print(f"  • Flash Size:   {config['flash']}")
    print(f"  • RAM Size:     {config['ram']} (Origin: {config['ram_origin']})")
    print(f"  • Generator:    {generator}")
    print(f"  • Default Mode: {'STM32 HAL C Mode' if config['hal'] else 'Bare-Metal C Mode'}")
    print(f"\n{BOLD}Updated Files:{RESET}")
    print(f"  • {tasks_path}")
    print(f"  • {debug_path}")
    print(f"\n{BOLD}Zed Editor Integration:{RESET}")
    print(f"  Press {CYAN}Ctrl+Alt+T{RESET} (or command palette {CYAN}task: spawn{RESET}) in Zed to run:")
    print(f"    1. {BOLD}CMake Configure (Bare-Metal){RESET} - Register-level build setup")
    print(f"    2. {BOLD}CMake Configure (HAL){RESET}        - Full HAL & CMSIS setup")
    print(f"    3. {BOLD}Compile Firmware{RESET}            - Build stm32_app.elf")
    print(f"    4. {BOLD}Flash Board (probe-rs){RESET}      - Flash directly via probe-rs")
    print(f"    5. {BOLD}Clean Build{RESET}                 - Clean build artifacts")
    print(f"\n  Press {CYAN}F5{RESET} in Zed to launch live hardware debugging with {BOLD}probe-rs DAP{RESET}.\n")

    return 0


if __name__ == "__main__":
    sys.exit(main())

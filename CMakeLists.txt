# ==============================================================================
# Universal Production-Grade STM32 CMake Workspace for Zed Editor
# Supports all 17 STM32 families with Dual Mode (Bare-Metal & HAL)
# ==============================================================================

cmake_minimum_required(VERSION 3.20)

# Set toolchain file if not explicitly supplied on command line
if(NOT CMAKE_TOOLCHAIN_FILE)
    set(CMAKE_TOOLCHAIN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/cmake/stm32_gcc.cmake" CACHE FILEPATH "Toolchain file")
endif()

# ------------------------------------------------------------------------------
# Target Hardware Parameters (Defaults configurable via CLI or configure.py)
# ------------------------------------------------------------------------------
set(STM32_FAMILY "F4"             CACHE STRING "STM32 Family (e.g. F0, F1, F4, H7, G0, U5, etc.)")
set(STM32_CHIP   "STM32F401RETx"  CACHE STRING "STM32 Part Number or probe-rs target identifier")
set(FLASH_SIZE   "512K"           CACHE STRING "Target MCU Flash size (e.g. 64K, 512K, 1024K)")
set(RAM_SIZE     "96K"            CACHE STRING "Target MCU RAM size (e.g. 20K, 96K, 128K)")
set(RAM_ORIGIN   "0x20000000"     CACHE STRING "Target MCU RAM start address (e.g. 0x20000000, 0x24000000)")
option(USE_HAL   "Enable STM32 HAL and CMSIS library integration" OFF)

# Project Definition
project(stm32_app C ASM)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# ------------------------------------------------------------------------------
# Include Family Architecture Dispatcher
# ------------------------------------------------------------------------------
include("${CMAKE_CURRENT_SOURCE_DIR}/cmake/stm32_families.cmake")
configure_stm32_architecture("${STM32_FAMILY}" "${STM32_CHIP}")

# ------------------------------------------------------------------------------
# Dynamic Linker Script Generation
# ------------------------------------------------------------------------------
set(LINKER_SCRIPT_IN "${CMAKE_CURRENT_SOURCE_DIR}/linker.ld.in")
set(LINKER_SCRIPT_OUT "${CMAKE_CURRENT_BINARY_DIR}/linker.ld")

configure_file("${LINKER_SCRIPT_IN}" "${LINKER_SCRIPT_OUT}" @ONLY)
message(STATUS "Generated Linker Script: ${LINKER_SCRIPT_OUT} [FLASH: ${FLASH_SIZE}, RAM: ${RAM_SIZE} @ ${RAM_ORIGIN}]")

# ------------------------------------------------------------------------------
# Compiler & Linker Warnings and Optimization Flags
# ------------------------------------------------------------------------------
add_compile_options(
    -Wall
    -Wextra
    -Wpedantic
    -Wshadow
    -fdata-sections
    -ffunction-sections
    -fno-common
)

# Debug / Release specific flags
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_compile_options(-O0 -g3)
else()
    add_compile_options(-Os -g)
endif()

add_link_options(
    -T${LINKER_SCRIPT_OUT}
    -Wl,--gc-sections
    -Wl,-Map=${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}.map
    -Wl,--no-warn-rwx-segments
    --specs=nano.specs
    --specs=nosys.specs
    -lc
    -lm
)

# ------------------------------------------------------------------------------
# Header Include Paths
# ------------------------------------------------------------------------------
include_directories(
    "${CMAKE_CURRENT_SOURCE_DIR}/Inc"
    "${CMAKE_CURRENT_SOURCE_DIR}/Drivers/CMSIS/Include"
    "${CMAKE_CURRENT_SOURCE_DIR}/Drivers/CMSIS/Device/ST/Include"
)

# ------------------------------------------------------------------------------
# Source File Collection & Development Mode Configuration
# ------------------------------------------------------------------------------
set(APP_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/startup/startup_stm32.s"
    "${CMAKE_CURRENT_SOURCE_DIR}/Src/system_stm32.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/Src/syscalls.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/Src/main.c"
)

if(USE_HAL)
    message(STATUS "Build Mode: STM32 HAL C Mode (Full HAL & CMSIS Integration)")
    add_compile_definitions(USE_HAL=1)
    include_directories("${CMAKE_CURRENT_SOURCE_DIR}/Drivers/HAL/Inc")
    file(GLOB_RECURSE HAL_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/Drivers/HAL/Src/*.c")
    list(APPEND APP_SOURCES ${HAL_SOURCES})
else()
    message(STATUS "Build Mode: Bare-Metal C Mode (Zero-Dependency Direct Register Access)")
    add_compile_definitions(USE_HAL=0)
endif()

# ------------------------------------------------------------------------------
# Target Executable & Artifacts
# ------------------------------------------------------------------------------
add_executable(${PROJECT_NAME}.elf ${APP_SOURCES})

# Target-specific post-build commands: generate binary, hex, and report memory usage
add_custom_command(TARGET ${PROJECT_NAME}.elf POST_BUILD
    COMMAND ${CMAKE_OBJCOPY} -O binary ${PROJECT_NAME}.elf ${PROJECT_NAME}.bin
    COMMAND ${CMAKE_OBJCOPY} -O ihex ${PROJECT_NAME}.elf ${PROJECT_NAME}.hex
    COMMENT "Generating raw binary (${PROJECT_NAME}.bin) and Intel Hex (${PROJECT_NAME}.hex)"
)

add_custom_command(TARGET ${PROJECT_NAME}.elf POST_BUILD
    COMMAND ${CMAKE_SIZE} --format=berkeley ${PROJECT_NAME}.elf
    COMMENT "Firmware Memory Consumption Report:"
)

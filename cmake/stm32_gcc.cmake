# ==============================================================================
# Universal ARM GCC Toolchain File for STM32
# ==============================================================================
# Usage: cmake -B build -DCMAKE_TOOLCHAIN_FILE=cmake/stm32_gcc.cmake ...
# ==============================================================================

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR arm)

# Bypass full executable linking test during initial CMake compiler detection
# since bare-metal targets require a valid linker script and startup code.
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

# Toolchain prefix: default to arm-none-eabi-
if(NOT DEFINED TOOLCHAIN_PREFIX)
    set(TOOLCHAIN_PREFIX arm-none-eabi-)
endif()

# Determine executable extension on host system (.exe for Windows)
if(CMAKE_HOST_WIN32)
    set(EXE_EXT ".exe")
else()
    set(EXE_EXT "")
endif()

# Discover toolchain executables
find_program(CMAKE_C_COMPILER NAMES ${TOOLCHAIN_PREFIX}gcc${EXE_EXT} PATHS ENV PATH)
if(NOT CMAKE_C_COMPILER)
    set(CMAKE_C_COMPILER ${TOOLCHAIN_PREFIX}gcc${EXE_EXT})
endif()

find_program(CMAKE_ASM_COMPILER NAMES ${TOOLCHAIN_PREFIX}gcc${EXE_EXT} PATHS ENV PATH)
if(NOT CMAKE_ASM_COMPILER)
    set(CMAKE_ASM_COMPILER ${TOOLCHAIN_PREFIX}gcc${EXE_EXT})
endif()

find_program(CMAKE_CXX_COMPILER NAMES ${TOOLCHAIN_PREFIX}g++${EXE_EXT} PATHS ENV PATH)
if(NOT CMAKE_CXX_COMPILER)
    set(CMAKE_CXX_COMPILER ${TOOLCHAIN_PREFIX}g++${EXE_EXT})
endif()

find_program(CMAKE_OBJCOPY NAMES ${TOOLCHAIN_PREFIX}objcopy${EXE_EXT} PATHS ENV PATH)
if(NOT CMAKE_OBJCOPY)
    set(CMAKE_OBJCOPY ${TOOLCHAIN_PREFIX}objcopy${EXE_EXT})
endif()

find_program(CMAKE_OBJDUMP NAMES ${TOOLCHAIN_PREFIX}objdump${EXE_EXT} PATHS ENV PATH)
if(NOT CMAKE_OBJDUMP)
    set(CMAKE_OBJDUMP ${TOOLCHAIN_PREFIX}objdump${EXE_EXT})
endif()

find_program(CMAKE_SIZE NAMES ${TOOLCHAIN_PREFIX}size${EXE_EXT} PATHS ENV PATH)
if(NOT CMAKE_SIZE)
    set(CMAKE_SIZE ${TOOLCHAIN_PREFIX}size${EXE_EXT})
endif()

# Cross-compiling search behaviors
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

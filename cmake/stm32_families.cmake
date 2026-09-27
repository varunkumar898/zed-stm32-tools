# ==============================================================================
# Universal STM32 Family Architecture & FPU Flag Dispatcher
# Supports all 17 STM32 micro-controller families
# ==============================================================================

function(configure_stm32_architecture FAMILY CHIP)
    # Normalize family string: uppercase and strip "STM32" prefix if present
    string(TOUPPER "${FAMILY}" _RAW_UPPER)
    string(REGEX REPLACE "^STM32" "" _FAM "${_RAW_UPPER}")

    set(_CPU "")
    set(_FPU "")
    set(_FLOAT_ABI "")
    set(_CORE "")
    set(_WIRELESS FALSE)
    set(_DUAL_CORE FALSE)

    # --------------------------------------------------------------------------
    # Multi-Family Hardware Matrix & CPU Flags
    # --------------------------------------------------------------------------
    if(_FAM STREQUAL "F0")
        # Cortex-M0
        set(_CPU "cortex-m0")
        set(_FLOAT_ABI "soft")
        set(_CORE "CM0")

    elseif(_FAM STREQUAL "G0" OR _FAM STREQUAL "L0")
        # Cortex-M0+
        set(_CPU "cortex-m0plus")
        set(_FLOAT_ABI "soft")
        set(_CORE "CM0PLUS")

    elseif(_FAM STREQUAL "F1" OR _FAM STREQUAL "F2" OR _FAM STREQUAL "L1")
        # Cortex-M3
        set(_CPU "cortex-m3")
        set(_FLOAT_ABI "soft")
        set(_CORE "CM3")

    elseif(_FAM STREQUAL "F3" OR _FAM STREQUAL "F4" OR _FAM STREQUAL "G4" OR _FAM STREQUAL "L4")
        # Cortex-M4F
        set(_CPU "cortex-m4")
        set(_FPU "fpv4-sp-d16")
        set(_FLOAT_ABI "hard")
        set(_CORE "CM4")

    elseif(_FAM STREQUAL "WB" OR _FAM STREQUAL "WL")
        # Wireless Cortex-M4F (Primary Application Core)
        set(_CPU "cortex-m4")
        set(_FPU "fpv4-sp-d16")
        set(_FLOAT_ABI "hard")
        set(_CORE "CM4")
        set(_WIRELESS TRUE)
        set(_DUAL_CORE TRUE)

    elseif(_FAM STREQUAL "F7" OR _FAM STREQUAL "H7")
        # Cortex-M7F
        set(_CPU "cortex-m7")
        set(_FPU "fpv5-d16")
        set(_FLOAT_ABI "hard")
        set(_CORE "CM7")

    elseif(_FAM STREQUAL "H5" OR _FAM STREQUAL "L5" OR _FAM STREQUAL "U5")
        # Cortex-M33
        set(_CPU "cortex-m33")
        set(_FPU "fpv5-sp-d16")
        set(_FLOAT_ABI "hard")
        set(_CORE "CM33")

    else()
        message(FATAL_ERROR
            "Unknown or unsupported STM32 family: '${FAMILY}'.\n"
            "Supported families (17 total): F0, F1, F2, F3, F4, F7, G0, G4, H5, H7, L0, L1, L4, L5, U5, WB, WL"
        )
    endif()

    # Construct compiler and linker architecture flags
    set(_ARCH_FLAGS "-mcpu=${_CPU}" "-mthumb")
    if(_FPU)
        list(APPEND _ARCH_FLAGS "-mfpu=${_FPU}")
    endif()
    list(APPEND _ARCH_FLAGS "-mfloat-abi=${_FLOAT_ABI}")

    # Export variables to parent scope
    set(STM32_CPU "${_CPU}" PARENT_SCOPE)
    set(STM32_FPU "${_FPU}" PARENT_SCOPE)
    set(STM32_FLOAT_ABI "${_FLOAT_ABI}" PARENT_SCOPE)
    set(STM32_CORE "${_CORE}" PARENT_SCOPE)
    set(STM32_ARCH_FLAGS "${_ARCH_FLAGS}" PARENT_SCOPE)
    set(STM32_FAMILY_NORMALIZED "${_FAM}" PARENT_SCOPE)

    # Apply compiler and linker options globally
    add_compile_options(${_ARCH_FLAGS})
    add_link_options(${_ARCH_FLAGS})

    # Add core and family preprocessor defines
    add_compile_definitions("STM32${_FAM}")
    add_compile_definitions("CORE_${_CORE}")

    if(_WIRELESS)
        add_compile_definitions(STM32_WIRELESS)
    endif()

    if(_DUAL_CORE)
        add_compile_definitions(STM32_DUAL_CORE_PRIMARY)
        add_compile_definitions(CORE_CM4)
    endif()

    if(CHIP)
        string(MAKE_C_IDENTIFIER "${CHIP}" _CHIP_IDENTIFIER)
        string(TOUPPER "${_CHIP_IDENTIFIER}" _CHIP_MACRO)
        add_compile_definitions("${_CHIP_MACRO}")
    endif()

    message(STATUS "--------------------------------------------------------")
    message(STATUS " STM32 Architecture Configuration:")
    message(STATUS "   Family:    STM32${_FAM}")
    message(STATUS "   Core:      ${_CORE} (-mcpu=${_CPU})")
    if(_FPU)
        message(STATUS "   FPU:       ${_FPU} (-mfloat-abi=${_FLOAT_ABI})")
    else()
        message(STATUS "   FPU:       None (-mfloat-abi=${_FLOAT_ABI})")
    endif()
    if(CHIP)
        message(STATUS "   Target:    ${CHIP}")
    endif()
    if(_WIRELESS)
        message(STATUS "   Wireless:  Yes (Primary Cortex-M4 Core)")
    endif()
    message(STATUS "--------------------------------------------------------")
endfunction()

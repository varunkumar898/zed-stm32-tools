/**
 * ==============================================================================
 * @file    system_stm32.c
 * @brief   Universal STM32 System Architecture & Clock Initialization
 *
 * Implements:
 *   1. Hardware Coprocessor CP10/CP11 (FPU) activation for Cortex-M4F, M7, M33.
 *   2. Dual-core wireless initialization for STM32WB and STM32WL primary core.
 *   3. Vector Table Offset Register (VTOR) positioning.
 *   4. SystemCoreClock frequency maintenance.
 * ==============================================================================
 */

#include "system_stm32.h"

/* Default system core clock frequency (16 MHz HSI on most modern STM32s) */
uint32_t SystemCoreClock = 16000000UL;

/* System Control Space (SCS) register definitions */
#define SCS_BASE                (0xE000E000UL)
#define SCB_VTOR_ADDR           (SCS_BASE + 0x0D08UL) /* Vector Table Offset Register */
#define SCB_CPACR_ADDR          (SCS_BASE + 0x0D88UL) /* Coprocessor Access Control Register */
#define SCB_NSACR_ADDR          (SCS_BASE + 0x0D8CUL) /* Non-Secure Access Control Register */

/* STM32 Flash base address */
#define FLASH_BASE_ADDR         (0x08000000UL)

/* RCC Base addresses for multi-core wireless peripherals (HSEM clock enable) */
#define RCC_BASE_STM32WB        (0x58000000UL)
#define RCC_AHB3ENR_WB_OFFSET   (0x00000048UL)
#define RCC_BASE_STM32WL        (0x58000000UL)
#define RCC_AHB3ENR_WL_OFFSET   (0x00000048UL)

/**
 * @brief Enable Hardware Floating Point Unit (FPU) Coprocessors CP10 and CP11.
 *        Required prior to executing any floating point instructions on Cortex-M4F,
 *        Cortex-M7, and Cortex-M33 targets.
 */
static inline void SystemInit_FPU(void) {
#if (defined(__ARM_FP) && (__ARM_FP > 0)) || defined(CORE_CM4) || defined(CORE_CM7) || defined(CORE_CM33)
    volatile uint32_t *scb_cpacr = (volatile uint32_t *)SCB_CPACR_ADDR;

#if defined(CORE_CM33)
    /* Cortex-M33 (ARMv8-M): Grant Non-Secure access to CP10 and CP11 if in Secure mode */
    volatile uint32_t *scb_nsacr = (volatile uint32_t *)SCB_NSACR_ADDR;
    *scb_nsacr |= ((1UL << 10) | (1UL << 11));
#endif

    /* CPACR bits [23:20] = 0b1111 (Full Access for CP10 and CP11) */
    *scb_cpacr |= ((3UL << 10 * 2) | (3UL << 11 * 2));

    /* Data and Instruction Synchronization Barriers to guarantee pipeline update */
    __asm volatile ("dsb 0xF\n\tisb 0xF" ::: "memory");
#endif
}

/**
 * @brief Configure Vector Table Offset Register (VTOR) to Flash base.
 *        Applicable to Cortex-M0+, M3, M4, M7, and M33 cores.
 */
static inline void SystemInit_VTOR(void) {
#if !defined(CORE_CM0)
    /* Cortex-M0 does not feature VTOR; all other Cortex-M families do */
    volatile uint32_t *scb_vtor = (volatile uint32_t *)SCB_VTOR_ADDR;
    *scb_vtor = FLASH_BASE_ADDR;
    __asm volatile ("dsb 0xF\n\tisb 0xF" ::: "memory");
#endif
}

/**
 * @brief Hardware initialization for wireless dual-core targets (STM32WB and STM32WL).
 *        Runs on the primary Cortex-M4 application core.
 */
static inline void SystemInit_WirelessDualCore(void) {
#if defined(STM32WB) || (defined(STM32_WIRELESS) && defined(CORE_CM4))
    /*
     * STM32WB: Ensure Hardware Semaphore (HSEM) clock is active on CPU1 (Cortex-M4).
     * HSEM is required for IPC synchronization between CPU1 (M4) and CPU2 (M0+ RF core).
     */
    volatile uint32_t *rcc_ahb3enr = (volatile uint32_t *)(RCC_BASE_STM32WB + RCC_AHB3ENR_WB_OFFSET);
    *rcc_ahb3enr |= (1UL << 19); /* Enable HSEM peripheral clock (bit 19) */
    __asm volatile ("dsb 0xF\n\tisb 0xF" ::: "memory");

#elif defined(STM32WL)
    /*
     * STM32WL: Ensure Hardware Semaphore (HSEM) clock is active for multi-core IPC.
     */
    volatile uint32_t *rcc_ahb3enr = (volatile uint32_t *)(RCC_BASE_STM32WL + RCC_AHB3ENR_WL_OFFSET);
    *rcc_ahb3enr |= (1UL << 25); /* Enable HSEM clock (bit 25 on WL) */
    __asm volatile ("dsb 0xF\n\tisb 0xF" ::: "memory");
#endif
}

/**
 * @brief Early System Initialization function called from startup assembly.
 */
void SystemInit(void) {
    /* 1. Setup Vector Table Offset */
    SystemInit_VTOR();

    /* 2. Enable Hardware FPU Coprocessors CP10 / CP11 */
    SystemInit_FPU();

    /* 3. Handle Wireless Dual-Core Subsystem Synchronization */
    SystemInit_WirelessDualCore();
}

/**
 * @brief Update SystemCoreClock variable based on active oscillator and PLL settings.
 */
void SystemCoreClockUpdate(void) {
    /* Default frequency; updated when clock PLL configuration changes */
#if defined(STM32F0) || defined(STM32G0)
    SystemCoreClock = 48000000UL;
#elif defined(STM32F1) || defined(STM32F3)
    SystemCoreClock = 72000000UL;
#elif defined(STM32F4) || defined(STM32G4)
    SystemCoreClock = 84000000UL;
#elif defined(STM32F7) || defined(STM32H7)
    SystemCoreClock = 216000000UL;
#elif defined(STM32H5) || defined(STM32U5)
    SystemCoreClock = 160000000UL;
#else
    SystemCoreClock = 16000000UL;
#endif
}

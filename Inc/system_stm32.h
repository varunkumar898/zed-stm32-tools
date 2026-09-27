/**
 * ==============================================================================
 * @file    system_stm32.h
 * @brief   Universal STM32 System Architecture & Clock Configuration Header
 * ==============================================================================
 */

#ifndef SYSTEM_STM32_H
#define SYSTEM_STM32_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/**
 * @brief Global system core clock frequency in Hz.
 */
extern uint32_t SystemCoreClock;

/**
 * @brief  Early micro-controller hardware initialization.
 *         Configures FPU coprocessors (CP10/CP11), VTOR, and dual-core sync.
 *         Called automatically from Reset_Handler before main().
 */
void SystemInit(void);

/**
 * @brief  Updates the SystemCoreClock variable based on current clock register settings.
 */
void SystemCoreClockUpdate(void);

#ifdef __cplusplus
}
#endif

#endif /* SYSTEM_STM32_H */

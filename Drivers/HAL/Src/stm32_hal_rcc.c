/**
 * @file    stm32_hal_rcc.c
 * @brief   Universal STM32 HAL RCC Driver Implementation
 */

#include "stm32_hal.h"

uint32_t HAL_RCC_GetSysClockFreq(void) {
    return SystemCoreClock;
}

uint32_t HAL_RCC_GetHCLKFreq(void) {
    return SystemCoreClock;
}

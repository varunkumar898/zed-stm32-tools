/**
 * @file    stm32_hal_cortex.h
 * @brief   Universal STM32 HAL Cortex Driver Header
 */

#ifndef __STM32_HAL_CORTEX_H
#define __STM32_HAL_CORTEX_H

#ifdef __cplusplus
extern "C" {
#endif

#include "stm32_hal_def.h"

void HAL_NVIC_SetPriority(int32_t IRQn, uint32_t PreemptPriority, uint32_t SubPriority);
void HAL_NVIC_EnableIRQ(int32_t IRQn);
void HAL_NVIC_DisableIRQ(int32_t IRQn);
void HAL_NVIC_SystemReset(void);
uint32_t HAL_SYSTICK_Config(uint32_t TicksNumb);

#ifdef __cplusplus
}
#endif

#endif /* __STM32_HAL_CORTEX_H */

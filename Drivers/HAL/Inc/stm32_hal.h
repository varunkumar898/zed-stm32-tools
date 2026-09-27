/**
 * @file    stm32_hal.h
 * @brief   Universal STM32 HAL Main Header File
 */

#ifndef __STM32_HAL_H
#define __STM32_HAL_H

#ifdef __cplusplus
extern "C" {
#endif

#include "stm32_device.h"
#include "system_stm32.h"
#include "stm32_hal_def.h"
#include "stm32_hal_rcc.h"
#include "stm32_hal_gpio.h"
#include "stm32_hal_cortex.h"

/* Core HAL Functions */
HAL_StatusTypeDef HAL_Init(void);
HAL_StatusTypeDef HAL_DeInit(void);
void HAL_MspInit(void);
void HAL_MspDeInit(void);
HAL_StatusTypeDef HAL_InitTick(uint32_t TickPriority);

/* Time base functions */
void HAL_IncTick(void);
void HAL_Delay(uint32_t Delay);
uint32_t HAL_GetTick(void);
uint32_t HAL_GetTickPrio(void);
HAL_StatusTypeDef HAL_SetTickFreq(uint32_t Freq);
void HAL_SuspendTick(void);
void HAL_ResumeTick(void);

#ifdef __cplusplus
}
#endif

#endif /* __STM32_HAL_H */

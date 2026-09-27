/**
 * @file    stm32_hal_rcc.h
 * @brief   Universal STM32 HAL RCC Driver Header
 */

#ifndef __STM32_HAL_RCC_H
#define __STM32_HAL_RCC_H

#ifdef __cplusplus
extern "C" {
#endif

#include "stm32_hal_def.h"

/* Peripheral Clock Enable Macros */
#if defined(STM32F4) || defined(STM32F2) || defined(STM32F7)
  #define __HAL_RCC_GPIOA_CLK_ENABLE()   do { RCC->AHB1ENR |= (1UL << 0); } while(0U)
  #define __HAL_RCC_GPIOB_CLK_ENABLE()   do { RCC->AHB1ENR |= (1UL << 1); } while(0U)
  #define __HAL_RCC_GPIOC_CLK_ENABLE()   do { RCC->AHB1ENR |= (1UL << 2); } while(0U)
  #define __HAL_RCC_GPIOD_CLK_ENABLE()   do { RCC->AHB1ENR |= (1UL << 3); } while(0U)
#elif defined(STM32F1)
  #define __HAL_RCC_GPIOA_CLK_ENABLE()   do { RCC->APB2ENR |= (1UL << 2); } while(0U)
  #define __HAL_RCC_GPIOB_CLK_ENABLE()   do { RCC->APB2ENR |= (1UL << 3); } while(0U)
  #define __HAL_RCC_GPIOC_CLK_ENABLE()   do { RCC->APB2ENR |= (1UL << 4); } while(0U)
#elif defined(STM32H7)
  #define __HAL_RCC_GPIOA_CLK_ENABLE()   do { RCC->AHB4ENR |= (1UL << 0); } while(0U)
  #define __HAL_RCC_GPIOB_CLK_ENABLE()   do { RCC->AHB4ENR |= (1UL << 1); } while(0U)
  #define __HAL_RCC_GPIOC_CLK_ENABLE()   do { RCC->AHB4ENR |= (1UL << 2); } while(0U)
#else
  #define __HAL_RCC_GPIOA_CLK_ENABLE()   do { RCC->AHB1ENR |= (1UL << 0); } while(0U)
  #define __HAL_RCC_GPIOB_CLK_ENABLE()   do { RCC->AHB1ENR |= (1UL << 1); } while(0U)
  #define __HAL_RCC_GPIOC_CLK_ENABLE()   do { RCC->AHB1ENR |= (1UL << 2); } while(0U)
#endif

uint32_t HAL_RCC_GetSysClockFreq(void);
uint32_t HAL_RCC_GetHCLKFreq(void);

#ifdef __cplusplus
}
#endif

#endif /* __STM32_HAL_RCC_H */

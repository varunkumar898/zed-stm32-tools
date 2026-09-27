/**
 * @file    stm32_device.h
 * @brief   Universal STM32 CMSIS Device Peripheral Dispatcher & Register Definitions
 * Supports all 17 STM32 families with Cortex-M Core header dispatch and peripheral map.
 */

#ifndef __STM32_DEVICE_H
#define __STM32_DEVICE_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/* --------------------------------------------------------------------------
 * Core Selection & CMSIS Header Inclusion
 * -------------------------------------------------------------------------- */
#if defined(STM32F0)
  #include "core_cm0.h"
#elif defined(STM32G0) || defined(STM32L0)
  #include "core_cm0plus.h"
#elif defined(STM32F1) || defined(STM32F2) || defined(STM32L1)
  #include "core_cm3.h"
#elif defined(STM32F3) || defined(STM32F4) || defined(STM32G4) || defined(STM32L4) || \
      defined(STM32WB) || defined(STM32WL)
  #include "core_cm4.h"
#elif defined(STM32F7) || defined(STM32H7)
  #include "core_cm7.h"
#elif defined(STM32H5) || defined(STM32L5) || defined(STM32U5)
  #include "core_cm33.h"
#else
  #include "core_cm4.h"
#endif

/* --------------------------------------------------------------------------
 * Memory Base Addresses
 * -------------------------------------------------------------------------- */
#define FLASH_BASE            (0x08000000UL)

#if defined(STM32H7)
  #define SRAM1_BASE          (0x24000000UL) /* AXI SRAM on STM32H7 */
#else
  #define SRAM1_BASE          (0x20000000UL) /* Standard SRAM on Cortex-M */
#endif

#define PERIPH_BASE           (0x40000000UL)

/* --------------------------------------------------------------------------
 * Universal GPIO Register Definition
 * -------------------------------------------------------------------------- */
typedef struct {
  __IO uint32_t MODER;    /*!< GPIO port mode register,               Address offset: 0x00 */
  __IO uint32_t OTYPER;   /*!< GPIO port output type register,        Address offset: 0x04 */
  __IO uint32_t OSPEEDR;  /*!< GPIO port output speed register,       Address offset: 0x08 */
  __IO uint32_t PUPDR;    /*!< GPIO port pull-up/pull-down register,  Address offset: 0x0C */
  __IO uint32_t IDR;      /*!< GPIO port input data register,         Address offset: 0x10 */
  __IO uint32_t ODR;      /*!< GPIO port output data register,        Address offset: 0x14 */
  __IO uint32_t BSRR;     /*!< GPIO port bit set/reset register,      Address offset: 0x18 */
  __IO uint32_t LCKR;     /*!< GPIO port configuration lock register, Address offset: 0x1C */
  __IO uint32_t AFR[2];   /*!< GPIO alternate function registers,     Address offset: 0x20-0x24 */
} GPIO_TypeDef;

/* --------------------------------------------------------------------------
 * Family-specific GPIO / Peripheral Base Mapping
 * -------------------------------------------------------------------------- */
#if defined(STM32F1)
  #define GPIOA_BASE          (PERIPH_BASE + 0x00010800UL)
  #define GPIOB_BASE          (PERIPH_BASE + 0x00010C00UL)
  #define GPIOC_BASE          (PERIPH_BASE + 0x00011000UL)
  #define GPIOD_BASE          (PERIPH_BASE + 0x00011400UL)
  #define RCC_BASE            (PERIPH_BASE + 0x00021000UL)
#elif defined(STM32F4) || defined(STM32F2) || defined(STM32F7)
  #define AHB1PERIPH_BASE     (PERIPH_BASE + 0x00020000UL)
  #define GPIOA_BASE          (AHB1PERIPH_BASE + 0x0000UL)
  #define GPIOB_BASE          (AHB1PERIPH_BASE + 0x0400UL)
  #define GPIOC_BASE          (AHB1PERIPH_BASE + 0x0800UL)
  #define GPIOD_BASE          (AHB1PERIPH_BASE + 0x0C00UL)
  #define GPIOE_BASE          (AHB1PERIPH_BASE + 0x1000UL)
  #define GPIOH_BASE          (AHB1PERIPH_BASE + 0x1C00UL)
  #define RCC_BASE            (AHB1PERIPH_BASE + 0x3800UL)
#elif defined(STM32H7)
  #define D3_AHB1PERIPH_BASE  (0x58020000UL)
  #define GPIOA_BASE          (D3_AHB1PERIPH_BASE + 0x0000UL)
  #define GPIOB_BASE          (D3_AHB1PERIPH_BASE + 0x0400UL)
  #define GPIOC_BASE          (D3_AHB1PERIPH_BASE + 0x0800UL)
  #define RCC_BASE            (0x58024400UL)
#elif defined(STM32G0) || defined(STM32G4) || defined(STM32L4) || defined(STM32U5) || \
      defined(STM32H5) || defined(STM32WB) || defined(STM32WL)
  #define IOPORT_BASE         (0x48000000UL)
  #define GPIOA_BASE          (IOPORT_BASE + 0x0000UL)
  #define GPIOB_BASE          (IOPORT_BASE + 0x0400UL)
  #define GPIOC_BASE          (IOPORT_BASE + 0x0800UL)
  #define RCC_BASE            (PERIPH_BASE + 0x00021000UL)
#else
  #define GPIOA_BASE          (PERIPH_BASE + 0x00020000UL)
  #define GPIOB_BASE          (PERIPH_BASE + 0x00020400UL)
  #define GPIOC_BASE          (PERIPH_BASE + 0x00020800UL)
  #define RCC_BASE            (PERIPH_BASE + 0x00021000UL)
#endif

#define GPIOA                 ((GPIO_TypeDef *) GPIOA_BASE)
#define GPIOB                 ((GPIO_TypeDef *) GPIOB_BASE)
#define GPIOC                 ((GPIO_TypeDef *) GPIOC_BASE)

/* --------------------------------------------------------------------------
 * RCC Register Definition (Generic minimal layout)
 * -------------------------------------------------------------------------- */
typedef struct {
  __IO uint32_t CR;            /*!< RCC clock control register */
  __IO uint32_t PLLCFGR;       /*!< RCC PLL configuration register */
  __IO uint32_t CFGR;          /*!< RCC clock configuration register */
  __IO uint32_t CIR;           /*!< RCC clock interrupt register */
  __IO uint32_t AHB1RSTR;      /*!< RCC AHB1 peripheral reset register */
  __IO uint32_t AHB2RSTR;      /*!< RCC AHB2 peripheral reset register */
  __IO uint32_t AHB3RSTR;      /*!< RCC AHB3 peripheral reset register */
        uint32_t RESERVED0;
  __IO uint32_t APB1RSTR;      /*!< RCC APB1 peripheral reset register */
  __IO uint32_t APB2RSTR;      /*!< RCC APB2 peripheral reset register */
        uint32_t RESERVED1[2];
  __IO uint32_t AHB1ENR;       /*!< RCC AHB1 peripheral clock register */
  __IO uint32_t AHB2ENR;       /*!< RCC AHB2 peripheral clock register */
  __IO uint32_t AHB3ENR;       /*!< RCC AHB3 peripheral clock register */
        uint32_t RESERVED2;
  __IO uint32_t APB1ENR;       /*!< RCC APB1 peripheral clock register */
  __IO uint32_t APB2ENR;       /*!< RCC APB2 peripheral clock register */
} RCC_TypeDef;

#define RCC                   ((RCC_TypeDef *) RCC_BASE)

#ifdef __cplusplus
}
#endif

#endif /* __STM32_DEVICE_H */

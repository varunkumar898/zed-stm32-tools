/**
 * @file    stm32_hal_gpio.h
 * @brief   Universal STM32 HAL GPIO Driver Header
 */

#ifndef __STM32_HAL_GPIO_H
#define __STM32_HAL_GPIO_H

#ifdef __cplusplus
extern "C" {
#endif

#include "stm32_hal_def.h"

/**
 * @brief GPIO Configuration Structure
 */
typedef struct {
  uint32_t Pin;        /*!< Specifies the GPIO pins to be configured. */
  uint32_t Mode;       /*!< Specifies the operating mode for the selected pins. */
  uint32_t Pull;       /*!< Specifies the Pull-up or Pull-Down activation. */
  uint32_t Speed;      /*!< Specifies the speed for the selected pins. */
  uint32_t Alternate;  /*!< Peripheral to be connected to the selected pins. */
} GPIO_InitTypeDef;

typedef enum {
  GPIO_PIN_RESET = 0U,
  GPIO_PIN_SET
} GPIO_PinState;

/* GPIO Pin identifiers */
#define GPIO_PIN_0                 ((uint16_t)0x0001)
#define GPIO_PIN_1                 ((uint16_t)0x0002)
#define GPIO_PIN_2                 ((uint16_t)0x0004)
#define GPIO_PIN_3                 ((uint16_t)0x0008)
#define GPIO_PIN_4                 ((uint16_t)0x0010)
#define GPIO_PIN_5                 ((uint16_t)0x0020)
#define GPIO_PIN_6                 ((uint16_t)0x0040)
#define GPIO_PIN_7                 ((uint16_t)0x0080)
#define GPIO_PIN_8                 ((uint16_t)0x0100)
#define GPIO_PIN_9                 ((uint16_t)0x0200)
#define GPIO_PIN_10                ((uint16_t)0x0400)
#define GPIO_PIN_11                ((uint16_t)0x0800)
#define GPIO_PIN_12                ((uint16_t)0x1000)
#define GPIO_PIN_13                ((uint16_t)0x2000)
#define GPIO_PIN_14                ((uint16_t)0x4000)
#define GPIO_PIN_15                ((uint16_t)0x8000)
#define GPIO_PIN_All               ((uint16_t)0xFFFF)

/* GPIO Modes */
#define GPIO_MODE_INPUT            (0x00000000U)
#define GPIO_MODE_OUTPUT_PP        (0x00000001U)
#define GPIO_MODE_OUTPUT_OD        (0x00000011U)
#define GPIO_MODE_AF_PP            (0x00000002U)
#define GPIO_MODE_AF_OD            (0x00000012U)
#define GPIO_MODE_ANALOG           (0x00000003U)

/* GPIO Pull settings */
#define GPIO_NOPULL                (0x00000000U)
#define GPIO_PULLUP                (0x00000001U)
#define GPIO_PULLDOWN              (0x00000002U)

/* GPIO Speed settings */
#define GPIO_SPEED_FREQ_LOW        (0x00000000U)
#define GPIO_SPEED_FREQ_MEDIUM     (0x00000001U)
#define GPIO_SPEED_FREQ_HIGH       (0x00000002U)
#define GPIO_SPEED_FREQ_VERY_HIGH  (0x00000003U)

/* Public Function Prototypes */
void HAL_GPIO_Init(GPIO_TypeDef *GPIOx, GPIO_InitTypeDef *GPIO_Init);
void HAL_GPIO_DeInit(GPIO_TypeDef *GPIOx, uint32_t GPIO_Pin);
GPIO_PinState HAL_GPIO_ReadPin(GPIO_TypeDef *GPIOx, uint16_t GPIO_Pin);
void HAL_GPIO_WritePin(GPIO_TypeDef *GPIOx, uint16_t GPIO_Pin, GPIO_PinState PinState);
void HAL_GPIO_TogglePin(GPIO_TypeDef *GPIOx, uint16_t GPIO_Pin);

#ifdef __cplusplus
}
#endif

#endif /* __STM32_HAL_GPIO_H */

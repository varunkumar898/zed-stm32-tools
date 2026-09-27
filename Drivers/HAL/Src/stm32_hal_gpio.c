/**
 * @file    stm32_hal_gpio.c
 * @brief   Universal STM32 HAL GPIO Driver Implementation
 */

#include "stm32_hal.h"

void HAL_GPIO_Init(GPIO_TypeDef *GPIOx, GPIO_InitTypeDef *GPIO_Init) {
    uint32_t position;
    uint32_t ioposition;
    uint32_t iocurrent;

    if (GPIOx == NULL || GPIO_Init == NULL) {
        return;
    }

    for (position = 0U; position < 16U; position++) {
        ioposition = 1U << position;
        iocurrent = (GPIO_Init->Pin) & ioposition;

        if (iocurrent == ioposition) {
            /* 1. Configure Mode (2 bits per pin) */
            uint32_t mode = GPIO_Init->Mode & 0x03U;
            GPIOx->MODER &= ~(0x03U << (position * 2U));
            GPIOx->MODER |=  (mode << (position * 2U));

            /* 2. Configure Output type (Push-Pull or Open-Drain) */
            if ((GPIO_Init->Mode == GPIO_MODE_OUTPUT_OD) || (GPIO_Init->Mode == GPIO_MODE_AF_OD)) {
                GPIOx->OTYPER |= (1U << position);
            } else {
                GPIOx->OTYPER &= ~(1U << position);
            }

            /* 3. Configure Speed */
            GPIOx->OSPEEDR &= ~(0x03U << (position * 2U));
            GPIOx->OSPEEDR |=  ((GPIO_Init->Speed & 0x03U) << (position * 2U));

            /* 4. Configure Pull-up / Pull-down */
            GPIOx->PUPDR &= ~(0x03U << (position * 2U));
            GPIOx->PUPDR |=  ((GPIO_Init->Pull & 0x03U) << (position * 2U));

            /* 5. Configure Alternate Function (AFR[0] for pins 0..7, AFR[1] for 8..15) */
            if ((GPIO_Init->Mode == GPIO_MODE_AF_PP) || (GPIO_Init->Mode == GPIO_MODE_AF_OD)) {
                uint32_t afrIndex = position >> 3U;
                uint32_t afrOffset = (position & 0x07U) * 4U;
                GPIOx->AFR[afrIndex] &= ~(0x0FU << afrOffset);
                GPIOx->AFR[afrIndex] |=  ((GPIO_Init->Alternate & 0x0FU) << afrOffset);
            }
        }
    }
}

void HAL_GPIO_DeInit(GPIO_TypeDef *GPIOx, uint32_t GPIO_Pin) {
    if (GPIOx == NULL) return;
    for (uint32_t position = 0U; position < 16U; position++) {
        if (GPIO_Pin & (1U << position)) {
            GPIOx->MODER &= ~(0x03U << (position * 2U));
            GPIOx->PUPDR &= ~(0x03U << (position * 2U));
            GPIOx->OTYPER &= ~(1U << position);
            GPIOx->OSPEEDR &= ~(0x03U << (position * 2U));
        }
    }
}

GPIO_PinState HAL_GPIO_ReadPin(GPIO_TypeDef *GPIOx, uint16_t GPIO_Pin) {
    if (GPIOx == NULL) return GPIO_PIN_RESET;
    return ((GPIOx->IDR & GPIO_Pin) != 0U) ? GPIO_PIN_SET : GPIO_PIN_RESET;
}

void HAL_GPIO_WritePin(GPIO_TypeDef *GPIOx, uint16_t GPIO_Pin, GPIO_PinState PinState) {
    if (GPIOx == NULL) return;
    if (PinState != GPIO_PIN_RESET) {
        GPIOx->BSRR = (uint32_t)GPIO_Pin;
    } else {
        GPIOx->BSRR = (uint32_t)GPIO_Pin << 16U;
    }
}

void HAL_GPIO_TogglePin(GPIO_TypeDef *GPIOx, uint16_t GPIO_Pin) {
    if (GPIOx == NULL) return;
    uint32_t odr = GPIOx->ODR;
    GPIOx->BSRR = ((odr & GPIO_Pin) << 16U) | (~odr & GPIO_Pin);
}

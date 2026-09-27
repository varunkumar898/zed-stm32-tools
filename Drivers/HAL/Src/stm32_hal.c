/**
 * @file    stm32_hal.c
 * @brief   Universal STM32 HAL Core Implementation
 */

#include "stm32_hal.h"

static volatile uint32_t uwTick = 0U;
static uint32_t uwTickPrio = 0x0FU;

/**
 * @brief  Initializes the HAL Library; initializes the Flash interface,
 *         the Systick timer, and configures priority grouping.
 */
HAL_StatusTypeDef HAL_Init(void) {
    /* Initialize low-level hardware / clocks */
    HAL_MspInit();

    /* Initialize 1ms SysTick time base */
    if (HAL_InitTick(uwTickPrio) != HAL_OK) {
        return HAL_ERROR;
    }

    return HAL_OK;
}

/**
 * @brief  De-initializes common HAL components.
 */
HAL_StatusTypeDef HAL_DeInit(void) {
    uwTick = 0U;
    return HAL_OK;
}

/**
 * @brief  Initializes the MSP (Microcontroller Support Package).
 *         Weak implementation to be overridden in user code if needed.
 */
__attribute__((weak)) void HAL_MspInit(void) {
    /* Optional user MSP initialization */
}

/**
 * @brief  De-initializes the MSP.
 */
__attribute__((weak)) void HAL_MspDeInit(void) {
    /* Optional user MSP de-initialization */
}

/**
 * @brief  Configures the SysTick to generate an interrupt every 1 millisecond.
 */
HAL_StatusTypeDef HAL_InitTick(uint32_t TickPriority) {
    /* Configure SysTick for 1000 Hz ticks */
    if (SysTick_Config(SystemCoreClock / 1000U) != 0U) {
        return HAL_ERROR;
    }

    uwTickPrio = TickPriority;
    return HAL_OK;
}

/**
 * @brief  Increment tick counter (called from SysTick_Handler).
 */
void HAL_IncTick(void) {
    uwTick++;
}

/**
 * @brief  Returns the current system tick count in milliseconds.
 */
uint32_t HAL_GetTick(void) {
    return uwTick;
}

/**
 * @brief  Provides a blocking delay in milliseconds.
 */
void HAL_Delay(uint32_t Delay) {
    uint32_t tickstart = HAL_GetTick();
    uint32_t wait = Delay;

    if (wait < 0xFFFFFFFFU) {
        wait += 1U;
    }

    while ((HAL_GetTick() - tickstart) < wait) {
        __NOP();
    }
}

void HAL_SuspendTick(void) {
    SysTick->CTRL &= ~SysTick_CTRL_TICKINT_Msk;
}

void HAL_ResumeTick(void) {
    SysTick->CTRL |= SysTick_CTRL_TICKINT_Msk;
}

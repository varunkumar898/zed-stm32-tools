/**
 * @file    stm32_hal_cortex.c
 * @brief   Universal STM32 HAL Cortex Driver Implementation
 */

#include "stm32_hal.h"

void HAL_NVIC_SetPriority(int32_t IRQn, uint32_t PreemptPriority, uint32_t SubPriority) {
    UNUSED(SubPriority);
    NVIC_SetPriority(IRQn, PreemptPriority);
}

void HAL_NVIC_EnableIRQ(int32_t IRQn) {
    NVIC_EnableIRQ(IRQn);
}

void HAL_NVIC_DisableIRQ(int32_t IRQn) {
    NVIC_DisableIRQ(IRQn);
}

void HAL_NVIC_SystemReset(void) {
    __DSB();
    SCB->AIRCR = ((0x5FAUL << 16U) | (SCB->AIRCR & (0x700UL)) | (1UL << 2U));
    __DSB();
    while (1) {
        __NOP();
    }
}

uint32_t HAL_SYSTICK_Config(uint32_t TicksNumb) {
    return SysTick_Config(TicksNumb);
}

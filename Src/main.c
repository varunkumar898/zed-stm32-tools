/**
 * ==============================================================================
 * @file    main.c
 * @brief   Universal STM32 Application Entry Point
 *
 * Dual Mode Architecture:
 *   - Bare-Metal Mode (USE_HAL = OFF): Zero-dependency direct register access.
 *   - STM32 HAL Mode  (USE_HAL = ON) : Full CMSIS and HAL driver integration.
 * ==============================================================================
 */

#include <stdint.h>
#include "system_stm32.h"

#if defined(USE_HAL) && (USE_HAL == 1)
/* ==============================================================================
 * STM32 HAL C DEVELOPMENT MODE (USE_HAL = ON)
 * ============================================================================== */
#include "stm32_hal.h"

/**
 * @brief SysTick Interrupt Handler - increments the HAL millisecond tick counter.
 */
void SysTick_Handler(void) {
    HAL_IncTick();
}

/**
 * @brief System Clock Configuration (HSI default / PLL setup)
 */
static void SystemClock_Config(void) {
    /* Update Core Clock variable with selected oscillator */
    SystemCoreClockUpdate();
}

/**
 * @brief GPIO Initialization: Configure PA5 (standard user LED on Nucleo boards)
 */
static void MX_GPIO_Init(void) {
    GPIO_InitTypeDef GPIO_InitStruct = {0};

    /* Enable peripheral clock for GPIOA */
    __HAL_RCC_GPIOA_CLK_ENABLE();

    /* Initialize PA5 in Push-Pull Output mode */
    HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, GPIO_PIN_RESET);
    GPIO_InitStruct.Pin = GPIO_PIN_5;
    GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
    HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);
}

/**
 * @brief Application entry point for HAL mode.
 */
int main(void) {
    /* 1. Reset all peripherals, initialize Flash interface and SysTick */
    HAL_Init();

    /* 2. Configure the system clock */
    SystemClock_Config();

    /* 3. Initialize GPIO peripherals */
    MX_GPIO_Init();

    /* 4. Main application infinite loop */
    while (1) {
        /* Toggle User LED (PA5) */
        HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_5);

        /* 500 ms non-blocking time-base delay */
        HAL_Delay(500);
    }

    return 0;
}

#else
/* ==============================================================================
 * BARE-METAL REGISTER-LEVEL C MODE (USE_HAL = OFF)
 * Zero external library dependencies, maximum speed, minimal binary footprint.
 * ============================================================================== */

#include "stm32_device.h"

/**
 * @brief Calibrated software delay loop using volatile assembly NOPs.
 */
static void delay_cycles(volatile uint32_t count) {
    while (count--) {
        __asm volatile ("nop");
    }
}

/**
 * @brief Direct hardware register configuration for LED toggle.
 */
static void baremetal_gpio_init(void) {
#if defined(STM32F1)
    /* STM32F1: RCC APB2ENR bit 2 = IOPB EN, bit 4 = IOPC EN */
    volatile uint32_t *rcc_apb2enr = (volatile uint32_t *)(RCC_BASE + 0x18UL);
    *rcc_apb2enr |= (1UL << 2) | (1UL << 4); /* Enable GPIOA & GPIOC */

    /* Configure PC13 as Output Push-Pull 2MHz (BluePill onboard LED) */
    volatile uint32_t *gpioc_crh = (volatile uint32_t *)(GPIOC_BASE + 0x04UL);
    *gpioc_crh &= ~(0xFUL << 20);
    *gpioc_crh |=  (0x2UL << 20);

#elif defined(STM32F4) || defined(STM32F2) || defined(STM32F7)
    /* STM32F4/F2/F7: RCC AHB1ENR bit 0 = GPIOAEN */
    volatile uint32_t *rcc_ahb1enr = (volatile uint32_t *)(RCC_BASE + 0x30UL);
    *rcc_ahb1enr |= (1UL << 0);

    /* Configure PA5 as Output (MODER[11:10] = 01) */
    GPIOA->MODER &= ~(3UL << 10);
    GPIOA->MODER |=  (1UL << 10);

#else
    /* Universal fallback: GPIOA clock enable & MODER config */
    volatile uint32_t *rcc_ahb = (volatile uint32_t *)(RCC_BASE + 0x30UL);
    *rcc_ahb |= (1UL << 0);

    GPIOA->MODER &= ~(3UL << 10);
    GPIOA->MODER |=  (1UL << 10);
#endif
}

/**
 * @brief Application entry point for Bare-Metal mode.
 */
int main(void) {
    /* 1. Update Core Clock reference */
    SystemCoreClockUpdate();

    /* 2. Configure GPIO hardware registers */
    baremetal_gpio_init();

    /* 3. Main execution loop */
    while (1) {
#if defined(STM32F1)
        /* Toggle PC13 (BluePill LED) */
        GPIOC->ODR ^= (1UL << 13);
#else
        /* Toggle PA5 (Nucleo LED) */
        GPIOA->ODR ^= (1UL << 5);
#endif

        /* Delay approximately 500 ms */
        delay_cycles(400000UL);
    }

    return 0;
}

#endif /* USE_HAL */

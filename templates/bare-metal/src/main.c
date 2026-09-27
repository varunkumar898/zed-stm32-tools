/**
 * @file main.c
 * @brief STM32 Bare-Metal Register-Level Blinky Example
 */

#include <stdint.h>

#define RCC_BASE      0x40023800UL
#define RCC_AHB1ENR   (*(volatile uint32_t *)(RCC_BASE + 0x30))

#define GPIOA_BASE    0x40020000UL
#define GPIOA_MODER   (*(volatile uint32_t *)(GPIOA_BASE + 0x00))
#define GPIOA_ODR     (*(volatile uint32_t *)(GPIOA_BASE + 0x14))

static void delay_cycles(volatile uint32_t cycles) {
    while (cycles--) {
        __asm volatile ("nop");
    }
}

int main(void) {
    /* 1. Enable GPIOA peripheral clock */
    RCC_AHB1ENR |= (1U << 0);

    /* 2. Configure PA5 as General Purpose Output (01 in MODER[11:10]) */
    GPIOA_MODER &= ~(3U << 10);
    GPIOA_MODER |=  (1U << 10);

    while (1) {
        /* 3. Toggle PA5 (LED on Nucleo-64 F401RE / F411RE / F446RE) */
        GPIOA_ODR ^= (1U << 5);
        delay_cycles(500000);
    }

    return 0;
}

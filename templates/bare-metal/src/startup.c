/**
 * @file startup.c
 * @brief Cortex-M Minimal Vector Table and Reset Handler
 */

#include <stdint.h>

extern uint32_t _estack;
extern uint32_t _sidata;
extern uint32_t _sdata;
extern uint32_t _edata;
extern uint32_t _sbss;
extern uint32_t _ebss;

extern int main(void);

void Reset_Handler(void) {
    uint32_t *pSrc = &_sidata;
    uint32_t *pDst = &_sdata;

    /* Copy .data segment from Flash to RAM */
    while (pDst < &_edata) {
        *pDst++ = *pSrc++;
    }

    /* Zero fill .bss segment in RAM */
    pDst = &_sbss;
    while (pDst < &_ebss) {
        *pDst++ = 0;
    }

    main();

    while (1) {}
}

void Default_Handler(void) {
    while (1) {}
}

__attribute__((weak, alias("Default_Handler"))) void NMI_Handler(void);
__attribute__((weak, alias("Default_Handler"))) void HardFault_Handler(void);

__attribute__((section(".isr_vector"), used))
const uint32_t g_pfnVectors[] = {
    (uint32_t)&_estack,
    (uint32_t)&Reset_Handler,
    (uint32_t)&NMI_Handler,
    (uint32_t)&HardFault_Handler,
};

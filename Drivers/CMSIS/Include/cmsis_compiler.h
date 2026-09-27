/**
 * @file    cmsis_compiler.h
 * @brief   CMSIS compiler generic header
 */

#ifndef __CMSIS_COMPILER_H
#define __CMSIS_COMPILER_H

#include <stdint.h>

#if defined(__GNUC__)
  #include "cmsis_gcc.h"
#else
  #error "Unsupported toolchain compiler"
#endif

#endif /* __CMSIS_COMPILER_H */

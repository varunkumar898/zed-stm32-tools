/**
 * ==============================================================================
 * @file    syscalls.c
 * @brief   Minimal Newlib System Call Implementations for Embedded ARM Cortex-M
 * ==============================================================================
 */

#include <sys/stat.h>
#include <stdlib.h>
#include <errno.h>
#include <stdio.h>
#include <signal.h>
#include <time.h>
#include <sys/time.h>
#include <sys/times.h>

#undef errno
extern int errno;

extern int __io_putchar(int ch) __attribute__((weak));
extern int __io_getchar(void) __attribute__((weak));

__attribute__((weak))
caddr_t _sbrk(int incr) {
    extern char end asm("end");
    static char *heap_end;
    char *prev_heap_end;
    char *sp;

    __asm volatile ("mov %0, sp" : "=r" (sp));

    if (heap_end == 0) {
        heap_end = &end;
    }

    prev_heap_end = heap_end;
    if (heap_end + incr > sp) {
        errno = ENOMEM;
        return (caddr_t)-1;
    }

    heap_end += incr;
    return (caddr_t)prev_heap_end;
}

__attribute__((weak))
int _close(int file) {
    (void)file;
    return -1;
}

__attribute__((weak))
int _fstat(int file, struct stat *st) {
    (void)file;
    st->st_mode = S_IFCHR;
    return 0;
}

__attribute__((weak))
int _isatty(int file) {
    (void)file;
    return 1;
}

__attribute__((weak))
int _lseek(int file, int ptr, int dir) {
    (void)file;
    (void)ptr;
    (void)dir;
    return 0;
}

__attribute__((weak))
int _read(int file, char *ptr, int len) {
    (void)file;
    int DataIdx;
    for (DataIdx = 0; DataIdx < len; DataIdx++) {
        if (__io_getchar) {
            *ptr++ = (char)__io_getchar();
        } else {
            *ptr++ = 0;
        }
    }
    return len;
}

__attribute__((weak))
int _write(int file, char *ptr, int len) {
    (void)file;
    int DataIdx;
    for (DataIdx = 0; DataIdx < len; DataIdx++) {
        if (__io_putchar) {
            __io_putchar(*ptr++);
        } else {
            ptr++;
        }
    }
    return len;
}

__attribute__((weak))
void _exit(int status) {
    (void)status;
    while (1) {}
}

__attribute__((weak))
int _kill(int pid, int sig) {
    (void)pid;
    (void)sig;
    errno = EINVAL;
    return -1;
}

__attribute__((weak))
int _getpid(void) {
    return 1;
}

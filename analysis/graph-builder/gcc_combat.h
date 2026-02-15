/* analysis/graph-builder/gcc_compat.h */
#ifndef GCC_COMPAT_H
#define GCC_COMPAT_H

// 1. Neutralisiere GCC Built-ins
#define __builtin_va_list void*
#define __builtin_va_start(ap, last)
#define __builtin_va_end(ap)
#define __builtin_va_arg(ap, type) ((type)0)
#define __builtin_expect(exp, c) (exp)
#define __inline inline
#define __restrict restrict
#define __extension__
#define __attribute__(x)

// 2. Standard-Typen definieren, falls System-Header fehlen
typedef int size_t;
typedef int ptrdiff_t;
typedef int wchar_t;
typedef void* FILE;
typedef void* va_list;

// 3. Häufige GCC-spezifische Makros
#define __asm__(x) 
#define __volatile__(x)

#endif
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>   // ✅ für INT_MAX / INT_MIN

/* ================= SAFE FUNCTIONS ================= */

/* Sicheres Ausgeben eines Strings */

#include "safe.h"

void safe_print(const char *msg) {
    if (msg) {
        printf("[SAFE PRINT] %s\n", msg);
    }
}

void safe_copy(char *dest, size_t dest_size, const char *src) {
    if (!dest || !src || dest_size == 0) {
        return;
    }
    strncpy(dest, src, dest_size - 1);
    dest[dest_size - 1] = '\0';
}

int safe_add(int a, int b, int *out) {
    if (!out) return 0;
    if ((b > 0 && a > INT_MAX - b) ||
        (b < 0 && a < INT_MIN - b)) {
        return 0; // overflow
    }
    *out = a + b;
    return 1;
}

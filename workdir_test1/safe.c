#include <stdio.h>
#include <string.h>

/* ================= SAFE FUNCTION 1 ================= */
/* Gibt einen String sicher aus (keine Speicheroperation) */

#include "safe.h"

void safe_print(const char *text) {
    if (text != NULL) {
        printf("Text: %s\n", text);
    }
}

void safe_copy(char *dest, size_t dest_size, const char *src) {
    if (dest != NULL && src != NULL && dest_size > 0) {
        strncpy(dest, src, dest_size - 1);
        dest[dest_size - 1] = '\0'; // garantiert Null-Terminierung
    }
}

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>   // ✅ für INT_MAX / INT_MIN

/* ================= SAFE FUNCTIONS ================= */

/* Sicheres Ausgeben eines Strings */

#include "safe.h"

void unsafe_concat(char *dest, const char *src) {
    strcat(dest, src); // ❌ Buffer Overflow möglich
}

int main(void) {
    char buffer[16];
    int sum;

    safe_print("Start example2");

    safe_copy(buffer, sizeof(buffer), "Hello");
    unsafe_concat(buffer, "World!!!"); // potenziell unsicher

    safe_print(buffer);

    if (safe_add(100, 23, &sum)) {
        printf("Sum: %d\n", sum);
    } else {
        printf("Overflow detected\n");
    }

    return 0;
}

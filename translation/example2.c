#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>   // ✅ für INT_MAX / INT_MIN

/* ================= SAFE FUNCTIONS ================= */

/* Sicheres Ausgeben eines Strings */
void safe_print(const char *msg) {
    if (msg) {
        printf("[SAFE PRINT] %s\n", msg);
    }
}

/* Sicheres Kopieren mit Längenprüfung */
void safe_copy(char *dest, size_t dest_size, const char *src) {
    if (!dest || !src || dest_size == 0) {
        return;
    }
    strncpy(dest, src, dest_size - 1);
    dest[dest_size - 1] = '\0';
}

/* Sichere Addition mit Überlaufprüfung */
int safe_add(int a, int b, int *out) {
    if (!out) return 0;
    if ((b > 0 && a > INT_MAX - b) ||
        (b < 0 && a < INT_MIN - b)) {
        return 0; // overflow
    }
    *out = a + b;
    return 1;
}

/* ================= UNSAFE FUNCTIONS ================= */

/* UNSICHER: kein Bounds-Check */
void unsafe_concat(char *dest, const char *src) {
    strcat(dest, src); // ❌ Buffer Overflow möglich
}

/* ================= MAIN (UNSAFE) ================= */

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

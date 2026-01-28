#include <stdio.h>
#include <string.h>

/* ================= SAFE FUNCTION 1 ================= */
/* Gibt einen String sicher aus (keine Speicheroperation) */

#include "safe.h"

void unsafe_copy(char *dest, const char *src) {
    /* KEINE Längenprüfung */
    strcpy(dest, src);  // ❌ unsicher
}

int main(void) {
    char buffer1[10];
    char buffer2[10];

    safe_print("Hallo");

    safe_copy(buffer1, sizeof(buffer1), "HalloWelt123");
    printf("buffer1: %s\n", buffer1);

    /* Achtung: Demonstration einer unsicheren Funktion */
    unsafe_copy(buffer2, "HalloWelt123"); // kann Speicher überschreiben
    printf("buffer2: %s\n", buffer2);

    return 0;
}

#include <stdio.h>
#include <string.h>

/* ================= SAFE FUNCTION 1 ================= */
/* Gibt einen String sicher aus (keine Speicheroperation) */
void safe_print(const char *text) {
    if (text != NULL) {
        printf("Text: %s\n", text);
    }
}

/* ================= SAFE FUNCTION 2 ================= */
/* Kopiert einen String mit Längenbegrenzung */
void safe_copy(char *dest, size_t dest_size, const char *src) {
    if (dest != NULL && src != NULL && dest_size > 0) {
        strncpy(dest, src, dest_size - 1);
        dest[dest_size - 1] = '\0'; // garantiert Null-Terminierung
    }
}

/* ================= UNSAFE FUNCTION ================= */
/* UNSICHER: Kann Buffer Overflow verursachen */
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

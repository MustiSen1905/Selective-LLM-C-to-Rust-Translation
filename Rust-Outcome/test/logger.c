#include <stdio.h>
#include <string.h>
#include "logger.h"

void log_message(const char* msg) {
    FILE *f = fopen("system.log", "a");
    if (f == NULL) return;

    // UNSAFE: fprintf ohne Format-Validierung
    // Wenn msg "%s" enthält, kracht es hier eventuell.
    fprintf(f, msg);
    fprintf(f, "\n");
    fclose(f);
}

void read_log_unsafe(char* user_buffer) {
    FILE *f = fopen("system.log", "r");
    if (f == NULL) return;

    // UNSAFE: fscanf ohne Breitenbegrenzung
    // Liest das File in den Buffer, bis ein Leerzeichen kommt, 
    // egal wie groß der Buffer ist.
    fscanf(f, "%s", user_buffer);
    fclose(f);
}
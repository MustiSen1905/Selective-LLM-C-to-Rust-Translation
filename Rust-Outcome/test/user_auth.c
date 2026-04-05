#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "user_auth.h"

User* create_user(const char* name) {
    User* u = (User*)malloc(sizeof(User));
    // UNSAFE: strcpy ohne Längenprüfung auf u->username (16 Bytes)
    strcpy(u->username, name);
    u->isAdmin = 0;
    u->session_token = (char*)malloc(32);
    strcpy(u->session_token, "INIT_TOKEN_ABC123");
    return u;
}

void delete_user(User* u) {
    if (u) {
        free(u->session_token);
        free(u);
    }
    // UNSAFE: Double Free Gefahr, wenn diese Funktion zweimal aufgerufen wird
    // oder Use-After-Free, wenn der Pointer in main.c nicht genullt wird.
}

void elevate_privileges(User* u, int level) {
    // UNSAFE: Integer Overflow Potential
    // Wenn 'level' extrem groß ist, könnte die Addition negativ werden oder 
    // unerwartete Werte annehmen, falls damit später gerechnet wird.
    u->isAdmin += level; 
    printf("Privilegien auf %d gesetzt.\n", u->isAdmin);
}
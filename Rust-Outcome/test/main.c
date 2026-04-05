#include <stdio.h>
#include <string.h>
#include "user_auth.h"
#include "logger.h"

int main() {
    char input[128];
    char log_buf[32]; // Kleiner Buffer für Demonstrationszwecke

    printf("Neuen Usernamen eingeben: ");
    scanf("%s", input); // UNSAFE: Klassischer Overflow

    User *currentUser = create_user(input);
    log_message(input);

    printf("Soll der User gelöscht werden? (1=Ja): ");
    int choice;
    scanf("%d", &choice);

    if (choice == 1) {
        delete_user(currentUser);
        // UNSAFE: currentUser wird hier NICHT auf NULL gesetzt.
    }

    // FEHLER: Use-After-Free
    // Wenn choice 1 war, greifen wir hier auf bereits befreiten Speicher zu.
    printf("Session des Users %s ist noch aktiv.\n", currentUser->username);

    // FEHLER: Buffer Overflow durch Dateiinhalt
    printf("Lese letzten Log-Eintrag...\n");
    read_log_unsafe(log_buf); 
    printf("Log: %s\n", log_buf);

    return 0;
}
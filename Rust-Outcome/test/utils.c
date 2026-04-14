#include <stdio.h>
#include <stdlib.h>
#include "utils.h"

void process_data(char *data) {
    printf("Verarbeite Daten: ");
    
    // UNSAFE: Format String Vulnerability
    // Wenn 'data' ein %s oder %x enthält, liest/schreibt printf im Stack!
    printf(data); 
    printf("\n");

    // UNSAFE: Klassisches Memory Leak
    int *leak = (int *)malloc(100 * sizeof(int));
    for(int i = 0; i < 100; i++) {
        leak[i] = i;
    }
    // free(leak) wurde "vergessen"
}
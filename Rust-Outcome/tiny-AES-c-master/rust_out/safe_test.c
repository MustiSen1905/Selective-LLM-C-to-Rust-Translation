#include <stdio.h>
#include <string.h>
#include <stdint.h>

// Enable ECB, CTR and CBC mode. Note this can be done before including aes.h or at compile-time.
// E.g. with GCC by using the -D flag: gcc -c aes.c -DCBC=0 -DCTR=1 -DECB=1
#define CBC 1
#define CTR 1
#define ECB 1

#include "aes.h"


static void phex(uint8_t* str);
static int test_encrypt_cbc(void);
static int test_decrypt_cbc(void);
static int test_encrypt_ctr(void);
static int test_decrypt_ctr(void);
static int test_encrypt_ecb(void);
static int test_decrypt_ecb(void);
static void test_encrypt_ecb_verbose(void);


int main(void)
{
    int exit;

#if defined(AES256)
    printf("\nTesting AES256\n\n");
#elif defined(AES192)
    printf("\nTesting AES192\n\n");
#elif defined(AES128)
    printf("\nTesting AES128\n\n");
#else
    printf("You need to specify a symbol between AES128, AES192 or AES256. Exiting");
    return 0;
#endif

    exit = test_encrypt_cbc() + test_decrypt_cbc() +
	test_encrypt_ctr() + test_decrypt_ctr() +
	test_decrypt_ecb() + test_encrypt_ecb();
    test_encrypt_ecb_verbose();

    return exit;
}


// prints string as hex
void phex(uint8_t* str)
;

void test_encrypt_ecb_verbose(void)
;


int test_encrypt_ecb(void)
;

int test_decrypt_cbc(void)
;

int test_encrypt_cbc(void)
;

static int test_xcrypt_ctr(const char* xcrypt);
int test_encrypt_ctr(void)
;

int test_decrypt_ctr(void)
;

int test_xcrypt_ctr(const char* xcrypt)
;


int test_decrypt_ecb(void)
;



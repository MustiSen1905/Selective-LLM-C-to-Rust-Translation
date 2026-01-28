#pragma once
#include <stddef.h>

void safe_print(const char *msg);

void safe_copy(char *dest, size_t dest_size, const char *src);

int safe_add(int a, int b, int *out);

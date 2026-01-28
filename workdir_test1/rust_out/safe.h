#pragma once
#include <stddef.h>

void safe_print(const char *text);

void safe_copy(char *dest, size_t dest_size, const char *src);

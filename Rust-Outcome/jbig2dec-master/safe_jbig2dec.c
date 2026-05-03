/* Copyright (C) 2001-2023 Artifex Software, Inc.
   All Rights Reserved.

   This software is provided AS-IS with no warranty, either express or
   implied.

   This software is distributed under license and may not be copied,
   modified or distributed except as expressly authorized under the terms
   of the license contained in the file LICENSE in this distribution.

   Refer to licensing information at http://www.artifex.com or contact
   Artifex Software, Inc.,  39 Mesa Street, Suite 108A, San Francisco,
   CA 94129, USA, for further information.
*/

/*
    jbig2dec
*/

#ifdef HAVE_CONFIG_H
#include "config.h"
#endif

#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <string.h>

#ifdef HAVE_GETOPT_H
# include <getopt.h>
#else
# include "getopt.h"
#endif

#include "os_types.h"
#include "sha1.h"

#ifdef JBIG_EXTERNAL_MEMENTO_H
#include JBIG_EXTERNAL_MEMENTO_H
#else
#include "memento.h"
#endif

#include "jbig2.h"
#include "jbig2_priv.h"
#include "jbig2_image.h"
#include "jbig2_image_rw.h"

typedef enum {
    usage, dump, render
} jbig2dec_mode;

typedef enum {
    jbig2dec_format_none,
    jbig2dec_format_jbig2,
    jbig2dec_format_pbm,
#ifdef HAVE_LIBPNG
    jbig2dec_format_png,
#endif
} jbig2dec_format;

typedef struct {
    jbig2dec_mode mode;
    int verbose, hash, embedded;
    SHA1_CTX *hash_ctx;
    char *output_filename;
    jbig2dec_format output_format;
    size_t memory_limit;
} jbig2dec_params_t;

typedef struct {
    int verbose;
    char *last_message;
    Jbig2Severity severity;
    char *type;
    long repeats;
} jbig2dec_error_callback_state_t;

typedef struct {
    Jbig2Allocator super;
    Jbig2Ctx *ctx;
    size_t memory_limit;
    size_t memory_used;
    size_t memory_peak;
} jbig2dec_allocator_t;

static int print_version(void);
static int print_usage(void);

#define ALIGNMENT 16
#define KBYTE 1024
#define MBYTE (1024 * KBYTE)

static void *jbig2dec_reached_limit(jbig2dec_allocator_t *allocator, size_t oldsize, size_t size)
{
    size_t limit_mb = allocator->memory_limit / MBYTE;
    size_t used_mb = allocator->memory_used / MBYTE;
    size_t oldsize_mb = oldsize / MBYTE;
    size_t size_mb = size / MBYTE;

    if (oldsize == 0)
        jbig2_error(allocator->ctx, JBIG2_SEVERITY_FATAL, -1, "memory: limit reached: limit: %zu (%zu Mbyte) used: %zu (%zu Mbyte) allocation: %zu (%zu Mbyte)",
            allocator->memory_limit, limit_mb,
            allocator->memory_used, used_mb,
            size, size_mb);
    else
        jbig2_error(allocator->ctx, JBIG2_SEVERITY_FATAL, -1, "memory: limit reached: limit: %zu (%zu Mbyte) used: %zu (%zu Mbyte) reallocation: %zu (%zu Mbyte) -> %zu (%zu Mbyte)",
            allocator->memory_limit, limit_mb,
            allocator->memory_used, used_mb,
            oldsize, oldsize_mb,
            size, size_mb);

    return NULL;
}

void jbig2dec_peak(jbig2dec_allocator_t *allocator)
;

static void *jbig2dec_alloc(Jbig2Allocator *allocator_, size_t size)
{
    jbig2dec_allocator_t *allocator = (jbig2dec_allocator_t *) allocator_;
    void *ptr;

    if (size == 0)
        return NULL;
    if (size > SIZE_MAX - ALIGNMENT)
        return NULL;

    if (size + ALIGNMENT > allocator->memory_limit - allocator->memory_used)
        return jbig2dec_reached_limit(allocator, 0, size + ALIGNMENT);

    ptr = malloc(size + ALIGNMENT);
    if (ptr == NULL)
        return NULL;
    memcpy(ptr, &size, sizeof(size));
    allocator->memory_used += size + ALIGNMENT;

    jbig2dec_peak(allocator);

    return (unsigned char *) ptr + ALIGNMENT;
}

void jbig2dec_free(Jbig2Allocator *allocator_, void *p)
;

static void *jbig2dec_realloc(Jbig2Allocator *allocator_, void *p, size_t size)
{
    jbig2dec_allocator_t *allocator = (jbig2dec_allocator_t *) allocator_;
    unsigned char *oldp;
    size_t oldsize;

    if (p == NULL)
        return jbig2dec_alloc(allocator_, size);
    if (p < (void *) ALIGNMENT)
        return NULL;

    if (size == 0) {
        jbig2dec_free(allocator_, p);
        return NULL;
    }
    if (size > SIZE_MAX - ALIGNMENT)
        return NULL;

    oldp = (unsigned char *) p - ALIGNMENT;
    memcpy(&oldsize, oldp, sizeof(oldsize));

    if (size + ALIGNMENT > allocator->memory_limit - allocator->memory_used + oldsize + ALIGNMENT)
        return jbig2dec_reached_limit(allocator, oldsize + ALIGNMENT, size + ALIGNMENT);

    p = realloc(oldp, size + ALIGNMENT);
    if (p == NULL)
        return NULL;

    allocator->memory_used -= oldsize + ALIGNMENT;
    memcpy(p, &size, sizeof(size));
    allocator->memory_used += size + ALIGNMENT;

    jbig2dec_peak(allocator);

    return (unsigned char *) p + ALIGNMENT;
}

/* page hashing functions */
static void
hash_init(jbig2dec_params_t *params)
;

static void
hash_image(jbig2dec_params_t *params, Jbig2Image *image)
;

static void
hash_print(jbig2dec_params_t *params, FILE *out)
;

static void
hash_free(jbig2dec_params_t *params)
;

static int
set_output_format(jbig2dec_params_t *params, const char *format)
;

static int
parse_options(int argc, char *argv[], jbig2dec_params_t *params)
;

static int
print_version(void)
;

static int
print_usage(void)
;

static void
error_callback(void *error_callback_data, const char *message, Jbig2Severity severity, uint32_t seg_idx)
;

static void
flush_errors(jbig2dec_error_callback_state_t *state)
;

static char *
make_output_filename(const char *input_filename, const char *extension)
;

static int
write_page_image(jbig2dec_params_t *params, FILE *out, Jbig2Image *image)
;

static int
write_document_hash(jbig2dec_params_t *params)
;

int
main(int argc, char **argv)
;

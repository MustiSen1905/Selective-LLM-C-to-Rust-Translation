/******************************************************************************
 * pdf.c
 *
 * pdfresurrect - PDF history extraction tool
 * https://github.com/enferex/pdfresurrect
 *
 * See https://github.com/enferex/pdfresurrect/blob/master/LICENSE for license
 * information.
 * SPDX-License-Identifier: BSD-3-Clause
 *
 * Special thanks to all of the contributors:  See AUTHORS.
 * Special thanks to 757labs (757 crew), they are a great group
 * of people to hack on projects and brainstorm with.
 *****************************************************************************/

#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "pdf.h"
#include "main.h"


/*
 * Macros
 */

/* SAFE_F
 *
 * Safe file read: use for fgetc() calls, this is really ugly looking.
 * _fp:   FILE * handle
 * _expr: The expression with fgetc() in it:
 *
 *  example: If we get a character from the file and it is ascii character 'a'
 *           This assumes the coder wants to store the 'a' in variable ch
 *           Kinda pointless if you already know that you have 'a', but for
 *           illustrative purposes.
 *
 *  if (SAFE_F(my_fp, ((c=fgetc(my_fp)) == 'a')))
 *              do_way_cool_stuff();
 */
#define SAFE_F(_fp, _expr) \
    ((!ferror(_fp) && !feof(_fp) && (_expr)))


/* FAIL
 *
 * Emit the diagnostic '_msg' and exit.
 * _msg: Message to emit prior to exiting.
 */
#define FAIL(_msg)      \
  do {                  \
    ERR(_msg);          \
    exit(EXIT_FAILURE); \
  } while (0)


/* SAFE_E
 *
 * Safe expression handling.  This macro is a wrapper
 * that compares the result of an expression (_expr) to the expected
 * value (_cmp).
 *
 * _expr: Expression to test.
 * _cmp:  Expected value, error if this returns false.
 * _msg:  What to say when an error occurs.
 */
#define SAFE_E(_expr, _cmp, _msg) \
 do {                             \
    if ((_expr) != (_cmp)) {      \
      FAIL(_msg);                  \
    }                             \
  } while (0)


/*
 * Forwards
 */

int is_valid_xref(FILE *fp, pdf_t *pdf, xref_t *xref);
void load_xref_entries(FILE *fp, xref_t *xref);
void load_xref_from_plaintext(FILE *fp, xref_t *xref);
void load_xref_from_stream(FILE *fp, xref_t *xref);
void get_xref_linear_skipped(FILE *fp, xref_t *xref);
void resolve_linearized_pdf(pdf_t *pdf);

pdf_creator_t *new_creator(int *n_elements);
void load_creator(FILE *fp, pdf_t *pdf);
void load_creator_from_buf(
    FILE       *fp,
    xref_t     *xref,
    const char *buf,
    size_t      buf_size);
void load_creator_from_xml(xref_t *xref, const char *buf);
void load_creator_from_old_format(
    FILE       *fp,
    xref_t     *xref,
    const char *buf,
    size_t      buf_size);

char *get_object_from_here(FILE *fp, size_t *size, int *is_stream);

char *get_object(
    FILE         *fp,
    int           obj_id,
    const xref_t *xref,
    size_t       *size,
    int          *is_stream);

const char *get_type(FILE *fp, int obj_id, const xref_t *xref);
/* static int get_page(int obj_id, const xref_t *xref); */
char *get_header(FILE *fp);

char *decode_text_string(const char *str, size_t str_len);
int get_next_eof(FILE *fp);


/*
 * Defined
 */

pdf_t *pdf_new(const char *name)
{
    const char *n;
    pdf_t      *pdf;

    pdf = safe_calloc(sizeof(pdf_t));

    if (name)
    {
        /* Just get the file name (not path) */
        if ((n = strrchr(name, '/')))
          ++n;
        else
          n = name;

        pdf->name = safe_calloc(strlen(n) + 1);
        strcpy(pdf->name, n);
    }
    else /* !name */
    {
        pdf->name = safe_calloc(strlen("Unknown") + 1);
        strcpy(pdf->name, "Unknown");
    }

    return pdf;
}


void pdf_delete(pdf_t *pdf)
;


int pdf_is_pdf(FILE *fp)
;


void pdf_get_version(FILE *fp, pdf_t *pdf)
;


int pdf_load_xrefs(FILE *fp, pdf_t *pdf)
;


/* Load page information */
char pdf_get_object_status(
    const pdf_t *pdf,
    int          xref_idx,
    int          entry_idx)
;


void pdf_zero_object(
    FILE        *fp,
    const pdf_t *pdf,
    int          xref_idx,
    int          entry_idx)
;


/* Output information per version */
void pdf_summarize(
    FILE        *fp,
    const pdf_t *pdf,
    const char  *name,
    pdf_flag_t   flags)
;


/* Returns '1' if we successfully display data (means its probably not xml) */
int pdf_display_creator(const pdf_t *pdf, int xref_idx)
;


/* Checks if the xref is valid and sets 'is_stream' flag if the xref is a
 * stream (PDF 1.5 or higher)
 */
int is_valid_xref(FILE *fp, pdf_t *pdf, xref_t *xref)
;


void load_xref_entries(FILE *fp, xref_t *xref)
;


void load_xref_from_plaintext(FILE *fp, xref_t *xref)
;


/* Load an xref table from a stream (PDF v1.5 +) */
void load_xref_from_stream(FILE *fp, xref_t *xref)
;


void get_xref_linear_skipped(FILE *fp, xref_t *xref)
;


/* This must only be called after all xref and entries have been acquired */
void resolve_linearized_pdf(pdf_t *pdf)
;


pdf_creator_t *new_creator(int *n_elements)
{
    pdf_creator_t *daddy;

    static const pdf_creator_t creator_template[] =
    {
        {"Title",        ""},
        {"Author",       ""},
        {"Subject",      ""},
        {"Keywords",     ""},
        {"Creator",      ""},
        {"Producer",     ""},
        {"CreationDate", ""},
        {"ModDate",      ""},
        {"Trapped",      ""},
    };

    daddy = safe_calloc(sizeof(creator_template));
    memcpy(daddy, creator_template, sizeof(creator_template));

    if (n_elements)
      *n_elements = sizeof(creator_template) / sizeof(creator_template[0]);

    return daddy;
}


#define END_OF_TRAILER(_c, _st, _fp) \
{                                    \
    if (_c == '>')                   \
    {                                \
        fseek(_fp, _st, SEEK_SET);   \
        continue;                    \
    }                                \
}
void load_creator(FILE *fp, pdf_t *pdf)
;


void load_creator_from_buf(
    FILE       *fp,
    xref_t     *xref,
    const char *buf,
    size_t      buf_size)
;


void load_creator_from_xml(xref_t *xref, const char *buf)
;


void load_creator_from_old_format(
    FILE       *fp,
    xref_t     *xref,
    const char *buf,
    size_t      buf_size)
;


/* Returns object data at the start of the file pointer
 * This interfaces to 'get_object'
 */
char *get_object_from_here(FILE *fp, size_t *size, int *is_stream)
{
    long         start;
    char         buf[256];
    int          obj_id;
    xref_t       xref;
    xref_entry_t entry;

    start = ftell(fp);

    /* Object ID */
    memset(buf, 0, 256);
    SAFE_E(fread(buf, 1, 255, fp), 255, "Failed to load object ID.\n");
    if (!(obj_id = atoi(buf)))
    {
        fseek(fp, start, SEEK_SET);
        return NULL;
    }

    /* Create xref entry to pass to the get_object routine */
    memset(&entry, 0, sizeof(xref_entry_t));
    entry.obj_id = obj_id;
    entry.offset = start;

    /* Xref and single entry for the object we want data from */
    memset(&xref, 0, sizeof(xref_t));
    xref.n_entries = 1;
    xref.entries = &entry;

    fseek(fp, start, SEEK_SET);
    return get_object(fp, obj_id, &xref, size, is_stream);
}


char *get_object(
    FILE         *fp,
    int           obj_id,
    const xref_t *xref,
    size_t       *size,
    int          *is_stream)
{
    static const int    blk_sz = 256;
    int                 i, total_sz, read_sz, n_blks, search, stream;
    size_t              obj_sz;
    char               *c, *data;
    long                start;
    const xref_entry_t *entry;

    if (size)
      *size = 0;

    if (is_stream)
      *is_stream = 0;

    start = ftell(fp);

    /* Find object */
    entry = NULL;
    for (i=0; i<xref->n_entries; i++)
      if (xref->entries[i].obj_id == obj_id)
      {
          entry = &xref->entries[i];
          break;
      }

    if (!entry)
      return NULL;

    /* Jump to object start */
    fseek(fp, entry->offset, SEEK_SET);

    /* Initial allocation */
    obj_sz = 0;    /* Bytes in object */
    total_sz = 0;  /* Bytes read in   */
    n_blks = 1;
    data = safe_calloc(blk_sz * n_blks);

    /* Suck in data */
    stream = 0;
    while ((read_sz = fread(data+total_sz, 1, blk_sz-1, fp)) && !ferror(fp))
    {
        total_sz += read_sz;

        *(data + total_sz) = '\0';

        if (total_sz + blk_sz >= (blk_sz * n_blks))
          data = realloc(data, blk_sz * (++n_blks));
        if (!data) {
          ERR("Failed to reallocate buffer.\n");
          exit(EXIT_FAILURE);
        }

        search = total_sz - read_sz;
        if (search < 0)
          search = 0;

        if ((c = strstr(data + search, "endobj")))
        {
            *(c + strlen("endobj") + 1) = '\0';
            obj_sz = (char *)strstr(data + search, "endobj") - (char *)data;
            obj_sz += strlen("endobj") + 1;
            break;
        }
        else if (strstr(data, "stream"))
          stream = 1;
    }

    clearerr(fp);
    fseek(fp, start, SEEK_SET);

    if (size) {
      *size = obj_sz;
      if (!obj_sz && data) {
        free(data);
        data = NULL;
      }
    }

    if (is_stream)
      *is_stream = stream;

    return data;
}


const char *get_type(FILE *fp, int obj_id, const xref_t *xref)
{
    int          is_stream;
    char        *c, *obj, *endobj;
    static char  buf[32];
    long         start;

    start = ftell(fp);

    if (!(obj = get_object(fp, obj_id, xref, NULL, &is_stream)) ||
        is_stream                                               ||
        !(endobj = strstr(obj, "endobj")))
    {
        free(obj);
        fseek(fp, start, SEEK_SET);

        if (is_stream)
          return "Stream";
        else
          return "Unknown";
    }

    /* Get the Type value (avoiding font names like Type1) */
    c = obj;
    while ((c = strstr(c, "/Type")) && (c < endobj))
      if (isdigit(*(c + strlen("/Type"))))
      {
          ++c;
          continue;
      }
      else
        break;

    if (!c || (c && (c > endobj)))
    {
        free(obj);
        fseek(fp, start, SEEK_SET);
        return "Unknown";
    }

    /* Skip to first blank/whitespace */
    c += strlen("/Type");
    while (isspace(*c) || *c == '/')
      ++c;

    /* 'c' should be pointing to the type name.  Find the end of the name. */
    size_t n_chars = 0;
    const char *name_itr = c;
    while ((name_itr < endobj) &&
           !(isspace(*name_itr) || *name_itr == '/' || *name_itr == '>')) {
        ++name_itr;
        ++n_chars;
    }
    if (n_chars >= sizeof(buf))
    {
        free(obj);
        fseek(fp, start, SEEK_SET);
        return "Unknown";
    }

    /* Return the value by storing it in static mem. */
    memcpy(buf, c, n_chars);
    buf[n_chars] = '\0';
    free(obj);
    fseek(fp, start, SEEK_SET);
    return buf;
}


char *get_header(FILE *fp)
{
    /* First 1024 bytes of doc must be header (1.7 spec pg 1102) */
    char *header = safe_calloc(1024);
    long start = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    SAFE_E(fread(header, 1, 1023, fp), 1023, "Failed to load PDF header.\n");
    fseek(fp, start, SEEK_SET);
    return header;
}


char *decode_text_string(const char *str, size_t str_len)
{
    int   idx, is_hex, is_utf16be, ascii_idx;
    char *ascii, hex_buf[5] = {0};

    is_hex = is_utf16be = idx = ascii_idx = 0;

    /* Regular encoding */
    if (str[0] == '(')
    {
        ascii = safe_calloc(str_len + 1);
        strncpy(ascii, str, str_len + 1);
        return ascii;
    }
    else if (str[0] == '<')
    {
        is_hex = 1;
        ++idx;
    }

    /* Text strings can be either PDFDocEncoding or UTF-16BE */
    if (is_hex && (str_len > 5) &&
        (str[idx] == 'F') && (str[idx+1] == 'E') &&
        (str[idx+2] == 'F') && (str[idx+3] == 'F'))
    {
        is_utf16be = 1;
        idx += 4;
    }
    else
      return NULL;

    /* Now decode as hex */
    ascii = safe_calloc(str_len);
    for ( ; idx<str_len; ++idx)
    {
        hex_buf[0] = str[idx++];
        hex_buf[1] = str[idx++];
        hex_buf[2] = str[idx++];
        hex_buf[3] = str[idx];
        ascii[ascii_idx++] = strtol(hex_buf, NULL, 16);
    }

    return ascii;
}


/* Return the offset to the beginning of the %%EOF string.
 * A negative value is returned when done scanning.
 */
int get_next_eof(FILE *fp)
;

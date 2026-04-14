/******************************************************************************
 * main.c
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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <sys/types.h>
#include "main.h"
#include "pdf.h"


static void usage(void)
;


static void write_version(
    FILE       *fp,
    const char *fname,
    const char *dirname,
    xref_t     *xref)
;


#ifdef PDFRESURRECT_EXPERIMENTAL
static void scrub_document(FILE *fp, const pdf_t *pdf)
;
#endif // PDFRESURRECT_EXPERIMENTAL


static void display_creator(FILE *fp, const pdf_t *pdf)
;


static pdf_t *init_pdf(FILE *fp, const char *name)
{
    pdf_t *pdf;

    pdf = pdf_new(name);
    pdf_get_version(fp, pdf);
    if (pdf_load_xrefs(fp, pdf) == -1) {
      pdf_delete(pdf);
      return NULL;
    }

    return pdf;
}


void *safe_calloc(size_t size) {
  void *addr;

  if (!size)
  {
    ERR("Invalid allocation size.\n");
    exit(EXIT_FAILURE);
  }
  if (!(addr = calloc(1, size)))
  {
      ERR("Failed to allocate requested number of bytes, out of memory?\n");
      exit(EXIT_FAILURE);
  }
  return addr;
}


int main(int argc, char **argv)
;

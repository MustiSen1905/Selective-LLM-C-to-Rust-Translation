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


extern void usage(void)
;


extern void write_version(
    FILE       *fp,
    const char *fname,
    const char *dirname,
    xref_t     *xref)
;


#ifdef PDFRESURRECT_EXPERIMENTAL
extern void scrub_document(FILE *fp, const pdf_t *pdf)
;
#endif // PDFRESURRECT_EXPERIMENTAL


extern void display_creator(FILE *fp, const pdf_t *pdf)
;


extern pdf_t *init_pdf(FILE *fp, const char *name)
;


void *safe_calloc(size_t size) ;


int main(int argc, char **argv)
{
    int         i, n_valid, do_write, do_scrub;
    char       *c, *dname, *name;
    DIR        *dir;
    FILE       *fp;
    pdf_t      *pdf;
    pdf_flag_t  flags;

    if (argc < 2)
      usage();

    /* Args */
    do_write = do_scrub = flags = 0;
    name = NULL;
    for (i=1; i<argc; i++)
    {
        if (strncmp(argv[i], "-w", 2) == 0)
          do_write = 1;
        else if (strncmp(argv[i], "-i", 2) == 0)
          flags |= PDF_FLAG_DISP_CREATOR;
        else if (strncmp(argv[i], "-q", 2) == 0)
          flags |= PDF_FLAG_QUIET;
#ifdef PDFRESURRECT_EXPERIMENTAL
        else if (strncmp(argv[i], "-s", 2) == 0)
          do_scrub = 1;
#endif
        else if (argv[i][0] != '-')
          name = argv[i];
        else if (argv[i][0] == '-')
          usage();
    }

    if (!name)
      usage();

    if (!(fp = fopen(name, "r")))
    {
        ERR("Could not open file '%s'\n", argv[1]);
        return -1;
    }
    else if (!pdf_is_pdf(fp))
    {
        ERR("'%s' specified is not a valid PDF\n", name);
        fclose(fp);
        return -1;
    }

    /* Load PDF */
    if (!(pdf = init_pdf(fp, name)))
    {
        fclose(fp);
        return -1;
    }

    /* Count valid xrefs */
    for (i=0, n_valid=0; i<pdf->n_xrefs; i++)
      if (pdf->xrefs[i].version)
        ++n_valid;

    /* Bail if we only have 1 valid */
    if (n_valid < 2)
    {
        if (!(flags & (PDF_FLAG_QUIET | PDF_FLAG_DISP_CREATOR)))
          printf("%s: There is only one version of this PDF\n", pdf->name);

        if (do_write)
        {
            fclose(fp);
            pdf_delete(pdf);
            return 0;
        }
    }

    dname = NULL;
    if (do_write)
    {
        /* Create directory to place the various versions in */
        if ((c = strrchr(name, '/')))
          name = c + 1;

        if ((c = strrchr(name, '.')))
          *c = '\0';

        dname = safe_calloc(strlen(name) + 16);
        sprintf(dname, "%s-versions", name);
        if (!(dir = opendir(dname)))
          mkdir(dname, S_IRWXU);
        else
        {
            ERR("This directory already exists, PDF version extraction will "
                "not occur.\n");
            fclose(fp);
            closedir(dir);
            free(dname);
            pdf_delete(pdf);
            return -1;
        }

        /* Write the pdf as a previous version */
        for (i=0; i<pdf->n_xrefs; i++)
          if (pdf->xrefs[i].version)
            write_version(fp, name, dname, &pdf->xrefs[i]);
    }

    /* Generate a per-object summary */
    pdf_summarize(fp, pdf, dname, flags);

#ifdef PDFRESURRECT_EXPERIMENTAL
    /* Have we been summoned to scrub history from this PDF */
    if (do_scrub)
      scrub_document(fp, pdf);
#endif

    /* Display extra information */
    if (flags & PDF_FLAG_DISP_CREATOR)
      display_creator(fp, pdf);

    fclose(fp);
    free(dname);
    pdf_delete(pdf);

    return 0;
}

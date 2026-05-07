/* +++Date last modified: 05-Jul-1997 */

/*
**  BITFILES.C - reading/writing bit files
**
**  Public domain by Aare Tali
*/

#include <stdlib.h>
#include "bitops.h"

bfile *bfopen(char *name, char *mode)
{
      bfile * bf;

      bf = malloc(sizeof(bfile));
      if (NULL == bf)
            return NULL;
      bf->file = fopen(name, mode);
      if (NULL == bf->file)
      {
            free(bf);
            return NULL;
      }
      bf->rcnt = 0;
      bf->wcnt = 0;
      return bf;
}

int bfread(bfile *bf)
;

void bfwrite(int bit, bfile *bf)
;

void bfclose(bfile *bf)
;

#ifdef TEST

void test1(void)
;

void test2(void)
;

main()
{
      test1();
      test2();
      return 0;
}

#endif /* TEST */

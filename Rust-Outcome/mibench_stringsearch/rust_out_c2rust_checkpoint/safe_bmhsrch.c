/* +++Date last modified: 05-Jul-1997 */

/*
**  Case-sensitive Boyer-Moore-Horspool pattern match
**
**  public domain by Raymond Gardner 7/92
**
**  limitation: pattern length + string length must be less than 32767
**
**  10/21/93 rdg  Fixed bug found by Jeff Dunlop
*/
#include <limits.h>                                         /* rdg 10/93 */
#include <stddef.h>
#include <string.h>
typedef unsigned char uchar;


#define LARGE 32767

static int patlen;
static int skip[UCHAR_MAX+1];                               /* rdg 10/93 */
static int skip2;
static uchar *pat;

void bmh_init(const char *pattern)
;

char *bmh_search(const char *string, const int stringlen)
{
      int i, j;
      char *s;

      i = patlen - 1 - stringlen;
      if (i >= 0)
            return NULL;
      string += stringlen;
      for ( ;; )
      {
            while ( (i += skip[((uchar *)string)[i]]) < 0 )
                  ;                           /* mighty fast inner loop */
            if (i < (LARGE - stringlen))
                  return NULL;
            i -= LARGE;
            j = patlen - 1;
            s = (char *)string + (i - j);
            while (--j >= 0 && s[j] == pat[j])
                  ;
            if ( j < 0 )                                    /* rdg 10/93 */
                  return s;                                 /* rdg 10/93 */
            if ( (i += skip2) >= 0 )                        /* rdg 10/93 */
                  return NULL;                              /* rdg 10/93 */
      }
}

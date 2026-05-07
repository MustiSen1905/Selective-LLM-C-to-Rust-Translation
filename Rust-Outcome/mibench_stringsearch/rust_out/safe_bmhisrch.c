/* +++Date last modified: 05-Jul-1997 */

/*
**  Case-Insensitive Boyer-Moore-Horspool pattern match
**
**  Public Domain version by Thad Smith 7/21/1992,
**  based on a 7/92 public domain BMH version by Raymond Gardner.
**
**  This program is written in ANSI C and inherits the compilers
**  ability (or lack thereof) to support non-"C" locales by use of
**  toupper() and tolower() to perform case conversions.
**  Limitation: pattern length + string length must be less than 32767.
**
**  10/21/93 rdg  Fixed bugs found by Jeff Dunlop
*/

#include <limits.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
typedef unsigned char uchar;

void bmhi_init(const char *);
char *bmhi_search(const char *, const int);
void bhmi_cleanup(void);


#define LARGE 32767             /* flag for last character match    */

static int patlen;              /* # chars in pattern               */
static int skip[UCHAR_MAX+1];   /* skip-ahead count for test chars  */
static int skip2;               /* skip-ahead after non-match with
                                ** matching final character         */
static uchar *pat = NULL;       /* uppercase copy of pattern        */

/*
** bmhi_init() is called prior to bmhi_search() to calculate the
** skip array for the given pattern.
** Error: exit(1) is called if no memory is available.
*/

void bmhi_init(const char *pattern)
;

char *bmhi_search(const char *string, const int stringlen)
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
            while ( --j >= 0 && toupper(s[j]) == pat[j] )
                  ;
            if ( j < 0 )                                    /* rdg 10/93 */
                  return s;                                 /* rdg 10/93 */
            if ( (i += skip2) >= 0 )                        /* rdg 10/93 */
                  return NULL;                              /* rdg 10/93 */
      }
}

void bhmi_cleanup(void)
;

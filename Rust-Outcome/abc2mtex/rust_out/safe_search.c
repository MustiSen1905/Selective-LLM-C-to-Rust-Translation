/************************************************************************
*									*
*			search.c - V1.6.1 (Jan 97)			*
*									*
*		by    Chris Walshaw (C.Walshaw@gre.ac.uk)		*
*									*
*	Copyright Chris Walshaw. Permission is granted to use and	*
*	copy provided that this copyright notice remains attached.	*
*	This code may not be sold.					*
*									*
************************************************************************/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#if defined(__MWERKS__)
#ifndef MACABC
#include <Console.h>
#endif
#endif
#include "index.h"

/* for PC version */
extern	unsigned _stklen = 8192;

enum	results { FAILURE, SUCCESS, CONTINUE };

#define	DEFAULT_FORMAT_SIZE "F<99X<99J<99O<99R<99C<99T<99M<99L<99K<99|<99"

int	hash_compare(int *array1,int *array2)
{
	int	dist = 0;

	while(*array1 != LAST && *array2 != LAST)
		dist += abs(*array1++ - *array2++);
	return(dist);
}

int	get_abc_entry(char *dflt_meter,char *dflt_origin,
			char *dflt_rhythm,char *entry,Record *abc,int *x)
{
	char	dummy[30][99];
	char	temp[1999],line[999];
	int	in_header = 0,in_tune = 0,m = 0,o = 0,r = 0;
	temp[0] = '\0';
	entry[0] = '\0';
	abc->LENGTH[0] = '\0';
	abc->BARS[0] = '\0';
	(void) strcpy(abc->METER,&dflt_meter[2]);
	abc->METER[strlen(abc->METER)-1] = '\0';
	while (!in_tune && (getsIn(line)) != NULL) {
		if ((strncmp("X:",line,2)) == 0) {
			*x = atoi(&line[2]);
			in_header = 1;
		}
		if (in_header) {
			(void) strcat(temp,line);
			if (line[1] == ':') {
				switch (line[0]) {
				case 'M':
					stripcpy(abc->METER,&line[2]);
					m = 1;
					break;
				case 'L':
					stripcpy(abc->LENGTH,&line[2]);
					break;
				case 'O':
					o = 1;
					break;
				case 'R':
					r = 1;
					break;
				case 'K':
					stripcpy(abc->KEY,&line[2]);
					process_abc(dummy,0,abc,"",abc->BARS,
						temp,(char *) NULL,NO_OUTPUT,
						TWO_BARS,0, (int *) NULL);
					in_tune = 1;
					in_header = 0;
					break;
				}
			}
		} else {
			if (line[1] == ':') {
				switch (line[0]) {
				case 'M':
					(void) strcpy(dflt_meter,line);
					(void) strcpy(abc->METER,
						&dflt_meter[2]);
					abc->METER[strlen(abc->METER)-1] = '\0';
					break;
				case 'O':
					(void) strcpy(dflt_origin,line);
					break;
				case 'R':
					(void) strcpy(dflt_rhythm,line);
					break;
				}
			}
		}
	}
	if (in_tune) {
		if (m == 0) (void) strcat(entry,dflt_meter);
		if (o == 0) (void) strcat(entry,dflt_origin);
		if (r == 0) (void) strcat(entry,dflt_rhythm);
		(void) strcat(entry,temp);
		return(1);
	} else
		return(0);
}

int	str_compare(char *s,char *t)
{
	while ((lower(*t)) == (lower(*s)) || *s == '.') {
		++s;
		++t;
		if (*s == '\0') return(SUCCESS);
		if (*t == '\0') return(FAILURE);
	}
	return(CONTINUE);
}

int	str_search(char *s,char *t)
{
	int	result;
	while (*s == '.') {
		++s;
		++t;
		if (*s == '\0') return(SUCCESS);
		if (*t == '\0') return(FAILURE);
	}
	while (*t) {
		if ((result = str_compare(s,t)) == CONTINUE)
			++t;
		else
			return(result);
	}
	return(0);
}

int	main(int argc,char *argv[])
;


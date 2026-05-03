/************************************************************************
*									*
*			sort_in.c - V1.6.1 (Jan 97)			*
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

typedef int (compFun)(const void*, const void*);

static	char	priority[99];

static	char	*next(char *s)
{
	while ((*s < 'a' || *s > 'z')
	    && (*s < 'A' || *s > 'Z')
	    && (*s < '0' || *s > '9')
	    && *s != ' ' && *s != '\t' && *s != '\0') {
		++s;
	}
	return(s);
}

static	int	str_cmp(char *s,char *t)
{
	char	u,v;
	if (s == NULL || t == NULL)
		return(s - t);
	/* ignore leading whitespace */
	while (*s == ' ') ++s;
	while (*t == ' ') ++t;
	while (*s != '\0' && *t != '\0') {
		s = next(s);
		u = lower(*s);
		t = next(t);
		v = lower(*t);
		if (u != v)
			return(u - v);
		++s;
		++t;
	}
	s = next(s);
	u = lower(*s);
	t = next(t);
	v = lower(*t);
	return(u - v);
}

static	int	compare(Record **entry1,Record **entry2)
{
	int	result = 0,p;
	for (p = 0; priority[p] != '\0' && result == 0; ++p) {
		if (strchr("TRFJOCMKL",priority[p]))
			result = str_cmp((*entry1)->fields[priority[p]-'A'],
					 (*entry2)->fields[priority[p]-'A']);
		else
			switch (priority[p]) {
			case 'X':
				result = atoi((*entry1)->fields['X'-'A'])
				       - atoi((*entry2)->fields['X'-'A']);
				break;
			case '|':
				result = str_cmp((*entry1)->bars,
						 (*entry2)->bars);
				break;
			default:
				g_error("in priority format");
				break;
			}
	}
	return(result);
}

int	main(int argc,char *argv[])
;


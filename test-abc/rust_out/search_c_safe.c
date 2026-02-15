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

static	int	hash_compare(int *array1,int *array2)
;

static	int	get_abc_entry(char *dflt_meter,char *dflt_origin,
			char *dflt_rhythm,char *entry,Record *abc,int *x)
;

static	int	str_compare(char *s,char *t)
{
	while ((lower(*t)) == (lower(*s)) || *s == '.') {
		++s;
		++t;
		if (*s == '\0') return(SUCCESS);
		if (*t == '\0') return(FAILURE);
	}
	return(CONTINUE);
}

static	int	str_search(char *s,char *t)
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


/************************************************************************
*									*
*			index.c - V1.6.1 (Jan 97)			*
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
#include <stdarg.h>
#include <string.h>
#include "index.h"

void	g_error(const char *fmt, ...)
{
	va_list	ap;
	va_start(ap,fmt);
	(void) fprintf(stderr,"ERROR: ");
	(void) vfprintf(stderr,fmt,ap);
	(void) fprintf(stderr,"\n");
	va_end(ap);
	exit(1);
}

void	get_index(char *fmt, char *fname)
;

void	size_record(char *fmt, int *size, char *field)
;

Record	*alloc_record(char *fmt, int *size)
;

void	free_record(Record *entry, char *fmt)
;

static	void	str_get(char *string, char *fmt, int *f, char *index, int *i)
;

int	get_record(char *fmt, FILE *In, Record *entry)
;

static	void	str_put(char *string, char *fmt, int *f, char *index, int *i)
;

int	put_record(char *fmt, FILE *Out, Record *entry)
;

char	lower(char c)
{
	if (c >= 'A' && c <= 'Z')
		return(c-'A'+'a');
	else
		return(c);
}


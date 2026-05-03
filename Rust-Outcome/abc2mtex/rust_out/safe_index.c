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
;

void	get_index(char *fmt, char *fname)
;

void	size_record(char *fmt, int *size, char *field)
;

Record	*alloc_record(char *fmt, int *size)
{
	Record	*entry;
	int	f;
	ALLOC(entry,Record,1);
	for (f = 0; fmt[f]; ++f) {
		if (strchr("FXJORCTMLK",fmt[f])) {
			ALLOC(entry->fields[fmt[f]-'A'],char,size[f]+1);
		} else if (fmt[f] == '|') {
			ALLOC(entry->BARS,char,size[f]+1);
		} else
			g_error("in size format");
	}
	return(entry);
}

void	free_record(Record *entry, char *fmt)
;

static	void	str_get(char *string, char *fmt, int *f, char *index, int *i)
{
	int	c;
	int	n;
	int	l;
	char	temp[99];
	*f += 2;
	n = atoi(&fmt[*f]);
	if (fmt[*f-1] == '<') {
		(void) strncpy(string,&index[*i],n);
		string[n] = '\0';
		/* remove trailing whitespace */
		l = strlen(string);
		for (c = l; c > 0 && string[c-1] == ' '; c--);
		string[c] = '\0';
		*i += n;
	} else if (fmt[*f-1] == '>') {
		(void) strncpy(temp,&index[*i],n);
		temp[n] = '\0';
		/* remove leading whitespace */
		for (c = 0; temp[c] == ' ' && c < n; c++);
		(void) strcpy(string,&temp[c]);
		*i += n;
	} else {
		(void) strncpy(string,&index[*i],n);
		l = strlen(string);
		if (string[l-1] == '\n')
			string[--l] = '\0';
		*i += strlen(string);
	}
	while (fmt[*f] >= '0' && fmt[*f] <= '9')
		(*f)++;
}

int	get_record(char *fmt, FILE *In, Record *entry)
;

static	void	str_put(char *string, char *fmt, int *f, char *index, int *i)
{
	int	c;
	int	n;
	int	l;
	*f += 2;
	n = atoi(&fmt[*f]);
	if (fmt[*f-1] == '<') {
		(void) strncpy(&index[*i],string,n);
/* might need to pad with whitespace */
		for (c = strlen(string); c < n; c++)
			index[*i+c] = ' ';
		*i += n;
		index[*i] = '\0';
	} else if (fmt[*f-1] == '>') {
/* might need to pad with whitespace */
		l = strlen(string);
		if (l < n) {
			for (c = 0; c < n-l; c++)
				index[*i+c] = ' ';
			index[*i+c] = '\0';
			(void) strcpy(&index[*i+c],string);
		} else
			(void) strncpy(&index[*i],string,n);
		*i += n;
		index[*i] = '\0';
	} else {
		(void) strcpy(&index[*i],string);
		*i += strlen(string);
	}
	while (fmt[*f] >= '0' && fmt[*f] <= '9')
		(*f)++;
}

int	put_record(char *fmt, FILE *Out, Record *entry)
;

char	lower(char c)
{
	if (c >= 'A' && c <= 'Z')
		return(c-'A'+'a');
	else
		return(c);
}


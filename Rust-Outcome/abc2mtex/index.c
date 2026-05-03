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
{
	FILE	*fp;
	int	colon;
	char	c;
	int	f = 0;

	if ((fp = fopen(fname,"r")) == NULL)
		g_error("file %s does not exist",fname);

	fmt[0] = '\0';
	while ((fgets(&fmt[strlen(fmt)],99,fp)) != NULL);

	/* now check for errors */
	while (fmt[f]) {
		if (strchr("FXJORCTMLK|",fmt[f])) {
			c = fmt[f];
			if (fmt[f++] == '|') {
				if (fmt[f] == '0') ++f;
				if (strchr("12",fmt[f])) ++f;
			}
			if (fmt[f] == ':')
				colon = 1;
			else if (fmt[f] == '<' || fmt[f] == '>')
				colon = 0;
			else
			  g_error("fmt file - %c not followed by [><:]",c);
			if (fmt[++f] < '0' || fmt[f] > '9')
			  g_error("fmt file - %c not followed by length",c);
			while (fmt[f] >= '0' && fmt[f] <= '9') ++f;
			if (colon && fmt[f] != '\n')
			  g_error("fmt file - %c: not at end of line",c);
		} else if (fmt[f] == '\\')
			f += 2;
		else
			f += 1;
	}
	if (fmt[f-1] != '\n')
		g_error("fmt file - not terminated by newline character");
}

void	size_record(char *fmt, int *size, char *field)
{
	int	s = 0;
	int	f = 0;
	while (fmt[f]) {
		if (strchr("FXJORCTMLK|",fmt[f])) {
			field[s] = fmt[f++];
			if (field[s] == '|' && fmt[f] == '0') ++f;
			if (field[s] == '|' && strchr("12",fmt[f])) ++f;
			size[s++] = atoi(&fmt[++f]);
			while (fmt[f] >= '0' && fmt[f] <= '9')
				f++;
		} else if (fmt[f] == '\\')
			f += 2;
		else
			f += 1;
	}
	field[s] = '\0';
	size[s] = 0;
}

Record	*alloc_record(char *fmt, int *size)
;

void	free_record(Record *entry, char *fmt)
{
	int	f;

	for (f = 0; fmt[f]; ++f) {
		if (strchr("FXJORCTMLK",fmt[f]))
			free(entry->fields[fmt[f]-'A']);
		else if (fmt[f] == '|')
			free(entry->BARS);
	}
	free(entry);
}

void	str_get(char *string, char *fmt, int *f, char *index, int *i)
;

int	get_record(char *fmt, FILE *In, Record *entry)
{
	int	new_line = 1;
	int	f = 0;
	int	i = 0;
	static	char	index[999];

	while (fmt[f]) {
		if (new_line == 1) {
			if ((fgets(index,999,In)) == NULL)
				return(0);
			new_line = 0;
		}
		if (strchr("FXJORCTMLK",fmt[f]))
			str_get(entry->fields[fmt[f]-'A'],fmt,&f,index,&i);
		else
			switch (fmt[f]) {
			case '|':
				if (fmt[f+1] == '0') ++f;
				if (strchr("12",fmt[f+1])) ++f;
				str_get(entry->BARS,fmt,&f,index,&i);
				break;
			case '\n':
				new_line = 1;
				f += 1;
				i = 0;
				break;
			case '\\':
				f += 1;
				/*FALLTHROUGH*/
			default:
				f += 1;
				i += 1;
				break;
			}
	}
	return(1);
}

void	str_put(char *string, char *fmt, int *f, char *index, int *i)
;

int	put_record(char *fmt, FILE *Out, Record *entry)
{
	int	f = 0;
	int	i = 0;
	static	char	index[999];

	while (fmt[f]) {
		if (i == 0)
			index[0] = '\0';
		if (strchr("FXJORCTMLK",fmt[f]))
			str_put(entry->fields[fmt[f]-'A'],fmt,&f,index,&i);
		else
			switch(fmt[f]) {
			case '|':
				if (fmt[f+1] == '0') ++f;
				if (strchr("12",fmt[f+1])) ++f;
				str_put(entry->BARS,fmt,&f,index,&i);
				break;
			case '\\':
				f += 1;
				/*FALLTHROUGH*/
			default:
				(void) strncpy(&index[i++],&fmt[f],1);
				index[i] = '\0';
				if (fmt[f++] == '\n') {
					(void) fputs(index,Out);
					i = 0;
				}
				break;
			}
	}
	return(1);
}

char	lower(char c)
;


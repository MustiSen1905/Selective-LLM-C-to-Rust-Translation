/************************************************************************
*									*
*			fields.c - V1.6.1 (Jan 97)			*
*									*
*		by    Chris Walshaw (C.Walshaw@gre.ac.uk)		*
*									*
*	Copyright Chris Walshaw. Permission is granted to use and	*
*	copy provided that this copyright notice remains attached.	*
*	This code may not be sold.					*
*									*
************************************************************************/

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#if defined(__MWERKS__)
#ifndef MACABC
#include <Console.h>
#endif
#endif
#include "index.h"

/* for PC version */
extern	unsigned _stklen = 8192;

extern	void	abc_error(char*,...);
extern	void	abc_warning(char*,...);
extern	char	*end_of(char*);
extern	void	strip(char*,char*);
extern	void	get_dnl(Record*);
extern	void	output_transline(char*);
char *gets(char *s);

extern	void	open_TeX(char*,int);
extern	void	close_TeX(void);
extern	void	draw_text(char*,char*);

extern	FILE	*Trans;
extern	int	transpose;
extern	int	offset;
extern	Setting	settings;
extern	int	nblanks;

FILE	*Log;
FILE	*Index;

void	article(char *s,char *t)
{
	int	j;
	char	*i = s;
	const	char	*a[]={"The","An","A","Na","Da","Le","La","Lo","Lou",
				"Un","Une","Les","L'",""};
	while (*s && *s++ != ',');
	if (*(s-1) == ',') {
		s += 1;
		for (j=0; a[j][0]; ++j) {
			if ((strcmp(s,a[j])) == 0) {
				while (*s)
					*t++ = *s++;
				if (*(t-1) != '\'')
					*t++ = ' ';
				break;
			}
		}
	}
	while (*i) {
		if (*i == ',' && a[j][0])
			break;
		*t++ = *i++;
	}
	*t = '\0';
}

void	detex(char *s,char *t)
{
	while (*s) {
		if (*s == '\\') {
			if (*(++s) != '&')
				if (*(++s) == ' ')
					s += 1;
		}
		*t++ = *s++;
	}
	*t = '\0';
}

void	interval(char *s)
{
	offset = atoi(&s[1])-1;
	for (transpose = -1; transpose < 3; transpose++)
		if (offset%7 == (7+transpose*4)%7) break;
	if (transpose == 3 || offset < 1 || offset > 7)
		g_error("transpose interval not recognised");
	if (s[0] == '_') {
		transpose *= -1;
		offset *= -1;
	}
}

int	is_comment(char *str)
{
	char	*c_ptr;

	c_ptr = strchr(str,'%');
	if (c_ptr == NULL) return(0);
	while (c_ptr > str && (*(c_ptr-1) == ' ' || *(c_ptr-1) == '\t'))
		--c_ptr;
	if (c_ptr == str) return(1);
	return(0);
}

void	strip_path(char *filename,char *file)
{
	char	*f_ptr;
char	temp[99];
	stripcpy(temp,file);
	if ((strcmp(&temp[strlen(temp)-4],".abc")) == 0)
		temp[strlen(temp)-4] = '\0';
	if ((f_ptr = strrchr(temp,'/')))
		++f_ptr;
	else
		f_ptr = temp;
	(void) strcpy(filename,f_ptr);
}

int	main(int argc,char **argv)
;

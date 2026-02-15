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

static	void	article(char *s,char *t)
;

static	void	detex(char *s,char *t)
;

static	void	interval(char *s)
;

static	int	is_comment(char *str)
{
	char	*c_ptr;

	c_ptr = strchr(str,'%');
	if (c_ptr == NULL) return(0);
	while (c_ptr > str && (*(c_ptr-1) == ' ' || *(c_ptr-1) == '\t'))
		--c_ptr;
	if (c_ptr == str) return(1);
	return(0);
}

static	void	strip_path(char *filename,char *file)
;

int	main(int argc,char **argv)
;

/************************************************************************
*									*
*			tex.c - V1.6.1 (Jan 97)				*
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
#include "index.h"

#ifndef atof
extern	double	atof(const char*);
#endif

/*extern	void	set_dnl(char*);*/

extern	Setting	settings;
extern	int	bass, treble;
/*extern	int	dnl;*/

FILE	*Out;

enum	scan_types { SCAN_METER, SCAN_ALL };

static	int	in_tune = 0;
static	int	in_bar = 0;
static	int	bar_no, change_context, change_signature;
static	int	in_notes, hp, new_tune, *old_beam = NULL;
static	int	tempo_length = 0, bpm = 0;
static	int	stave;
static	int	slur_level = 0;
static	int	musix;

static	const	char	*mtex ={"ABCDEFGHIJKLMNabcdefghijklmnopqrstuvwxyz"};
static	const	char	*Accidental[] = { "", "<", "_", "=", "^", ">" };

static	int	abclog2(int x)
;

void	open_TeX(char *s, int musix_out)
{
	if (s[0] == '\0') Out = fopen("music.tex","w");
	else Out = fopen(s,"w");

	musix = musix_out;

	if (musix) {
		(void) fprintf(Out,"\\def\\abcmusix{Y}\n");
		if (musix == 1) (void) fprintf(Out,"\\def\\abcopus{N}\n");
		if (musix == 2) (void) fprintf(Out,"\\def\\abcopus{Y}\n");
	} else (void) fprintf(Out,"\\def\\abcmusix{N}\n");
	if (settings.mine)
		(void) fprintf(Out,"\\input dscgrphy\n");
	(void) fprintf(Out,"\\input header\n%%\n");
	if (musix == 1) (void) fprintf(Out,"\\startmuflex%%\n");
}

static	void	open_grace(void)
;

static	void	open_music(void)
;

static	void	close_grace(void)
;

static	void	close_music(void)
;

static	void	open_tune(void)
;

static	char	q_plus(int pitch,int beam)
;

static	char	n_plus(int pitch,int beam)
;

static	char	L_minus(int pitch,int beam)
;

static	void	draw_header(void)
;

void	draw_text(char *type,char *string)
{
	char	*ptr;
	if (type == NULL) /* TeX string */
		(void) fprintf(Out,"%s%%\n",string);
	else if ((strcmp(type,"Z")) == 0)
		(void) fprintf(Out,"\\message{%s}%%\n",string);
	else {
		for (ptr = string; (ptr = strpbrk(ptr,"#%&")); ptr += 1)
			if (*(ptr-1) != '\\')
			g_error("unescaped special TeX character %c detected\n\
	this will cause TeX to choke",*ptr);
		if (string[0])
			(void) fprintf(Out,
				"\\def\\%strue{Y}\\def\\%sstring{%s}\n",
				type,type,string);
		else
			(void) fprintf(Out,"\\def\\%strue{N}\n",type);
	}
}

static	void	draw_tempo(Field *tempo)
;

static	void	close_open(void)
;

static	void	next_stave(void)
;

static	void	draw_rest(int level)
;

static	void	draw_pt(Note note)
;

static	void	draw_slur(Note note,char type,char ud, int change)
;

static	void	draw_attributes(Note note,char ul,char lu,char ud,int beam)
;

static	void	draw_usercmd(char *s)
;

static	void	draw_chord(Symbol *root)
;

static	void	draw_tie(Note note,char *str)
;

static	void	draw_part(char *part)
;

static	void	draw_tex(char *line)
;

static	void	draw_size(char *size)
;

void	close_TeX(void)
;

static	void	draw_old_repeat(int repeat)
;

static	void	draw_meter_new(Field *meter)
;

static	void	key2tex(Field *f)
;

static	void	staves(void)
;

static	void	scan_fields(Symbol *s, int scan)
;

static	void	beam2tex(int n, Symbol *first, int beam)
;

static	void	bar2tex(Symbol *s)
;

static	void	fields2tex(Field *f)
;

static	void	end_tune(void)
;

void	tune2tex(char title[][99], int titles, Record *entry, int n_symbols,
		Symbol *symbols, Field *key_field, Field *meter_field,
		Field *tempo_field)
{
	int	i,j,n;
	char	other_titles[999];
	int	ttl;

	new_tune = 1;
	hp = 0;

	draw_text("X",entry->NUMBER);
	draw_text("T",title[0]);
	draw_text("S",entry->SOURCE);
	draw_text("C",entry->COMPOSER);
	draw_text("A",entry->AREA);
	draw_text("N",entry->NOTES);
	other_titles[0] = '\0';
	if (titles > 1) {
		for (ttl = 1; ttl < titles; ++ttl) {
			(void) strcat(other_titles,title[ttl]);
			if (ttl < titles-1)
			(void) strcat(other_titles,"; ");
		}
	}
	if (titles < 6) {
		draw_text("Ta",other_titles);
		draw_text("Tb","");
	} else {
		draw_text("Tb","");
		draw_text("Ta",other_titles);
	}
	draw_text("P",entry->PARTS);

	draw_header();
	key2tex(key_field);
	staves();
	draw_meter_new(meter_field);
	/*if (entry->LENGTH[0]) set_dnl(entry->LENGTH);*/
	draw_size(entry->ELEMSKIP);
	if (entry->TEMPO[0]) draw_tempo(tempo_field);

	in_bar = 1;

	symbols = symbols->next;
	for (i = 1; i < n_symbols;) {
		switch (symbols->type) {
		case NOTE:
			n = symbols->u.note.n_notes;
			if (symbols->u.note.pitch == 0)
				draw_usercmd(symbols->u.note.attributes);
			else if (symbols->u.note.type == GRACE) {
				if (in_notes) close_music();
				beam2tex(n,symbols,-1);
			} else
				beam2tex(n,symbols,0);
			for (j = 0; j < n; ++j)
				symbols = symbols->next;
			i += j;
			break;
		case BAR_LINE:
			bar2tex(symbols);
			i += 1;
			symbols = symbols->next;
			break;
		case FIELD:
			fields2tex(&symbols->u.field);
			i += 1;
			symbols = symbols->next;
			break;
		case MISC:
			if (symbols->u.misc.level == 2)
				close_open();
			else if (symbols->u.misc.level == 1)
				next_stave();
			else
				g_error("");
			symbols = symbols->next;
			i += 1;
			break;
		default:
			g_error("");
			break;
		}
	}

	end_tune();
}


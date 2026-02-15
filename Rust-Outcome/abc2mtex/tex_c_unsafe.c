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
;

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
{
	char	*str = &tempo->string[2];
	int	level,old_tempo_length = tempo_length,length = 1,ctr = 0,j;

	if (new_tune) open_tune();

	if (str[ctr] == 'C') {
		if (str[++ctr] >= '0' && str[ctr] <= '9') {
			length = atoi(&str[ctr++]);
			if (length > 9) ctr += 1;
		}
		tempo_length = length*tempo->info1;
		if (str[ctr] != '=') g_error("in Q field");
		ctr += 1;
	} else if ((strncmp(&str[ctr],"1/",2)) == 0) {
		ctr += 2;
		tempo_length = 32/atoi(&str[ctr++]);
		if (tempo_length <= 2) ctr += 1;
		if (str[ctr] != '=') g_error("in Q field");
		ctr += 1;
	} else
		tempo_length = tempo->info1;
	if (str[ctr] == 'C') {
		if (str[++ctr] >= '0' && str[ctr] <= '9') {
			length = atoi(&str[ctr]);
		}
		bpm = (bpm*old_tempo_length)/(length*tempo->info1);
	} else if (str[ctr] >= '0' && str[ctr] <= '9') {
		bpm = atoi(&str[ctr]);
	} else
		g_error("in Q field");

	old_tempo_length = tempo_length;

	if (in_notes) close_music();
	(void) fprintf(Out,"\\notes\\Uptext{\\metron{\\");
	if (tempo_length%3 == 0) (void) fprintf(Out,"pt1\\");
	level = abclog2(tempo_length);
	if (level == 5)
		(void) fprintf(Out,"wh");
	else if (level == 4)
		(void) fprintf(Out,"hu");
	else if (level == 3)
		(void) fprintf(Out,"qu");
	else {
		for (j = 0; j < 3-level; ++j)
			(void) fprintf(Out,"c");
		(void) fprintf(Out,"u");
	}
	(void) fprintf(Out,"}{%d}}\\enotes%%\n",bpm);
}

static	void	close_open(void)
;

static	void	next_stave(void)
;

static	void	draw_rest(int level)
;

static	void	draw_pt(Note note)
;

static	void	draw_slur(Note note,char type,char ud, int change)
{
	int	pitch;
	char	mtype[5];

	if (note.pitch == -1) {
/*
		if (ud == 'u') pitch = 16;
		else pitch = 24;
*/
		pitch = 20;
	} else
		pitch = note.pitch;
	if (type == 's') (void) strcpy(mtype,"slur");
	else if (type == 't') (void) strcpy(mtype,"tie");
	if (strchr(note.attributes,'.') && type == 's') {
		if (ud == 'u') pitch += 2;
		else pitch -= 2;
	}
	if (change > 0) {
		(void) fprintf(Out,"\\i%s%c%d{%c}",mtype,ud,slur_level,
			mtex[pitch]);
	} else if (change < 0) {
		(void) fprintf(Out,"\\t%s%d",mtype,slur_level - 1);
		if (type == 's')
			(void) fprintf(Out,"{%c}",mtex[pitch]);
	}
	slur_level += change;
}

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
{
	double	esize;
	if (settings.mine && musix) esize = 7.0;
	else esize = 8.5;
	if (size[0] && '0' <= size[0] && size[0] <= '9')
		/* ignore non numerical */
		esize = atof(size);
	if (musix == 0 && new_tune) (void) fprintf(Out,"\\normal");
	if (musix == 0 || (size[0] && '0' <= size[0] && size[0] <= '9')
	 || settings.mine) /* don't print anything for MusiXTeX unless asked */
		(void) fprintf(Out,"\\elemskip=%.1fpt%%\n",esize);
}

void	close_TeX(void)
;

static	void	draw_old_repeat(int repeat)
;

static	void	draw_meter_new(Field *meter)
;

static	void	key2tex(Field *f)
{
	if (f->info2 == 2) /* key is HP */
		(void) fprintf(Out,"\\generalsignature{0}%%\n");
	else
		(void) fprintf(Out,"\\generalsignature{%d}%%\n",f->info1);

	if (old_beam) free(old_beam);
	ALLOC(old_beam,int,bass+treble);

	if (new_tune) {
		if (f->info2 == 1) /* key is Hp */
			(void) fprintf(Out,"\\beginHp\n");
		hp = f->info2;
	} else {
		if (in_bar) (void) fprintf(Out,"\\changesignature%%\n");
		else change_signature = 1;
	}

}

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
{
	free(old_beam);
	old_beam = NULL;
	if (in_tune) {
		if (musix)
			(void) fprintf(Out,"\\zstoppiece%%\n");
		else
			(void) fprintf(Out,"\\zsuspmorceau%%\n");
		in_tune = 0;
	}
	(void) fprintf(Out,"}%%\n");
	if (hp == 1) /* key is Hp */
		(void) fprintf(Out,"\\endHp%%\n");
	(void) fprintf(Out,"\n");
	(void) fflush(Out);
}

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


/************************************************************************
*									*
*			abc.c - V1.6.1 (Jan 97)				*
*									*
*		by    Chris Walshaw (C.Walshaw@gre.ac.uk)		*
*									*
*	Copyright Chris Walshaw. Permission is granted to use and	*
*	copy provided that this copyright notice remains attached.	*
*	This code may not be sold.					*
*									*
************************************************************************/

#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>
#include "index.h"

#define	CROTCHET	8
#define	QUAVER		4
#define	SEMIQUAVER	2
#define	DEMISEMIQUAVER	1

extern	void	tune2tex(char[][99],int,Record*,int,Symbol*,Field*,Field*,
				Field*);

FILE	*Trans;
Setting	settings = { 0, 0, 0, 0, 0, 0, 0 };
int	bass, treble;
int	dnl = 0;
int	transpose;
int	offset;
int	nblanks;

typedef struct {
	int	type;
	int	transpose;
	int	nbars;
	int	warnings;
} Output;

static	FILE	*In;
static	char	abc_file[99];
static	int	input_line;
static	int	just_comment;

static	Barline	*bar_start;
static	Note	*beam_start = NULL;
static	Note	*note;
static	Symbol	*current = NULL;
static	int	n_notes = 0;

static	const	char	*Key_array[] = {
				"Fb","Cb","Gb","Db","Ab","Eb","Bb",
				"F", "C", "G", "D", "A", "E", "B",
				"F#","C#","G#","D#","A#","E#","B#",
			"" };
static	const	char	**Key = Key_array+8;

static	const	int	Csemitones[] = { 0, 2, 4, 5, 7, 9, 11 };
static		int	 semitones[7];

static	int	max = 0; /* max notes in a beam */

static	int	token, prev_token;
static	int	token_length = 0;
static	char	*token_ptr = NULL;

#define	MAX_N_BLOCKS	10
#define	BLOCK_LENGTH	400

static	int	n_blocks;
static	Symbol	*block[MAX_N_BLOCKS];
static	int	n_symbols = 0;

static	int	compound_time = 0;
static	frac	fbar_total;
static	int	bar_no;
static	char	bar[3][99];
static	char	current_bar[99];
static	int	in_old_slur = 0;
static	int	bar_length;
static	int	stave;
static	Output	output;
static	int	tnote = 0;
static	int	beam_length = 0;
static	Note	*chord_root = NULL;	/* first note of chord */
static	int	tuplet_n_notes = 0;	/* number of notes required in tuplet */
static	int	tuplet_note_no = 0;	/* number of notes in tuplet so far */
static	frac	tuplet_fraction = {1,1};/* fractional length of a tuplet note */
static	frac	broken = { 0, 0 };
static	int	continuation = 0;
static	int	ignore;

enum	env_types {
		IN_GRACE,
		IN_CHORD,
		IN_BROKEN,
		IN_TIE,
		IN_TUPLET,
		MAX_ENVS
	};

static	int	env[MAX_ENVS] = { 0, 0, 0, 0, 0 };

static	const	char	*token_names[] = {
				"gchord",
				"accent",
				"accidental",
				"note",
				"octaver",
				"length",
				"divisor",
				"broken",
				"bar",
				"repeat",
				"space",
				"tie",
				"tuplet",
				"macro",
				"continuation",
				"justification",
				"newline",
				"open grace",
				"close grace",
				"open chord",
				"close chord",
				"open slur",
				"close slur",
				"ampersand",
				"comment",
				""
			};

enum token_types {
	GCHORD_TKN,	/* "*" */
	ACCENT_TKN,	/* ~.uv */
	ACCIDENTAL_TKN,	/* ^^ ^ = _ __ */
	NOTE_TKN,	/* A-Ga-gz */
	OCTAVER_TKN,	/* ', */
	LENGTH_TKN,	/* 1-99 */
	DIVISOR_TKN,	/* / */
	BROKEN_TKN,	/* > >> >>> < << <<< */
	BAR_TKN,	/* | [| || |] |: :: :| |1 :|2 */
	REPEAT_TKN,	/* [1 [2 */
	SPACE_TKN,	/*   \t */
	TIE_TKN,	/* - */
	TUPLET_TKN,	/* (n */
	MACRO_TKN,	/* H-Z */
	CONTINUE_TKN,	/* \ */
	JUSTIFY_TKN,	/* * */
	NEWLINE_TKN,	/* \n */
	OPEN_GRACE_TKN,	/* { */
	CLOSE_GRACE_TKN,/* } */
	OPEN_CHORD_TKN,	/* [ */
	CLOSE_CHORD_TKN,/* ] */
	OPEN_SLUR_TKN,	/* ( */
	CLOSE_SLUR_TKN,	/* ) */
	AMPERSAND_TKN,	/* & */
	TRAILING_TKN,	/* % */
	MAX_TKNS
};

static	int	syntax_table[MAX_TKNS+1][MAX_TKNS+1] = {
		/*	  " ~ = A ' 2 / > | [   - t H \ * n { } [ ] ( ) & % */
		/* " */	{ 0,1,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,1,0,0,0,0 },
		/* ~ */	{ 0,1,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0 },
		/* = */	{ 0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 },
		/* A */	{ 1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0 },
		/* ' */	{ 1,1,1,1,0,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0 },
		/* 2 */	{ 1,1,1,1,0,0,1,1,1,0,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,0 },
		/* / */	{ 1,1,1,1,0,0,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,0 },
		/* > */	{ 1,1,1,1,0,0,0,0,0,0,1,1,0,1,0,0,0,1,0,0,0,1,0,1,1,0 },
		/* | */	{ 1,1,1,1,0,0,0,0,1,1,1,0,1,1,1,1,1,1,0,1,0,1,0,0,1,0 },
		/* [ */	{ 1,1,1,1,0,0,0,0,1,0,1,0,1,1,1,0,0,1,0,1,0,1,0,0,1,0 },
		/*   */	{ 1,1,1,1,0,0,0,0,1,1,1,0,1,1,1,1,0,1,0,1,0,1,0,1,0,0 },
		/* - */	{ 1,1,1,1,0,0,0,0,1,0,1,0,1,1,1,1,1,1,0,0,0,1,0,1,1,0 },
		/* t */	{ 1,1,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,1,1,1,1,1,0,0,0,0 },
		/* H */	{ 1,1,1,1,0,0,0,0,1,0,1,0,1,1,1,1,1,1,0,1,0,1,0,1,1,0 },
		/* \ */	{ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0 },
		/* * */	{ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0 },
		/* n */	{ 1,1,1,1,0,0,0,0,1,1,1,0,1,1,0,0,1,1,0,1,0,1,0,0,1,0 },
		/* { */	{ 0,0,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0 },
		/* } */	{ 1,1,1,1,0,0,0,0,1,0,1,0,1,1,1,1,1,1,0,1,0,1,0,1,1,0 },
		/* [ */	{ 0,1,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0 },
		/* ] */	{ 1,1,1,1,0,0,0,0,1,0,1,0,1,1,1,1,1,1,0,1,0,1,0,1,1,0 },
		/* ( */	{ 0,1,1,1,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0 },
		/* ) */	{ 1,1,1,1,0,0,0,1,1,0,1,1,1,1,1,1,1,1,0,1,0,1,1,1,1,0 },
		/* & */	{ 1,1,1,1,0,0,0,0,1,0,1,0,1,1,0,0,0,1,0,1,0,1,0,0,0,0 },
		/* % */	{ 1,1,1,1,0,0,0,0,1,1,1,0,1,1,0,0,1,1,0,1,0,1,0,0,1,0 },
			{ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 }
	};

#ifdef TEST
void	main(void)
{
	int	i,j;

	for (i = 0; i < MAX_TKNS-1; ++i) {
		for (j = 0; j < MAX_TKNS-1; ++j) {
			(void) printf("%s followed by %s is ",
				token_names[i],token_names[j]);
			if (syntax_table[i][j])
				(void) printf("OK\n");
			if (syntax_table[i][j] == 0)
				(void) printf("BAD\n");
		}
	}
}
#else

void	abc_warning(char *fmt, ...)
{
	va_list	ap;
	if (output.warnings == 0) return;
	va_start(ap,fmt);
	(void) fprintf(stdout,"WARNING: line no. %d - ",input_line);
	(void) vfprintf(stdout,fmt,ap);
	(void) fprintf(stdout,"\n");
	va_end(ap);
}

void	abc_error(char *fmt, ...)
{
	va_list	ap;
	va_start(ap,fmt);
	(void) fprintf(stderr,"error in input file %s: line no. %d - ",
				abc_file,input_line);
	(void) vfprintf(stderr,fmt,ap);
	(void) fprintf(stderr,"\n");
	va_end(ap);
	exit(1);
}

FILE	*openIn(char *filename)
;

char	*getsIn(char *s)
{
	if (fgets(s,999,In) == NULL) {
		return(NULL);
	} else {
		input_line += 1;
		if (s[0] == '%') just_comment = 1;
		else just_comment = 0;
		return(s);
	}
}

void	closeIn(void)
{
	(void) fclose(In);
}

void	read_settings(void)
;

static	int	hcf(int a, int b)
{
	int	ans = 1, p;
	const	int	prime[] = { 2, 3, 5, 7, 11, 13, 17, 0 };

	for (p = 0; prime[p] && a > prime[p] && b > prime[p]; ++p) {
		while (a%prime[p] == 0 && b%prime[p] == 0) {
			a /= prime[p];
			b /= prime[p];
			ans *= prime[p];
		}
	}
	return(ans);
}

static	void	add_frac(frac a, frac b, frac *ans)
;

char	*end_of(char *s)
;

int	is_field(char *line)
{
	if (line[1] == ':' && 'A' <= line[0] && line[0] <= 'Z')
		return(1);
	else
		return(0);
}

void	strip(char *str, char *comment)
;

void	stripcpy(char *out_str, char *in_str)
{
	static	char	dummy[999];
	strip(in_str,dummy);
	(void) strcpy(out_str,in_str);
}

static	const	char	*Notes[]={
				"",  "",
				"",  "",  "",  "",  "",  "",  "",
				"C,","D,","E,","F,","G,","A,","B,",
				"C", "D", "E", "F", "G", "A", "B",
				"c", "d", "e", "f", "g", "a", "b",
				"c'","d'","e'","f'","g'","a'","b'"
			};

static	void	transpose_note(void)
{

	const	char	*Accidental_array[] = { "__", "_", "=", "^", "^^" };
	const	char	**Accidental = Accidental_array+2;

	if (note->iaccidental)
		(void) fprintf(Trans,"%s",Accidental[note->iaccidental-3]);
	if (note->pitch == -1)
		(void) fprintf(Trans,"z");
	else
		(void) fprintf(Trans,"%s",Notes[note->pitch]);
	tnote = 0;
}

static	void	end_beam(void)
;

static	int	global_accidentals[7];

static	void	new_symbol(int type)
;

static	void	output_transpose(void)
{
	int	i;

	if (output.transpose == 0) return;

	if (note && tnote) transpose_note();

	for (i = 0; i < token_length; ++i)
		(void) fputc(*(token_ptr+i),Trans);
}

void	output_transline(char *s)
{
	nblanks = 0;
	(void) fputs(s,Trans);
}

static	void	save_two_bars(void)
;

static	void	check_syntax(int new_token)
{
	int	e;
	const	char	*env_names[] = {
				"in grace",
				"in chord",
				"in broken",
				"in tie",
				"in tuplet",
				""
			};
	const	int	env_table[MAX_ENVS][MAX_TKNS+1] = {
	/*		  " ~ = A ' 2 / > | [   - t H \ * n { } [ ] ( ) & % */
	/* grace  */	{ 0,0,1,1,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,1,1,0,0,0 },
	/* chord  */	{ 0,1,1,1,1,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,1,1,1,0,0,0 },
	/* broken */	{ 1,1,1,1,0,0,0,0,0,0,1,1,0,1,0,0,0,1,1,0,0,1,0,0,0,0 },
	/* tie	  */	{ 1,1,1,1,0,0,0,0,1,1,1,0,1,1,1,1,1,1,1,0,0,1,1,0,1,0 },
	/* tuplet */	{ 1,1,1,1,1,1,1,0,0,0,0,0,0,1,0,0,0,1,1,1,1,1,1,0,0,0 },
	};

	prev_token = token;
	token = new_token;

	for (e = 0; e < MAX_ENVS; ++e)
		if (env[e] && env_table[e][token] == 0)
			abc_error("syntax error - %s cannot be used %s",
				token_names[token],env_names[e]);

	if (syntax_table[prev_token][token] == 0)
		abc_error("syntax error - %s cannot follow %s",
			token_names[token],token_names[prev_token]);
}

static	void	set_base(Field *meter)
;

void	get_dnl(Record *entry)
;

void	set_dnl(char *dnl_str)
{
	while (*dnl_str == ' ') ++dnl_str;

	     if ((strncmp(dnl_str,"1/4" ,3)) == 0) dnl=CROTCHET;
	else if ((strncmp(dnl_str,"1/8" ,3)) == 0) dnl=QUAVER;
	else if ((strncmp(dnl_str,"1/16",3)) == 0) dnl=SEMIQUAVER;
	else if ((strncmp(dnl_str,"1/32",4)) == 0) dnl=DEMISEMIQUAVER;
	else {
		(void) printf("default note length not recognised\n");
		dnl = QUAVER;
	}
}

static	void	ignore_space(char *s, int *c)
{
	while (s[*c] == ' ') {
		if (transpose || offset)
			(void) fprintf(Trans," ");
		*c += 1;
	}
}

static	void	ignore_alpha(char *s, int *c)
{
	while ((s[*c] >= 'A' && s[*c] <= 'Z')
	    || (s[*c] >= 'a' && s[*c] <= 'z')) {
		if (transpose || offset)
			(void) fprintf(Trans,"%c",s[*c]);
		*c += 1;
	}
}

static	void	transpose_accidental(int pitch)
{
	int	sharpness = note->iaccidental - 3;
	sharpness += ((54-2*(pitch-9))%7+transpose+14)/7-2;
	if (sharpness < -2)
		g_error("transpose - too many flats");
	if (sharpness >  2)
		g_error("transpose - too many sharps");
	note->iaccidental = sharpness + 3;
}

static	int	mystrncasecmp(char *s1, char *s2, int n)
{
	char	tmp1[99],tmp2[99];
	int	i;

	if (n > 98) g_error("string too long in mystrncasecmp");

	for (i = 0; i < n && s1[i]; ++i)
		tmp1[i] = lower(s1[i]);
	tmp1[i] = '\0';
	for (i = 0; i < n && s2[i]; ++i)
		tmp2[i] = lower(s2[i]);
	tmp2[i] = '\0';
	return(strncmp(tmp1,tmp2,n));
}

static	void	sharps_flats(Field *key)
{
	Note	dummy;
	int	pitch;
	int	acc;
	int	c = 0,s_f, i;
	char	*s = &key->string[2];
	const	char	*key_letter = { "FCGDAEB" };

	if (key->string[0] == 'K' && (offset || transpose))
		(void) fprintf(Trans,"K:");

	ignore_space(s,&c);

	for (s_f = -1; s_f < 6; ++s_f)
		if (s[c] == key_letter[s_f+1]) break;

	if (s_f == 6) {
		       if (s[c] == 'H' && s[c+1] == 'p') {
			s_f = 2;
			key->info2 = 1;
			c += 1;
		} else if (s[c] == 'H' && s[c+1] == 'P') {
			s_f = 2;
			key->info2 = 2;
			c += 1;
		} else {
			if (key->string[0] == 'G') {
			(void) printf("Gchord not recognised - using C\n");
			} else {
				(void) printf("Key not recognised - using C\n");
				bass = 0;
				treble = 1;
			}
			key->info1 = transpose;
			return;
		}
	}
	c += 1;

	       if (s[c] == '#') {
		s_f += 7;
		c += 1;
	} else if (s[c] == 'b') {
		s_f -= 7;
		c += 1;
	}

	if (offset || transpose)
		(void) fprintf(Trans,"%s",Key[s_f+transpose]);

	ignore_space(s,&c);

	       if ((mystrncasecmp(&s[c],"Lyd",3)) == 0) {	/* Lydian */
		s_f += 1;
		ignore_alpha(s,&c);
	} else if ((mystrncasecmp(&s[c],"Maj",3)) == 0		/* Major */
		|| (mystrncasecmp(&s[c],"Ion",3)) == 0) {	/* Ionian */
		ignore_alpha(s,&c);
	} else if ((mystrncasecmp(&s[c],"Mix",3)) == 0) {	/* Mixolydian */
		s_f -= 1;
		ignore_alpha(s,&c);
	} else if ((mystrncasecmp(&s[c],"Dor",3)) == 0) {	/* Dorian */
		s_f -= 2;
		ignore_alpha(s,&c);
	} else if ((mystrncasecmp(&s[c],"Min",3)) == 0		/* Minor */
		|| (mystrncasecmp(&s[c],"Aeo",3)) == 0) {	/* Aeolian */
		s_f -= 3;
		ignore_alpha(s,&c);
	} else if ((mystrncasecmp(&s[c],"Phr",3)) == 0) {	/* Phrygian */
		s_f -= 4;
		ignore_alpha(s,&c);
	} else if ((mystrncasecmp(&s[c],"Loc",3)) == 0) {	/* Locrian */
		s_f -= 5;
		ignore_alpha(s,&c);
	} else if (s[c] == 'm') {
		s_f -= 3;
		ignore_alpha(s,&c);
	}

	ignore_space(s,&c);

	if ('0' <= s[c] && s[c] <= '9'
	 && s[c+1] == '&'
	 && '0' <= s[c+2] && s[c+2] <= '9') { /* get bass & treble clef info */
		bass = s[c] - '0';
		treble = s[c+2] - '0';
		if (transpose || offset) {
			for (i = 0; i < 3; ++i)
				(void) fprintf(Trans,"%c",s[c+i]);
		}
	} else {
		bass = 0;
		treble = 1;
	}

	/* reset global accidentals */
	for (i = 0; i < 7; ++i)
		global_accidentals[i] = '\0';

	for (; s[c] != '\n' && s[c] != '\0'; ++c) {
		ignore_space(s,&c);
		if (strchr("_=^",s[c]) && s[c+1]) {
			c += 1;
			switch (s[c-1]) {
			case '^':
				if (s[c] == '^' && s[c+1]) {
					c += 1;
					acc = DBL_SHARP;
				} else acc = SHARP;
				break;
			case '_':
				if (s[c] == '_' && s[c+1]) {
					c += 1;
					acc = DBL_FLAT;
				} else acc = FLAT;
				break;
			case '=':
				acc = NATURAL;
				break;
			}
			note = &dummy;
			note->iaccidental = acc;

			switch (s[c]) {
			case 'c': case 'd': case 'e': case 'f': case 'g':
				pitch = s[c]-'c'+23+offset;
				break;
			case 'a': case 'b':
				pitch = s[c]-'a'+28+offset;
				break;
			default:
				abc_error("unrecognised accidental '%c'"
					  "in key string",s[c]);
				break;
			}

			if (transpose || offset) {
				transpose_accidental(pitch);
				while (pitch > 29) pitch -= 7;
				while (pitch < 23) pitch += 7;
				note->pitch = pitch;
				transpose_note();
			}

			global_accidentals[Notes[pitch][0]-'a']
				= note->iaccidental;
			note = NULL;
		}
	}

	key->info1 = s_f + transpose;
}

int	range(int *first,int *last,int *yfirst,int *ylast,char **input)
{
	*ylast = 4999;
	if (**input == '\0') {
		*first = 4999;
		*last = 4999;
		*yfirst = 4999;
		return(1);
	}
	if (**input == '-') {
		*first = 1;
		*yfirst = 1;
	} else if (**input >= '0' && **input <= '9') {
		(void) sscanf(*input,"%d",first);
		while (**input >= '0' && **input <= '9')
			(*input)++;
		if (**input == '.') {
			(*input)++;
			if (**input >= '1' && **input <= '9')
				*yfirst = **input - '0';
			else if (**input >= 'a' && **input <= 'z')
				*yfirst = **input - 'a' + 10;
			else
				return(0);
			(*input)++;
		} else {
			*yfirst = 0;
		}
	} else
		return(0);

	if (**input == '-') {
		(*input)++;
		if (**input == '\0') {
			*last = 4999;
		} else if (**input >= '0' && **input <= '9') {
			(void) sscanf(*input,"%d",last);
			while (**input >= '0' && **input <= '9')
				(*input)++;
			if (**input == '.') {
				(*input)++;
				if (**input >= '1' && **input <= '9')
					*ylast = **input - '0';
				else if (**input >= 'a' && **input <= 'z')
					*ylast = **input - 'a' + 10;
				else
					return(0);
				(*input)++;
			}
			if (**input == ',')
				(*input)++;
		} else
			return(0);
	} else if (**input == ',') {
		*last = *first;
		if (*yfirst != 0) *ylast = *yfirst;
		(*input)++;
	} else if (**input == '\0') {
		*last = *first;
		if (*yfirst != 0) *ylast = *yfirst;
	} else
		return(0);
	return(1);
}

static	void	process_field(char *str)
;

static	void	process_trailing(void)
;

static	void	process_gchord(char *str)
{
	int	ctr, g;
	char	temp[99];
	char	gchord[5];
	Field	gchord_field;

/*
	if (!note || note->pitch)
*/
		new_symbol(NOTE);

	check_syntax(GCHORD_TKN);

	if (offset || transpose) {
		temp[0] = '\0';
		ctr = 0;
		if (str[ctr] == '(')
			(void) sprintf(end_of(temp),"%c",str[ctr++]);
		gchord_field.string = &gchord[0];
		gchord_field.info1 = gchord_field.info2 = 0;
		(void) strcpy(gchord,"G:");
		g = 2;
		gchord[g++] = str[ctr++];
		if (str[ctr] == '#' || str[ctr] == 'b')
			gchord[g++] = str[ctr++];
		gchord[g] = '\0';
		(void) fprintf(Trans,"\"");
		sharps_flats(&gchord_field);
		(void) sprintf(end_of(temp),"%s%s",
			Key[gchord_field.info1],&str[ctr]);
		(void) fprintf(Trans,"%s\"",&str[ctr]);
		(void) strcpy(str,temp);
	}

	if (note->gchord)
		abc_error("gchord already set");
	ALLOC(note->gchord,char,(strlen(str)+1));
	(void) strcpy(note->gchord,str);
}

static	void	process_macro(char c)
{
	check_syntax(MACRO_TKN);

	if (!note || note->pitch || strchr(note->end,'b'))
		new_symbol(NOTE);

	(void) sprintf(end_of(note->attributes),"%c",c);

	output_transpose();
}

static	void	process_accent(char c)
{
	if (!note || note->pitch || strchr(note->end,'b'))
		new_symbol(NOTE);

	check_syntax(ACCENT_TKN);

	output_transpose();

/* not sure about this, maybe remove later */
	if (!env[IN_GRACE] && (!env[IN_CHORD] || chord_root->chord == 0))
		save_two_bars();

	(void) sprintf(end_of(note->attributes),"%c",c);
}

static	void	process_accidental(int accidental)
{
	if (!note || note->pitch || strchr(note->end,'b'))
		new_symbol(NOTE);

	check_syntax(ACCIDENTAL_TKN);

	if (!env[IN_GRACE] && (!env[IN_CHORD] || chord_root->chord == 0))
		save_two_bars();

	if (note->iaccidental)
		abc_error("accidental already set");

	note->iaccidental = accidental;
}

static	void	process_note(int pitch)
;

static	void	process_octaver(int octaver)
{
	check_syntax(OCTAVER_TKN);

	if (!note || note->pitch <= 0)
		abc_error("octaver attached to non note");

	if (note->type == NORMAL) save_two_bars();

	note->pitch += octaver;

	if (note->pitch <= 0)
		abc_error("octaver puts note out of range");
}

static	void	process_length(int length)
{
	check_syntax(LENGTH_TKN);

	output_transpose();

	if (note->type == NORMAL) save_two_bars();

	if (!note || note->pitch == 0)
		abc_error("length attached to non note");
	if (length > 99)
		abc_error("length too long");

	note->length *= length;
}

static	void	process_divisor(int length)
;

static	void	process_broken(int power)
{
	int	i;

	check_syntax(BROKEN_TKN);

	output_transpose();

	save_two_bars();

	if (!note || note->pitch == 0)
		abc_error("length attached to non note");
	broken.d = 1;
	for (i = 0; i < abs(power); ++i)
		broken.d *= 2;
	if (power > 0) {
		note->length *= (2*broken.d - 1);
		broken.n = 1;
	} else {
		broken.n = 2*broken.d - 1;
	}
	if (note->length%broken.d)
		abc_error("note length will not divide by %d",broken.d);
	note->length /= broken.d;
	env[IN_BROKEN] = 1;
}

static	void	process_bar(int bar_type)
;

static	void	process_repeat(int no)
{
	Symbol	*bar;

	check_syntax(REPEAT_TKN);

	output_transpose();

/* not sure about this, maybe remove later */
	save_two_bars();

	for (bar = current; bar && bar->type != BAR_LINE; bar = bar->prev);
	if (!bar) abc_error("");
	if (bar->u.bar.repeat_no)
		abc_error("repeat already set");
	if (no == 1 && bar->u.bar.type == BAR)
		bar->u.bar.type = BAR1;
	else if (no == 2 && bar->u.bar.type == RREPEAT)
		bar->u.bar.type = RREPEAT2;
	else
		abc_error("repeat inappropriate for bar type");
}

static	void	process_space(void)
{
	end_beam();

	check_syntax(SPACE_TKN);

	output_transpose();

	save_two_bars();
}

static	void	process_tie(void)
;

static	void	process_continuation(void)
{
	end_beam();

	check_syntax(CONTINUE_TKN);

	output_transpose();

	continuation = 1;
}

static	void	process_newline(void)
{
	end_beam();

	check_syntax(NEWLINE_TKN);

	output_transpose();

	if (!continuation) {
		current->newline = 1;
		if (settings.justification) current->justify = 1;
	}
}

static	void	process_open_chord(void)
{
	if (!note || note->pitch || strchr(note->end,'b'))
		new_symbol(NOTE);

	check_syntax(OPEN_CHORD_TKN);

	output_transpose();

	env[IN_CHORD] = 1;
	chord_root = &current->u.note;
	chord_root->chord = -1;
}

static	void	process_close_chord(void)
{
	check_syntax(CLOSE_CHORD_TKN);
	if (!env[IN_CHORD]) abc_error("not in chord");

	if (chord_root->length == 0) abc_error("empty chord");
	if (chord_root->chord <= 0) abc_error("empty chord");

	if (chord_root->chord == 1
	 && chord_root->length == note->length
	 && chord_root->pitch == note->pitch) { /* this is a unison */
		(void) strcat(chord_root->start,"u");
		end_beam();
	}

	output_transpose();

	env[IN_CHORD] = 0;

	chord_root = NULL;
}

static	void	process_open_close_chord(void)
{
	if (settings.old_chords == 0)
		abc_error("++ syntax obsolete; use [] or change settings file");
	if (env[IN_CHORD]) process_close_chord();
	else process_open_chord();
}

static	void	process_open_grace(void)
{
	check_syntax(OPEN_GRACE_TKN);

	output_transpose();

	env[IN_GRACE] = 1;
}

static	void	process_close_grace(void)
{
	check_syntax(CLOSE_GRACE_TKN);
	if (!env[IN_GRACE]) abc_error("not in grace");

	output_transpose();

	if (beam_start->type == GRACE) end_beam();
	env[IN_GRACE] = 0;
}

static	void	process_open_slur(void)
;

static	void	process_close_slur(void)
;

static	void	process_open_close_slur(void)
;

static	void	process_tuplet(char *s)
;

static	void	process_justify(void)
{
	check_syntax(JUSTIFY_TKN);

	output_transpose();

	current->justify = 1;
}

static	void	process_ampersand(int level)
{
	end_beam();

	new_symbol(MISC);

	check_syntax(AMPERSAND_TKN);

	output_transpose();

	if (level == 1) stave += 1;
	else if (level == 2) stave = 0;

	current->u.misc.level = level;
}

static	void	process_beams(int n, Symbol *s)
{
	Note	*beam = NULL;
	Note	*grace = NULL;
	int	i;

	for (i = 0; i < n; ++i) {
		if (s->type != NOTE || s->u.note.pitch == 0) {
			s = s->next;
			continue;
		}
		beam = &s->u.note;
		beam->n_notes = 1;
		if (!strchr(beam->start,'t') && beam->pitch == -1) {
			s = s->next;
			continue;
		}
		while (s->next && s->next->type == NOTE
		 && s->next->u.note.pitch && !strchr(s->u.note.end,'b')
		 && (s->next->u.note.type == CHORD
		  || (beam->length < CROTCHET
		  && s->next->u.note.length < CROTCHET))) {
			if (!strchr(beam->start,'t')
			 && s->next->u.note.pitch == -1) break;
			if (beam->type == GRACE) {
				if (s->next->u.note.type != GRACE) break;
			} else { /* beam->type != GRACE */
				if (s->next->u.note.type == GRACE) {
					if (!grace) grace = &s->next->u.note;
					grace->n_notes += 1;
				} else { /* s->next->u.note.type != GRACE */
					grace = NULL;
				}
			}
			beam->n_notes += 1;
			s = s->next;
			i += 1;
		}
		if (grace) {
			beam->n_notes -= grace->n_notes;
			grace = NULL;
		}
		s = s->next;
	}
}

static	void	set_semitones(int s_f)
{
	int	i,no;

	for (i = 0; i < 7; ++i)
		semitones[i] = Csemitones[i];

	if (s_f > 0) {
		no = 3;
		for (i = 0; i < s_f; ++i) {
			semitones[no] += 1;
			no = (no+4)%7;
		}
	} else if (s_f < 0) {
		no = 6;
		for (i = 0; i > s_f; --i) {
			semitones[no] -= 1;
			no = (no+3)%7;
		}
	}
}

static	int	distance_from_C(Note *n)
{
	int	note, octave;

	if (n->pitch == -1) return(REST);

	note = (n->pitch - 2)%7;
	octave = ((n->pitch - 2)/7) - 2;

	if (n->iaccidental) {
		if (semitones[note] == Csemitones[note])
			semitones[note] += n->iaccidental - 3;
		else if (semitones[note] == Csemitones[note] + 1)
			semitones[note] += n->iaccidental - 4;
		else if (semitones[note] == Csemitones[note] - 1)
			semitones[note] += n->iaccidental - 2;
		else
			g_error("semitones wrong");
	}

	return(semitones[note] + 12*octave);
}

static	void	tune2hash(Field *key, int *hash_array, int force)
{
	int	h = 0;
	int	note1,note2,first_note;
	int	plet = 0;
	Symbol	*s;
	int	j;

	set_semitones(key->info1);

	s = &block[0][0];
	if (!force) { /* force makes the conversion from all symbols */
		      /* - used for search strings */
		for (; s && s->u.bar.bar_no == 0; s = s->next)
			while (s->next && s->next->type != BAR_LINE)
				s = s->next;
	}

	while (s && (s->type != NOTE || s->u.note.type != NORMAL)) s = s->next;
	if (s) {
		if (strchr(s->u.note.start,'p')) plet = 1;
		note1 = distance_from_C(&s->u.note);
		first_note = note1;
		for (j = 1; j < s->u.note.length; ++j)
			hash_array[h++] = 0;
		s = s->next;
	}

	while (s) {
		while (s && (s->type != NOTE || s->u.note.type != NORMAL)) {
			if (s->type == BAR_LINE)
				set_semitones(key->info1);
			s = s->next;
		}
		if (s) {
			note2 = distance_from_C(&s->u.note);
			if (strchr(s->u.note.start,'p')) plet = 2;
			if (note2 == REST)
				note2 = note1;
/* plet doesn't work for other than triplets or quadruplets */
			if (plet == 1) /* second note of a tuplet */
				plet = 0; /* ignore it */
			else { /* print out the internote distances */
				hash_array[h++] = note2 - first_note;
				/* relative notes
				hash_array[h++] = note2 - note1; */
				for (j = 1; j < s->u.note.length; ++j)
					hash_array[h++] = note2 - first_note;
					/* relative notes
					hash_array[h++] = 0; */
				if (plet == 2) /* first note of a tuplet */
					plet = 1; /* reset plet */
				note1 = note2;
			}
			s = s->next;
		}
	}
	if (h == 0) hash_array[h++] = 999;
	hash_array[h++] = LAST;
}

void	process_abc(char title[][99], int titles, Record *entry,
		char *key_comment, char *bars, char *tune, char *input,
		int output_type, int nbars, int force, int *hash_array)
;

#endif


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
;

void	closeIn(void)
;

void	read_settings(void)
{
	FILE	*fp;
	char	line[99];

	if ((fp = fopen("settings","r")) == NULL) return;
	while (fgets(line,99,fp) != NULL) {
		     if (strncmp(line,"justify",strlen("justify")) == 0)
			settings.justification = 1;
		else if (strncmp(line,"gchords above",
				strlen("gchords above")) == 0)
			settings.gchords_above = 1;
		else if (strncmp(line,"autobeam",strlen("autobeam")) == 0)
			settings.autobeam = 1;
		else if (strncmp(line,"oldrepeats",strlen("oldrepeats")) == 0)
			settings.old_repeats = 1;
		else if (strncmp(line,"oldchords",strlen("oldchords")) == 0)
			settings.old_chords = 1;
		else if (strncmp(line,"oldslurs",strlen("oldslurs")) == 0)
			settings.old_slurs = 1;
		else if (strncmp(line,"mine",strlen("mine")) == 0)
			settings.mine = 1;
		else
			g_error("in settings - unrecognised line: %s",line);
	}
	(void) fclose(fp);
}

int	hcf(int a, int b)
;

void	add_frac(frac a, frac b, frac *ans)
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
{	/* removes trailing space, comment and
	   newline and copies it to comment */
	char	*c_ptr;

	comment[0] = '\0';
	c_ptr = strchr(str,'%');
	if (c_ptr == NULL) c_ptr = strchr(str,'\n');
	while (c_ptr > str && (*(c_ptr-1) == ' ' || *(c_ptr-1) == '\t'))
		--c_ptr;
	if (c_ptr) {
		(void) strcpy(comment,c_ptr);
		*c_ptr = '\0';
	}
}

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

void	transpose_note(void)
;

void	end_beam(void)
;

static	int	global_accidentals[7];

void	new_symbol(int type)
;

void	output_transpose(void)
;

void	output_transline(char *s)
{
	nblanks = 0;
	(void) fputs(s,Trans);
}

void	save_two_bars(void)
;

void	check_syntax(int new_token)
;

void	set_base(Field *meter)
;

void	get_dnl(Record *entry)
{
	char	meter[99];
	Field	meter_field;
	meter_field.string = &meter[0];
	meter_field.info1 = meter_field.info2 = 0;
	(void) strcpy(meter,"M:");
	(void) strcat(meter,entry->METER);
	set_base(&meter_field);
	(void) sprintf(entry->LENGTH,"1/%d",32/dnl);
}

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

void	ignore_space(char *s, int *c)
;

void	ignore_alpha(char *s, int *c)
;

void	transpose_accidental(int pitch)
;

int	mystrncasecmp(char *s1, char *s2, int n)
;

void	sharps_flats(Field *key)
;

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

void	process_field(char *str)
;

void	process_trailing(void)
;

void	process_gchord(char *str)
;

void	process_macro(char c)
;

void	process_accent(char c)
;

void	process_accidental(int accidental)
;

void	process_note(int pitch)
;

void	process_octaver(int octaver)
;

void	process_length(int length)
;

void	process_divisor(int length)
;

void	process_broken(int power)
;

void	process_bar(int bar_type)
;

void	process_repeat(int no)
;

void	process_space(void)
;

void	process_tie(void)
;

void	process_continuation(void)
;

void	process_newline(void)
;

void	process_open_chord(void)
;

void	process_close_chord(void)
;

void	process_open_close_chord(void)
;

void	process_open_grace(void)
;

void	process_close_grace(void)
;

void	process_open_slur(void)
;

void	process_close_slur(void)
;

void	process_open_close_slur(void)
;

void	process_tuplet(char *s)
;

void	process_justify(void)
;

void	process_ampersand(int level)
;

void	process_beams(int n, Symbol *s)
;

void	set_semitones(int s_f)
;

int	distance_from_C(Note *n)
;

void	tune2hash(Field *key, int *hash_array, int force)
;

void	process_abc(char title[][99], int titles, Record *entry,
		char *key_comment, char *bars, char *tune, char *input,
		int output_type, int nbars, int force, int *hash_array)
{
	int	c;
	int	i;
	char	*line;
	char	str[99];
	char	key[99];
	Field	key_field;
	char	meter[99];
	Field	meter_field;
	char	tempo[99];
	Field	tempo_field;
	Symbol	*s;

	output.type = output_type;
	if (offset || transpose)
		output.transpose = 1;
	else
		output.transpose = 0;
	output.nbars = nbars;
	if (output_type == TEX_OUTPUT || output_type == INDEX_OUTPUT)
		output.warnings = 1;
	else
		output.warnings = 0;

	/* initialisation */
	ignore = 0;
	n_symbols = -1;
	n_blocks = -1;
	current = NULL;
	note = NULL;
	beam_length = 0;
	new_symbol(BAR_LINE);
	if (!input) {
		ALLOC(line,char,999);
	}
	bar_start = &current->u.bar;
	token = NEWLINE_TKN;
	stave = bass = treble = 0;
	current_bar[0] = bar[0][0] = bar[1][0] = bar[2][0] = '\0';

	key_field.string = &key[0];
	key_field.info1 = key_field.info2 = 0;
	(void) strcpy(key,"K:");
	(void) strcat(key,entry->KEY);
	sharps_flats(&key_field);
	if (offset || transpose) (void) fprintf(Trans,"%s",key_comment);

	meter_field.string = &meter[0];
	meter_field.info1 = meter_field.info2 = 0;
	(void) strcpy(meter,"M:");
	(void) strcat(meter,entry->METER);
	set_base(&meter_field);
	if (entry->LENGTH[0]) set_dnl(entry->LENGTH);

	tempo_field.string = &tempo[0];
	tempo_field.info1 = tempo_field.info2 = 0;
	if (entry->TEMPO && entry->TEMPO[0]) {
		(void) strcpy(tempo,"Q:");
		(void) strcat(tempo,entry->TEMPO);
		tempo_field.info1 = dnl;
	}

	while (input || (getsIn(line)) != NULL) {

	fbar_total.n = 0;
	fbar_total.d = 1;
	bar_no = 0;

	if (input) line = input;

	if (tune[0]) (void) strcat(tune,line);

	for (i = 0; line[i] == ' ' || line[i] == '\t'; ++i);
	if (line[i] == '\n' ) {
		token_ptr = line;
		token_length = i+1;
		output_transpose();
		nblanks = 1;
		ignore = 0;
		break;
	}

	if (output.type == INDEX_OUTPUT && output.nbars == 0) continue;

	if (line[0] == '\\' || is_field(line)) {
		/* this is a field or raw tex */
		token_ptr = line;
		token_length = strlen(line);
		process_field(line);
		if (output.nbars) {
			     if (line[0] == 'L')
				(void) strcpy(entry->LENGTH,&line[2]);
			else if (line[0] == 'M')
				(void) strcpy(entry->METER,&line[2]);
		}
		continue;
	}

	continuation = 0;

	for (c = 0; line[c]; c += token_length) {

	if (output.nbars == 0 && nbars) break;

	token_ptr = &line[c];
	token_length = 1;

	switch(line[c]) {
	case 'C': case 'D': case 'E': case 'F': case 'G':
		process_note(line[c]-'C'+16+offset);
		break;
	case 'A': case 'B':
		process_note(line[c]-'A'+21+offset);
		break;
	case 'c': case 'd': case 'e': case 'f': case 'g':
		process_note(line[c]-'c'+23+offset);
		break;
	case 'a': case 'b':
		process_note(line[c]-'a'+28+offset);
		break;
	case 'z':
		process_note(-1);
		break;
	case ' ': case '\t':
		while (line[c+token_length] == ' '
		    || line[c+token_length] == '\t')
			++token_length;
		if (line[c+token_length] != '%'
		 && line[c+token_length] != '\n') {
			process_space();
			break;
		}
		/*FALLTHROUGH*/
	case '%':
		while (line[c+token_length] && line[c+token_length] != '\n')
			++token_length;
		if (just_comment && line[c+token_length] == '\n')
			++token_length;
		process_trailing();
		break;
	case '|':
		if (line[c+1] == '|') {
			token_length += 1;
			process_bar(DBL_BAR);
		} else if (line[c+1] == ']') {
			token_length += 1;
			process_bar(RDBL_BAR);
		} else if (line[c+1] == ':') {
			token_length += 1;
			process_bar(LREPEAT);
		} else if (line[c+1] == '1') {
			token_length += 1;
			process_bar(BAR1);
		} else {
			process_bar(BAR);
		}
		break;
	case ':':
		if (line[c+1] == '|') {
			token_length += 1;
			if (line[c+2] == '2') {
				token_length += 1;
				process_bar(RREPEAT2);
			} else {
				process_bar(RREPEAT);
			}
		} else if (line[c+1] == ':') {
			token_length += 1;
			process_bar(REPEAT);
		} else
			abc_error("");
		break;
	case '[':
		if (line[c+1] == '1' || line[c+1] == '2') {
			token_length += 1;
			process_repeat(atoi(&line[c+1]));
		} else if (line[c+1] == '|') {
			token_length += 1;
			process_bar(LDBL_BAR);
		} else
			process_open_chord();
		break;
	case ']':
		process_close_chord();
		break;
	case '1': case '2': case '3': case '4':
	case '5': case '6': case '7': case '8':
		while (strchr("0123456789",line[c+token_length]))
			token_length += 1;
		process_length(atoi(&line[c]));
		break;
	case '/':
		if (strchr("0123456789",line[c+1])) {
			while (strchr("0123456789",line[c+token_length]))
				token_length += 1;
			process_divisor(atoi(&line[c+1]));
		} else if (line[c+1] == '/') {
			token_length += 1;
			process_divisor(4);
		} else
			process_divisor(2);
		break;
	case '>':
		while (line[c+token_length] == '>')
			token_length += 1;
		process_broken(token_length);
		break;
	case '<':
		while (line[c+token_length] == '<')
			token_length += 1;
		process_broken(-1*token_length);
		break;
/* obsolete */
	case '+':
		process_open_close_chord();
		break;
	case 's':
		process_open_close_slur();
		break;
/* obsolete */
	case '*':
		process_justify();
		break;
	case '{':
		process_open_grace();
		break;
	case '}':
		process_close_grace();
		break;
	case '(':
		if (strchr("123456789",line[c+1])) {
			while (strchr(":0123456789",line[c+token_length]))
				token_length += 1;
			process_tuplet(&line[c+1]);
		} else process_open_slur();
		break;
	case ')':
		process_close_slur();
		break;
	case '-':
		process_tie();
		break;
	case '~': case '.': case 'u': case 'v':
		process_accent(line[c]);
		break;
	case '^':
		if (line[c+1] == '^') {
			token_length += 1;
			process_accidental(DBL_SHARP);
		} else process_accidental(SHARP);
		break;
	case '_':
		if (line[c+1] == '_') {
			token_length += 1;
			process_accidental(DBL_FLAT);
		} else process_accidental(FLAT);
		break;
	case '=':
		process_accidental(NATURAL);
		break;
	case '\'':
		process_octaver(7);
		break;
	case ',':
		process_octaver(-7);
		break;
	case '"':
		for (; line[c+token_length] && line[c+token_length] != '"';
				++token_length)
			str[token_length-1] = line[c+token_length];
		if (line[c+token_length] == '\0')
			abc_error("line ended mid gchord");
		token_length += 1;
		str[token_length-2] = '\0';
		process_gchord(str);
		break;
	case '\\':
		process_continuation();
		break;
	case '\n':
		process_newline();
		break;
	case '&':
		if (line[c+1] == '&') token_length += 1;
		process_ampersand(token_length);
		break;
	default:
		if (strchr("HIJKLMNOPQRSTUVWXYZ",line[c]))
			process_macro(line[c]);
		else
			abc_error("unrecognised symbol");
		break;
	}
	}

	if (just_comment == 0) {
		if (output.nbars && current_bar[0] && bar_no < 3)
			(void) strcpy(bar[bar_no],current_bar);
		output.nbars = 0;
	}

	if (input) break;

	}

	end_beam();

	current->newline = 2;
	if (settings.justification) current->justify = 1;

	n_symbols += 1;

	process_beams(n_symbols,&block[0][0]);

	if (output.type == TEX_OUTPUT)
		tune2tex(title,titles,entry,n_symbols,&block[0][0],
			&key_field,&meter_field,&tempo_field);
	else if (hash_array)
		tune2hash(&key_field,hash_array,force);
	else if (nbars) {
		bars[0] = '\0';
		if (nbars == ONE_BAR_PLUS || nbars == TWO_BARS_PLUS)
			(void) strcpy(bars,bar[0]);
		(void) strcat(bars,bar[1]);
		if (nbars == TWO_BARS || nbars == TWO_BARS_PLUS)
			(void) strcat(bars,bar[2]);
	}

	for (s = &block[0][0]; s; s = s->next) {
		if (s->type == NOTE && s->u.note.gchord)
			free(s->u.note.gchord);
		else if (s->type == FIELD)
			free(s->u.field.string);
	}
	for (i = 0; i <= n_blocks; ++i)
		free(block[i]);
	if (!input) free(line);
}

#endif


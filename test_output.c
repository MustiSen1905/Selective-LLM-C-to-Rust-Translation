# 0 "C-projects/abc2mtex/index.h"
# 0 "<built-in>"
# 0 "<command-line>"
# 1 "C-projects/abc2mtex/index.h"
# 41 "C-projects/abc2mtex/index.h"
struct record {
 char *fields[26];
 char *bars;
 struct record *next;
} ;

typedef struct record Record;

typedef struct {
 int n;
 int d;
} frac ;

typedef struct {
 int length;
 int type;
 int pitch;
 char attributes[9];
 char *gchord;
 int chord;
 int tuplet;
 char start[9];
 char end[9];
 int n_notes;
 int iaccidental;
 frac broken;
} Note ;

typedef struct {
 int gchords_above;
 int autobeam;
 int old_slurs;
 int old_chords;
 int old_repeats;
 int justification;
 int mine;
} Setting ;

typedef struct {
 int type;
 int repeat_no;
 int bar_no;
} Barline;

typedef struct {
 char *string;
 int info1,info2;
} Field;

typedef struct {
 int level;
} Misc;

struct symbol {
 int type;
 union {
  Note note;
  Barline bar;
  Field field;
  Misc misc;
 } u;
 int newline;
 int justify;
 struct symbol *next;
 struct symbol *prev;
} ;

typedef struct symbol Symbol;


extern int atoi(const char*);



extern void g_error(const char*,...);
extern void get_index(char*,char*);
extern void size_record(char*,int*,char*);
extern Record *alloc_record(char*,int*);
extern void free_record(struct record*,char*);
extern int get_record(char*,FILE*,struct record*);
extern int put_record(char*,FILE*,struct record*);
extern char lower(char);

char *gets(char *s);


extern FILE *openIn(char*);
extern char *getsIn(char*);
extern void closeIn(void);
extern void read_settings(void);
extern int is_field(char*);
extern void stripcpy(char*,char*);
extern int range(int*,int*,int*,int*,char**);
extern void process_abc(char[][99],int,Record*,char*,char*,char*,char*,int,
   int,int,int*);

enum output_types {
 NO_OUTPUT,
 TEX_OUTPUT,
 INDEX_OUTPUT,
 HASH_OUTPUT
};

enum two_bar_types { NO_BARS, ONE_BAR, TWO_BARS, ONE_BAR_PLUS, TWO_BARS_PLUS };

enum symbol_types { UNDETERMINED, BAR_LINE, NOTE, FIELD, MISC};
enum bar_types { BAR, BAR1, DBL_BAR, LDBL_BAR, RDBL_BAR, REPEAT, LREPEAT,
  RREPEAT, RREPEAT2 };
enum accidental_types { NONE, DBL_FLAT, FLAT, NATURAL, SHARP, DBL_SHARP };

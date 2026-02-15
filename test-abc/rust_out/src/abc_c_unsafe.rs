extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn sprintf(
        __s: *mut core::ffi::c_char,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn sscanf(
        __s: *const core::ffi::c_char,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fgets(
        __s: *mut core::ffi::c_char,
        __n: core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut core::ffi::c_char;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
    fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strncat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
        __n: size_t,
    ) -> *mut core::ffi::c_char;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn g_error(_: *const core::ffi::c_char, ...);
    fn getsIn(_: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    fn is_field(_: *mut core::ffi::c_char) -> core::ffi::c_int;
    fn tune2tex(
        _: *mut [core::ffi::c_char; 99],
        _: core::ffi::c_int,
        _: *mut Record,
        _: core::ffi::c_int,
        _: *mut Symbol,
        _: *mut Field,
        _: *mut Field,
        _: *mut Field,
    );
    fn abc_warning(fmt: *mut core::ffi::c_char, ...);
    fn abc_error(fmt: *mut core::ffi::c_char, ...);
    fn hcf(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int;
    fn transpose_note();
    fn output_transpose();
    fn check_syntax(new_token: core::ffi::c_int);
    fn set_dnl(dnl_str: *mut core::ffi::c_char);
    fn transpose_accidental(pitch: core::ffi::c_int);
    fn sharps_flats(key: *mut Field);
    fn process_gchord(str: *mut core::ffi::c_char);
    fn process_macro(c: core::ffi::c_char);
    fn process_accent(c: core::ffi::c_char);
    fn process_accidental(accidental: core::ffi::c_int);
    fn process_octaver(octaver: core::ffi::c_int);
    fn process_length(length: core::ffi::c_int);
    fn process_broken(power: core::ffi::c_int);
    fn process_repeat(no: core::ffi::c_int);
    fn process_space();
    fn process_continuation();
    fn process_newline();
    fn process_open_chord();
    fn process_close_chord();
    fn process_open_close_chord();
    fn process_open_grace();
    fn process_close_grace();
    fn process_justify();
    fn process_ampersand(level: core::ffi::c_int);
    fn process_beams(n: core::ffi::c_int, s: *mut Symbol);
    fn tune2hash(
        key: *mut Field,
        hash_array: *mut core::ffi::c_int,
        force: core::ffi::c_int,
    );
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record {
    pub fields: [*mut core::ffi::c_char; 26],
    pub bars: *mut core::ffi::c_char,
    pub next: *mut record,
}
pub type Record = record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac {
    pub n: core::ffi::c_int,
    pub d: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Note {
    pub length: core::ffi::c_int,
    pub type_0: core::ffi::c_int,
    pub pitch: core::ffi::c_int,
    pub attributes: [core::ffi::c_char; 9],
    pub gchord: *mut core::ffi::c_char,
    pub chord: core::ffi::c_int,
    pub tuplet: core::ffi::c_int,
    pub start: [core::ffi::c_char; 9],
    pub end: [core::ffi::c_char; 9],
    pub n_notes: core::ffi::c_int,
    pub iaccidental: core::ffi::c_int,
    pub broken: frac,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Setting {
    pub gchords_above: core::ffi::c_int,
    pub autobeam: core::ffi::c_int,
    pub old_slurs: core::ffi::c_int,
    pub old_chords: core::ffi::c_int,
    pub old_repeats: core::ffi::c_int,
    pub justification: core::ffi::c_int,
    pub mine: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barline {
    pub type_0: core::ffi::c_int,
    pub repeat_no: core::ffi::c_int,
    pub bar_no: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Field {
    pub string: *mut core::ffi::c_char,
    pub info1: core::ffi::c_int,
    pub info2: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Misc {
    pub level: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub type_0: core::ffi::c_int,
    pub u: C2RustUnnamed,
    pub newline: core::ffi::c_int,
    pub justify: core::ffi::c_int,
    pub next: *mut symbol,
    pub prev: *mut symbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub note: Note,
    pub bar: Barline,
    pub field: Field,
    pub misc: Misc,
}
pub type Symbol = symbol;
pub const FIELD: symbol_types = 3;
pub const NOTE: symbol_types = 2;
pub const TWO_BARS_PLUS: two_bar_types = 4;
pub const TWO_BARS: two_bar_types = 2;
pub const ONE_BAR_PLUS: two_bar_types = 3;
pub const TEX_OUTPUT: output_types = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Output {
    pub type_0: core::ffi::c_int,
    pub transpose: core::ffi::c_int,
    pub nbars: core::ffi::c_int,
    pub warnings: core::ffi::c_int,
}
pub const IN_TUPLET: env_types = 4;
pub const IN_GRACE: env_types = 0;
pub const NATURAL: accidental_types = 3;
pub const FLAT: accidental_types = 2;
pub const DBL_FLAT: accidental_types = 1;
pub const SHARP: accidental_types = 4;
pub const DBL_SHARP: accidental_types = 5;
pub const IN_TIE: env_types = 3;
pub const SPACE_TKN: token_types = 10;
pub const TIE_TKN: token_types = 11;
pub const CLOSE_SLUR_TKN: token_types = 22;
pub const OPEN_SLUR_TKN: token_types = 21;
pub const TUPLET_TKN: token_types = 12;
pub const DIVISOR_TKN: token_types = 6;
pub const LDBL_BAR: bar_types = 3;
pub const BAR1: bar_types = 1;
pub const BAR: bar_types = 0;
pub const BAR_TKN: token_types = 8;
pub const BAR_LINE: symbol_types = 1;
pub const REPEAT: bar_types = 5;
pub const RREPEAT: bar_types = 7;
pub const RREPEAT2: bar_types = 8;
pub const LREPEAT: bar_types = 6;
pub const RDBL_BAR: bar_types = 4;
pub const DBL_BAR: bar_types = 2;
pub const TRAILING_TKN: token_types = 24;
pub const IN_CHORD: env_types = 1;
pub const IN_BROKEN: env_types = 2;
pub const NOTE_TKN: token_types = 3;
pub const NEWLINE_TKN: token_types = 16;
pub const INDEX_OUTPUT: output_types = 2;
pub type output_types = core::ffi::c_uint;
pub const HASH_OUTPUT: output_types = 3;
pub const NO_OUTPUT: output_types = 0;
pub type two_bar_types = core::ffi::c_uint;
pub const ONE_BAR: two_bar_types = 1;
pub const NO_BARS: two_bar_types = 0;
pub type symbol_types = core::ffi::c_uint;
pub const MISC: symbol_types = 4;
pub const UNDETERMINED: symbol_types = 0;
pub type bar_types = core::ffi::c_uint;
pub type accidental_types = core::ffi::c_uint;
pub const NONE: accidental_types = 0;
pub type env_types = core::ffi::c_uint;
pub const MAX_ENVS: env_types = 5;
pub type token_types = core::ffi::c_uint;
pub const MAX_TKNS: token_types = 25;
pub const AMPERSAND_TKN: token_types = 23;
pub const CLOSE_CHORD_TKN: token_types = 20;
pub const OPEN_CHORD_TKN: token_types = 19;
pub const CLOSE_GRACE_TKN: token_types = 18;
pub const OPEN_GRACE_TKN: token_types = 17;
pub const JUSTIFY_TKN: token_types = 15;
pub const CONTINUE_TKN: token_types = 14;
pub const MACRO_TKN: token_types = 13;
pub const REPEAT_TKN: token_types = 9;
pub const BROKEN_TKN: token_types = 7;
pub const LENGTH_TKN: token_types = 5;
pub const OCTAVER_TKN: token_types = 4;
pub const ACCIDENTAL_TKN: token_types = 2;
pub const ACCENT_TKN: token_types = 1;
pub const GCHORD_TKN: token_types = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const CHORD: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const NORMAL: core::ffi::c_int = 0 as core::ffi::c_int;
pub const GRACE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const QUAVER: core::ffi::c_int = 4 as core::ffi::c_int;
#[no_mangle]
pub static mut Trans: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut settings: Setting = {
    let mut init = Setting {
        gchords_above: 0 as core::ffi::c_int,
        autobeam: 0 as core::ffi::c_int,
        old_slurs: 0 as core::ffi::c_int,
        old_chords: 0 as core::ffi::c_int,
        old_repeats: 0 as core::ffi::c_int,
        justification: 0 as core::ffi::c_int,
        mine: 0 as core::ffi::c_int,
    };
    init
};
#[no_mangle]
pub static mut bass: core::ffi::c_int = 0;
#[no_mangle]
pub static mut treble: core::ffi::c_int = 0;
#[no_mangle]
pub static mut dnl: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub static mut transpose: core::ffi::c_int = 0;
#[no_mangle]
pub static mut offset: core::ffi::c_int = 0;
#[no_mangle]
pub static mut nblanks: core::ffi::c_int = 0;
static mut In: *mut FILE = 0 as *const FILE as *mut FILE;
static mut abc_file: [core::ffi::c_char; 99] = [0; 99];
static mut input_line: core::ffi::c_int = 0;
static mut just_comment: core::ffi::c_int = 0;
static mut bar_start: *mut Barline = 0 as *const Barline as *mut Barline;
static mut beam_start: *mut Note = 0 as *const Note as *mut Note;
static mut note: *mut Note = 0 as *const Note as *mut Note;
static mut current: *mut Symbol = 0 as *const Symbol as *mut Symbol;
static mut n_notes: core::ffi::c_int = 0 as core::ffi::c_int;
static mut max: core::ffi::c_int = 0 as core::ffi::c_int;
static mut token: core::ffi::c_int = 0;
static mut token_length: core::ffi::c_int = 0 as core::ffi::c_int;
static mut token_ptr: *mut core::ffi::c_char = 0 as *const core::ffi::c_char
    as *mut core::ffi::c_char;
pub const MAX_N_BLOCKS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const BLOCK_LENGTH: core::ffi::c_int = 400 as core::ffi::c_int;
static mut n_blocks: core::ffi::c_int = 0;
static mut block: [*mut Symbol; 10] = [0 as *const Symbol as *mut Symbol; 10];
static mut n_symbols: core::ffi::c_int = 0 as core::ffi::c_int;
static mut compound_time: core::ffi::c_int = 0 as core::ffi::c_int;
static mut fbar_total: frac = frac { n: 0, d: 0 };
static mut bar_no: core::ffi::c_int = 0;
static mut bar: [[core::ffi::c_char; 99]; 3] = [[0; 99]; 3];
static mut current_bar: [core::ffi::c_char; 99] = [0; 99];
static mut in_old_slur: core::ffi::c_int = 0 as core::ffi::c_int;
static mut bar_length: core::ffi::c_int = 0;
static mut stave: core::ffi::c_int = 0;
static mut output: Output = Output {
    type_0: 0,
    transpose: 0,
    nbars: 0,
    warnings: 0,
};
static mut tnote: core::ffi::c_int = 0 as core::ffi::c_int;
static mut beam_length: core::ffi::c_int = 0 as core::ffi::c_int;
static mut chord_root: *mut Note = 0 as *const Note as *mut Note;
static mut tuplet_n_notes: core::ffi::c_int = 0 as core::ffi::c_int;
static mut tuplet_note_no: core::ffi::c_int = 0 as core::ffi::c_int;
static mut tuplet_fraction: frac = {
    let mut init = frac {
        n: 1 as core::ffi::c_int,
        d: 1 as core::ffi::c_int,
    };
    init
};
static mut broken: frac = {
    let mut init = frac {
        n: 0 as core::ffi::c_int,
        d: 0 as core::ffi::c_int,
    };
    init
};
static mut continuation: core::ffi::c_int = 0 as core::ffi::c_int;
static mut ignore: core::ffi::c_int = 0;
static mut env: [core::ffi::c_int; 5] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn openIn(mut filename: *mut core::ffi::c_char) -> *mut FILE {
    let mut savename: [core::ffi::c_char; 99] = [0; 99];
    if strcmp(filename, b"stdin\0" as *const u8 as *const core::ffi::c_char)
        == 0 as core::ffi::c_int
    {
        In = stdin;
    } else {
        In = fopen(filename, b"r\0" as *const u8 as *const core::ffi::c_char)
            as *mut FILE;
        if In.is_null() {
            strcpy(savename.as_mut_ptr(), filename);
            strcat(filename, b".abc\0" as *const u8 as *const core::ffi::c_char);
            In = fopen(filename, b"r\0" as *const u8 as *const core::ffi::c_char)
                as *mut FILE;
            if In.is_null() {
                printf(
                    b"file \"%s\" does not exist\n\0" as *const u8
                        as *const core::ffi::c_char,
                    savename.as_mut_ptr(),
                );
                strcpy(filename, savename.as_mut_ptr());
            }
        }
    }
    strcpy(abc_file.as_mut_ptr(), filename);
    input_line = 0 as core::ffi::c_int;
    just_comment = 0 as core::ffi::c_int;
    return In;
}
#[no_mangle]
pub unsafe extern "C" fn read_settings() {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: [core::ffi::c_char; 99] = [0; 99];
    fp = fopen(
        b"settings\0" as *const u8 as *const core::ffi::c_char,
        b"r\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        return;
    }
    while !(fgets(line.as_mut_ptr(), 99 as core::ffi::c_int, fp)).is_null() {
        if strncmp(
            line.as_mut_ptr(),
            b"justify\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"justify\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            settings.justification = 1 as core::ffi::c_int;
        } else if strncmp(
            line.as_mut_ptr(),
            b"gchords above\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"gchords above\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            settings.gchords_above = 1 as core::ffi::c_int;
        } else if strncmp(
            line.as_mut_ptr(),
            b"autobeam\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"autobeam\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            settings.autobeam = 1 as core::ffi::c_int;
        } else if strncmp(
            line.as_mut_ptr(),
            b"oldrepeats\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"oldrepeats\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            settings.old_repeats = 1 as core::ffi::c_int;
        } else if strncmp(
            line.as_mut_ptr(),
            b"oldchords\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"oldchords\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            settings.old_chords = 1 as core::ffi::c_int;
        } else if strncmp(
            line.as_mut_ptr(),
            b"oldslurs\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"oldslurs\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            settings.old_slurs = 1 as core::ffi::c_int;
        } else if strncmp(
            line.as_mut_ptr(),
            b"mine\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"mine\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            settings.mine = 1 as core::ffi::c_int;
        } else {
            g_error(
                b"in settings - unrecognised line: %s\0" as *const u8
                    as *const core::ffi::c_char,
                line.as_mut_ptr(),
            );
        }
    }
    fclose(fp);
}
unsafe extern "C" fn add_frac(mut a: frac, mut b: frac, mut ans: *mut frac) {
    let mut f: core::ffi::c_int = 0;
    (*ans).n = a.n * b.d + b.n * a.d;
    (*ans).d = a.d * b.d;
    f = hcf((*ans).n, (*ans).d);
    (*ans).n /= f;
    (*ans).d /= f;
}
#[no_mangle]
pub unsafe extern "C" fn end_of(
    mut s: *mut core::ffi::c_char,
) -> *mut core::ffi::c_char {
    return &mut *s
        .offset(
            (strlen as unsafe extern "C" fn(*const core::ffi::c_char) -> size_t)(s)
                as isize,
        ) as *mut core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn strip(
    mut str: *mut core::ffi::c_char,
    mut comment: *mut core::ffi::c_char,
) {
    let mut c_ptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    *comment.offset(0 as core::ffi::c_int as isize) = '\0' as i32 as core::ffi::c_char;
    c_ptr = strchr(str, '%' as i32);
    if c_ptr.is_null() {
        c_ptr = strchr(str, '\n' as i32);
    }
    while c_ptr > str
        && (*c_ptr.offset(-(1 as core::ffi::c_int as isize)) as core::ffi::c_int
            == ' ' as i32
            || *c_ptr.offset(-(1 as core::ffi::c_int as isize)) as core::ffi::c_int
                == '\t' as i32)
    {
        c_ptr = c_ptr.offset(-1);
    }
    if !c_ptr.is_null() {
        strcpy(comment, c_ptr);
        *c_ptr = '\0' as i32 as core::ffi::c_char;
    }
}
unsafe extern "C" fn end_beam() {
    if !note.is_null() {
        if env[IN_GRACE as core::ffi::c_int as usize] == 0
            && env[IN_TUPLET as core::ffi::c_int as usize] != 0
        {
            if tuplet_note_no == 0 as core::ffi::c_int {
                abc_error(
                    b"empty tuplet\0" as *const u8 as *const core::ffi::c_char
                        as *mut core::ffi::c_char,
                );
            }
            if tuplet_n_notes / 2 as core::ffi::c_int == tuplet_note_no {
                (*note).tuplet = env[IN_TUPLET as core::ffi::c_int as usize];
            }
            tuplet_note_no += 1;
            if tuplet_note_no == tuplet_n_notes {
                strcat(
                    ((*note).end).as_mut_ptr(),
                    b"s\0" as *const u8 as *const core::ffi::c_char,
                );
                env[IN_TUPLET as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
                tuplet_note_no = 0 as core::ffi::c_int;
                tuplet_n_notes = tuplet_note_no;
            }
        }
        n_notes = 0 as core::ffi::c_int;
        strcat(
            ((*note).end).as_mut_ptr(),
            b"b\0" as *const u8 as *const core::ffi::c_char,
        );
    } else if n_notes != 0 {
        g_error(b"no note\0" as *const u8 as *const core::ffi::c_char);
    }
    beam_length = 0 as core::ffi::c_int;
}
static mut global_accidentals: [core::ffi::c_int; 7] = [0; 7];
unsafe extern "C" fn new_symbol(mut type_0: core::ffi::c_int) {
    let mut index: core::ffi::c_int = 0;
    let mut fnote_length: frac = frac { n: 0, d: 0 };
    if !note.is_null() && (*note).pitch != 0 {
        if tnote != 0 {
            transpose_note();
        }
        if (*note).pitch != -(1 as core::ffi::c_int)
            && (*note).iaccidental == 0 as core::ffi::c_int
        {
            (*note).iaccidental = global_accidentals[((*note).pitch
                % 7 as core::ffi::c_int) as usize];
        }
    }
    if !note.is_null() && (*note).pitch == 0 as core::ffi::c_int {
        (*note).n_notes = 1 as core::ffi::c_int;
    }
    if !note.is_null() && (*note).broken.n != 0 {
        (*note).length *= (*note).broken.n;
        if (*note).length % (*note).broken.d != 0 {
            abc_error(
                b"note length will not divide by %d\0" as *const u8
                    as *const core::ffi::c_char as *mut core::ffi::c_char,
                (*note).broken.d,
            );
        }
        (*note).length /= (*note).broken.d;
        (*note).broken.d = 0 as core::ffi::c_int;
        (*note).broken.n = (*note).broken.d;
    }
    if !note.is_null() && (*note).type_0 == NORMAL {
        if env[IN_TUPLET as core::ffi::c_int as usize] != 0 {
            if tuplet_n_notes / 2 as core::ffi::c_int == tuplet_note_no {
                (*note).tuplet = env[IN_TUPLET as core::ffi::c_int as usize];
            }
            tuplet_note_no += 1;
            if tuplet_note_no == tuplet_n_notes {
                strcat(
                    ((*note).end).as_mut_ptr(),
                    b"s\0" as *const u8 as *const core::ffi::c_char,
                );
                env[IN_TUPLET as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
                tuplet_note_no = 0 as core::ffi::c_int;
                tuplet_n_notes = tuplet_note_no;
                end_beam();
            }
        } else {
            beam_length += (*note).length;
            if settings.autobeam != 0 && beam_length >= max {
                end_beam();
            }
        }
        fnote_length.n = (*note).length * tuplet_fraction.n;
        fnote_length.d = tuplet_fraction.d;
        if env[IN_TUPLET as core::ffi::c_int as usize] == 0 as core::ffi::c_int {
            tuplet_fraction.d = 1 as core::ffi::c_int;
            tuplet_fraction.n = tuplet_fraction.d;
        }
        if fbar_total.n >= 0 as core::ffi::c_int {
            add_frac(fbar_total, fnote_length, &mut fbar_total);
        }
    }
    n_symbols += 1 as core::ffi::c_int;
    if n_symbols % BLOCK_LENGTH == 0 as core::ffi::c_int {
        n_blocks += 1 as core::ffi::c_int;
        if n_blocks >= MAX_N_BLOCKS {
            g_error(
                b"out of memory - increase MAX_N_BLOCKS\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        block[n_blocks as usize] = calloc(
            400 as core::ffi::c_int as core::ffi::c_uint as size_t,
            ::core::mem::size_of::<Symbol>() as size_t,
        ) as *mut Symbol;
        if (block[n_blocks as usize]).is_null() {
            g_error(
                b"alloc failure in Symbol_array\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    index = n_symbols - n_symbols / BLOCK_LENGTH * BLOCK_LENGTH;
    if !current.is_null() {
        (*current).next = &mut *(*block.as_mut_ptr().offset(n_blocks as isize))
            .offset(index as isize) as *mut Symbol as *mut symbol;
        let ref mut fresh0 = (*(block[n_blocks as usize]).offset(index as isize)).prev;
        *fresh0 = current as *mut symbol;
        current = (*current).next as *mut Symbol;
    } else {
        current = &mut *(*block.as_mut_ptr().offset(n_blocks as isize))
            .offset(index as isize) as *mut Symbol;
    }
    (*current).type_0 = type_0;
    if type_0 == NOTE as core::ffi::c_int {
        note = &mut (*current).u.note;
    } else {
        note = 0 as *mut Note;
    };
}
unsafe extern "C" fn save_two_bars() {
    if output.nbars == 0 as core::ffi::c_int {
        return;
    }
    if token == SPACE_TKN as core::ffi::c_int {
        strcat(
            current_bar.as_mut_ptr(),
            b" \0" as *const u8 as *const core::ffi::c_char,
        );
    } else {
        strncat(current_bar.as_mut_ptr(), token_ptr, token_length as size_t);
    };
}
unsafe extern "C" fn set_base(mut meter: *mut Field) {
    let mut meter_str: *mut core::ffi::c_char = &mut *((*meter).string)
        .offset(2 as core::ffi::c_int as isize) as *mut core::ffi::c_char;
    let mut l: core::ffi::c_int = 0;
    dnl = QUAVER;
    l = (strlen(meter_str)).wrapping_sub(1 as size_t) as core::ffi::c_int;
    while *meter_str.offset(l as isize) as core::ffi::c_int == ' ' as i32 {
        l -= 1;
    }
    if *meter_str.offset(l as isize) as core::ffi::c_int == 'l' as i32 {
        dnl /= 2 as core::ffi::c_int;
    } else if *meter_str.offset(l as isize) as core::ffi::c_int == 's' as i32 {
        dnl *= 2 as core::ffi::c_int;
    }
    max = 16 as core::ffi::c_int;
    if strncmp(meter_str, b"2/4\0" as *const u8 as *const core::ffi::c_char, 3 as size_t)
        == 0 as core::ffi::c_int
    {
        max = 8 as core::ffi::c_int;
    } else if strncmp(
        meter_str,
        b"3/2\0" as *const u8 as *const core::ffi::c_char,
        3 as size_t,
    ) == 0 as core::ffi::c_int
    {
        max = 8 as core::ffi::c_int;
    } else if strncmp(
        meter_str,
        b"3/4\0" as *const u8 as *const core::ffi::c_char,
        3 as size_t,
    ) == 0 as core::ffi::c_int
    {
        max = 12 as core::ffi::c_int;
    } else if strncmp(
        meter_str,
        b"3/8\0" as *const u8 as *const core::ffi::c_char,
        3 as size_t,
    ) == 0 as core::ffi::c_int
    {
        max = 12 as core::ffi::c_int;
    } else if strncmp(
        meter_str,
        b"5/8\0" as *const u8 as *const core::ffi::c_char,
        3 as size_t,
    ) == 0 as core::ffi::c_int
    {
        max = 12 as core::ffi::c_int;
    } else if strncmp(
        meter_str,
        b"6/8\0" as *const u8 as *const core::ffi::c_char,
        3 as size_t,
    ) == 0 as core::ffi::c_int
    {
        max = 12 as core::ffi::c_int;
    } else if strncmp(
        meter_str,
        b"9/8\0" as *const u8 as *const core::ffi::c_char,
        3 as size_t,
    ) == 0 as core::ffi::c_int
    {
        max = 12 as core::ffi::c_int;
    } else if strncmp(
        meter_str,
        b"12/8\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) == 0 as core::ffi::c_int
    {
        max = 12 as core::ffi::c_int;
    }
    if *meter_str.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        == 'C' as i32
    {
        (*meter).info2 = 4 as core::ffi::c_int;
        if *meter_str.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            == '|' as i32
        {
            (*meter).info1 = -(2 as core::ffi::c_int);
        } else {
            (*meter).info1 = -(4 as core::ffi::c_int);
        }
        bar_length = 32 as core::ffi::c_int;
    } else {
        sscanf(
            meter_str,
            b"%d/%d\0" as *const u8 as *const core::ffi::c_char,
            &mut (*meter).info1 as *mut core::ffi::c_int,
            &mut (*meter).info2 as *mut core::ffi::c_int,
        );
        if (*meter).info1 < 1 as core::ffi::c_int
            || (*meter).info1 > 100 as core::ffi::c_int
            || ((*meter).info2 < 1 as core::ffi::c_int
                || (*meter).info2 > 100 as core::ffi::c_int)
        {
            printf(b"meter not recognised\n\0" as *const u8 as *const core::ffi::c_char);
            (*meter).info1 = -(4 as core::ffi::c_int);
            (*meter).info2 = 4 as core::ffi::c_int;
        } else {
            bar_length = (*meter).info1 * 32 as core::ffi::c_int / (*meter).info2;
            if (((*meter).info1 as core::ffi::c_float
                / (*meter).info2 as core::ffi::c_float) as core::ffi::c_double) < 0.75f64
            {
                dnl /= 2 as core::ffi::c_int;
            }
        }
    }
    if abs((*meter).info1) % 3 as core::ffi::c_int == 0 as core::ffi::c_int {
        compound_time = 1 as core::ffi::c_int;
    } else {
        compound_time = 0 as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_dnl(mut entry: *mut Record) {
    let mut meter: [core::ffi::c_char; 99] = [0; 99];
    let mut meter_field: Field = Field {
        string: 0 as *mut core::ffi::c_char,
        info1: 0,
        info2: 0,
    };
    meter_field.string = &mut *meter.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut core::ffi::c_char;
    meter_field.info2 = 0 as core::ffi::c_int;
    meter_field.info1 = meter_field.info2;
    strcpy(meter.as_mut_ptr(), b"M:\0" as *const u8 as *const core::ffi::c_char);
    strcat(meter.as_mut_ptr(), (*entry).fields[12 as core::ffi::c_int as usize]);
    set_base(&mut meter_field);
    sprintf(
        (*entry).fields[11 as core::ffi::c_int as usize],
        b"1/%d\0" as *const u8 as *const core::ffi::c_char,
        32 as core::ffi::c_int / dnl,
    );
}
unsafe extern "C" fn process_field(mut str: *mut core::ffi::c_char) {
    static mut comment: [core::ffi::c_char; 999] = [0; 999];
    let mut previous: *mut Symbol = current;
    if ignore != 0 {
        return;
    }
    check_syntax(NEWLINE_TKN as core::ffi::c_int);
    strip(str, comment.as_mut_ptr());
    if *str.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 'K' as i32
        && (offset != 0 || transpose != 0)
    {
        fprintf(Trans, b"%s\0" as *const u8 as *const core::ffi::c_char, str);
        fprintf(
            Trans,
            b"%s\0" as *const u8 as *const core::ffi::c_char,
            comment.as_mut_ptr(),
        );
    }
    new_symbol(FIELD as core::ffi::c_int);
    if *str.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int == 'M' as i32 {
        (*current).u.field.string = calloc(
            20 as core::ffi::c_int as core::ffi::c_uint as size_t,
            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
        ) as *mut core::ffi::c_char;
        if ((*current).u.field.string).is_null() {
            g_error(
                b"alloc failure in char_array\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    } else {
        (*current).u.field.string = calloc(
            (strlen(str)).wrapping_add(1 as size_t) as core::ffi::c_uint as size_t,
            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
        ) as *mut core::ffi::c_char;
        if ((*current).u.field.string).is_null() {
            g_error(
                b"alloc failure in char_array\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    strcpy((*current).u.field.string, str);
    match *str.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int {
        76 => {
            set_dnl(&mut *str.offset(2 as core::ffi::c_int as isize));
        }
        77 => {
            set_base(&mut (*current).u.field);
        }
        84 | 87 => {
            (*previous).newline = 2 as core::ffi::c_int;
            if settings.justification != 0 {
                (*previous).justify = 1 as core::ffi::c_int;
            }
        }
        75 => {
            sharps_flats(&mut (*current).u.field);
            if offset != 0 || transpose != 0 {
                fprintf(
                    Trans,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    comment.as_mut_ptr(),
                );
            }
        }
        81 => {
            (*current).u.field.info1 = dnl;
        }
        73 | 69 | 80 | 92 => {}
        _ => {
            abc_error(
                b"%c field not allowed in tune body\0" as *const u8
                    as *const core::ffi::c_char as *mut core::ffi::c_char,
                *str.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            );
        }
    };
}
unsafe extern "C" fn process_trailing() {
    check_syntax(TRAILING_TKN as core::ffi::c_int);
    output_transpose();
}
unsafe extern "C" fn process_note(mut pitch: core::ffi::c_int) {
    if note.is_null() || (*note).pitch != 0
        || !(strchr(((*note).end).as_mut_ptr(), 'b' as i32)).is_null()
    {
        new_symbol(NOTE as core::ffi::c_int);
    }
    check_syntax(NOTE_TKN as core::ffi::c_int);
    if offset != 0 || transpose != 0 {
        if (*note).iaccidental != 0 {
            transpose_accidental(pitch);
        }
        tnote = 1 as core::ffi::c_int;
    }
    if n_notes == 0 as core::ffi::c_int {
        beam_start = &mut (*current).u.note;
    }
    n_notes += 1 as core::ffi::c_int;
    (*note).pitch = pitch;
    if stave < bass && pitch != -(1 as core::ffi::c_int) {
        (*note).pitch -= 7 as core::ffi::c_int;
    }
    if env[IN_GRACE as core::ffi::c_int as usize] != 0 {
        (*note).length = 1 as core::ffi::c_int;
    } else {
        (*note).length = dnl;
    }
    if env[IN_GRACE as core::ffi::c_int as usize] != 0 {
        (*note).type_0 = GRACE;
    } else {
        if env[IN_TIE as core::ffi::c_int as usize] != 0 {
            strcat(
                ((*note).end).as_mut_ptr(),
                b"t\0" as *const u8 as *const core::ffi::c_char,
            );
            env[IN_TIE as core::ffi::c_int as usize] = 0 as core::ffi::c_int;
        }
        if env[IN_BROKEN as core::ffi::c_int as usize] != 0 {
            (*note).broken = broken;
            broken.d = 0 as core::ffi::c_int;
            broken.n = broken.d;
            env[IN_BROKEN as core::ffi::c_int as usize] = broken.n;
        }
        if env[IN_TUPLET as core::ffi::c_int as usize] != 0
            && tuplet_note_no == 0 as core::ffi::c_int
        {
            strcat(
                ((*note).start).as_mut_ptr(),
                b"s\0" as *const u8 as *const core::ffi::c_char,
            );
            strcat(
                ((*note).start).as_mut_ptr(),
                b"p\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    if env[IN_CHORD as core::ffi::c_int as usize] != 0 {
        (*chord_root).chord += 1 as core::ffi::c_int;
        if (*chord_root).chord > 0 as core::ffi::c_int {
            (*note).type_0 = CHORD;
        }
    }
    if (*note).type_0 == NORMAL {
        save_two_bars();
    }
}
unsafe extern "C" fn process_divisor(mut length: core::ffi::c_int) {
    check_syntax(DIVISOR_TKN as core::ffi::c_int);
    output_transpose();
    if (*note).type_0 == NORMAL {
        save_two_bars();
    }
    if note.is_null() || (*note).pitch == 0 as core::ffi::c_int {
        abc_error(
            b"divisor attached to non note\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char,
        );
    }
    if (*note).length % length != 0 {
        abc_error(
            b"note length will not divide by %d\0" as *const u8
                as *const core::ffi::c_char as *mut core::ffi::c_char,
            length,
        );
    }
    (*note).length /= length;
}
unsafe extern "C" fn process_bar(mut bar_type: core::ffi::c_int) {
    let mut bar_total: core::ffi::c_int = 0;
    end_beam();
    new_symbol(BAR_LINE as core::ffi::c_int);
    check_syntax(BAR_TKN as core::ffi::c_int);
    output_transpose();
    save_two_bars();
    if fbar_total.n >= 0 as core::ffi::c_int && bass + treble == 1 as core::ffi::c_int {
        if bar_no != 0 as core::ffi::c_int && fbar_total.n % fbar_total.d != 0 {
            abc_error(
                b"non-integer bar length\0" as *const u8 as *const core::ffi::c_char
                    as *mut core::ffi::c_char,
            );
        }
        bar_total = fbar_total.n / fbar_total.d;
        if bar_no == 0 as core::ffi::c_int && bar_total == bar_length {
            (*bar_start).bar_no = 1 as core::ffi::c_int;
            bar_no = 1 as core::ffi::c_int;
        }
        if bar_no != 0 as core::ffi::c_int && bar_total < bar_length
            && (bar_type == BAR as core::ffi::c_int
                || bar_type == BAR1 as core::ffi::c_int)
        {
            abc_warning(
                b"bar number %d is too short\0" as *const u8 as *const core::ffi::c_char
                    as *mut core::ffi::c_char,
                bar_no,
            );
        }
        if bar_total > bar_length {
            abc_warning(
                b"bar number %d is too long\0" as *const u8 as *const core::ffi::c_char
                    as *mut core::ffi::c_char,
                bar_no,
            );
        }
        if output.nbars != 0 && bar_no < 3 as core::ffi::c_int {
            strcpy((bar[bar_no as usize]).as_mut_ptr(), current_bar.as_mut_ptr());
        }
        current_bar[0 as core::ffi::c_int as usize] = '\0' as i32 as core::ffi::c_char;
        if bar_type == BAR as core::ffi::c_int || bar_type == BAR1 as core::ffi::c_int {
            bar_no += 1 as core::ffi::c_int;
        } else {
            if bar_no != 0 {
                output.nbars = 0 as core::ffi::c_int;
            }
            bar_no = 0 as core::ffi::c_int;
        }
    }
    fbar_total.n = 0 as core::ffi::c_int;
    fbar_total.d = 1 as core::ffi::c_int;
    if bar_no > 2 as core::ffi::c_int {
        output.nbars = 0 as core::ffi::c_int;
    }
    (*current).u.bar.type_0 = bar_type;
    bar_start = &mut (*current).u.bar;
    (*bar_start).bar_no = bar_no;
    stave = 0 as core::ffi::c_int;
    beam_length = 0 as core::ffi::c_int;
}
unsafe extern "C" fn process_tie() {
    check_syntax(TIE_TKN as core::ffi::c_int);
    output_transpose();
    save_two_bars();
    env[IN_TIE as core::ffi::c_int as usize] = 1 as core::ffi::c_int;
    strcat(
        ((*note).start).as_mut_ptr(),
        b"t\0" as *const u8 as *const core::ffi::c_char,
    );
    end_beam();
}
unsafe extern "C" fn process_open_slur() {
    if note.is_null() || (*note).pitch != 0
        || !(strchr(((*note).end).as_mut_ptr(), 'b' as i32)).is_null()
    {
        new_symbol(NOTE as core::ffi::c_int);
    }
    check_syntax(OPEN_SLUR_TKN as core::ffi::c_int);
    output_transpose();
    strcat(
        ((*note).start).as_mut_ptr(),
        b"s\0" as *const u8 as *const core::ffi::c_char,
    );
}
unsafe extern "C" fn process_close_slur() {
    check_syntax(CLOSE_SLUR_TKN as core::ffi::c_int);
    output_transpose();
    strcat(((*note).end).as_mut_ptr(), b"s\0" as *const u8 as *const core::ffi::c_char);
}
unsafe extern "C" fn process_open_close_slur() {
    if settings.old_slurs == 0 as core::ffi::c_int {
        abc_error(
            b"ss syntax obsolete; use () or change settings file\0" as *const u8
                as *const core::ffi::c_char as *mut core::ffi::c_char,
        );
    }
    if note.is_null() || (*note).pitch != 0
        || !(strchr(((*note).end).as_mut_ptr(), 'b' as i32)).is_null()
    {
        new_symbol(NOTE as core::ffi::c_int);
    }
    check_syntax(OPEN_SLUR_TKN as core::ffi::c_int);
    output_transpose();
    if in_old_slur != 0 {
        strcat(
            ((*note).end).as_mut_ptr(),
            b"s\0" as *const u8 as *const core::ffi::c_char,
        );
        in_old_slur = 0 as core::ffi::c_int;
    } else {
        strcat(
            ((*note).start).as_mut_ptr(),
            b"s\0" as *const u8 as *const core::ffi::c_char,
        );
        in_old_slur = 1 as core::ffi::c_int;
    };
}
unsafe extern "C" fn process_tuplet(mut s: *mut core::ffi::c_char) {
    let mut p: core::ffi::c_int = 0;
    let mut q: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut r: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut save_s: *mut core::ffi::c_char = s;
    if n_notes != 0 {
        end_beam();
    }
    check_syntax(TUPLET_TKN as core::ffi::c_int);
    output_transpose();
    save_two_bars();
    if note.is_null() || (*note).pitch != 0
        || !(strchr(((*note).end).as_mut_ptr(), 'b' as i32)).is_null()
    {
        new_symbol(NOTE as core::ffi::c_int);
    }
    p = atoi(s);
    while '0' as i32 <= *s as core::ffi::c_int && *s as core::ffi::c_int <= '9' as i32 {
        s = s.offset(1);
    }
    if *s as core::ffi::c_int == ':' as i32 {
        s = s.offset(1);
        if '0' as i32 <= *s as core::ffi::c_int && *s as core::ffi::c_int <= '9' as i32 {
            q = atoi(s);
            while '0' as i32 <= *s as core::ffi::c_int
                && *s as core::ffi::c_int <= '9' as i32
            {
                s = s.offset(1);
            }
        }
    }
    if *s as core::ffi::c_int == ':' as i32 {
        s = s.offset(1);
        if '0' as i32 <= *s as core::ffi::c_int && *s as core::ffi::c_int <= '9' as i32 {
            r = atoi(s);
            while '0' as i32 <= *s as core::ffi::c_int
                && *s as core::ffi::c_int <= '9' as i32
            {
                s = s.offset(1);
            }
        }
    }
    if s.offset_from(save_s) as core::ffi::c_long
        != (token_length - 1 as core::ffi::c_int) as core::ffi::c_long
    {
        abc_error(
            b"in tuplet\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char,
        );
    }
    if q == -(1 as core::ffi::c_int) {
        match p {
            2 | 4 | 8 => {
                q = 3 as core::ffi::c_int;
            }
            3 | 6 => {
                q = 2 as core::ffi::c_int;
            }
            5 | 7 | 9 => {
                if compound_time != 0 {
                    q = 3 as core::ffi::c_int;
                } else {
                    q = 2 as core::ffi::c_int;
                }
            }
            _ => {
                abc_error(
                    b"tuplet not recognised\0" as *const u8 as *const core::ffi::c_char
                        as *mut core::ffi::c_char,
                );
            }
        }
    }
    if r == -(1 as core::ffi::c_int) {
        r = p;
    }
    tuplet_fraction.n = q;
    tuplet_fraction.d = p;
    env[IN_TUPLET as core::ffi::c_int as usize] = p;
    tuplet_n_notes = r;
    tuplet_note_no = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn process_abc(
    mut title: *mut [core::ffi::c_char; 99],
    mut titles: core::ffi::c_int,
    mut entry: *mut Record,
    mut key_comment: *mut core::ffi::c_char,
    mut bars: *mut core::ffi::c_char,
    mut tune: *mut core::ffi::c_char,
    mut input: *mut core::ffi::c_char,
    mut output_type: core::ffi::c_int,
    mut nbars: core::ffi::c_int,
    mut force: core::ffi::c_int,
    mut hash_array: *mut core::ffi::c_int,
) {
    let mut c: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut line: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut str: [core::ffi::c_char; 99] = [0; 99];
    let mut key: [core::ffi::c_char; 99] = [0; 99];
    let mut key_field: Field = Field {
        string: 0 as *mut core::ffi::c_char,
        info1: 0,
        info2: 0,
    };
    let mut meter: [core::ffi::c_char; 99] = [0; 99];
    let mut meter_field: Field = Field {
        string: 0 as *mut core::ffi::c_char,
        info1: 0,
        info2: 0,
    };
    let mut tempo: [core::ffi::c_char; 99] = [0; 99];
    let mut tempo_field: Field = Field {
        string: 0 as *mut core::ffi::c_char,
        info1: 0,
        info2: 0,
    };
    let mut s: *mut Symbol = 0 as *mut Symbol;
    output.type_0 = output_type;
    if offset != 0 || transpose != 0 {
        output.transpose = 1 as core::ffi::c_int;
    } else {
        output.transpose = 0 as core::ffi::c_int;
    }
    output.nbars = nbars;
    if output_type == TEX_OUTPUT as core::ffi::c_int
        || output_type == INDEX_OUTPUT as core::ffi::c_int
    {
        output.warnings = 1 as core::ffi::c_int;
    } else {
        output.warnings = 0 as core::ffi::c_int;
    }
    ignore = 0 as core::ffi::c_int;
    n_symbols = -(1 as core::ffi::c_int);
    n_blocks = -(1 as core::ffi::c_int);
    current = 0 as *mut Symbol;
    note = 0 as *mut Note;
    beam_length = 0 as core::ffi::c_int;
    new_symbol(BAR_LINE as core::ffi::c_int);
    if input.is_null() {
        line = calloc(
            999 as core::ffi::c_int as core::ffi::c_uint as size_t,
            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
        ) as *mut core::ffi::c_char;
        if line.is_null() {
            g_error(
                b"alloc failure in char_array\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    bar_start = &mut (*current).u.bar;
    token = NEWLINE_TKN as core::ffi::c_int;
    treble = 0 as core::ffi::c_int;
    bass = treble;
    stave = bass;
    bar[2 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = '\0' as i32
        as core::ffi::c_char;
    bar[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = bar[2
        as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
    bar[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = bar[1
        as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
    current_bar[0 as core::ffi::c_int as usize] = bar[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize];
    key_field.string = &mut *key.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut core::ffi::c_char;
    key_field.info2 = 0 as core::ffi::c_int;
    key_field.info1 = key_field.info2;
    strcpy(key.as_mut_ptr(), b"K:\0" as *const u8 as *const core::ffi::c_char);
    strcat(key.as_mut_ptr(), (*entry).fields[10 as core::ffi::c_int as usize]);
    sharps_flats(&mut key_field);
    if offset != 0 || transpose != 0 {
        fprintf(Trans, b"%s\0" as *const u8 as *const core::ffi::c_char, key_comment);
    }
    meter_field.string = &mut *meter.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut core::ffi::c_char;
    meter_field.info2 = 0 as core::ffi::c_int;
    meter_field.info1 = meter_field.info2;
    strcpy(meter.as_mut_ptr(), b"M:\0" as *const u8 as *const core::ffi::c_char);
    strcat(meter.as_mut_ptr(), (*entry).fields[12 as core::ffi::c_int as usize]);
    set_base(&mut meter_field);
    if *((*entry).fields[11 as core::ffi::c_int as usize])
        .offset(0 as core::ffi::c_int as isize) != 0
    {
        set_dnl((*entry).fields[11 as core::ffi::c_int as usize]);
    }
    tempo_field.string = &mut *tempo.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut core::ffi::c_char;
    tempo_field.info2 = 0 as core::ffi::c_int;
    tempo_field.info1 = tempo_field.info2;
    if !((*entry).fields[16 as core::ffi::c_int as usize]).is_null()
        && *((*entry).fields[16 as core::ffi::c_int as usize])
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
    {
        strcpy(tempo.as_mut_ptr(), b"Q:\0" as *const u8 as *const core::ffi::c_char);
        strcat(tempo.as_mut_ptr(), (*entry).fields[16 as core::ffi::c_int as usize]);
        tempo_field.info1 = dnl;
    }
    while !input.is_null() || !(getsIn(line)).is_null() {
        fbar_total.n = 0 as core::ffi::c_int;
        fbar_total.d = 1 as core::ffi::c_int;
        bar_no = 0 as core::ffi::c_int;
        if !input.is_null() {
            line = input;
        }
        if *tune.offset(0 as core::ffi::c_int as isize) != 0 {
            strcat(tune, line);
        }
        i = 0 as core::ffi::c_int;
        while *line.offset(i as isize) as core::ffi::c_int == ' ' as i32
            || *line.offset(i as isize) as core::ffi::c_int == '\t' as i32
        {
            i += 1;
        }
        if *line.offset(i as isize) as core::ffi::c_int == '\n' as i32 {
            token_ptr = line;
            token_length = i + 1 as core::ffi::c_int;
            output_transpose();
            nblanks = 1 as core::ffi::c_int;
            ignore = 0 as core::ffi::c_int;
            break;
        } else {
            if output.type_0 == INDEX_OUTPUT as core::ffi::c_int
                && output.nbars == 0 as core::ffi::c_int
            {
                continue;
            }
            if *line.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == '\\' as i32 || is_field(line) != 0
            {
                token_ptr = line;
                token_length = strlen(line) as core::ffi::c_int;
                process_field(line);
                if output.nbars != 0 {
                    if *line.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        == 'L' as i32
                    {
                        strcpy(
                            (*entry).fields[11 as core::ffi::c_int as usize],
                            &mut *line.offset(2 as core::ffi::c_int as isize),
                        );
                    } else if *line.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int == 'M' as i32
                    {
                        strcpy(
                            (*entry).fields[12 as core::ffi::c_int as usize],
                            &mut *line.offset(2 as core::ffi::c_int as isize),
                        );
                    }
                }
            } else {
                continuation = 0 as core::ffi::c_int;
                c = 0 as core::ffi::c_int;
                while *line.offset(c as isize) != 0 {
                    if output.nbars == 0 as core::ffi::c_int && nbars != 0 {
                        break;
                    }
                    token_ptr = &mut *line.offset(c as isize) as *mut core::ffi::c_char;
                    token_length = 1 as core::ffi::c_int;
                    let mut current_block_188: u64;
                    match *line.offset(c as isize) as core::ffi::c_int {
                        67 | 68 | 69 | 70 | 71 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'C' as i32
                                    + 16 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        65 | 66 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'A' as i32
                                    + 21 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        99 | 100 | 101 | 102 | 103 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'c' as i32
                                    + 23 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        97 | 98 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'a' as i32
                                    + 28 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        122 => {
                            process_note(-(1 as core::ffi::c_int));
                            current_block_188 = 13193481930143188038;
                        }
                        32 | 9 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == ' ' as i32
                                || *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int == '\t' as i32
                            {
                                token_length += 1;
                            }
                            if *line.offset((c + token_length) as isize)
                                as core::ffi::c_int != '%' as i32
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int != '\n' as i32
                            {
                                process_space();
                                current_block_188 = 13193481930143188038;
                            } else {
                                current_block_188 = 17252160251682948341;
                            }
                        }
                        37 => {
                            current_block_188 = 17252160251682948341;
                        }
                        124 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '|' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(DBL_BAR as core::ffi::c_int);
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == ']' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(RDBL_BAR as core::ffi::c_int);
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == ':' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(LREPEAT as core::ffi::c_int);
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '1' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(BAR1 as core::ffi::c_int);
                            } else {
                                process_bar(BAR as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        58 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '|' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                if *line.offset((c + 2 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int == '2' as i32
                                {
                                    token_length += 1 as core::ffi::c_int;
                                    process_bar(RREPEAT2 as core::ffi::c_int);
                                } else {
                                    process_bar(RREPEAT as core::ffi::c_int);
                                }
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == ':' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(REPEAT as core::ffi::c_int);
                            } else {
                                abc_error(
                                    b"\0" as *const u8 as *const core::ffi::c_char
                                        as *mut core::ffi::c_char,
                                );
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        91 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '1' as i32
                                || *line.offset((c + 1 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int == '2' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_repeat(
                                    atoi(
                                        &mut *line.offset((c + 1 as core::ffi::c_int) as isize),
                                    ),
                                );
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '|' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(LDBL_BAR as core::ffi::c_int);
                            } else {
                                process_open_chord();
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        93 => {
                            process_close_chord();
                            current_block_188 = 13193481930143188038;
                        }
                        49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 => {
                            while !(strchr(
                                b"0123456789\0" as *const u8 as *const core::ffi::c_char,
                                *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_length(atoi(&mut *line.offset(c as isize)));
                            current_block_188 = 13193481930143188038;
                        }
                        47 => {
                            if !(strchr(
                                b"0123456789\0" as *const u8 as *const core::ffi::c_char,
                                *line.offset((c + 1 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                while !(strchr(
                                    b"0123456789\0" as *const u8 as *const core::ffi::c_char,
                                    *line.offset((c + token_length) as isize)
                                        as core::ffi::c_int,
                                ))
                                    .is_null()
                                {
                                    token_length += 1 as core::ffi::c_int;
                                }
                                process_divisor(
                                    atoi(
                                        &mut *line.offset((c + 1 as core::ffi::c_int) as isize),
                                    ),
                                );
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '/' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_divisor(4 as core::ffi::c_int);
                            } else {
                                process_divisor(2 as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        62 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == '>' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_broken(token_length);
                            current_block_188 = 13193481930143188038;
                        }
                        60 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == '<' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_broken(-(1 as core::ffi::c_int) * token_length);
                            current_block_188 = 13193481930143188038;
                        }
                        43 => {
                            process_open_close_chord();
                            current_block_188 = 13193481930143188038;
                        }
                        115 => {
                            process_open_close_slur();
                            current_block_188 = 13193481930143188038;
                        }
                        42 => {
                            process_justify();
                            current_block_188 = 13193481930143188038;
                        }
                        123 => {
                            process_open_grace();
                            current_block_188 = 13193481930143188038;
                        }
                        125 => {
                            process_close_grace();
                            current_block_188 = 13193481930143188038;
                        }
                        40 => {
                            if !(strchr(
                                b"123456789\0" as *const u8 as *const core::ffi::c_char,
                                *line.offset((c + 1 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                while !(strchr(
                                    b":0123456789\0" as *const u8 as *const core::ffi::c_char,
                                    *line.offset((c + token_length) as isize)
                                        as core::ffi::c_int,
                                ))
                                    .is_null()
                                {
                                    token_length += 1 as core::ffi::c_int;
                                }
                                process_tuplet(
                                    &mut *line.offset((c + 1 as core::ffi::c_int) as isize),
                                );
                            } else {
                                process_open_slur();
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        41 => {
                            process_close_slur();
                            current_block_188 = 13193481930143188038;
                        }
                        45 => {
                            process_tie();
                            current_block_188 = 13193481930143188038;
                        }
                        126 | 46 | 117 | 118 => {
                            process_accent(*line.offset(c as isize));
                            current_block_188 = 13193481930143188038;
                        }
                        94 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '^' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_accidental(DBL_SHARP as core::ffi::c_int);
                            } else {
                                process_accidental(SHARP as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        95 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '_' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_accidental(DBL_FLAT as core::ffi::c_int);
                            } else {
                                process_accidental(FLAT as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        61 => {
                            process_accidental(NATURAL as core::ffi::c_int);
                            current_block_188 = 13193481930143188038;
                        }
                        39 => {
                            process_octaver(7 as core::ffi::c_int);
                            current_block_188 = 13193481930143188038;
                        }
                        44 => {
                            process_octaver(-(7 as core::ffi::c_int));
                            current_block_188 = 13193481930143188038;
                        }
                        34 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int != 0
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int != '"' as i32
                            {
                                str[(token_length - 1 as core::ffi::c_int) as usize] = *line
                                    .offset((c + token_length) as isize);
                                token_length += 1;
                            }
                            if *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == '\0' as i32
                            {
                                abc_error(
                                    b"line ended mid gchord\0" as *const u8
                                        as *const core::ffi::c_char as *mut core::ffi::c_char,
                                );
                            }
                            token_length += 1 as core::ffi::c_int;
                            str[(token_length - 2 as core::ffi::c_int) as usize] = '\0'
                                as i32 as core::ffi::c_char;
                            process_gchord(str.as_mut_ptr());
                            current_block_188 = 13193481930143188038;
                        }
                        92 => {
                            process_continuation();
                            current_block_188 = 13193481930143188038;
                        }
                        10 => {
                            process_newline();
                            current_block_188 = 13193481930143188038;
                        }
                        38 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '&' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_ampersand(token_length);
                            current_block_188 = 13193481930143188038;
                        }
                        _ => {
                            if !(strchr(
                                b"HIJKLMNOPQRSTUVWXYZ\0" as *const u8
                                    as *const core::ffi::c_char,
                                *line.offset(c as isize) as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                process_macro(*line.offset(c as isize));
                            } else {
                                abc_error(
                                    b"unrecognised symbol\0" as *const u8
                                        as *const core::ffi::c_char as *mut core::ffi::c_char,
                                );
                            }
                            current_block_188 = 13193481930143188038;
                        }
                    }
                    match current_block_188 {
                        17252160251682948341 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int != 0
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int != '\n' as i32
                            {
                                token_length += 1;
                            }
                            if just_comment != 0
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int == '\n' as i32
                            {
                                token_length += 1;
                            }
                            process_trailing();
                        }
                        _ => {}
                    }
                    c += token_length;
                }
                if just_comment == 0 as core::ffi::c_int {
                    if output.nbars != 0
                        && current_bar[0 as core::ffi::c_int as usize]
                            as core::ffi::c_int != 0 && bar_no < 3 as core::ffi::c_int
                    {
                        strcpy(
                            (bar[bar_no as usize]).as_mut_ptr(),
                            current_bar.as_mut_ptr(),
                        );
                    }
                    output.nbars = 0 as core::ffi::c_int;
                }
                if !input.is_null() {
                    break;
                }
            }
        }
    }
    end_beam();
    (*current).newline = 2 as core::ffi::c_int;
    if settings.justification != 0 {
        (*current).justify = 1 as core::ffi::c_int;
    }
    n_symbols += 1 as core::ffi::c_int;
    process_beams(
        n_symbols,
        &mut *(*block.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
            .offset(0 as core::ffi::c_int as isize),
    );
    if output.type_0 == TEX_OUTPUT as core::ffi::c_int {
        tune2tex(
            title,
            titles,
            entry,
            n_symbols,
            &mut *(*block.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                .offset(0 as core::ffi::c_int as isize),
            &mut key_field,
            &mut meter_field,
            &mut tempo_field,
        );
    } else if !hash_array.is_null() {
        tune2hash(&mut key_field, hash_array, force);
    } else if nbars != 0 {
        *bars.offset(0 as core::ffi::c_int as isize) = '\0' as i32 as core::ffi::c_char;
        if nbars == ONE_BAR_PLUS as core::ffi::c_int
            || nbars == TWO_BARS_PLUS as core::ffi::c_int
        {
            strcpy(bars, (bar[0 as core::ffi::c_int as usize]).as_mut_ptr());
        }
        strcat(bars, (bar[1 as core::ffi::c_int as usize]).as_mut_ptr());
        if nbars == TWO_BARS as core::ffi::c_int
            || nbars == TWO_BARS_PLUS as core::ffi::c_int
        {
            strcat(bars, (bar[2 as core::ffi::c_int as usize]).as_mut_ptr());
        }
    }
    s = &mut *(*block.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
        .offset(0 as core::ffi::c_int as isize) as *mut Symbol;
    while !s.is_null() {
        if (*s).type_0 == NOTE as core::ffi::c_int && !((*s).u.note.gchord).is_null() {
            free((*s).u.note.gchord as *mut core::ffi::c_void);
        } else if (*s).type_0 == FIELD as core::ffi::c_int {
            free((*s).u.field.string as *mut core::ffi::c_void);
        }
        s = (*s).next as *mut Symbol;
    }
    i = 0 as core::ffi::c_int;
    while i <= n_blocks {
        free(block[i as usize] as *mut core::ffi::c_void);
        i += 1;
    }
    if input.is_null() {
        free(line as *mut core::ffi::c_void);
    }
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
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
    fn vfprintf(
        __s: *mut FILE,
        __format: *const core::ffi::c_char,
        __arg: ::core::ffi::VaList,
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
    fn fputs(__s: *const core::ffi::c_char, __stream: *mut FILE) -> core::ffi::c_int;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
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
    fn end_beam();
    fn new_symbol(type_0: core::ffi::c_int);
    fn output_transpose();
    fn set_base(meter: *mut Field);
    fn sharps_flats(key: *mut Field);
    fn process_field(str: *mut core::ffi::c_char);
    fn process_trailing();
    fn process_gchord(str: *mut core::ffi::c_char);
    fn process_macro(c: core::ffi::c_char);
    fn process_accent(c: core::ffi::c_char);
    fn process_accidental(accidental: core::ffi::c_int);
    fn process_note(pitch: core::ffi::c_int);
    fn process_octaver(octaver: core::ffi::c_int);
    fn process_length(length: core::ffi::c_int);
    fn process_divisor(length: core::ffi::c_int);
    fn process_broken(power: core::ffi::c_int);
    fn process_bar(bar_type: core::ffi::c_int);
    fn process_repeat(no: core::ffi::c_int);
    fn process_space();
    fn process_tie();
    fn process_continuation();
    fn process_newline();
    fn process_open_chord();
    fn process_close_chord();
    fn process_open_close_chord();
    fn process_open_grace();
    fn process_close_grace();
    fn process_open_slur();
    fn process_close_slur();
    fn process_open_close_slur();
    fn process_tuplet(s: *mut core::ffi::c_char);
    fn process_justify();
    fn process_ampersand(level: core::ffi::c_int);
    fn process_beams(n: core::ffi::c_int, s: *mut Symbol);
    fn tune2hash(
        key: *mut Field,
        hash_array: *mut core::ffi::c_int,
        force: core::ffi::c_int,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
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
pub const NATURAL: accidental_types = 3;
pub const FLAT: accidental_types = 2;
pub const DBL_FLAT: accidental_types = 1;
pub const SHARP: accidental_types = 4;
pub const DBL_SHARP: accidental_types = 5;
pub const LDBL_BAR: bar_types = 3;
pub const REPEAT: bar_types = 5;
pub const RREPEAT: bar_types = 7;
pub const RREPEAT2: bar_types = 8;
pub const BAR: bar_types = 0;
pub const BAR1: bar_types = 1;
pub const LREPEAT: bar_types = 6;
pub const RDBL_BAR: bar_types = 4;
pub const DBL_BAR: bar_types = 2;
pub const INDEX_OUTPUT: output_types = 2;
pub const NEWLINE_TKN: token_types = 16;
pub const BAR_LINE: symbol_types = 1;
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
pub type token_types = core::ffi::c_uint;
pub const MAX_TKNS: token_types = 25;
pub const TRAILING_TKN: token_types = 24;
pub const AMPERSAND_TKN: token_types = 23;
pub const CLOSE_SLUR_TKN: token_types = 22;
pub const OPEN_SLUR_TKN: token_types = 21;
pub const CLOSE_CHORD_TKN: token_types = 20;
pub const OPEN_CHORD_TKN: token_types = 19;
pub const CLOSE_GRACE_TKN: token_types = 18;
pub const OPEN_GRACE_TKN: token_types = 17;
pub const JUSTIFY_TKN: token_types = 15;
pub const CONTINUE_TKN: token_types = 14;
pub const MACRO_TKN: token_types = 13;
pub const TUPLET_TKN: token_types = 12;
pub const TIE_TKN: token_types = 11;
pub const SPACE_TKN: token_types = 10;
pub const REPEAT_TKN: token_types = 9;
pub const BAR_TKN: token_types = 8;
pub const BROKEN_TKN: token_types = 7;
pub const DIVISOR_TKN: token_types = 6;
pub const LENGTH_TKN: token_types = 5;
pub const OCTAVER_TKN: token_types = 4;
pub const NOTE_TKN: token_types = 3;
pub const ACCIDENTAL_TKN: token_types = 2;
pub const ACCENT_TKN: token_types = 1;
pub const GCHORD_TKN: token_types = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const CROTCHET: core::ffi::c_int = 8 as core::ffi::c_int;
pub const QUAVER: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SEMIQUAVER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const DEMISEMIQUAVER: core::ffi::c_int = 1 as core::ffi::c_int;
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
static mut abc_file: [core::ffi::c_char; 99] = [0; 99];
static mut input_line: core::ffi::c_int = 0;
static mut just_comment: core::ffi::c_int = 0;
static mut bar_start: *mut Barline = 0 as *const Barline as *mut Barline;
static mut note: *mut Note = 0 as *const Note as *mut Note;
static mut current: *mut Symbol = 0 as *const Symbol as *mut Symbol;
static mut token: core::ffi::c_int = 0;
static mut token_length: core::ffi::c_int = 0 as core::ffi::c_int;
static mut token_ptr: *mut core::ffi::c_char = 0 as *const core::ffi::c_char
    as *mut core::ffi::c_char;
static mut n_blocks: core::ffi::c_int = 0;
static mut block: [*mut Symbol; 10] = [0 as *const Symbol as *mut Symbol; 10];
static mut n_symbols: core::ffi::c_int = 0 as core::ffi::c_int;
static mut fbar_total: frac = frac { n: 0, d: 0 };
static mut bar_no: core::ffi::c_int = 0;
static mut bar: [[core::ffi::c_char; 99]; 3] = [[0; 99]; 3];
static mut current_bar: [core::ffi::c_char; 99] = [0; 99];
static mut stave: core::ffi::c_int = 0;
static mut output: Output = Output {
    type_0: 0,
    transpose: 0,
    nbars: 0,
    warnings: 0,
};
static mut beam_length: core::ffi::c_int = 0 as core::ffi::c_int;
static mut continuation: core::ffi::c_int = 0 as core::ffi::c_int;
static mut ignore: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn abc_warning(mut fmt: *mut core::ffi::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    if output.warnings == 0 as core::ffi::c_int {
        return;
    }
    ap = args.clone();
    fprintf(
        stdout,
        b"WARNING: line no. %d - \0" as *const u8 as *const core::ffi::c_char,
        input_line,
    );
    vfprintf(stdout, fmt, ap.as_va_list());
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn abc_error(mut fmt: *const i8, mut args: ...) {
    println!("{}", std::ffi::CStr::from_ptr(fmt).to_string_lossy()); // This assumes that the format string is ASCII. If not, additional steps are required to correctly handle encoding.
    exit(1);
}
use std::ffi::CStr;
#[no_mangle]
pub unsafe extern "C" fn is_field(mut line: *mut core::ffi::c_char) -> core::ffi::c_int {
    if *line.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int == ':' as i32
        && 'A' as i32 <= *line.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        && *line.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int <= 'Z' as i32
    {
        return 1 as core::ffi::c_int
    } else {
        return 0 as core::ffi::c_int
    };
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
#[no_mangle]
pub unsafe extern "C" fn stripcpy(mut out_str: *mut core::ffi::c_char, mut in_str: *mut core::ffi::c_char) {
    static mut DUMMY: [core::ffi::c_char; 999] = [0; 999];
    strip(in_str, DUMMY.as_mut_ptr());
    strcpy(out_str, in_str);
}
#[no_mangle]
pub unsafe extern "C" fn output_transline(s: *mut ::std::ffi::c_char) {
    fputs(s, Trans);
}
#[no_mangle]
pub unsafe extern "C" fn get_dnl(entry: *mut Record) {
    let mut meter = [0 as core::ffi::c_char; 99];
    let mut meter_field = Field{ string: &mut *meter.as_mut_ptr().offset(0 as core::ffi::c_int as isize), info1: 0, info2: 0 };
    
    strcpy(meter.as_mut_ptr(), b"M:\0".as_ptr() as *const u8 as *const core::ffi::c_char);
    strcat(meter.as_mut_ptr(), (*entry).fields[12]);
    set_base(&mut meter_field);
    
    sprintf((*entry).fields[11], b"1/%d\0".as_ptr() as *const u8 as *const core::ffi::c_char, 32 / dnl);
}
#[no_mangle]
pub unsafe extern "C" fn set_dnl(mut dnl_str: *mut core::ffi::c_char) {
    let mut const_DNL = 0; // Defining const_DNL as a mutable variable 

    while *dnl_str as core::ffi::c_int == ' ' as i32 {
        dnl_str = dnl_str.offset(1);
    }
    
    if strncmp(dnl_str, b"1/4\0".as_ptr() as *const _, 3) == 0 as core::ffi::c_int {
        const_DNL = CROTCHET;
    } else if strncmp(dnl_str, b"1/8\0".as_ptr() as *const _, 3) == 0 as core::ffi::c_int {
        const_DNL = QUAVER;
    } else if strncmp(dnl_str, b"1/16\0".as_ptr() as *const _, 4) == 0 as core::ffi::c_int {
        const_DNL = SEMIQUAVER;
    } else if strncmp(dnl_str, b"1/32\0".as_ptr() as *const _, 5) == 0 as core::ffi::c_int {
        const_DNL = DEMISEMIQUAVER;
    } else {
        printf(b"default note length not recognised\n\0".as_ptr() as *const _);
        const_DNL = QUAVER;
    }
}
#[no_mangle]
pub unsafe extern "C" fn range(
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut yfirst: *mut core::ffi::c_int,
    mut ylast: *mut core::ffi::c_int,
    mut input: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    *ylast = 4999 as core::ffi::c_int;
    if **input as core::ffi::c_int == '\0' as i32 {
        *first = 4999 as core::ffi::c_int;
        *last = 4999 as core::ffi::c_int;
        *yfirst = 4999 as core::ffi::c_int;
        return 1 as core::ffi::c_int;
    }
    if **input as core::ffi::c_int == '-' as i32 {
        *first = 1 as core::ffi::c_int;
        *yfirst = 1 as core::ffi::c_int;
    } else if **input as core::ffi::c_int >= '0' as i32
        && **input as core::ffi::c_int <= '9' as i32
    {
        sscanf(*input, b"%d\0" as *const u8 as *const core::ffi::c_char, first);
        while **input as core::ffi::c_int >= '0' as i32
            && **input as core::ffi::c_int <= '9' as i32
        {
            *input = (*input).offset(1);
        }
        if **input as core::ffi::c_int == '.' as i32 {
            *input = (*input).offset(1);
            if **input as core::ffi::c_int >= '1' as i32
                && **input as core::ffi::c_int <= '9' as i32
            {
                *yfirst = **input as core::ffi::c_int - '0' as i32;
            } else if **input as core::ffi::c_int >= 'a' as i32
                && **input as core::ffi::c_int <= 'z' as i32
            {
                *yfirst = **input as core::ffi::c_int - 'a' as i32
                    + 10 as core::ffi::c_int;
            } else {
                return 0 as core::ffi::c_int
            }
            *input = (*input).offset(1);
        } else {
            *yfirst = 0 as core::ffi::c_int;
        }
    } else {
        return 0 as core::ffi::c_int
    }
    if **input as core::ffi::c_int == '-' as i32 {
        *input = (*input).offset(1);
        if **input as core::ffi::c_int == '\0' as i32 {
            *last = 4999 as core::ffi::c_int;
        } else if **input as core::ffi::c_int >= '0' as i32
            && **input as core::ffi::c_int <= '9' as i32
        {
            sscanf(*input, b"%d\0" as *const u8 as *const core::ffi::c_char, last);
            while **input as core::ffi::c_int >= '0' as i32
                && **input as core::ffi::c_int <= '9' as i32
            {
                *input = (*input).offset(1);
            }
            if **input as core::ffi::c_int == '.' as i32 {
                *input = (*input).offset(1);
                if **input as core::ffi::c_int >= '1' as i32
                    && **input as core::ffi::c_int <= '9' as i32
                {
                    *ylast = **input as core::ffi::c_int - '0' as i32;
                } else if **input as core::ffi::c_int >= 'a' as i32
                    && **input as core::ffi::c_int <= 'z' as i32
                {
                    *ylast = **input as core::ffi::c_int - 'a' as i32
                        + 10 as core::ffi::c_int;
                } else {
                    return 0 as core::ffi::c_int
                }
                *input = (*input).offset(1);
            }
            if **input as core::ffi::c_int == ',' as i32 {
                *input = (*input).offset(1);
            }
        } else {
            return 0 as core::ffi::c_int
        }
    } else if **input as core::ffi::c_int == ',' as i32 {
        *last = *first;
        if *yfirst != 0 as core::ffi::c_int {
            *ylast = *yfirst;
        }
        *input = (*input).offset(1);
    } else if **input as core::ffi::c_int == '\0' as i32 {
        *last = *first;
        if *yfirst != 0 as core::ffi::c_int {
            *ylast = *yfirst;
        }
    } else {
        return 0 as core::ffi::c_int
    }
    return 1 as core::ffi::c_int;
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
                                current_block_188 = 11473691171471082492;
                            }
                        }
                        37 => {
                            current_block_188 = 11473691171471082492;
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
                        11473691171471082492 => {
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

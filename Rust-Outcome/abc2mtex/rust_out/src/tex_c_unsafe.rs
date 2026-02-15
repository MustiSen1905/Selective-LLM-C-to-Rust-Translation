extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fflush(__stream: *mut FILE) -> core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn atof(__nptr: *const core::ffi::c_char) -> core::ffi::c_double;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
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
    fn strpbrk(
        __s: *const core::ffi::c_char,
        __accept: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn g_error(_: *const core::ffi::c_char, ...);
    static mut settings: Setting;
    static mut bass: core::ffi::c_int;
    static mut treble: core::ffi::c_int;
    fn abclog2(x: core::ffi::c_int) -> core::ffi::c_int;
    fn close_music();
    fn open_tune();
    fn draw_header();
    fn close_open();
    fn next_stave();
    fn draw_usercmd(s: *mut core::ffi::c_char);
    fn draw_meter_new(meter: *mut Field);
    fn staves();
    fn beam2tex(n: core::ffi::c_int, first: *mut Symbol, beam: core::ffi::c_int);
    fn bar2tex(s: *mut Symbol);
    fn fields2tex(f: *mut Field);
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
pub type symbol_types = core::ffi::c_uint;
pub const MISC: symbol_types = 4;
pub const FIELD: symbol_types = 3;
pub const NOTE: symbol_types = 2;
pub const BAR_LINE: symbol_types = 1;
pub const UNDETERMINED: symbol_types = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const GRACE: core::ffi::c_int = 1 as core::ffi::c_int;
#[no_mangle]
pub static mut Out: *mut FILE = 0 as *const FILE as *mut FILE;
static mut in_tune: core::ffi::c_int = 0 as core::ffi::c_int;
static mut in_bar: core::ffi::c_int = 0 as core::ffi::c_int;
static mut change_signature: core::ffi::c_int = 0;
static mut in_notes: core::ffi::c_int = 0;
static mut hp: core::ffi::c_int = 0;
static mut new_tune: core::ffi::c_int = 0;
static mut old_beam: *mut core::ffi::c_int = 0 as *const core::ffi::c_int
    as *mut core::ffi::c_int;
static mut tempo_length: core::ffi::c_int = 0 as core::ffi::c_int;
static mut bpm: core::ffi::c_int = 0 as core::ffi::c_int;
static mut musix: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn draw_text(
    mut type_0: *mut core::ffi::c_char,
    mut string: *mut core::ffi::c_char,
) {
    let mut ptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    if type_0.is_null() {
        fprintf(Out, b"%s%%\n\0" as *const u8 as *const core::ffi::c_char, string);
    } else if strcmp(type_0, b"Z\0" as *const u8 as *const core::ffi::c_char)
        == 0 as core::ffi::c_int
    {
        fprintf(
            Out,
            b"\\message{%s}%%\n\0" as *const u8 as *const core::ffi::c_char,
            string,
        );
    } else {
        ptr = string;
        loop {
            ptr = strpbrk(ptr, b"#%&\0" as *const u8 as *const core::ffi::c_char);
            if ptr.is_null() {
                break;
            }
            if *ptr.offset(-(1 as core::ffi::c_int as isize)) as core::ffi::c_int
                != '\\' as i32
            {
                g_error(
                    b"unescaped special TeX character %c detected\n\tthis will cause TeX to choke\0"
                        as *const u8 as *const core::ffi::c_char,
                    *ptr as core::ffi::c_int,
                );
            }
            ptr = ptr.offset(1 as core::ffi::c_int as isize);
        }
        if *string.offset(0 as core::ffi::c_int as isize) != 0 {
            fprintf(
                Out,
                b"\\def\\%strue{Y}\\def\\%sstring{%s}\n\0" as *const u8
                    as *const core::ffi::c_char,
                type_0,
                type_0,
                string,
            );
        } else {
            fprintf(
                Out,
                b"\\def\\%strue{N}\n\0" as *const u8 as *const core::ffi::c_char,
                type_0,
            );
        }
    };
}
unsafe extern "C" fn draw_tempo(mut tempo: *mut Field) {
    let mut str: *mut core::ffi::c_char = &mut *((*tempo).string)
        .offset(2 as core::ffi::c_int as isize) as *mut core::ffi::c_char;
    let mut level: core::ffi::c_int = 0;
    let mut old_tempo_length: core::ffi::c_int = tempo_length;
    let mut length: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut ctr: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut j: core::ffi::c_int = 0;
    if new_tune != 0 {
        open_tune();
    }
    if *str.offset(ctr as isize) as core::ffi::c_int == 'C' as i32 {
        ctr += 1;
        if *str.offset(ctr as isize) as core::ffi::c_int >= '0' as i32
            && *str.offset(ctr as isize) as core::ffi::c_int <= '9' as i32
        {
            let fresh0 = ctr;
            ctr = ctr + 1;
            length = atoi(&mut *str.offset(fresh0 as isize));
            if length > 9 as core::ffi::c_int {
                ctr += 1 as core::ffi::c_int;
            }
        }
        tempo_length = length * (*tempo).info1;
        if *str.offset(ctr as isize) as core::ffi::c_int != '=' as i32 {
            g_error(b"in Q field\0" as *const u8 as *const core::ffi::c_char);
        }
        ctr += 1 as core::ffi::c_int;
    } else if strncmp(
        &mut *str.offset(ctr as isize),
        b"1/\0" as *const u8 as *const core::ffi::c_char,
        2 as size_t,
    ) == 0 as core::ffi::c_int
    {
        ctr += 2 as core::ffi::c_int;
        let fresh1 = ctr;
        ctr = ctr + 1;
        tempo_length = 32 as core::ffi::c_int / atoi(&mut *str.offset(fresh1 as isize));
        if tempo_length <= 2 as core::ffi::c_int {
            ctr += 1 as core::ffi::c_int;
        }
        if *str.offset(ctr as isize) as core::ffi::c_int != '=' as i32 {
            g_error(b"in Q field\0" as *const u8 as *const core::ffi::c_char);
        }
        ctr += 1 as core::ffi::c_int;
    } else {
        tempo_length = (*tempo).info1;
    }
    if *str.offset(ctr as isize) as core::ffi::c_int == 'C' as i32 {
        ctr += 1;
        if *str.offset(ctr as isize) as core::ffi::c_int >= '0' as i32
            && *str.offset(ctr as isize) as core::ffi::c_int <= '9' as i32
        {
            length = atoi(&mut *str.offset(ctr as isize));
        }
        bpm = bpm * old_tempo_length / (length * (*tempo).info1);
    } else if *str.offset(ctr as isize) as core::ffi::c_int >= '0' as i32
        && *str.offset(ctr as isize) as core::ffi::c_int <= '9' as i32
    {
        bpm = atoi(&mut *str.offset(ctr as isize));
    } else {
        g_error(b"in Q field\0" as *const u8 as *const core::ffi::c_char);
    }
    old_tempo_length = tempo_length;
    if in_notes != 0 {
        close_music();
    }
    fprintf(
        Out,
        b"\\notes\\Uptext{\\metron{\\\0" as *const u8 as *const core::ffi::c_char,
    );
    if tempo_length % 3 as core::ffi::c_int == 0 as core::ffi::c_int {
        fprintf(Out, b"pt1\\\0" as *const u8 as *const core::ffi::c_char);
    }
    level = abclog2(tempo_length);
    if level == 5 as core::ffi::c_int {
        fprintf(Out, b"wh\0" as *const u8 as *const core::ffi::c_char);
    } else if level == 4 as core::ffi::c_int {
        fprintf(Out, b"hu\0" as *const u8 as *const core::ffi::c_char);
    } else if level == 3 as core::ffi::c_int {
        fprintf(Out, b"qu\0" as *const u8 as *const core::ffi::c_char);
    } else {
        j = 0 as core::ffi::c_int;
        while j < 3 as core::ffi::c_int - level {
            fprintf(Out, b"c\0" as *const u8 as *const core::ffi::c_char);
            j += 1;
        }
        fprintf(Out, b"u\0" as *const u8 as *const core::ffi::c_char);
    }
    fprintf(Out, b"}{%d}}\\enotes%%\n\0" as *const u8 as *const core::ffi::c_char, bpm);
}
unsafe extern "C" fn draw_size(mut size: *mut core::ffi::c_char) {
    let mut esize: core::ffi::c_double = 0.;
    if settings.mine != 0 && musix != 0 {
        esize = 7.0f64;
    } else {
        esize = 8.5f64;
    }
    if *size.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
        && '0' as i32 <= *size.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        && *size.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int <= '9' as i32
    {
        esize = atof(size);
    }
    if musix == 0 as core::ffi::c_int && new_tune != 0 {
        fprintf(Out, b"\\normal\0" as *const u8 as *const core::ffi::c_char);
    }
    if musix == 0 as core::ffi::c_int
        || *size.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
            && '0' as i32
                <= *size.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            && *size.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                <= '9' as i32 || settings.mine != 0
    {
        fprintf(
            Out,
            b"\\elemskip=%.1fpt%%\n\0" as *const u8 as *const core::ffi::c_char,
            esize,
        );
    }
}
unsafe extern "C" fn key2tex(mut f: *mut Field) {
    if (*f).info2 == 2 as core::ffi::c_int {
        fprintf(
            Out,
            b"\\generalsignature{0}%%\n\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {
        fprintf(
            Out,
            b"\\generalsignature{%d}%%\n\0" as *const u8 as *const core::ffi::c_char,
            (*f).info1,
        );
    }
    if !old_beam.is_null() {
        free(old_beam as *mut core::ffi::c_void);
    }
    old_beam = calloc(
        (bass as core::ffi::c_uint).wrapping_add(treble as core::ffi::c_uint) as size_t,
        ::core::mem::size_of::<core::ffi::c_int>() as size_t,
    ) as *mut core::ffi::c_int;
    if old_beam.is_null() {
        g_error(
            b"alloc failure in int_array\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if new_tune != 0 {
        if (*f).info2 == 1 as core::ffi::c_int {
            fprintf(Out, b"\\beginHp\n\0" as *const u8 as *const core::ffi::c_char);
        }
        hp = (*f).info2;
    } else if in_bar != 0 {
        fprintf(
            Out,
            b"\\changesignature%%\n\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {
        change_signature = 1 as core::ffi::c_int;
    };
}
unsafe extern "C" fn end_tune() {
    free(old_beam as *mut core::ffi::c_void);
    old_beam = 0 as *mut core::ffi::c_int;
    if in_tune != 0 {
        if musix != 0 {
            fprintf(Out, b"\\zstoppiece%%\n\0" as *const u8 as *const core::ffi::c_char);
        } else {
            fprintf(
                Out,
                b"\\zsuspmorceau%%\n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        in_tune = 0 as core::ffi::c_int;
    }
    fprintf(Out, b"}%%\n\0" as *const u8 as *const core::ffi::c_char);
    if hp == 1 as core::ffi::c_int {
        fprintf(Out, b"\\endHp%%\n\0" as *const u8 as *const core::ffi::c_char);
    }
    fprintf(Out, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fflush(Out);
}
#[no_mangle]
pub unsafe extern "C" fn tune2tex(
    mut title: *mut [core::ffi::c_char; 99],
    mut titles: core::ffi::c_int,
    mut entry: *mut Record,
    mut n_symbols: core::ffi::c_int,
    mut symbols: *mut Symbol,
    mut key_field: *mut Field,
    mut meter_field: *mut Field,
    mut tempo_field: *mut Field,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    let mut other_titles: [core::ffi::c_char; 999] = [0; 999];
    let mut ttl: core::ffi::c_int = 0;
    new_tune = 1 as core::ffi::c_int;
    hp = 0 as core::ffi::c_int;
    draw_text(
        b"X\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[23 as core::ffi::c_int as usize],
    );
    draw_text(
        b"T\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*title.offset(0 as core::ffi::c_int as isize)).as_mut_ptr(),
    );
    draw_text(
        b"S\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[18 as core::ffi::c_int as usize],
    );
    draw_text(
        b"C\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[2 as core::ffi::c_int as usize],
    );
    draw_text(
        b"A\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[0 as core::ffi::c_int as usize],
    );
    draw_text(
        b"N\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[13 as core::ffi::c_int as usize],
    );
    other_titles[0 as core::ffi::c_int as usize] = '\0' as i32 as core::ffi::c_char;
    if titles > 1 as core::ffi::c_int {
        ttl = 1 as core::ffi::c_int;
        while ttl < titles {
            strcat(
                other_titles.as_mut_ptr(),
                (*title.offset(ttl as isize)).as_mut_ptr(),
            );
            if ttl < titles - 1 as core::ffi::c_int {
                strcat(
                    other_titles.as_mut_ptr(),
                    b"; \0" as *const u8 as *const core::ffi::c_char,
                );
            }
            ttl += 1;
        }
    }
    if titles < 6 as core::ffi::c_int {
        draw_text(
            b"Ta\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            other_titles.as_mut_ptr(),
        );
        draw_text(
            b"Tb\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        );
    } else {
        draw_text(
            b"Tb\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        );
        draw_text(
            b"Ta\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            other_titles.as_mut_ptr(),
        );
    }
    draw_text(
        b"P\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[15 as core::ffi::c_int as usize],
    );
    draw_header();
    key2tex(key_field);
    staves();
    draw_meter_new(meter_field);
    draw_size((*entry).fields[4 as core::ffi::c_int as usize]);
    if *((*entry).fields[16 as core::ffi::c_int as usize])
        .offset(0 as core::ffi::c_int as isize) != 0
    {
        draw_tempo(tempo_field);
    }
    in_bar = 1 as core::ffi::c_int;
    symbols = (*symbols).next as *mut Symbol;
    i = 1 as core::ffi::c_int;
    while i < n_symbols {
        match (*symbols).type_0 {
            2 => {
                n = (*symbols).u.note.n_notes;
                if (*symbols).u.note.pitch == 0 as core::ffi::c_int {
                    draw_usercmd(((*symbols).u.note.attributes).as_mut_ptr());
                } else if (*symbols).u.note.type_0 == GRACE {
                    if in_notes != 0 {
                        close_music();
                    }
                    beam2tex(n, symbols, -(1 as core::ffi::c_int));
                } else {
                    beam2tex(n, symbols, 0 as core::ffi::c_int);
                }
                j = 0 as core::ffi::c_int;
                while j < n {
                    symbols = (*symbols).next as *mut Symbol;
                    j += 1;
                }
                i += j;
            }
            1 => {
                bar2tex(symbols);
                i += 1 as core::ffi::c_int;
                symbols = (*symbols).next as *mut Symbol;
            }
            3 => {
                fields2tex(&mut (*symbols).u.field);
                i += 1 as core::ffi::c_int;
                symbols = (*symbols).next as *mut Symbol;
            }
            4 => {
                if (*symbols).u.misc.level == 2 as core::ffi::c_int {
                    close_open();
                } else if (*symbols).u.misc.level == 1 as core::ffi::c_int {
                    next_stave();
                } else {
                    g_error(b"\0" as *const u8 as *const core::ffi::c_char);
                }
                symbols = (*symbols).next as *mut Symbol;
                i += 1 as core::ffi::c_int;
            }
            _ => {
                g_error(b"\0" as *const u8 as *const core::ffi::c_char);
            }
        }
    }
    end_tune();
}

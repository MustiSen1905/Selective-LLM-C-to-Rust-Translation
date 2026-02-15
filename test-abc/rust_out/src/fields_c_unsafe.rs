extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> core::ffi::c_int;
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
    fn gets(__s: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
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
    fn strrchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn g_error(_: *const core::ffi::c_char, ...);
    fn get_index(_: *mut core::ffi::c_char, _: *mut core::ffi::c_char);
    fn put_record(
        _: *mut core::ffi::c_char,
        _: *mut FILE,
        _: *mut record,
    ) -> core::ffi::c_int;
    fn openIn(_: *mut core::ffi::c_char) -> *mut FILE;
    fn getsIn(_: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    fn closeIn();
    fn read_settings();
    fn is_field(_: *mut core::ffi::c_char) -> core::ffi::c_int;
    fn stripcpy(_: *mut core::ffi::c_char, _: *mut core::ffi::c_char);
    fn range(
        _: *mut core::ffi::c_int,
        _: *mut core::ffi::c_int,
        _: *mut core::ffi::c_int,
        _: *mut core::ffi::c_int,
        _: *mut *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn process_abc(
        _: *mut [core::ffi::c_char; 99],
        _: core::ffi::c_int,
        _: *mut Record,
        _: *mut core::ffi::c_char,
        _: *mut core::ffi::c_char,
        _: *mut core::ffi::c_char,
        _: *mut core::ffi::c_char,
        _: core::ffi::c_int,
        _: core::ffi::c_int,
        _: core::ffi::c_int,
        _: *mut core::ffi::c_int,
    );
    fn abc_error(_: *mut core::ffi::c_char, ...);
    fn abc_warning(_: *mut core::ffi::c_char, ...);
    fn end_of(_: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    fn strip(_: *mut core::ffi::c_char, _: *mut core::ffi::c_char);
    fn get_dnl(_: *mut Record);
    fn output_transline(_: *mut core::ffi::c_char);
    fn open_TeX(_: *mut core::ffi::c_char, _: core::ffi::c_int);
    fn close_TeX();
    fn draw_text(_: *mut core::ffi::c_char, _: *mut core::ffi::c_char);
    static mut Trans: *mut FILE;
    static mut transpose: core::ffi::c_int;
    static mut offset: core::ffi::c_int;
    static mut settings: Setting;
    static mut nblanks: core::ffi::c_int;
    fn is_comment(str: *mut core::ffi::c_char) -> core::ffi::c_int;
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
pub struct Setting {
    pub gchords_above: core::ffi::c_int,
    pub autobeam: core::ffi::c_int,
    pub old_slurs: core::ffi::c_int,
    pub old_chords: core::ffi::c_int,
    pub old_repeats: core::ffi::c_int,
    pub justification: core::ffi::c_int,
    pub mine: core::ffi::c_int,
}
pub type output_types = core::ffi::c_uint;
pub const HASH_OUTPUT: output_types = 3;
pub const INDEX_OUTPUT: output_types = 2;
pub const TEX_OUTPUT: output_types = 1;
pub const NO_OUTPUT: output_types = 0;
pub type two_bar_types = core::ffi::c_uint;
pub const TWO_BARS_PLUS: two_bar_types = 4;
pub const ONE_BAR_PLUS: two_bar_types = 3;
pub const TWO_BARS: two_bar_types = 2;
pub const ONE_BAR: two_bar_types = 1;
pub const NO_BARS: two_bar_types = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub static mut _stklen: core::ffi::c_uint = 8192 as core::ffi::c_uint;
#[no_mangle]
pub static mut Log: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut Index: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn article(
    mut s: *mut core::ffi::c_char,
    mut t: *mut core::ffi::c_char,
) {
    let mut j: core::ffi::c_int = 0;
    let mut i: *mut core::ffi::c_char = s;
    let mut a: [*const core::ffi::c_char; 14] = [
        b"The\0" as *const u8 as *const core::ffi::c_char,
        b"An\0" as *const u8 as *const core::ffi::c_char,
        b"A\0" as *const u8 as *const core::ffi::c_char,
        b"Na\0" as *const u8 as *const core::ffi::c_char,
        b"Da\0" as *const u8 as *const core::ffi::c_char,
        b"Le\0" as *const u8 as *const core::ffi::c_char,
        b"La\0" as *const u8 as *const core::ffi::c_char,
        b"Lo\0" as *const u8 as *const core::ffi::c_char,
        b"Lou\0" as *const u8 as *const core::ffi::c_char,
        b"Un\0" as *const u8 as *const core::ffi::c_char,
        b"Une\0" as *const u8 as *const core::ffi::c_char,
        b"Les\0" as *const u8 as *const core::ffi::c_char,
        b"L'\0" as *const u8 as *const core::ffi::c_char,
        b"\0" as *const u8 as *const core::ffi::c_char,
    ];
    while *s as core::ffi::c_int != 0
        && {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 as core::ffi::c_int != ',' as i32
        }
    {}
    if *s.offset(-(1 as core::ffi::c_int as isize)) as core::ffi::c_int == ',' as i32 {
        s = s.offset(1 as core::ffi::c_int as isize);
        j = 0 as core::ffi::c_int;
        while *(a[j as usize]).offset(0 as core::ffi::c_int as isize) != 0 {
            if strcmp(s, a[j as usize]) == 0 as core::ffi::c_int {
                while *s != 0 {
                    let fresh1 = s;
                    s = s.offset(1);
                    let fresh2 = t;
                    t = t.offset(1);
                    *fresh2 = *fresh1;
                }
                if *t.offset(-(1 as core::ffi::c_int as isize)) as core::ffi::c_int
                    != '\'' as i32
                {
                    let fresh3 = t;
                    t = t.offset(1);
                    *fresh3 = ' ' as i32 as core::ffi::c_char;
                }
                break;
            } else {
                j += 1;
            }
        }
    }
    while *i != 0 {
        if *i as core::ffi::c_int == ',' as i32
            && *(a[j as usize]).offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int != 0
        {
            break;
        }
        let fresh4 = i;
        i = i.offset(1);
        let fresh5 = t;
        t = t.offset(1);
        *fresh5 = *fresh4;
    }
    *t = '\0' as i32 as core::ffi::c_char;
}
unsafe extern "C" fn detex(
    mut s: *mut core::ffi::c_char,
    mut t: *mut core::ffi::c_char,
) {
    while *s != 0 {
        if *s as core::ffi::c_int == '\\' as i32 {
            s = s.offset(1);
            if *s as core::ffi::c_int != '&' as i32 {
                s = s.offset(1);
                if *s as core::ffi::c_int == ' ' as i32 {
                    s = s.offset(1 as core::ffi::c_int as isize);
                }
            }
        }
        let fresh6 = s;
        s = s.offset(1);
        let fresh7 = t;
        t = t.offset(1);
        *fresh7 = *fresh6;
    }
    *t = '\0' as i32 as core::ffi::c_char;
}
unsafe extern "C" fn interval(mut s: *mut core::ffi::c_char) {
    offset = atoi(&mut *s.offset(1 as core::ffi::c_int as isize))
        - 1 as core::ffi::c_int;
    transpose = -(1 as core::ffi::c_int);
    while transpose < 3 as core::ffi::c_int {
        if offset % 7 as core::ffi::c_int
            == (7 as core::ffi::c_int + transpose * 4 as core::ffi::c_int)
                % 7 as core::ffi::c_int
        {
            break;
        }
        transpose += 1;
    }
    if transpose == 3 as core::ffi::c_int || offset < 1 as core::ffi::c_int
        || offset > 7 as core::ffi::c_int
    {
        g_error(
            b"transpose interval not recognised\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if *s.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int == '_' as i32 {
        transpose *= -(1 as core::ffi::c_int);
        offset *= -(1 as core::ffi::c_int);
    }
}
unsafe extern "C" fn strip_path(
    mut filename: *mut core::ffi::c_char,
    mut file: *mut core::ffi::c_char,
) {
    let mut f_ptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    static mut temp: [core::ffi::c_char; 99] = [0; 99];
    stripcpy(temp.as_mut_ptr(), file);
    if strcmp(
        &mut *temp
            .as_mut_ptr()
            .offset(
                ((strlen
                    as unsafe extern "C" fn(
                        *const core::ffi::c_char,
                    ) -> size_t)(temp.as_mut_ptr()))
                    .wrapping_sub(4 as size_t) as isize,
            ),
        b".abc\0" as *const u8 as *const core::ffi::c_char,
    ) == 0 as core::ffi::c_int
    {
        temp[(strlen(temp.as_mut_ptr())).wrapping_sub(4 as size_t) as usize] = '\0'
            as i32 as core::ffi::c_char;
    }
    f_ptr = strrchr(temp.as_mut_ptr(), '/' as i32);
    if !f_ptr.is_null() {
        f_ptr = f_ptr.offset(1);
    } else {
        f_ptr = temp.as_mut_ptr();
    }
    strcpy(filename, f_ptr);
}
unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut c: core::ffi::c_int = 0;
    let mut first: core::ffi::c_int = 0;
    let mut last: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut logn: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut logn_total: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut log: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut log_total: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut trnsps: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut m: core::ffi::c_int = 0;
    let mut titles: core::ffi::c_int = 0;
    let mut ttl: core::ffi::c_int = 0;
    let mut in_tune: core::ffi::c_int = 0;
    let mut prev_x: core::ffi::c_int = 0;
    let mut njoint: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut arg: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut history: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut f: core::ffi::c_char = 'm' as i32 as core::ffi::c_char;
    let mut input: [core::ffi::c_char; 99] = [0; 99];
    let mut trans: [core::ffi::c_char; 99] = [0; 99];
    let mut filename: [core::ffi::c_char; 99] = [0; 99];
    let mut basename: [core::ffi::c_char; 99] = [0; 99];
    static mut title: [[core::ffi::c_char; 99]; 30] = [[0; 99]; 30];
    let mut dflt_meter: [core::ffi::c_char; 9] = [0; 9];
    let mut full_title: [core::ffi::c_char; 99] = [0; 99];
    let mut xline: [core::ffi::c_char; 999] = [0; 999];
    let mut bar_fmt: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut pinput: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut head: [core::ffi::c_char; 999] = [0; 999];
    let mut number: [core::ffi::c_char; 99] = [0; 99];
    let mut format: [core::ffi::c_char; 999] = [0; 999];
    let mut dflt_origin: [core::ffi::c_char; 99] = [0; 99];
    let mut dflt_rhythm: [core::ffi::c_char; 99] = [0; 99];
    let mut nbars: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut command_line: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut output: core::ffi::c_int = TEX_OUTPUT as core::ffi::c_int;
    let mut real_entry: Record = record {
        fields: [0 as *mut core::ffi::c_char; 26],
        bars: 0 as *mut core::ffi::c_char,
        next: 0 as *mut record,
    };
    let mut entry: *mut Record = 0 as *mut Record;
    let mut yfirst: core::ffi::c_int = 0;
    let mut ylast: core::ffi::c_int = 4999 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0;
    let mut musix_out: core::ffi::c_int = 0;
    let mut key_comment: [core::ffi::c_char; 999] = [0; 999];
    let mut dummy: [core::ffi::c_char; 999] = [0; 999];
    read_settings();
    entry = &mut real_entry;
    musix_out = 0 as core::ffi::c_int;
    filename[0 as core::ffi::c_int as usize] = '\0' as i32 as core::ffi::c_char;
    offset = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 26 as core::ffi::c_int {
        (*entry).fields[i as usize] = calloc(
            256 as core::ffi::c_int as core::ffi::c_uint as size_t,
            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
        ) as *mut core::ffi::c_char;
        if ((*entry).fields[i as usize]).is_null() {
            g_error(
                b"alloc failure in char_array\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        i += 1;
    }
    (*entry).bars = calloc(
        256 as core::ffi::c_int as core::ffi::c_uint as size_t,
        ::core::mem::size_of::<core::ffi::c_char>() as size_t,
    ) as *mut core::ffi::c_char;
    if ((*entry).bars).is_null() {
        g_error(
            b"alloc failure in char_array\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    Log = fopen(
        b"abc2mtex.log\0" as *const u8 as *const core::ffi::c_char,
        b"w\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    arg = 1 as core::ffi::c_int;
    while arg < argc
        && *(*argv.offset(arg as isize)).offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int == '-' as i32
        && *(*argv.offset(arg as isize)).offset(1 as core::ffi::c_int as isize)
            as core::ffi::c_int != 0
    {
        match *(*argv.offset(arg as isize)).offset(1 as core::ffi::c_int as isize)
            as core::ffi::c_int
        {
            120 => {
                musix_out = 1 as core::ffi::c_int;
            }
            121 => {
                musix_out = 2 as core::ffi::c_int;
            }
            116 => {
                if f as core::ffi::c_int == 'i' as i32 {
                    g_error(
                        b"option -t incompatible with -i\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if *(*argv.offset(arg as isize)).offset(2 as core::ffi::c_int as isize)
                    as core::ffi::c_int == ':' as i32
                {
                    if *(*argv.offset(arg as isize))
                        .offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                        == '^' as i32
                        || *(*argv.offset(arg as isize))
                            .offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            == '_' as i32
                    {
                        interval(
                            &mut *(*argv.offset(arg as isize))
                                .offset(3 as core::ffi::c_int as isize),
                        );
                    } else {
                        sscanf(
                            *argv.offset(arg as isize),
                            b"-t:%d:%d\0" as *const u8 as *const core::ffi::c_char,
                            &mut transpose as *mut core::ffi::c_int,
                            &mut offset as *mut core::ffi::c_int,
                        );
                    }
                    trnsps = 2 as core::ffi::c_int;
                } else {
                    trnsps = 1 as core::ffi::c_int;
                }
                Trans = fopen(
                    b"transpose.abc\0" as *const u8 as *const core::ffi::c_char,
                    b"w\0" as *const u8 as *const core::ffi::c_char,
                ) as *mut FILE;
            }
            105 => {
                if f as core::ffi::c_int == 't' as i32 {
                    g_error(
                        b"option -i incompatible with -t\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                f = 'i' as i32 as core::ffi::c_char;
            }
            111 => {
                arg += 1;
                if arg == argc
                    || *(*argv.offset(arg as isize))
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        == '-' as i32
                {
                    g_error(
                        b"no output file\0" as *const u8 as *const core::ffi::c_char,
                    );
                }
                strcpy(filename.as_mut_ptr(), *argv.offset(arg as isize));
            }
            _ => {
                g_error(
                    b"unrecognised option %s\0" as *const u8 as *const core::ffi::c_char,
                    *argv.offset(arg as isize),
                );
            }
        }
        arg += 1;
    }
    if argc > arg {
        command_line = 1 as core::ffi::c_int;
    }
    if f as core::ffi::c_int == 'm' as i32 {
        open_TeX(filename.as_mut_ptr(), musix_out);
    } else if f as core::ffi::c_int == 'i' as i32 {
        output = INDEX_OUTPUT as core::ffi::c_int;
        get_index(
            format.as_mut_ptr(),
            b"index.fmt\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char,
        );
        bar_fmt = format.as_mut_ptr();
        while *bar_fmt != 0 {
            if *bar_fmt as core::ffi::c_int == '|' as i32
                && *bar_fmt.offset(-(1 as core::ffi::c_int as isize)) as core::ffi::c_int
                    != '\\' as i32
            {
                bar_fmt = bar_fmt.offset(1);
                if *bar_fmt as core::ffi::c_int == '0' as i32 {
                    bar_fmt = bar_fmt.offset(1);
                    if *bar_fmt as core::ffi::c_int == '1' as i32 {
                        nbars = ONE_BAR_PLUS as core::ffi::c_int;
                    } else if *bar_fmt as core::ffi::c_int == '2' as i32 {
                        nbars = TWO_BARS_PLUS as core::ffi::c_int;
                    }
                } else if *bar_fmt as core::ffi::c_int == '1' as i32 {
                    nbars = ONE_BAR as core::ffi::c_int;
                } else if *bar_fmt as core::ffi::c_int == '2' as i32 {
                    nbars = TWO_BARS as core::ffi::c_int;
                } else {
                    nbars = TWO_BARS_PLUS as core::ffi::c_int;
                }
                break;
            } else {
                bar_fmt = bar_fmt.offset(1);
            }
        }
        if filename[0 as core::ffi::c_int as usize] as core::ffi::c_int == '\0' as i32 {
            strcpy(
                filename.as_mut_ptr(),
                b"index\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        Index = fopen(
            filename.as_mut_ptr(),
            b"w\0" as *const u8 as *const core::ffi::c_char,
        ) as *mut FILE;
    }
    loop {
        logn_total += logn;
        logn = 0 as core::ffi::c_int;
        log_total += log;
        log = 0 as core::ffi::c_int;
        if command_line != 0 {
            if strcmp(
                *argv.offset((argc - 1 as core::ffi::c_int) as isize),
                b"-\0" as *const u8 as *const core::ffi::c_char,
            ) == 0 as core::ffi::c_int
            {
                strcpy(
                    input.as_mut_ptr(),
                    b"stdin\0" as *const u8 as *const core::ffi::c_char,
                );
                if trnsps == 1 as core::ffi::c_int {
                    g_error(
                        b"cannot transpose without parameters\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            } else {
                if !(arg < argc) {
                    break;
                }
                let fresh8 = arg;
                arg = arg + 1;
                strcpy(input.as_mut_ptr(), *argv.offset(fresh8 as isize));
            }
        } else {
            printf(b"\nselect tunes: \0" as *const u8 as *const core::ffi::c_char);
            if (gets(input.as_mut_ptr())).is_null() {
                break;
            }
            printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        }
        if strcmp(input.as_mut_ptr(), b"q\0" as *const u8 as *const core::ffi::c_char)
            == 0
            || strcmp(
                input.as_mut_ptr(),
                b"quit\0" as *const u8 as *const core::ffi::c_char,
            ) == 0
        {
            break;
        }
        if input[0 as core::ffi::c_int as usize] as core::ffi::c_int == '\\' as i32 {
            draw_text(0 as *mut core::ffi::c_char, input.as_mut_ptr());
        } else {
            c = 0 as core::ffi::c_int;
            while input[c as usize] as core::ffi::c_int != ':' as i32
                && input[c as usize] as core::ffi::c_int != 0
            {
                filename[c as usize] = input[c as usize];
                c += 1;
            }
            filename[c as usize] = '\0' as i32 as core::ffi::c_char;
            if c == 0 as core::ffi::c_int {
                break;
            }
            if input[c as usize] as core::ffi::c_int == '\0' as i32 {
                strcpy(
                    &mut *input.as_mut_ptr().offset(c as isize),
                    b":-\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            pinput = &mut *input
                .as_mut_ptr()
                .offset((c + 1 as core::ffi::c_int) as isize) as *mut core::ffi::c_char;
            first = -(999 as core::ffi::c_int);
            last = -(999 as core::ffi::c_int);
            if !(openIn(filename.as_mut_ptr())).is_null() {
                printf(
                    b"\nselection: %s\n\0" as *const u8 as *const core::ffi::c_char,
                    filename.as_mut_ptr(),
                );
                strip_path(basename.as_mut_ptr(), filename.as_mut_ptr());
                x = 0 as core::ffi::c_int;
                nblanks = 0 as core::ffi::c_int;
                strcpy(
                    dflt_meter.as_mut_ptr(),
                    b"C\0" as *const u8 as *const core::ffi::c_char,
                );
                dflt_origin[0 as core::ffi::c_int as usize] = '\0' as i32
                    as core::ffi::c_char;
                dflt_rhythm[0 as core::ffi::c_int as usize] = '\0' as i32
                    as core::ffi::c_char;
                in_tune = 0 as core::ffi::c_int;
                while !(getsIn(xline.as_mut_ptr())).is_null() {
                    if strncmp(
                        xline.as_mut_ptr(),
                        b"M:\0" as *const u8 as *const core::ffi::c_char,
                        2 as size_t,
                    ) == 0 as core::ffi::c_int && in_tune == 0
                    {
                        if trnsps != 0 {
                            output_transline(xline.as_mut_ptr());
                        }
                        stripcpy(
                            dflt_meter.as_mut_ptr(),
                            &mut *xline
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as isize),
                        );
                    } else if strncmp(
                        xline.as_mut_ptr(),
                        b"F:\0" as *const u8 as *const core::ffi::c_char,
                        2 as size_t,
                    ) == 0 as core::ffi::c_int && in_tune == 0
                    {
                        if trnsps != 0 {
                            output_transline(xline.as_mut_ptr());
                        }
                        strip_path(
                            basename.as_mut_ptr(),
                            &mut *xline
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as isize),
                        );
                    } else if strncmp(
                        xline.as_mut_ptr(),
                        b"O:\0" as *const u8 as *const core::ffi::c_char,
                        2 as size_t,
                    ) == 0 as core::ffi::c_int && in_tune == 0
                    {
                        if trnsps != 0 {
                            output_transline(xline.as_mut_ptr());
                        }
                        stripcpy(
                            dflt_origin.as_mut_ptr(),
                            &mut *xline
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as isize),
                        );
                    } else if strncmp(
                        xline.as_mut_ptr(),
                        b"R:\0" as *const u8 as *const core::ffi::c_char,
                        2 as size_t,
                    ) == 0 as core::ffi::c_int && in_tune == 0
                    {
                        if trnsps != 0 {
                            output_transline(xline.as_mut_ptr());
                        }
                        stripcpy(
                            dflt_rhythm.as_mut_ptr(),
                            &mut *xline
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as isize),
                        );
                    } else if xline[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        == '\\' as i32 && in_tune == 0
                    {
                        if trnsps != 0 {
                            output_transline(xline.as_mut_ptr());
                        }
                        strip(xline.as_mut_ptr(), dummy.as_mut_ptr());
                        if f as core::ffi::c_int == 'm' as i32 {
                            draw_text(0 as *mut core::ffi::c_char, xline.as_mut_ptr());
                        }
                    } else if is_comment(xline.as_mut_ptr()) != 0 {
                        if trnsps != 0 && in_tune == 0 {
                            output_transline(xline.as_mut_ptr());
                        }
                    } else if xline[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        == '\n' as i32
                    {
                        if trnsps != 0 && nblanks == 0 as core::ffi::c_int {
                            output_transline(xline.as_mut_ptr());
                            nblanks += 1 as core::ffi::c_int;
                        }
                        in_tune = 0 as core::ffi::c_int;
                    } else {
                        if !(strncmp(
                            xline.as_mut_ptr(),
                            b"X:\0" as *const u8 as *const core::ffi::c_char,
                            2 as size_t,
                        ) == 0 as core::ffi::c_int)
                        {
                            continue;
                        }
                        in_tune = 1 as core::ffi::c_int;
                        strcpy(
                            number.as_mut_ptr(),
                            &mut *xline
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as isize),
                        );
                        prev_x = x;
                        x = atoi(number.as_mut_ptr());
                        if x != prev_x {
                            y = 1 as core::ffi::c_int;
                            njoint = 0 as core::ffi::c_int;
                        } else {
                            y += 1 as core::ffi::c_int;
                        }
                        if x > last || x == last && y > ylast {
                            if range(
                                &mut first,
                                &mut last,
                                &mut yfirst,
                                &mut ylast,
                                &mut pinput,
                            ) == 0
                            {
                                printf(
                                    b"error in input format\n\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                                printf(
                                    b"abandoning file \"%s\"\n\0" as *const u8
                                        as *const core::ffi::c_char,
                                    filename.as_mut_ptr(),
                                );
                                closeIn();
                                continue;
                            }
                        }
                        if x < first || x == first && y < yfirst {
                            continue;
                        }
                        logn += 1;
                        log += 1;
                        getsIn(full_title.as_mut_ptr());
                        if strncmp(
                            full_title.as_mut_ptr(),
                            b"T:\0" as *const u8 as *const core::ffi::c_char,
                            2 as size_t,
                        ) != 0 as core::ffi::c_int
                        {
                            abc_error(
                                b"T: field should follow X: field\0" as *const u8
                                    as *const core::ffi::c_char as *mut core::ffi::c_char,
                            );
                        }
                        if trnsps != 0 {
                            strcat(xline.as_mut_ptr(), full_title.as_mut_ptr());
                        }
                        strip(full_title.as_mut_ptr(), dummy.as_mut_ptr());
                        if f as core::ffi::c_int == 'm' as i32 {
                            article(
                                &mut *full_title
                                    .as_mut_ptr()
                                    .offset(2 as core::ffi::c_int as isize),
                                (title[0 as core::ffi::c_int as usize]).as_mut_ptr(),
                            );
                            detex(
                                (title[0 as core::ffi::c_int as usize]).as_mut_ptr(),
                                full_title.as_mut_ptr(),
                            );
                        } else {
                            detex(
                                &mut *full_title
                                    .as_mut_ptr()
                                    .offset(2 as core::ffi::c_int as isize),
                                (title[0 as core::ffi::c_int as usize]).as_mut_ptr(),
                            );
                            article(
                                (title[0 as core::ffi::c_int as usize]).as_mut_ptr(),
                                full_title.as_mut_ptr(),
                            );
                        }
                        printf(
                            b" processing \"%s%4d %s\"\n\0" as *const u8
                                as *const core::ffi::c_char,
                            basename.as_mut_ptr(),
                            x,
                            full_title.as_mut_ptr(),
                        );
                        fflush(stdout);
                        if f as core::ffi::c_int == 'm' as i32 {
                            m = 0 as core::ffi::c_int;
                            while m < 79 as core::ffi::c_int {
                                head[m as usize] = '*' as i32 as core::ffi::c_char;
                                m += 1;
                            }
                            head[10 as core::ffi::c_int as usize] = ' ' as i32
                                as core::ffi::c_char;
                            m = 0 as core::ffi::c_int;
                            while full_title[m as usize] != 0 {
                                head[(m + 11 as core::ffi::c_int) as usize] = full_title[m
                                    as usize];
                                m += 1;
                            }
                            head[(m + 11 as core::ffi::c_int) as usize] = ' ' as i32
                                as core::ffi::c_char;
                            head[79 as core::ffi::c_int as usize] = '\0' as i32
                                as core::ffi::c_char;
                            draw_text(
                                b"Z\0" as *const u8 as *const core::ffi::c_char
                                    as *mut core::ffi::c_char,
                                head.as_mut_ptr(),
                            );
                        }
                        if trnsps == 1 as core::ffi::c_int {
                            transpose = 0 as core::ffi::c_int;
                            printf(
                                b"  Transpose? \0" as *const u8 as *const core::ffi::c_char,
                            );
                            gets(trans.as_mut_ptr());
                            if trans[0 as core::ffi::c_int as usize] as core::ffi::c_int
                                == '^' as i32
                                || trans[0 as core::ffi::c_int as usize] as core::ffi::c_int
                                    == '_' as i32
                            {
                                interval(trans.as_mut_ptr());
                            } else {
                                sscanf(
                                    trans.as_mut_ptr(),
                                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                                    &mut transpose as *mut core::ffi::c_int,
                                );
                                offset = 0 as core::ffi::c_int;
                                if transpose != 0 {
                                    printf(
                                        b"  Note offset? \0" as *const u8
                                            as *const core::ffi::c_char,
                                    );
                                    gets(trans.as_mut_ptr());
                                    sscanf(
                                        trans.as_mut_ptr(),
                                        b"%d\0" as *const u8 as *const core::ffi::c_char,
                                        &mut offset as *mut core::ffi::c_int,
                                    );
                                }
                            }
                        }
                        if offset != 0 || transpose != 0 {
                            output_transline(xline.as_mut_ptr());
                        }
                        i = 0 as core::ffi::c_int;
                        while i < 26 as core::ffi::c_int {
                            *((*entry).fields[i as usize])
                                .offset(0 as core::ffi::c_int as isize) = '\0' as i32
                                as core::ffi::c_char;
                            i += 1;
                        }
                        *((*entry).bars).offset(0 as core::ffi::c_int as isize) = '\0'
                            as i32 as core::ffi::c_char;
                        strcpy(
                            (*entry).fields[5 as core::ffi::c_int as usize],
                            basename.as_mut_ptr(),
                        );
                        strcpy(
                            (*entry).fields[12 as core::ffi::c_int as usize],
                            dflt_meter.as_mut_ptr(),
                        );
                        strcpy(
                            (*entry).fields[14 as core::ffi::c_int as usize],
                            dflt_origin.as_mut_ptr(),
                        );
                        strcpy(
                            (*entry).fields[17 as core::ffi::c_int as usize],
                            dflt_rhythm.as_mut_ptr(),
                        );
                        stripcpy(
                            (*entry).fields[23 as core::ffi::c_int as usize],
                            number.as_mut_ptr(),
                        );
                        titles = 1 as core::ffi::c_int;
                        getsIn(xline.as_mut_ptr());
                        while strncmp(
                            xline.as_mut_ptr(),
                            b"K:\0" as *const u8 as *const core::ffi::c_char,
                            2 as size_t,
                        ) != 0 as core::ffi::c_int
                        {
                            if offset != 0 || transpose != 0 {
                                output_transline(xline.as_mut_ptr());
                            }
                            if is_field(xline.as_mut_ptr()) != 0 {
                                if !(strchr(
                                    b"BEGILMNOPQR\0" as *const u8 as *const core::ffi::c_char,
                                    xline[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                                ))
                                    .is_null()
                                {
                                    stripcpy(
                                        (*entry)
                                            .fields[(xline[0 as core::ffi::c_int as usize]
                                            as core::ffi::c_int - 'A' as i32) as usize],
                                        &mut *xline
                                            .as_mut_ptr()
                                            .offset(2 as core::ffi::c_int as isize),
                                    );
                                } else {
                                    strip(xline.as_mut_ptr(), dummy.as_mut_ptr());
                                    match xline[0 as core::ffi::c_int as usize]
                                        as core::ffi::c_int
                                    {
                                        84 => {
                                            logn += 1;
                                            if f as core::ffi::c_int == 'm' as i32 {
                                                let fresh9 = titles;
                                                titles = titles + 1;
                                                article(
                                                    &mut *xline
                                                        .as_mut_ptr()
                                                        .offset(2 as core::ffi::c_int as isize),
                                                    (title[fresh9 as usize]).as_mut_ptr(),
                                                );
                                            } else {
                                                if settings.mine != 0
                                                    && *((*entry).fields[18 as core::ffi::c_int as usize])
                                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                                        != 0
                                                {
                                                    strcat(
                                                        (title[(titles - 1 as core::ffi::c_int) as usize])
                                                            .as_mut_ptr(),
                                                        (*entry).fields[18 as core::ffi::c_int as usize],
                                                    );
                                                    *((*entry).fields[18 as core::ffi::c_int as usize])
                                                        .offset(0 as core::ffi::c_int as isize) = '\0' as i32
                                                        as core::ffi::c_char;
                                                }
                                                let fresh10 = titles;
                                                titles = titles + 1;
                                                detex(
                                                    &mut *xline
                                                        .as_mut_ptr()
                                                        .offset(2 as core::ffi::c_int as isize),
                                                    (title[fresh10 as usize]).as_mut_ptr(),
                                                );
                                            }
                                        }
                                        68 => {
                                            if settings.mine != 0 {
                                                if f as core::ffi::c_int == 'm' as i32 {
                                                    if *((*entry).fields[18 as core::ffi::c_int as usize])
                                                        .offset(0 as core::ffi::c_int as isize) != 0
                                                    {
                                                        strcat(
                                                            (*entry).fields[18 as core::ffi::c_int as usize],
                                                            b", \0" as *const u8 as *const core::ffi::c_char,
                                                        );
                                                    }
                                                    sprintf(
                                                        end_of((*entry).fields[18 as core::ffi::c_int as usize]),
                                                        b"{\\%.2s}\0" as *const u8 as *const core::ffi::c_char,
                                                        &mut *xline
                                                            .as_mut_ptr()
                                                            .offset(2 as core::ffi::c_int as isize)
                                                            as *mut core::ffi::c_char,
                                                    );
                                                    if strlen(xline.as_mut_ptr()) as core::ffi::c_int
                                                        > 7 as core::ffi::c_int
                                                    {
                                                        sprintf(
                                                            end_of((*entry).fields[18 as core::ffi::c_int as usize]),
                                                            b" -- %s\0" as *const u8 as *const core::ffi::c_char,
                                                            &mut *xline
                                                                .as_mut_ptr()
                                                                .offset(7 as core::ffi::c_int as isize)
                                                                as *mut core::ffi::c_char,
                                                        );
                                                    }
                                                } else {
                                                    if *((*entry).fields[18 as core::ffi::c_int as usize])
                                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                                        == '\0' as i32
                                                    {
                                                        strcpy(
                                                            (*entry).fields[18 as core::ffi::c_int as usize],
                                                            b" \0" as *const u8 as *const core::ffi::c_char,
                                                        );
                                                    }
                                                    if xline[6 as core::ffi::c_int as usize] as core::ffi::c_int
                                                        == '_' as i32
                                                    {
                                                        strncat(
                                                            (*entry).fields[18 as core::ffi::c_int as usize],
                                                            &mut *xline
                                                                .as_mut_ptr()
                                                                .offset(1 as core::ffi::c_int as isize),
                                                            5 as size_t,
                                                        );
                                                    } else {
                                                        strncat(
                                                            (*entry).fields[18 as core::ffi::c_int as usize],
                                                            &mut *xline
                                                                .as_mut_ptr()
                                                                .offset(1 as core::ffi::c_int as isize),
                                                            6 as size_t,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        83 => {
                                            if f as core::ffi::c_int == 'm' as i32 {
                                                if *((*entry).fields[18 as core::ffi::c_int as usize])
                                                    .offset(0 as core::ffi::c_int as isize) != 0
                                                {
                                                    strcat(
                                                        (*entry).fields[18 as core::ffi::c_int as usize],
                                                        b", \0" as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                strcat(
                                                    (*entry).fields[18 as core::ffi::c_int as usize],
                                                    &mut *xline
                                                        .as_mut_ptr()
                                                        .offset(2 as core::ffi::c_int as isize),
                                                );
                                            }
                                        }
                                        67 => {
                                            if f as core::ffi::c_int == 'i' as i32 {
                                                detex(
                                                    &mut *xline
                                                        .as_mut_ptr()
                                                        .offset(2 as core::ffi::c_int as isize),
                                                    (*entry).fields[2 as core::ffi::c_int as usize],
                                                );
                                            } else {
                                                strcpy(
                                                    (*entry).fields[2 as core::ffi::c_int as usize],
                                                    &mut *xline
                                                        .as_mut_ptr()
                                                        .offset(2 as core::ffi::c_int as isize),
                                                );
                                            }
                                        }
                                        65 => {
                                            if f as core::ffi::c_int == 'i' as i32 {
                                                detex(
                                                    &mut *xline
                                                        .as_mut_ptr()
                                                        .offset(2 as core::ffi::c_int as isize),
                                                    (*entry).fields[0 as core::ffi::c_int as usize],
                                                );
                                                sprintf(
                                                    end_of(
                                                        (title[(titles - 1 as core::ffi::c_int) as usize])
                                                            .as_mut_ptr(),
                                                    ),
                                                    b" (%s)\0" as *const u8 as *const core::ffi::c_char,
                                                    (*entry).fields[0 as core::ffi::c_int as usize],
                                                );
                                            } else {
                                                strcpy(
                                                    (*entry).fields[0 as core::ffi::c_int as usize],
                                                    &mut *xline
                                                        .as_mut_ptr()
                                                        .offset(2 as core::ffi::c_int as isize),
                                                );
                                            }
                                        }
                                        90 => {
                                            detex(
                                                &mut *xline
                                                    .as_mut_ptr()
                                                    .offset(2 as core::ffi::c_int as isize),
                                                (*entry).fields[25 as core::ffi::c_int as usize],
                                            );
                                            if f as core::ffi::c_int == 'm' as i32 {
                                                draw_text(
                                                    b"Z\0" as *const u8 as *const core::ffi::c_char
                                                        as *mut core::ffi::c_char,
                                                    (*entry).fields[25 as core::ffi::c_int as usize],
                                                );
                                            }
                                        }
                                        72 => {
                                            history = 1 as core::ffi::c_int;
                                            while !(getsIn(xline.as_mut_ptr())).is_null() {
                                                if xline[1 as core::ffi::c_int as usize] as core::ffi::c_int
                                                    == ':' as i32
                                                {
                                                    break;
                                                }
                                                if offset != 0 || transpose != 0 {
                                                    output_transline(xline.as_mut_ptr());
                                                }
                                            }
                                        }
                                        _ => {
                                            abc_warning(
                                                b"ignoring %c field in header\0" as *const u8
                                                    as *const core::ffi::c_char as *mut core::ffi::c_char,
                                                xline[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                                            );
                                        }
                                    }
                                }
                            } else if is_comment(xline.as_mut_ptr()) == 0 {
                                abc_error(
                                    b"unrecognised line\0" as *const u8
                                        as *const core::ffi::c_char as *mut core::ffi::c_char,
                                );
                            }
                            if history == 1 as core::ffi::c_int {
                                history = 0 as core::ffi::c_int;
                            } else {
                                getsIn(xline.as_mut_ptr());
                            }
                        }
                        strip(xline.as_mut_ptr(), key_comment.as_mut_ptr());
                        strcpy(
                            (*entry).fields[10 as core::ffi::c_int as usize],
                            &mut *xline
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as isize),
                        );
                        process_abc(
                            title.as_mut_ptr(),
                            titles,
                            entry,
                            key_comment.as_mut_ptr(),
                            (*entry).bars,
                            b"\0" as *const u8 as *const core::ffi::c_char
                                as *mut core::ffi::c_char,
                            NULL as *mut core::ffi::c_char,
                            output,
                            nbars,
                            0 as core::ffi::c_int,
                            NULL as *mut core::ffi::c_int,
                        );
                        if f as core::ffi::c_int == 'i' as i32 {
                            if settings.mine != 0
                                && *((*entry).fields[18 as core::ffi::c_int as usize])
                                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                    != 0
                            {
                                strcat(
                                    (title[(titles - 1 as core::ffi::c_int) as usize])
                                        .as_mut_ptr(),
                                    (*entry).fields[18 as core::ffi::c_int as usize],
                                );
                            }
                            if titles != 1 as core::ffi::c_int {
                                njoint += 1;
                                sprintf(
                                    (*entry).fields[9 as core::ffi::c_int as usize],
                                    b".%d\0" as *const u8 as *const core::ffi::c_char,
                                    njoint,
                                );
                            }
                            ttl = 0 as core::ffi::c_int;
                            while ttl < titles {
                                strcpy(
                                    (*entry).fields[19 as core::ffi::c_int as usize],
                                    (title[ttl as usize]).as_mut_ptr(),
                                );
                                if *((*entry).fields[11 as core::ffi::c_int as usize])
                                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                    == '\0' as i32
                                {
                                    get_dnl(entry);
                                }
                                put_record(
                                    format.as_mut_ptr(),
                                    Index,
                                    entry as *mut record,
                                );
                                ttl += 1;
                            }
                        }
                        in_tune = 0 as core::ffi::c_int;
                    }
                }
                if strcmp(
                    filename.as_mut_ptr(),
                    b"stdin\0" as *const u8 as *const core::ffi::c_char,
                ) == 0 as core::ffi::c_int
                {
                    logn_total += logn;
                    log_total += log;
                    break;
                } else {
                    closeIn();
                    printf(
                        b"end of file \"%s\"\n\0" as *const u8
                            as *const core::ffi::c_char,
                        filename.as_mut_ptr(),
                    );
                    fprintf(
                        Log,
                        b"%-20.20s %4d %4d\n\0" as *const u8 as *const core::ffi::c_char,
                        basename.as_mut_ptr(),
                        logn,
                        log,
                    );
                }
            }
        }
        if !(input[0 as core::ffi::c_int as usize] != 0) {
            break;
        }
    }
    printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
    if f as core::ffi::c_int == 'm' as i32 {
        close_TeX();
    }
    fprintf(
        Log,
        b"%-20.20s %4d %4d\n\0" as *const u8 as *const core::ffi::c_char,
        b"Total\0" as *const u8 as *const core::ffi::c_char,
        logn_total,
        log_total,
    );
    i = 0 as core::ffi::c_int;
    while i < 26 as core::ffi::c_int {
        free((*entry).fields[i as usize] as *mut core::ffi::c_void);
        i += 1;
    }
    free((*entry).bars as *mut core::ffi::c_void);
    return 0 as core::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut core::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}

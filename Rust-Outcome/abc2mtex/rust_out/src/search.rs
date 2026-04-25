extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn gets(__s: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    fn puts(__s: *const core::ffi::c_char) -> core::ffi::c_int;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
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
    fn strchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn g_error(_: *const core::ffi::c_char, ...);
    fn get_index(_: *mut core::ffi::c_char, _: *mut core::ffi::c_char);
    fn size_record(
        _: *mut core::ffi::c_char,
        _: *mut core::ffi::c_int,
        _: *mut core::ffi::c_char,
    );
    fn alloc_record(_: *mut core::ffi::c_char, _: *mut core::ffi::c_int) -> *mut Record;
    fn free_record(_: *mut record, _: *mut core::ffi::c_char);
    fn get_record(
        _: *mut core::ffi::c_char,
        _: *mut FILE,
        _: *mut record,
    ) -> core::ffi::c_int;
    fn put_record(
        _: *mut core::ffi::c_char,
        _: *mut FILE,
        _: *mut record,
    ) -> core::ffi::c_int;
    fn openIn(_: *mut core::ffi::c_char) -> *mut FILE;
    fn closeIn();
    fn read_settings();
    fn is_field(_: *mut core::ffi::c_char) -> core::ffi::c_int;
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
    fn hash_compare(
        array1: *mut core::ffi::c_int,
        array2: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_abc_entry(
        dflt_meter: *mut core::ffi::c_char,
        dflt_origin: *mut core::ffi::c_char,
        dflt_rhythm: *mut core::ffi::c_char,
        entry: *mut core::ffi::c_char,
        abc: *mut Record,
        x: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn str_search(
        s: *mut core::ffi::c_char,
        t: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
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
pub type output_types = core::ffi::c_uint;
pub const HASH_OUTPUT: output_types = 3;
pub const INDEX_OUTPUT: output_types = 2;
pub const TEX_OUTPUT: output_types = 1;
pub const NO_OUTPUT: output_types = 0;
pub type results = core::ffi::c_uint;
pub const CONTINUE: results = 2;
pub const SUCCESS: results = 1;
pub const FAILURE: results = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
// _stklen already defined in fields.rs (c2rust emits it per-translation-unit;
// when modules are linked into a single Rust crate, only one definition is allowed).
// pub static mut _stklen: core::ffi::c_uint = 8192 as core::ffi::c_uint;
pub const DEFAULT_FORMAT_SIZE: [core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<
        [u8; 45],
        [core::ffi::c_char; 45],
    >(*b"F<99X<99J<99O<99R<99C<99T<99M<99L<99K<99|<99\0")
};
unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut dummy: [[core::ffi::c_char; 99]; 30] = [[0; 99]; 30];
    static mut cor: [core::ffi::c_char; 4] = [
        'C' as i32 as core::ffi::c_char,
        'O' as i32 as core::ffi::c_char,
        'R' as i32 as core::ffi::c_char,
        '\0' as i32 as core::ffi::c_char,
    ];
    static mut m2: [*mut core::ffi::c_char; 6] = [
        b"C\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"C|\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"4/4\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"2/4\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"2/2\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    ];
    static mut m3: [*mut core::ffi::c_char; 7] = [
        b"3/8\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"6/8\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"9/8\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"12/8\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"3/4\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"3/2\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    ];
    let mut input: [[core::ffi::c_char; 99]; 30] = [[0; 99]; 30];
    let mut file: [core::ffi::c_char; 99] = [0; 99];
    let mut fmt_file: [core::ffi::c_char; 99] = [0; 99];
    let mut format: [core::ffi::c_char; 999] = [0; 999];
    let mut arg: core::ffi::c_int = 0;
    let mut abc_input: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut max_dist: core::ffi::c_int = 6 as core::ffi::c_int;
    let mut dist: core::ffi::c_int = 0;
    let mut search: *mut Record = 0 as *mut Record;
    let mut abc: *mut Record = 0 as *mut Record;
    let mut ientry: *mut Record = 0 as *mut Record;
    let mut title_strs: [[core::ffi::c_char; 99]; 10] = [[0; 99]; 10];
    let mut abc_fields: [[core::ffi::c_char; 99]; 30] = [[0; 99]; 30];
    let mut cptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut fptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut fnext: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut temp: [core::ffi::c_char; 999] = [0; 999];
    let mut titles: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ttl: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut fields: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut fld: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut t: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut m: core::ffi::c_int = 0;
    let mut first: core::ffi::c_int = 0;
    let mut last: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut yfirst: core::ffi::c_int = 0;
    let mut ylast: core::ffi::c_int = 4999 as core::ffi::c_int;
    let mut prev_x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ranges_ptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut ranges: [core::ffi::c_char; 99] = [0; 99];
    let mut meter_group: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut result: core::ffi::c_int = 0;
    let mut command_line: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut hash_array: [core::ffi::c_int; 999] = [0; 999];
    let mut dflt_meter: [core::ffi::c_char; 99] = [0; 99];
    let mut dflt_origin: [core::ffi::c_char; 99] = [0; 99];
    let mut dflt_rhythm: [core::ffi::c_char; 99] = [0; 99];
    let mut abc_entry: [core::ffi::c_char; 1999] = [0; 1999];
    let mut ihash_array: [core::ffi::c_int; 999] = [0; 999];
    let mut fmt_fields: [core::ffi::c_char; 12] = [0; 12];
    let mut bar_fmt: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut sizes: [core::ffi::c_int; 12] = [0; 12];
    let mut no_meter: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut In: *mut FILE = 0 as *mut FILE;
    ranges[0 as core::ffi::c_int as usize] = '\0' as i32 as core::ffi::c_char;
    read_settings();
    strcpy(file.as_mut_ptr(), b"index\0" as *const u8 as *const core::ffi::c_char);
    size_record(
        DEFAULT_FORMAT_SIZE.as_ptr() as *mut core::ffi::c_char,
        sizes.as_mut_ptr(),
        fmt_fields.as_mut_ptr(),
    );
    search = alloc_record(fmt_fields.as_mut_ptr(), sizes.as_mut_ptr());
    strcpy(
        (*search).fields[11 as core::ffi::c_int as usize],
        b"1/8\0" as *const u8 as *const core::ffi::c_char,
    );
    arg = 1 as core::ffi::c_int;
    while arg < argc {
        if strcmp(
            *argv.offset(arg as isize),
            b"-f\0" as *const u8 as *const core::ffi::c_char,
        ) == 0 as core::ffi::c_int
        {
            arg += 1;
            strcpy(file.as_mut_ptr(), *argv.offset(arg as isize));
            if arg == argc {
                g_error(b"missing argument\0" as *const u8 as *const core::ffi::c_char);
            }
        } else if strcmp(
            *argv.offset(arg as isize),
            b"-abc\0" as *const u8 as *const core::ffi::c_char,
        ) == 0 as core::ffi::c_int
        {
            abc_input = 1 as core::ffi::c_int;
            abc = alloc_record(fmt_fields.as_mut_ptr(), sizes.as_mut_ptr());
            arg += 1;
            break;
        } else if *(*argv.offset(arg as isize)).offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int >= '0' as i32
            && *(*argv.offset(arg as isize)).offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int <= '9' as i32
        {
            max_dist = atoi(*argv.offset(arg as isize));
        } else if strcmp(
            *argv.offset(arg as isize),
            b"-i\0" as *const u8 as *const core::ffi::c_char,
        ) == 0 as core::ffi::c_int
        {
            command_line = 1 as core::ffi::c_int;
            let fresh0 = i;
            i = i + 1;
            arg += 1;
            strcpy((input[fresh0 as usize]).as_mut_ptr(), *argv.offset(arg as isize));
            if arg == argc {
                g_error(b"missing argument\0" as *const u8 as *const core::ffi::c_char);
            }
        } else {
            g_error(
                b"unrecognised argument %s\0" as *const u8 as *const core::ffi::c_char,
                *argv.offset(arg as isize),
            );
        }
        arg += 1;
    }
    if command_line != 0 {
        input[i as usize][0 as core::ffi::c_int as usize] = '\0' as i32
            as core::ffi::c_char;
    } else {
        gets((input[0 as core::ffi::c_int as usize]).as_mut_ptr());
    }
    i = 0 as core::ffi::c_int;
    while input[i as usize][0 as core::ffi::c_int as usize] != 0 {
        if input[i as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int
            == ':' as i32
            && !(strchr(
                b"MLKX\0" as *const u8 as *const core::ffi::c_char,
                input[i as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int,
            ))
                .is_null()
        {
            match input[i as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int {
                77 => {
                    if strcmp(
                        &mut *(*input.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as isize),
                        b"2\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0 as core::ffi::c_int
                    {
                        meter_group = 2 as core::ffi::c_int;
                    } else if strcmp(
                        &mut *(*input.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as isize),
                        b"3\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0 as core::ffi::c_int
                    {
                        meter_group = 3 as core::ffi::c_int;
                    } else {
                        strcpy(
                            (*search).fields[12 as core::ffi::c_int as usize],
                            &mut *(*input.as_mut_ptr().offset(i as isize))
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as isize),
                        );
                        *((*search).fields[11 as core::ffi::c_int as usize])
                            .offset(0 as core::ffi::c_int as isize) = '\0' as i32
                            as core::ffi::c_char;
                    }
                }
                76 => {
                    strcpy(
                        (*search).fields[11 as core::ffi::c_int as usize],
                        &mut *(*input.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as isize),
                    );
                }
                75 => {
                    strcpy(
                        (*search).fields[10 as core::ffi::c_int as usize],
                        &mut *(*input.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as isize),
                    );
                    if command_line != 0 {
                        i += 1;
                        strcpy((*search).bars, (input[i as usize]).as_mut_ptr());
                    } else {
                        gets((*search).bars);
                    }
                    if *((*search).fields[12 as core::ffi::c_int as usize])
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        == '\0' as i32
                    {
                        strcpy(
                            (*search).fields[12 as core::ffi::c_int as usize],
                            b"C\0" as *const u8 as *const core::ffi::c_char,
                        );
                        no_meter = 1 as core::ffi::c_int;
                    }
                    process_abc(
                        dummy.as_mut_ptr(),
                        0 as core::ffi::c_int,
                        search,
                        b"\0" as *const u8 as *const core::ffi::c_char
                            as *mut core::ffi::c_char,
                        NULL as *mut core::ffi::c_char,
                        b"\0" as *const u8 as *const core::ffi::c_char
                            as *mut core::ffi::c_char,
                        (*search).bars,
                        HASH_OUTPUT as core::ffi::c_int,
                        0 as core::ffi::c_int,
                        1 as core::ffi::c_int,
                        hash_array.as_mut_ptr(),
                    );
                    if no_meter != 0 {
                        *((*search).fields[12 as core::ffi::c_int as usize])
                            .offset(0 as core::ffi::c_int as isize) = '\0' as i32
                            as core::ffi::c_char;
                        no_meter = 0 as core::ffi::c_int;
                    }
                }
                88 => {
                    strcpy(
                        ranges.as_mut_ptr(),
                        &mut *(*input.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as isize),
                    );
                }
                _ => {}
            }
        } else if abc_input != 0 {
            if is_field((input[i as usize]).as_mut_ptr()) == 0 {
                g_error(
                    b"input field not recognised - %s\0" as *const u8
                        as *const core::ffi::c_char,
                    (input[i as usize]).as_mut_ptr(),
                );
            } else if fields < 30 as core::ffi::c_int {
                let fresh1 = fields;
                fields = fields + 1;
                strcpy(
                    (abc_fields[fresh1 as usize]).as_mut_ptr(),
                    (input[i as usize]).as_mut_ptr(),
                );
            } else {
                printf(
                    b"too many search fields\n\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        } else if input[i as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int
            == ':' as i32
            && !(strchr(
                b"OCRF\0" as *const u8 as *const core::ffi::c_char,
                input[i as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int,
            ))
                .is_null()
        {
            strcpy(
                (*search)
                    .fields[(input[i as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_int - 'A' as i32) as usize],
                &mut *(*input.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
            );
        } else if strncmp(
            (input[i as usize]).as_mut_ptr(),
            b"T:\0" as *const u8 as *const core::ffi::c_char,
            2 as size_t,
        ) == 0 as core::ffi::c_int
        {
            if titles < 10 as core::ffi::c_int {
                let fresh2 = titles;
                titles = titles + 1;
                strcpy(
                    (title_strs[fresh2 as usize]).as_mut_ptr(),
                    &mut *(*input.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as isize),
                );
            } else {
                printf(
                    b"too many title strings\n\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        } else {
            printf(b"line not recognised\n\0" as *const u8 as *const core::ffi::c_char);
        }
        if command_line != 0 {
            i += 1 as core::ffi::c_int;
        } else {
            gets((input[0 as core::ffi::c_int as usize]).as_mut_ptr());
        }
    }
    loop {
        if abc_input != 0 {
            strcpy(file.as_mut_ptr(), *argv.offset(arg as isize));
        }
        In = openIn(file.as_mut_ptr());
        if !In.is_null() {
            if abc_input != 0 {
                strcpy(
                    dflt_meter.as_mut_ptr(),
                    b"M:C\n\0" as *const u8 as *const core::ffi::c_char,
                );
                dflt_origin[0 as core::ffi::c_int as usize] = '\0' as i32
                    as core::ffi::c_char;
                dflt_rhythm[0 as core::ffi::c_int as usize] = '\0' as i32
                    as core::ffi::c_char;
                first = -(999 as core::ffi::c_int);
                last = -(999 as core::ffi::c_int);
                x = 0 as core::ffi::c_int;
                ranges_ptr = &mut *ranges
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut core::ffi::c_char;
                while get_abc_entry(
                    dflt_meter.as_mut_ptr(),
                    dflt_origin.as_mut_ptr(),
                    dflt_rhythm.as_mut_ptr(),
                    abc_entry.as_mut_ptr(),
                    abc,
                    &mut x,
                ) != 0 as core::ffi::c_int
                {
                    if x == prev_x {
                        y += 1 as core::ffi::c_int;
                    } else {
                        y = 1 as core::ffi::c_int;
                    }
                    prev_x = x;
                    if ranges[0 as core::ffi::c_int as usize] != 0 {
                        if x > last || x == last && y > ylast {
                            if range(
                                &mut first,
                                &mut last,
                                &mut yfirst,
                                &mut ylast,
                                &mut ranges_ptr,
                            ) == 0
                            {
                                g_error(
                                    b"in range\0" as *const u8 as *const core::ffi::c_char,
                                );
                            }
                        }
                        if x < first || x == first && y < yfirst {
                            continue;
                        }
                    }
                    fld = 0 as core::ffi::c_int;
                    fld = 0 as core::ffi::c_int;
                    while fld < fields {
                        fptr = abc_entry.as_mut_ptr();
                        loop {
                            fnext = strchr(fptr, '\n' as i32);
                            if fnext.is_null() {
                                break;
                            }
                            t = 0 as core::ffi::c_int;
                            cptr = fptr.offset(2 as core::ffi::c_int as isize);
                            while cptr != fnext {
                                let fresh3 = cptr;
                                cptr = cptr.offset(1);
                                let fresh4 = t;
                                t = t + 1;
                                temp[fresh4 as usize] = *fresh3;
                            }
                            temp[t as usize] = '\0' as i32 as core::ffi::c_char;
                            if *fptr as core::ffi::c_int
                                == abc_fields[fld as usize][0 as core::ffi::c_int as usize]
                                    as core::ffi::c_int
                                && {
                                    result = str_search(
                                        &mut *(*abc_fields.as_mut_ptr().offset(fld as isize))
                                            .as_mut_ptr()
                                            .offset(2 as core::ffi::c_int as isize),
                                        temp.as_mut_ptr(),
                                    );
                                    result == SUCCESS as core::ffi::c_int
                                }
                            {
                                break;
                            }
                            if strncmp(
                                b"\nK:\0" as *const u8 as *const core::ffi::c_char,
                                fnext,
                                3 as size_t,
                            ) == 0 as core::ffi::c_int
                            {
                                break;
                            }
                            fptr = fnext.offset(1 as core::ffi::c_int as isize);
                            result = CONTINUE as core::ffi::c_int;
                        }
                        if result != SUCCESS as core::ffi::c_int {
                            break;
                        }
                        fld += 1;
                    }
                    if fld != fields {
                        continue;
                    }
                    if meter_group == 2 as core::ffi::c_int {
                        m = 0 as core::ffi::c_int;
                        while *(m2[m as usize]).offset(0 as core::ffi::c_int as isize)
                            != 0
                        {
                            if strcmp(
                                (*abc).fields[12 as core::ffi::c_int as usize],
                                m2[m as usize],
                            ) == 0 as core::ffi::c_int
                            {
                                break;
                            }
                            m += 1;
                        }
                        if *(m2[m as usize]).offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int == '\0' as i32
                        {
                            continue;
                        }
                    } else if meter_group == 3 as core::ffi::c_int {
                        m = 0 as core::ffi::c_int;
                        while *(m3[m as usize]).offset(0 as core::ffi::c_int as isize)
                            != 0
                        {
                            if strcmp(
                                (*abc).fields[12 as core::ffi::c_int as usize],
                                m3[m as usize],
                            ) == 0 as core::ffi::c_int
                            {
                                break;
                            }
                            m += 1;
                        }
                        if *(m3[m as usize]).offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int == '\0' as i32
                        {
                            continue;
                        }
                    } else if *((*search).fields[12 as core::ffi::c_int as usize])
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
                        && strcmp(
                            (*search).fields[12 as core::ffi::c_int as usize],
                            (*abc).fields[12 as core::ffi::c_int as usize],
                        ) != 0
                    {
                        continue;
                    }
                    if *((*search).bars).offset(0 as core::ffi::c_int as isize) != 0 {
                        process_abc(
                            dummy.as_mut_ptr(),
                            0 as core::ffi::c_int,
                            abc,
                            b"\0" as *const u8 as *const core::ffi::c_char
                                as *mut core::ffi::c_char,
                            NULL as *mut core::ffi::c_char,
                            b"\0" as *const u8 as *const core::ffi::c_char
                                as *mut core::ffi::c_char,
                            (*abc).bars,
                            HASH_OUTPUT as core::ffi::c_int,
                            0 as core::ffi::c_int,
                            0 as core::ffi::c_int,
                            ihash_array.as_mut_ptr(),
                        );
                        dist = hash_compare(
                            hash_array.as_mut_ptr(),
                            ihash_array.as_mut_ptr(),
                        );
                        if dist > max_dist {
                            continue;
                        }
                        printf(
                            b"distance = %d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            dist,
                        );
                    }
                    printf(
                        b"F:%s\n\0" as *const u8 as *const core::ffi::c_char,
                        file.as_mut_ptr(),
                    );
                    puts(abc_entry.as_mut_ptr());
                }
            } else {
                strcpy(fmt_file.as_mut_ptr(), file.as_mut_ptr());
                strcat(
                    fmt_file.as_mut_ptr(),
                    b".fmt\0" as *const u8 as *const core::ffi::c_char,
                );
                get_index(format.as_mut_ptr(), fmt_file.as_mut_ptr());
                bar_fmt = format.as_mut_ptr();
                while *bar_fmt != 0 {
                    if *bar_fmt as core::ffi::c_int == '|' as i32
                        && *bar_fmt.offset(-(1 as core::ffi::c_int as isize))
                            as core::ffi::c_int != '\\' as i32
                    {
                        bar_fmt = bar_fmt.offset(1);
                        break;
                    } else {
                        bar_fmt = bar_fmt.offset(1);
                    }
                }
                size_record(
                    format.as_mut_ptr(),
                    sizes.as_mut_ptr(),
                    fmt_fields.as_mut_ptr(),
                );
                ientry = alloc_record(fmt_fields.as_mut_ptr(), sizes.as_mut_ptr());
                if ranges[0 as core::ffi::c_int as usize] != 0 {
                    if range(
                        &mut first,
                        &mut last,
                        &mut yfirst,
                        &mut ylast,
                        &mut ranges_ptr,
                    ) == 0
                    {
                        g_error(b"in range\0" as *const u8 as *const core::ffi::c_char);
                    }
                    x = first;
                    if ((*ientry).fields[23 as core::ffi::c_int as usize]).is_null() {
                        g_error(
                            b"X field not in index\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                }
                i = 0 as core::ffi::c_int;
                while cor[i as usize] != 0 {
                    if *((*search)
                        .fields[(cor[i as usize] as core::ffi::c_int - 'A' as i32)
                        as usize])
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
                        && ((*ientry)
                            .fields[(cor[i as usize] as core::ffi::c_int - 'A' as i32)
                            as usize])
                            .is_null()
                    {
                        g_error(
                            b"%c field not in index\0" as *const u8
                                as *const core::ffi::c_char,
                            cor[i as usize] as core::ffi::c_int,
                        );
                    }
                    i += 1;
                }
                if titles != 0
                    && ((*ientry).fields[19 as core::ffi::c_int as usize]).is_null()
                {
                    g_error(
                        b"T field not in index\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if (*((*search).fields[12 as core::ffi::c_int as usize])
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
                    || meter_group != 0)
                    && ((*ientry).fields[12 as core::ffi::c_int as usize]).is_null()
                {
                    g_error(
                        b"M field not in index\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                while get_record(format.as_mut_ptr(), In, ientry as *mut record)
                    != 0 as core::ffi::c_int
                {
                    if *((*search).fields[5 as core::ffi::c_int as usize])
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
                        && strcmp(
                            (*search).fields[5 as core::ffi::c_int as usize],
                            (*ientry).fields[5 as core::ffi::c_int as usize],
                        ) != 0
                    {
                        continue;
                    }
                    if x != 0
                        && x != atoi((*ientry).fields[23 as core::ffi::c_int as usize])
                    {
                        continue;
                    }
                    i = 0 as core::ffi::c_int;
                    while cor[i as usize] != 0 {
                        if *((*search)
                            .fields[(cor[i as usize] as core::ffi::c_int - 'A' as i32)
                            as usize])
                            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                            != 0
                            && strcmp(
                                (*search)
                                    .fields[(cor[i as usize] as core::ffi::c_int - 'A' as i32)
                                    as usize],
                                (*ientry)
                                    .fields[(cor[i as usize] as core::ffi::c_int - 'A' as i32)
                                    as usize],
                            ) != 0
                        {
                            break;
                        }
                        i += 1;
                    }
                    if cor[i as usize] != 0 {
                        continue;
                    }
                    ttl = 0 as core::ffi::c_int;
                    ttl = 0 as core::ffi::c_int;
                    while ttl < titles {
                        if str_search(
                            (title_strs[ttl as usize]).as_mut_ptr(),
                            (*ientry).fields[19 as core::ffi::c_int as usize],
                        ) == FAILURE as core::ffi::c_int
                        {
                            break;
                        }
                        ttl += 1;
                    }
                    if ttl != titles {
                        continue;
                    }
                    if meter_group == 2 as core::ffi::c_int {
                        m = 0 as core::ffi::c_int;
                        while *(m2[m as usize]).offset(0 as core::ffi::c_int as isize)
                            != 0
                        {
                            if strcmp(
                                (*ientry).fields[12 as core::ffi::c_int as usize],
                                m2[m as usize],
                            ) == 0 as core::ffi::c_int
                            {
                                break;
                            }
                            m += 1;
                        }
                        if *(m2[m as usize]).offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int == '\0' as i32
                        {
                            continue;
                        }
                    } else if meter_group == 3 as core::ffi::c_int {
                        m = 0 as core::ffi::c_int;
                        while *(m3[m as usize]).offset(0 as core::ffi::c_int as isize)
                            != 0
                        {
                            if strcmp(
                                (*ientry).fields[12 as core::ffi::c_int as usize],
                                m3[m as usize],
                            ) == 0 as core::ffi::c_int
                            {
                                break;
                            }
                            m += 1;
                        }
                        if *(m3[m as usize]).offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int == '\0' as i32
                        {
                            continue;
                        }
                    } else if *((*search).fields[12 as core::ffi::c_int as usize])
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
                        && strcmp(
                            (*search).fields[12 as core::ffi::c_int as usize],
                            (*ientry).fields[12 as core::ffi::c_int as usize],
                        ) != 0
                    {
                        continue;
                    }
                    if *((*search).bars).offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int != 0 && *bar_fmt as core::ffi::c_int != 0
                    {
                        process_abc(
                            dummy.as_mut_ptr(),
                            0 as core::ffi::c_int,
                            ientry,
                            b"\0" as *const u8 as *const core::ffi::c_char
                                as *mut core::ffi::c_char,
                            NULL as *mut core::ffi::c_char,
                            b"\0" as *const u8 as *const core::ffi::c_char
                                as *mut core::ffi::c_char,
                            (*ientry).bars,
                            HASH_OUTPUT as core::ffi::c_int,
                            0 as core::ffi::c_int,
                            0 as core::ffi::c_int,
                            ihash_array.as_mut_ptr(),
                        );
                        dist = hash_compare(
                            hash_array.as_mut_ptr(),
                            ihash_array.as_mut_ptr(),
                        );
                        if dist > max_dist {
                            continue;
                        }
                        printf(
                            b"distance = %d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            dist,
                        );
                    } else if *((*search).bars).offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int != 0
                        && *bar_fmt as core::ffi::c_int == '\0' as i32
                    {
                        continue;
                    }
                    put_record(format.as_mut_ptr(), stdout, ientry as *mut record);
                }
            }
            closeIn();
        }
        arg += 1;
        if !(arg < argc) {
            break;
        }
    }
    if abc_input == 0 {
        free_record(ientry as *mut record, fmt_fields.as_mut_ptr());
    }
    size_record(
        DEFAULT_FORMAT_SIZE.as_ptr() as *mut core::ffi::c_char,
        sizes.as_mut_ptr(),
        fmt_fields.as_mut_ptr(),
    );
    free_record(search as *mut record, fmt_fields.as_mut_ptr());
    if abc_input != 0 {
        free_record(abc as *mut record, fmt_fields.as_mut_ptr());
    }
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

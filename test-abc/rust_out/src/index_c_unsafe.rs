extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fgets(
        __s: *mut core::ffi::c_char,
        __n: core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut core::ffi::c_char;
    fn fputs(__s: *const core::ffi::c_char, __stream: *mut FILE) -> core::ffi::c_int;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strncpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
        __n: size_t,
    ) -> *mut core::ffi::c_char;
    fn strchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn g_error(_: *const core::ffi::c_char, ...);
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn get_index(
    mut fmt: *mut core::ffi::c_char,
    mut fname: *mut core::ffi::c_char,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut colon: core::ffi::c_int = 0;
    let mut c: core::ffi::c_char = 0;
    let mut f: core::ffi::c_int = 0 as core::ffi::c_int;
    fp = fopen(fname, b"r\0" as *const u8 as *const core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        g_error(
            b"file %s does not exist\0" as *const u8 as *const core::ffi::c_char,
            fname,
        );
    }
    *fmt.offset(0 as core::ffi::c_int as isize) = '\0' as i32 as core::ffi::c_char;
    while !(fgets(
        &mut *fmt
            .offset(
                (strlen as unsafe extern "C" fn(*const core::ffi::c_char) -> size_t)(fmt)
                    as isize,
            ),
        99 as core::ffi::c_int,
        fp,
    ))
        .is_null()
    {}
    while *fmt.offset(f as isize) != 0 {
        if !(strchr(
            b"FXJORCTMLK|\0" as *const u8 as *const core::ffi::c_char,
            *fmt.offset(f as isize) as core::ffi::c_int,
        ))
            .is_null()
        {
            c = *fmt.offset(f as isize);
            let fresh0 = f;
            f = f + 1;
            if *fmt.offset(fresh0 as isize) as core::ffi::c_int == '|' as i32 {
                if *fmt.offset(f as isize) as core::ffi::c_int == '0' as i32 {
                    f += 1;
                }
                if !(strchr(
                    b"12\0" as *const u8 as *const core::ffi::c_char,
                    *fmt.offset(f as isize) as core::ffi::c_int,
                ))
                    .is_null()
                {
                    f += 1;
                }
            }
            if *fmt.offset(f as isize) as core::ffi::c_int == ':' as i32 {
                colon = 1 as core::ffi::c_int;
            } else if *fmt.offset(f as isize) as core::ffi::c_int == '<' as i32
                || *fmt.offset(f as isize) as core::ffi::c_int == '>' as i32
            {
                colon = 0 as core::ffi::c_int;
            } else {
                g_error(
                    b"fmt file - %c not followed by [><:]\0" as *const u8
                        as *const core::ffi::c_char,
                    c as core::ffi::c_int,
                );
            }
            f += 1;
            if (*fmt.offset(f as isize) as core::ffi::c_int) < '0' as i32
                || *fmt.offset(f as isize) as core::ffi::c_int > '9' as i32
            {
                g_error(
                    b"fmt file - %c not followed by length\0" as *const u8
                        as *const core::ffi::c_char,
                    c as core::ffi::c_int,
                );
            }
            while *fmt.offset(f as isize) as core::ffi::c_int >= '0' as i32
                && *fmt.offset(f as isize) as core::ffi::c_int <= '9' as i32
            {
                f += 1;
            }
            if colon != 0 && *fmt.offset(f as isize) as core::ffi::c_int != '\n' as i32 {
                g_error(
                    b"fmt file - %c: not at end of line\0" as *const u8
                        as *const core::ffi::c_char,
                    c as core::ffi::c_int,
                );
            }
        } else if *fmt.offset(f as isize) as core::ffi::c_int == '\\' as i32 {
            f += 2 as core::ffi::c_int;
        } else {
            f += 1 as core::ffi::c_int;
        }
    }
    if *fmt.offset((f - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        != '\n' as i32
    {
        g_error(
            b"fmt file - not terminated by newline character\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn size_record(
    mut fmt: *mut core::ffi::c_char,
    mut size: *mut core::ffi::c_int,
    mut field: *mut core::ffi::c_char,
) {
    let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut f: core::ffi::c_int = 0 as core::ffi::c_int;
    while *fmt.offset(f as isize) != 0 {
        if !(strchr(
            b"FXJORCTMLK|\0" as *const u8 as *const core::ffi::c_char,
            *fmt.offset(f as isize) as core::ffi::c_int,
        ))
            .is_null()
        {
            let fresh1 = f;
            f = f + 1;
            *field.offset(s as isize) = *fmt.offset(fresh1 as isize);
            if *field.offset(s as isize) as core::ffi::c_int == '|' as i32
                && *fmt.offset(f as isize) as core::ffi::c_int == '0' as i32
            {
                f += 1;
            }
            if *field.offset(s as isize) as core::ffi::c_int == '|' as i32
                && !(strchr(
                    b"12\0" as *const u8 as *const core::ffi::c_char,
                    *fmt.offset(f as isize) as core::ffi::c_int,
                ))
                    .is_null()
            {
                f += 1;
            }
            f += 1;
            let fresh2 = s;
            s = s + 1;
            *size.offset(fresh2 as isize) = atoi(&mut *fmt.offset(f as isize));
            while *fmt.offset(f as isize) as core::ffi::c_int >= '0' as i32
                && *fmt.offset(f as isize) as core::ffi::c_int <= '9' as i32
            {
                f += 1;
            }
        } else if *fmt.offset(f as isize) as core::ffi::c_int == '\\' as i32 {
            f += 2 as core::ffi::c_int;
        } else {
            f += 1 as core::ffi::c_int;
        }
    }
    *field.offset(s as isize) = '\0' as i32 as core::ffi::c_char;
    *size.offset(s as isize) = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_record(
    mut fmt: *mut core::ffi::c_char,
    mut size: *mut core::ffi::c_int,
) -> *mut Record {
    let mut entry: *mut Record = 0 as *mut Record;
    let mut f: core::ffi::c_int = 0;
    entry = calloc(
        1 as core::ffi::c_int as core::ffi::c_uint as size_t,
        ::core::mem::size_of::<Record>() as size_t,
    ) as *mut Record;
    if entry.is_null() {
        g_error(
            b"alloc failure in Record_array\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    f = 0 as core::ffi::c_int;
    while *fmt.offset(f as isize) != 0 {
        if !(strchr(
            b"FXJORCTMLK\0" as *const u8 as *const core::ffi::c_char,
            *fmt.offset(f as isize) as core::ffi::c_int,
        ))
            .is_null()
        {
            (*entry)
                .fields[(*fmt.offset(f as isize) as core::ffi::c_int - 'A' as i32)
                as usize] = calloc(
                (*size.offset(f as isize) as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as size_t,
                ::core::mem::size_of::<core::ffi::c_char>() as size_t,
            ) as *mut core::ffi::c_char;
            if ((*entry)
                .fields[(*fmt.offset(f as isize) as core::ffi::c_int - 'A' as i32)
                as usize])
                .is_null()
            {
                g_error(
                    b"alloc failure in char_array\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        } else if *fmt.offset(f as isize) as core::ffi::c_int == '|' as i32 {
            (*entry).bars = calloc(
                (*size.offset(f as isize) as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as size_t,
                ::core::mem::size_of::<core::ffi::c_char>() as size_t,
            ) as *mut core::ffi::c_char;
            if ((*entry).bars).is_null() {
                g_error(
                    b"alloc failure in char_array\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        } else {
            g_error(b"in size format\0" as *const u8 as *const core::ffi::c_char);
        }
        f += 1;
    }
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn free_record(
    mut entry: *mut Record,
    mut fmt: *mut core::ffi::c_char,
) {
    let mut f: core::ffi::c_int = 0;
    f = 0 as core::ffi::c_int;
    while *fmt.offset(f as isize) != 0 {
        if !(strchr(
            b"FXJORCTMLK\0" as *const u8 as *const core::ffi::c_char,
            *fmt.offset(f as isize) as core::ffi::c_int,
        ))
            .is_null()
        {
            free(
                (*entry)
                    .fields[(*fmt.offset(f as isize) as core::ffi::c_int - 'A' as i32)
                    as usize] as *mut core::ffi::c_void,
            );
        } else if *fmt.offset(f as isize) as core::ffi::c_int == '|' as i32 {
            free((*entry).bars as *mut core::ffi::c_void);
        }
        f += 1;
    }
    free(entry as *mut core::ffi::c_void);
}
unsafe extern "C" fn str_get(
    mut string: *mut core::ffi::c_char,
    mut fmt: *mut core::ffi::c_char,
    mut f: *mut core::ffi::c_int,
    mut index: *mut core::ffi::c_char,
    mut i: *mut core::ffi::c_int,
) {
    let mut c: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    let mut l: core::ffi::c_int = 0;
    let mut temp: [core::ffi::c_char; 99] = [0; 99];
    *f += 2 as core::ffi::c_int;
    n = atoi(&mut *fmt.offset(*f as isize));
    if *fmt.offset((*f - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        == '<' as i32
    {
        strncpy(string, &mut *index.offset(*i as isize), n as size_t);
        *string.offset(n as isize) = '\0' as i32 as core::ffi::c_char;
        l = strlen(string) as core::ffi::c_int;
        c = l;
        while c > 0 as core::ffi::c_int
            && *string.offset((c - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                == ' ' as i32
        {
            c -= 1;
        }
        *string.offset(c as isize) = '\0' as i32 as core::ffi::c_char;
        *i += n;
    } else if *fmt.offset((*f - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        == '>' as i32
    {
        strncpy(temp.as_mut_ptr(), &mut *index.offset(*i as isize), n as size_t);
        temp[n as usize] = '\0' as i32 as core::ffi::c_char;
        c = 0 as core::ffi::c_int;
        while temp[c as usize] as core::ffi::c_int == ' ' as i32 && c < n {
            c += 1;
        }
        strcpy(string, &mut *temp.as_mut_ptr().offset(c as isize));
        *i += n;
    } else {
        strncpy(string, &mut *index.offset(*i as isize), n as size_t);
        l = strlen(string) as core::ffi::c_int;
        if *string.offset((l - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
            == '\n' as i32
        {
            l -= 1;
            *string.offset(l as isize) = '\0' as i32 as core::ffi::c_char;
        }
        *i = (*i as size_t).wrapping_add(strlen(string)) as core::ffi::c_int
            as core::ffi::c_int;
    }
    while *fmt.offset(*f as isize) as core::ffi::c_int >= '0' as i32
        && *fmt.offset(*f as isize) as core::ffi::c_int <= '9' as i32
    {
        *f += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_record(
    mut fmt: *mut core::ffi::c_char,
    mut In: *mut FILE,
    mut entry: *mut Record,
) -> core::ffi::c_int {
    let mut new_line: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut f: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    static mut index: [core::ffi::c_char; 999] = [0; 999];
    while *fmt.offset(f as isize) != 0 {
        if new_line == 1 as core::ffi::c_int {
            if (fgets(index.as_mut_ptr(), 999 as core::ffi::c_int, In)).is_null() {
                return 0 as core::ffi::c_int;
            }
            new_line = 0 as core::ffi::c_int;
        }
        if !(strchr(
            b"FXJORCTMLK\0" as *const u8 as *const core::ffi::c_char,
            *fmt.offset(f as isize) as core::ffi::c_int,
        ))
            .is_null()
        {
            str_get(
                (*entry)
                    .fields[(*fmt.offset(f as isize) as core::ffi::c_int - 'A' as i32)
                    as usize],
                fmt,
                &mut f,
                index.as_mut_ptr(),
                &mut i,
            );
        } else {
            let mut current_block_16: u64;
            match *fmt.offset(f as isize) as core::ffi::c_int {
                124 => {
                    if *fmt.offset((f + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int == '0' as i32
                    {
                        f += 1;
                    }
                    if !(strchr(
                        b"12\0" as *const u8 as *const core::ffi::c_char,
                        *fmt.offset((f + 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int,
                    ))
                        .is_null()
                    {
                        f += 1;
                    }
                    str_get((*entry).bars, fmt, &mut f, index.as_mut_ptr(), &mut i);
                    current_block_16 = 5689001924483802034;
                }
                10 => {
                    new_line = 1 as core::ffi::c_int;
                    f += 1 as core::ffi::c_int;
                    i = 0 as core::ffi::c_int;
                    current_block_16 = 5689001924483802034;
                }
                92 => {
                    f += 1 as core::ffi::c_int;
                    current_block_16 = 17546347598942715757;
                }
                _ => {
                    current_block_16 = 17546347598942715757;
                }
            }
            match current_block_16 {
                17546347598942715757 => {
                    f += 1 as core::ffi::c_int;
                    i += 1 as core::ffi::c_int;
                }
                _ => {}
            }
        }
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C" fn str_put(
    mut string: *mut core::ffi::c_char,
    mut fmt: *mut core::ffi::c_char,
    mut f: *mut core::ffi::c_int,
    mut index: *mut core::ffi::c_char,
    mut i: *mut core::ffi::c_int,
) {
    let mut c: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    let mut l: core::ffi::c_int = 0;
    *f += 2 as core::ffi::c_int;
    n = atoi(&mut *fmt.offset(*f as isize));
    if *fmt.offset((*f - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        == '<' as i32
    {
        strncpy(&mut *index.offset(*i as isize), string, n as size_t);
        c = strlen(string) as core::ffi::c_int;
        while c < n {
            *index.offset((*i + c) as isize) = ' ' as i32 as core::ffi::c_char;
            c += 1;
        }
        *i += n;
        *index.offset(*i as isize) = '\0' as i32 as core::ffi::c_char;
    } else if *fmt.offset((*f - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
        == '>' as i32
    {
        l = strlen(string) as core::ffi::c_int;
        if l < n {
            c = 0 as core::ffi::c_int;
            while c < n - l {
                *index.offset((*i + c) as isize) = ' ' as i32 as core::ffi::c_char;
                c += 1;
            }
            *index.offset((*i + c) as isize) = '\0' as i32 as core::ffi::c_char;
            strcpy(&mut *index.offset((*i + c) as isize), string);
        } else {
            strncpy(&mut *index.offset(*i as isize), string, n as size_t);
        }
        *i += n;
        *index.offset(*i as isize) = '\0' as i32 as core::ffi::c_char;
    } else {
        strcpy(&mut *index.offset(*i as isize), string);
        *i = (*i as size_t).wrapping_add(strlen(string)) as core::ffi::c_int
            as core::ffi::c_int;
    }
    while *fmt.offset(*f as isize) as core::ffi::c_int >= '0' as i32
        && *fmt.offset(*f as isize) as core::ffi::c_int <= '9' as i32
    {
        *f += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn put_record(
    mut fmt: *mut core::ffi::c_char,
    mut Out: *mut FILE,
    mut entry: *mut Record,
) -> core::ffi::c_int {
    let mut f: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    static mut index: [core::ffi::c_char; 999] = [0; 999];
    while *fmt.offset(f as isize) != 0 {
        if i == 0 as core::ffi::c_int {
            index[0 as core::ffi::c_int as usize] = '\0' as i32 as core::ffi::c_char;
        }
        if !(strchr(
            b"FXJORCTMLK\0" as *const u8 as *const core::ffi::c_char,
            *fmt.offset(f as isize) as core::ffi::c_int,
        ))
            .is_null()
        {
            str_put(
                (*entry)
                    .fields[(*fmt.offset(f as isize) as core::ffi::c_int - 'A' as i32)
                    as usize],
                fmt,
                &mut f,
                index.as_mut_ptr(),
                &mut i,
            );
        } else {
            let mut current_block_14: u64;
            match *fmt.offset(f as isize) as core::ffi::c_int {
                124 => {
                    if *fmt.offset((f + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int == '0' as i32
                    {
                        f += 1;
                    }
                    if !(strchr(
                        b"12\0" as *const u8 as *const core::ffi::c_char,
                        *fmt.offset((f + 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int,
                    ))
                        .is_null()
                    {
                        f += 1;
                    }
                    str_put((*entry).bars, fmt, &mut f, index.as_mut_ptr(), &mut i);
                    current_block_14 = 1054647088692577877;
                }
                92 => {
                    f += 1 as core::ffi::c_int;
                    current_block_14 = 13259074699098553214;
                }
                _ => {
                    current_block_14 = 13259074699098553214;
                }
            }
            match current_block_14 {
                13259074699098553214 => {
                    let fresh3 = i;
                    i = i + 1;
                    strncpy(
                        &mut *index.as_mut_ptr().offset(fresh3 as isize),
                        &mut *fmt.offset(f as isize),
                        1 as size_t,
                    );
                    index[i as usize] = '\0' as i32 as core::ffi::c_char;
                    let fresh4 = f;
                    f = f + 1;
                    if *fmt.offset(fresh4 as isize) as core::ffi::c_int == '\n' as i32 {
                        fputs(index.as_mut_ptr(), Out);
                        i = 0 as core::ffi::c_int;
                    }
                }
                _ => {}
            }
        }
    }
    return 1 as core::ffi::c_int;
}

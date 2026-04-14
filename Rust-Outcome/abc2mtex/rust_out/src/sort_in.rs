extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn compare(entry1: *mut *mut Record, entry2: *mut *mut Record) -> core::ffi::c_int;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record {
    pub fields: [*mut core::ffi::c_char; 26],
    pub bars: *mut core::ffi::c_char,
    pub next: *mut record,
}
pub type Record = record;
pub type compFun = unsafe extern "C" fn(
    *const core::ffi::c_void,
    *const core::ffi::c_void,
) -> core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut priority: [core::ffi::c_char; 99] = [0; 99];
unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut format: [core::ffi::c_char; 999] = [0; 999];
    let mut file: [core::ffi::c_char; 99] = [0; 99];
    let mut fmt_file: [core::ffi::c_char; 99] = [0; 99];
    let mut index: *mut FILE = 0 as *mut FILE;
    let mut fields: [core::ffi::c_char; 12] = [0; 12];
    let mut sizes: [core::ffi::c_int; 12] = [0; 12];
    let mut entry: *mut Record = 0 as *mut Record;
    let mut first_entry: *mut Record = 0 as *mut Record;
    let mut prev_entry: *mut Record = 0 as *mut Record;
    let mut next_entry: *mut Record = 0 as *mut Record;
    let mut list: *mut *mut Record = 0 as *mut *mut Record;
    let mut i: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut arg: core::ffi::c_int = 0;
    strcpy(file.as_mut_ptr(), b"index\0" as *const u8 as *const core::ffi::c_char);
    strcpy(priority.as_mut_ptr(), b"T\0" as *const u8 as *const core::ffi::c_char);
    arg = 1 as core::ffi::c_int;
    while arg < argc {
        if arg + 1 as core::ffi::c_int == argc {
            g_error(b"missing argument\0" as *const u8 as *const core::ffi::c_char);
        }
        if strcmp(
            *argv.offset(arg as isize),
            b"-f\0" as *const u8 as *const core::ffi::c_char,
        ) == 0 as core::ffi::c_int
        {
            arg += 1;
            strcpy(file.as_mut_ptr(), *argv.offset(arg as isize));
        } else if strcmp(
            *argv.offset(arg as isize),
            b"-p\0" as *const u8 as *const core::ffi::c_char,
        ) == 0 as core::ffi::c_int
        {
            arg += 1;
            strcpy(priority.as_mut_ptr(), *argv.offset(arg as isize));
        } else {
            g_error(
                b"unrecognised argument %s\0" as *const u8 as *const core::ffi::c_char,
                *argv.offset(arg as isize),
            );
        }
        arg += 1;
    }
    if strcmp(file.as_mut_ptr(), b"-\0" as *const u8 as *const core::ffi::c_char)
        == 0 as core::ffi::c_int
    {
        index = stdin;
        strcpy(
            fmt_file.as_mut_ptr(),
            b"index\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {
        index = fopen(file.as_mut_ptr(), b"r\0" as *const u8 as *const core::ffi::c_char)
            as *mut FILE;
        if index.is_null() {
            g_error(
                b"cannot open file %s\0" as *const u8 as *const core::ffi::c_char,
                file.as_mut_ptr(),
            );
        }
        strcpy(fmt_file.as_mut_ptr(), file.as_mut_ptr());
    }
    strcat(fmt_file.as_mut_ptr(), b".fmt\0" as *const u8 as *const core::ffi::c_char);
    get_index(format.as_mut_ptr(), fmt_file.as_mut_ptr());
    size_record(format.as_mut_ptr(), sizes.as_mut_ptr(), fields.as_mut_ptr());
    entry = alloc_record(fields.as_mut_ptr(), sizes.as_mut_ptr());
    first_entry = entry;
    while get_record(format.as_mut_ptr(), index, entry as *mut record)
        != 0 as core::ffi::c_int
    {
        prev_entry = entry;
        entry = alloc_record(fields.as_mut_ptr(), sizes.as_mut_ptr());
        (*prev_entry).next = entry as *mut record;
        n += 1 as core::ffi::c_int;
    }
    if strcmp(file.as_mut_ptr(), b"-\0" as *const u8 as *const core::ffi::c_char)
        != 0 as core::ffi::c_int
    {
        fclose(index);
    }
    list = calloc(
        n as core::ffi::c_uint as size_t,
        ::core::mem::size_of::<*mut Record>() as size_t,
    ) as *mut *mut Record;
    if list.is_null() {
        g_error(
            b"alloc failure in Record*_array\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    let ref mut fresh0 = *list.offset(0 as core::ffi::c_int as isize);
    *fresh0 = first_entry;
    i = 1 as core::ffi::c_int;
    while i < n {
        let ref mut fresh1 = *list.offset(i as isize);
        *fresh1 = (**list.offset((i - 1 as core::ffi::c_int) as isize)).next
            as *mut Record;
        i += 1;
    }
    qsort(
        list as *mut core::ffi::c_char as *mut core::ffi::c_void,
        n as core::ffi::c_uint as size_t,
        ::core::mem::size_of::<*mut Record>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut *mut Record,
                    *mut *mut Record,
                ) -> core::ffi::c_int,
            >,
            __compar_fn_t,
        >(
            Some(
                compare
                    as unsafe extern "C" fn(
                        *mut *mut Record,
                        *mut *mut Record,
                    ) -> core::ffi::c_int,
            ),
        ),
    );
    if strcmp(file.as_mut_ptr(), b"-\0" as *const u8 as *const core::ffi::c_char)
        == 0 as core::ffi::c_int
    {
        index = stdout;
    } else {
        index = fopen(file.as_mut_ptr(), b"w\0" as *const u8 as *const core::ffi::c_char)
            as *mut FILE;
        if index.is_null() {
            g_error(
                b"cannot open file %s\0" as *const u8 as *const core::ffi::c_char,
                file.as_mut_ptr(),
            );
        }
    }
    i = 0 as core::ffi::c_int;
    while i < n {
        put_record(format.as_mut_ptr(), index, *list.offset(i as isize));
        i += 1;
    }
    if strcmp(file.as_mut_ptr(), b"-\0" as *const u8 as *const core::ffi::c_char)
        != 0 as core::ffi::c_int
    {
        fclose(index);
    }
    entry = first_entry;
    while !entry.is_null() {
        next_entry = (*entry).next as *mut Record;
        free_record(entry as *mut record, fields.as_mut_ptr());
        entry = next_entry;
    }
    free(list as *mut core::ffi::c_void);
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

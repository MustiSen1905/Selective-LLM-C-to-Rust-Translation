#[derive(Debug, Clone)]
pub struct Safe_IO_FILE {
    pub flags: i32,
    pub read_ptr: String,
    pub read_end: String,
    pub read_base: String,
    pub write_base: String,
    pub write_ptr: String,
    pub write_end: String,
    pub buf_base: String,
    pub buf_end: String,
    pub save_base: String,
    pub backup_base: String,
    pub save_end: String,
    pub markers: Box<Safe_IO_marker>,
    pub chain: Box<Safe_IO_FILE>,
    pub fileno: i32,
    pub flags2: i32,
    pub old_offset: isize,
    pub cur_column: u16,
    pub vtable_offset: i8,
    pub shortbuf: String,
    pub offset: isize,
    pub codecvt: Box<Safe_IO_codecvt>,
    pub wide_data: Box<Safe_IO_wide_data>,
    pub freeres_list: Box<Safe_IO_FILE>,
    pub freeres_buf: *mut core::ffi::c_void, // This could be Box<T> or Vec<T> depending on the actual use case.
    pub pad5: usize,
    pub mode: i32,
    pub unused2: String,
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Safe_IO_marker { /* TODO */ }
impl Safe_IO_marker { unsafe fn from_ptr(_ptr: *const _IO_marker) -> Self { unimplemented!() }}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Safe_IO_codecvt { /* TODO */ }
impl Safe_IO_codecvt {

}}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Safe_IO_wide_data { /* TODO */ }
impl Safe_IO_wide_data {

}}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SafeMyStringStruct {
    pub qstring: String,
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stderr: *mut FILE;
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
    fn fscanf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
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
pub struct myStringStruct {
    pub qstring: [core::ffi::c_char; 128],
}
#[no_mangle]
pub unsafe extern "C" fn compare(
    mut elem1: *const core::ffi::c_void,
    mut elem2: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let mut result: core::ffi::c_int = 0;
    result = strcmp(
        ((*(elem1 as *mut myStringStruct)).qstring).as_mut_ptr(),
        ((*(elem2 as *mut myStringStruct)).qstring).as_mut_ptr(),
    );
    return if result < 0 as core::ffi::c_int {
        1 as core::ffi::c_int
    } else if result == 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
}
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut array: *mut myStringStruct = 0 as *mut myStringStruct;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut size: core::ffi::c_int = 0;
    if argc < 3 as core::ffi::c_int {
        fprintf(
            stderr,
            b"Usage: qsort_small <size> <file>\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        exit(-(1 as core::ffi::c_int));
    } else {
        size = atoi(*argv.offset(1 as core::ffi::c_int as isize));
        fp = fopen(
            *argv.offset(2 as core::ffi::c_int as isize),
            b"r\0" as *const u8 as *const core::ffi::c_char,
        ) as *mut FILE;
        array = malloc(
            (size as size_t)
                .wrapping_mul(::core::mem::size_of::<myStringStruct>() as size_t),
        ) as *mut myStringStruct;
        while fscanf(
            fp,
            b"%s\0" as *const u8 as *const core::ffi::c_char,
            &mut (*array.offset(count as isize)).qstring as *mut [core::ffi::c_char; 128],
        ) == 1 as core::ffi::c_int && count < size
        {
            count += 1;
        }
    }
    printf(
        b"\nSorting %d elements.\n\n\0" as *const u8 as *const core::ffi::c_char,
        count,
    );
    qsort(
        array as *mut core::ffi::c_void,
        count as size_t,
        ::core::mem::size_of::<myStringStruct>() as size_t,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    i = 0 as core::ffi::c_int;
    while i < count {
        printf(
            b"%s\n\0" as *const u8 as *const core::ffi::c_char,
            ((*array.offset(i as isize)).qstring).as_mut_ptr(),
        );
        i += 1;
    }
    free(array as *mut core::ffi::c_void);
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
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}

// --- auto-generated manual Default impls (raw-pointer structs) ---
impl Default for Safe_IO_FILE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

extern "C" {
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> core::ffi::c_int;
    fn exit(__status: core::ffi::c_int) -> !;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn tolower(__c: core::ffi::c_int) -> core::ffi::c_int;
    fn toupper(__c: core::ffi::c_int) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type uchar = core::ffi::c_uchar;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const LARGE: core::ffi::c_int = 32767 as core::ffi::c_int;
static mut patlen: core::ffi::c_int = 0;
static mut skip: [core::ffi::c_int; 256] = [0; 256];
static mut skip2: core::ffi::c_int = 0;
static mut pat: *mut uchar = 0 as *const uchar as *mut uchar;
#[no_mangle]
pub unsafe extern "C" fn bmhi_init(mut pattern: *const core::ffi::c_char) {
    let mut i: core::ffi::c_int = 0;
    let mut lastpatchar: core::ffi::c_int = 0;
    patlen = strlen(pattern) as core::ffi::c_int;
    pat = realloc(pat as *mut core::ffi::c_void, patlen as size_t) as *mut uchar;
    if pat.is_null() {
        exit(1 as core::ffi::c_int);
    } else {
        atexit(Some(bhmi_cleanup as unsafe extern "C" fn() -> ()));
    }
    i = 0 as core::ffi::c_int;
    while i < patlen {
        *pat.offset(i as isize) = toupper(
            *pattern.offset(i as isize) as core::ffi::c_int,
        ) as uchar;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i <= UCHAR_MAX {
        skip[i as usize] = patlen;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < patlen - 1 as core::ffi::c_int {
        skip[*pat.offset(i as isize) as usize] = patlen - i - 1 as core::ffi::c_int;
        skip[tolower(*pat.offset(i as isize) as core::ffi::c_int) as usize] = patlen - i
            - 1 as core::ffi::c_int;
        i += 1;
    }
    lastpatchar = *pat.offset((patlen - 1 as core::ffi::c_int) as isize)
        as core::ffi::c_int;
    skip[lastpatchar as usize] = LARGE;
    skip[tolower(lastpatchar) as usize] = LARGE;
    skip2 = patlen;
    i = 0 as core::ffi::c_int;
    while i < patlen - 1 as core::ffi::c_int {
        if *pat.offset(i as isize) as core::ffi::c_int == lastpatchar {
            skip2 = patlen - i - 1 as core::ffi::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bhmi_cleanup() {
    free(pat as *mut core::ffi::c_void);
}
pub const __SCHAR_MAX__: core::ffi::c_int = 127 as core::ffi::c_int;
pub const UCHAR_MAX: core::ffi::c_int = __SCHAR_MAX__ * 2 as core::ffi::c_int
    + 1 as core::ffi::c_int;

extern "C" {
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type uchar = core::ffi::c_uchar;
pub const LARGE: core::ffi::c_int = 32767 as core::ffi::c_int;
static mut patlen: core::ffi::c_int = 0;
static mut skip: [core::ffi::c_int; 256] = [0; 256];
static mut skip2: core::ffi::c_int = 0;
static mut pat: *mut uchar = 0 as *const uchar as *mut uchar;
#[no_mangle]
pub unsafe extern "C" fn bmh_init(mut pattern: *const core::ffi::c_char) {
    let mut i: core::ffi::c_int = 0;
    let mut lastpatchar: core::ffi::c_int = 0;
    pat = pattern as *mut uchar;
    patlen = strlen(pattern) as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i <= UCHAR_MAX {
        skip[i as usize] = patlen;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < patlen {
        skip[*pat.offset(i as isize) as usize] = patlen - i - 1 as core::ffi::c_int;
        i += 1;
    }
    lastpatchar = *pat.offset((patlen - 1 as core::ffi::c_int) as isize)
        as core::ffi::c_int;
    skip[lastpatchar as usize] = LARGE;
    skip2 = patlen;
    i = 0 as core::ffi::c_int;
    while i < patlen - 1 as core::ffi::c_int {
        if *pat.offset(i as isize) as core::ffi::c_int == lastpatchar {
            skip2 = patlen - i - 1 as core::ffi::c_int;
        }
        i += 1;
    }
}
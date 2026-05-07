extern "C" {
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
static mut table: [size_t; 256] = [0; 256];
static mut len: size_t = 0;
static mut findme: *mut core::ffi::c_char = 0 as *const core::ffi::c_char
    as *mut core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn init_search(mut string: *const core::ffi::c_char) {
    let mut i: size_t = 0;
    len = strlen(string);
    i = 0 as size_t;
    while i <= UCHAR_MAX as size_t {
        table[i as usize] = len;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < len {
        table[*string.offset(i as isize) as core::ffi::c_uchar as usize] = len
            .wrapping_sub(i)
            .wrapping_sub(1 as size_t);
        i = i.wrapping_add(1);
    }
    findme = string as *mut core::ffi::c_char;
}
pub const __SCHAR_MAX__: core::ffi::c_int = 127 as core::ffi::c_int;
pub const UCHAR_MAX: core::ffi::c_int = __SCHAR_MAX__ * 2 as core::ffi::c_int
    + 1 as core::ffi::c_int;

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn safe_print(msg: *const core::ffi::c_char);
    fn safe_copy(
        dest: *mut core::ffi::c_char,
        dest_size: size_t,
        src: *const core::ffi::c_char,
    );
    fn safe_add(
        a: core::ffi::c_int,
        b: core::ffi::c_int,
        out: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn unsafe_concat(
    mut dest: *mut core::ffi::c_char,
    mut src: *const core::ffi::c_char,
) {
    strcat(dest, src);
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut buffer: [core::ffi::c_char; 16] = [0; 16];
    let mut sum: core::ffi::c_int = 0;
    safe_print(b"Start example2\0" as *const u8 as *const core::ffi::c_char);
    safe_copy(
        buffer.as_mut_ptr(),
        ::core::mem::size_of::<[core::ffi::c_char; 16]>() as size_t,
        b"Hello\0" as *const u8 as *const core::ffi::c_char,
    );
    unsafe_concat(
        buffer.as_mut_ptr(),
        b"World!!!\0" as *const u8 as *const core::ffi::c_char,
    );
    safe_print(buffer.as_mut_ptr());
    if safe_add(100 as core::ffi::c_int, 23 as core::ffi::c_int, &mut sum) != 0 {
        printf(b"Sum: %d\n\0" as *const u8 as *const core::ffi::c_char, sum);
    } else {
        printf(b"Overflow detected\n\0" as *const u8 as *const core::ffi::c_char);
    }
    return 0 as core::ffi::c_int;
}
pub fn c_main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

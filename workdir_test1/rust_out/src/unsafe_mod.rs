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
    fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn safe_print(text: *const core::ffi::c_char);
    fn safe_copy(
        dest: *mut core::ffi::c_char,
        dest_size: size_t,
        src: *const core::ffi::c_char,
    );
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn unsafe_copy(
    mut dest: *mut core::ffi::c_char,
    mut src: *const core::ffi::c_char,
) {
    strcpy(dest, src);
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut buffer1: [core::ffi::c_char; 10] = [0; 10];
    let mut buffer2: [core::ffi::c_char; 10] = [0; 10];
    safe_print(b"Hallo\0" as *const u8 as *const core::ffi::c_char);
    safe_copy(
        buffer1.as_mut_ptr(),
        ::core::mem::size_of::<[core::ffi::c_char; 10]>() as size_t,
        b"HalloWelt123\0" as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"buffer1: %s\n\0" as *const u8 as *const core::ffi::c_char,
        buffer1.as_mut_ptr(),
    );
    unsafe_copy(
        buffer2.as_mut_ptr(),
        b"HalloWelt123\0" as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"buffer2: %s\n\0" as *const u8 as *const core::ffi::c_char,
        buffer2.as_mut_ptr(),
    );
    return 0 as core::ffi::c_int;
}
pub fn c_main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

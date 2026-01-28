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
}
#[no_mangle]
pub unsafe extern "C" fn add(
    mut a: core::ffi::c_int,
    mut b: core::ffi::c_int,
) -> core::ffi::c_int {
    return a + b;
}
unsafe fn main_0() -> core::ffi::c_int {
    printf(
        b"%d\n\0" as *const u8 as *const core::ffi::c_char,
        add(2 as core::ffi::c_int, 3 as core::ffi::c_int),
    );
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

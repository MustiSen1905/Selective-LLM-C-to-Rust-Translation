// ===== FILE: src/example.rs =====
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {







unsafe extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
pub fn add(
    a: core::ffi::c_int,
    b: core::ffi::c_int,
) -> core::ffi::c_int;
fn main_0() -> core::ffi::c_int;
pub fn main();
}

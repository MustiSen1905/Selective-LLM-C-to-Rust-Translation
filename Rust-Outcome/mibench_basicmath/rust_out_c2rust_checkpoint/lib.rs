#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]


extern crate f128;#[macro_use]
extern crate num_traits;
pub mod src {
pub mod basicmath;
pub mod cubic;
pub mod isqrt;
pub mod rad2deg;
} // mod src

/// C-compatible entry point exported as `main` for hybrid/c2rust linking.
#[no_mangle]
pub unsafe extern "C" fn main(
    argc: core::ffi::c_int,
    argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    src::basicmath::main_0(argc, argv)
}

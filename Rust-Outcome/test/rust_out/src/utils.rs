extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn process_data(mut data: *mut core::ffi::c_char) {
    printf(b"Verarbeite Daten: \0" as *const u8 as *const core::ffi::c_char);
    printf(data);
    printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
    let mut leak: *mut core::ffi::c_int = malloc(
        (100 as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) as *mut core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 100 as core::ffi::c_int {
        *leak.offset(i as isize) = i;
        i += 1;
    }
}

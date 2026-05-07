extern "C" {
    fn atan(__x: core::ffi::c_double) -> core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rad2deg(mut rad: core::ffi::c_double) -> core::ffi::c_double {
    return 180.0f64 * rad
        / (4 as core::ffi::c_int as core::ffi::c_double
            * atan(1 as core::ffi::c_int as core::ffi::c_double));
}
#[no_mangle]
pub unsafe extern "C" fn deg2rad(mut deg: core::ffi::c_double) -> core::ffi::c_double {
    PI * deg / 180.0
}
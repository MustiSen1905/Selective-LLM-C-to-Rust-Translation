#[no_mangle]
pub unsafe extern "C" fn helper_mul(
    mut a: core::ffi::c_int,
    mut b: core::ffi::c_int,
) -> core::ffi::c_int {
    return a * b;
}
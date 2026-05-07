#[no_mangle]
pub unsafe extern "C" fn bit_count(mut x: core::ffi::c_long) -> core::ffi::c_int {
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    if x != 0 {
        loop {
            n += 1;
            x = x & x - 1 as core::ffi::c_long;
            if !(0 as core::ffi::c_long != x) {
                break;
            }
        }
    }
    return n;
}

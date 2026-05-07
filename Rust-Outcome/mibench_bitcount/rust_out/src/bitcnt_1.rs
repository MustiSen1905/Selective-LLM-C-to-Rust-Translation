#[no_mangle]
pub unsafe extern "C" fn bit_count(mut x: core::ffi::c_long) -> core::ffi::c_int {
    let mut n = 0;
    if x != 0 {
        loop {
            n += 1;
            x = x & (x - 1);
            if 0 == x {
                break;
            }
        }
    }
    n
}
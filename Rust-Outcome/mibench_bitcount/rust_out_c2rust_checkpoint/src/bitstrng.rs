#[no_mangle]
pub unsafe extern "C" fn bitstring(
    mut str: *mut core::ffi::c_char,
    mut byze: core::ffi::c_long,
    mut biz: core::ffi::c_int,
    mut strwid: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    j = strwid
        - (biz + (biz >> 2 as core::ffi::c_int)
            - (if biz % 4 as core::ffi::c_int != 0 {
                0 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            }));
    i = 0 as core::ffi::c_int;
    while i < j {
        let fresh0 = str;
        str = str.offset(1);
        *fresh0 = ' ' as i32 as core::ffi::c_char;
        i += 1;
    }
    loop {
        biz -= 1;
        if !(biz >= 0 as core::ffi::c_int) {
            break;
        }
        let fresh1 = str;
        str = str.offset(1);
        *fresh1 = ((byze >> biz & 1 as core::ffi::c_long)
            + '0' as i32 as core::ffi::c_long) as core::ffi::c_char;
        if biz % 4 as core::ffi::c_int == 0 && biz != 0 {
            let fresh2 = str;
            str = str.offset(1);
            *fresh2 = ' ' as i32 as core::ffi::c_char;
        }
    }
    *str = '\0' as i32 as core::ffi::c_char;
}

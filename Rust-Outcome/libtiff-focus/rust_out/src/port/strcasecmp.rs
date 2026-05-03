extern "C" {
    fn tolower(__c: core::ffi::c_int) -> core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strcasecmp(
    mut s1: *const core::ffi::c_char,
    mut s2: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut us1: *const core::ffi::c_uchar = s1 as *const core::ffi::c_uchar;
    let mut us2: *const core::ffi::c_uchar = s2 as *const core::ffi::c_uchar;
    loop {
        let fresh0 = us2;
        us2 = us2.offset(1);
        if !(tolower(*us1 as core::ffi::c_int) == tolower(*fresh0 as core::ffi::c_int)) {
            break;
        }
        let fresh1 = us1;
        us1 = us1.offset(1);
        if *fresh1 as core::ffi::c_int == '\0' as i32 {
            return 0 as core::ffi::c_int;
        }
    }
    us2 = us2.offset(-1);
    return tolower(*us1 as core::ffi::c_int) - tolower(*us2 as core::ffi::c_int);
}

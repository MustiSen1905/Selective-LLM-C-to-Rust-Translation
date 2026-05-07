extern "C" {
    fn strchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_i(mut cptr: *mut core::ffi::c_char) -> core::ffi::c_uint {
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while !cptr.is_null() && *cptr as core::ffi::c_int != 0
        && !(strchr(
            b"01\0" as *const u8 as *const core::ffi::c_char,
            *cptr as core::ffi::c_int,
        ))
            .is_null()
    {
        let fresh0 = cptr;
        cptr = cptr.offset(1);
        i = (*fresh0 as core::ffi::c_int - '0' as i32) as core::ffi::c_uint;
        j <<= 1 as core::ffi::c_int;
        j |= i & 0x1 as core::ffi::c_uint;
    }
    return j;
}

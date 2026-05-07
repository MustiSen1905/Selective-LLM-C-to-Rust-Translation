extern "C" {
    fn strchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_i(mut cptr: *mut core::ffi::c_char) -> core::ffi::c_uint {
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0;
    
    while !cptr.is_null() && *cptr as core::ffi::c_int != 0  {
        let fresh0 = cptr;
        cptr = cptr.offset(1);
        i = (*fresh0 as core::ffi::c_int - '0' as i32) as core::ffi::c_uint;
        j <<= 1;
        j |= (i & 0x01);
    }
    
    return j;
}
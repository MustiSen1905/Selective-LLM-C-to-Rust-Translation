extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int_sqrt {
    pub sqrt: core::ffi::c_uint,
    pub frac: core::ffi::c_uint,
}
pub const BITSPERLONG: core::ffi::c_int = 32 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn usqrt(mut x: core::ffi::c_ulong, mut q: *mut int_sqrt) {
    let mut a: core::ffi::c_ulong = 0 as core::ffi::c_ulong;
    let mut r: core::ffi::c_ulong = 0 as core::ffi::c_ulong;
    let mut e: core::ffi::c_ulong = 0 as core::ffi::c_ulong;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < BITSPERLONG {
        r = (r << 2 as core::ffi::c_int)
            .wrapping_add(
                (x
                    & ((3 as core::ffi::c_long) << BITSPERLONG - 2 as core::ffi::c_int)
                        as core::ffi::c_ulong) >> BITSPERLONG - 2 as core::ffi::c_int,
            );
        x <<= 2 as core::ffi::c_int;
        a <<= 1 as core::ffi::c_int;
        e = (a << 1 as core::ffi::c_int).wrapping_add(1 as core::ffi::c_ulong);
        if r >= e {
            r = r.wrapping_sub(e);
            a = a.wrapping_add(1);
        }
        i += 1;
    }
    memcpy(
        q as *mut core::ffi::c_void,
        &mut a as *mut core::ffi::c_ulong as *const core::ffi::c_void,
        ::core::mem::size_of::<core::ffi::c_long>() as size_t,
    );
}

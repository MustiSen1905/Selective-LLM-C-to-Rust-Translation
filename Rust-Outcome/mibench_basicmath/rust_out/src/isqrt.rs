#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SafeIntSqrt {
    pub sqrt: u32,
    pub frac: u32,
}

impl SafeIntSqrt {
    pub unsafe fn from_ptr(ptr: *const int_sqrt) -> Self {
        if ptr.is_null() {
            return SafeIntSqrt {
                sqrt: 0,
                frac: 0,
            };
        }
        
        let orig = &*ptr;

        SafeIntSqrt {
            sqrt: orig.sqrt as u32,
            frac: orig.frac as u32,
        }
    }
}

extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;

pub const BITSPERLONG: core::ffi::c_int = 32 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn usqrt(mut x: core::ffi::c_ulong, mut q: *mut int_sqrt) {
    let mut q_ptr = q;

    let mut a: core::ffi::c_ulong = 0;
    let mut r: core::ffi::c_ulong = 0;
    let mut e: core::ffi::c_ulong = 0;
    let mut i: core::ffi::c_int = 0;
    
    while i < BITSPERLONG {
        r = (r << 2) + ((x & ((3 as core::ffi::c_long) << (BITSPERLONG - 2))) >> (BITSPERLONG - 2));
        x <<= 2;
        a <<= 1;
        e = (a << 1) + 1;
        
        if r >= e {
            r -= e as usize;
            a += 1;
        }
        
        i += 1;
    }
    
    let q: &mut SafeIntSqrt = unsafe { &mut *q_ptr };
    q.sqrt = a;
}
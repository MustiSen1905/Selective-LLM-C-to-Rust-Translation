extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(__status: core::ffi::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sin(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn IsPowerOfTwo(x: core::ffi::c_uint) -> core::ffi::c_int;
    fn NumberOfBitsNeeded(PowerOfTwo: core::ffi::c_uint) -> core::ffi::c_uint;
    fn ReverseBits(
        index: core::ffi::c_uint,
        NumBits: core::ffi::c_uint,
    ) -> core::ffi::c_uint;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub const DDC_PI: core::ffi::c_double = 3.14159265358979323846f64;
#[no_mangle]
pub unsafe extern "C" fn CheckPointer(
    mut p: *mut core::ffi::c_void,
    mut name: *mut core::ffi::c_char,
) {
    if p.is_null() {
        fprintf(
            stderr,
            b"Error in fft_float():  %s == NULL\n\0" as *const u8
                as *const core::ffi::c_char,
            name,
        );
        exit(1 as core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fft_float(
    mut NumSamples: core::ffi::c_uint,
    mut InverseTransform: core::ffi::c_int,
    mut RealIn: *mut core::ffi::c_float,
    mut ImagIn: *mut core::ffi::c_float,
    mut RealOut: *mut core::ffi::c_float,
    mut ImagOut: *mut core::ffi::c_float,
) {
    let mut NumBits: core::ffi::c_uint = 0;
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0;
    let mut k: core::ffi::c_uint = 0;
    let mut n: core::ffi::c_uint = 0;
    let mut BlockSize: core::ffi::c_uint = 0;
    let mut BlockEnd: core::ffi::c_uint = 0;
    let mut angle_numerator: core::ffi::c_double = 2.0f64 * DDC_PI;
    let mut tr: core::ffi::c_double = 0.;
    let mut ti: core::ffi::c_double = 0.;
    if IsPowerOfTwo(NumSamples) == 0 {
        fprintf(
            stderr,
            b"Error in fft():  NumSamples=%u is not power of two\n\0" as *const u8
                as *const core::ffi::c_char,
            NumSamples,
        );
        exit(1 as core::ffi::c_int);
    }
    if InverseTransform != 0 {
        angle_numerator = -angle_numerator;
    }
    CheckPointer(
        RealIn as *mut core::ffi::c_void,
        b"RealIn\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    );
    CheckPointer(
        RealOut as *mut core::ffi::c_void,
        b"RealOut\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    );
    CheckPointer(
        ImagOut as *mut core::ffi::c_void,
        b"ImagOut\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    );
    NumBits = NumberOfBitsNeeded(NumSamples);
    i = 0 as core::ffi::c_uint;
    while i < NumSamples {
        j = ReverseBits(i, NumBits);
        *RealOut.offset(j as isize) = *RealIn.offset(i as isize);
        *ImagOut.offset(j as isize) = (if ImagIn.is_null() {
            0.0f64
        } else {
            *ImagIn.offset(i as isize) as core::ffi::c_double
        }) as core::ffi::c_float;
        i = i.wrapping_add(1);
    }
    BlockEnd = 1 as core::ffi::c_uint;
    BlockSize = 2 as core::ffi::c_uint;
    while BlockSize <= NumSamples {
        let mut delta_angle: core::ffi::c_double = angle_numerator
            / BlockSize as core::ffi::c_double;
        let mut sm2: core::ffi::c_double = sin(
            -(2 as core::ffi::c_int) as core::ffi::c_double * delta_angle,
        );
        let mut sm1: core::ffi::c_double = sin(-delta_angle);
        let mut cm2: core::ffi::c_double = cos(
            -(2 as core::ffi::c_int) as core::ffi::c_double * delta_angle,
        );
        let mut cm1: core::ffi::c_double = cos(-delta_angle);
        let mut w: core::ffi::c_double = 2 as core::ffi::c_int as core::ffi::c_double
            * cm1;
        let mut ar: [core::ffi::c_double; 3] = [0.; 3];
        let mut ai: [core::ffi::c_double; 3] = [0.; 3];
        let mut temp: core::ffi::c_double = 0.;
        i = 0 as core::ffi::c_uint;
        while i < NumSamples {
            ar[2 as core::ffi::c_int as usize] = cm2;
            ar[1 as core::ffi::c_int as usize] = cm1;
            ai[2 as core::ffi::c_int as usize] = sm2;
            ai[1 as core::ffi::c_int as usize] = sm1;
            j = i;
            n = 0 as core::ffi::c_uint;
            while n < BlockEnd {
                ar[0 as core::ffi::c_int as usize] = w
                    * ar[1 as core::ffi::c_int as usize]
                    - ar[2 as core::ffi::c_int as usize];
                ar[2 as core::ffi::c_int as usize] = ar[1 as core::ffi::c_int as usize];
                ar[1 as core::ffi::c_int as usize] = ar[0 as core::ffi::c_int as usize];
                ai[0 as core::ffi::c_int as usize] = w
                    * ai[1 as core::ffi::c_int as usize]
                    - ai[2 as core::ffi::c_int as usize];
                ai[2 as core::ffi::c_int as usize] = ai[1 as core::ffi::c_int as usize];
                ai[1 as core::ffi::c_int as usize] = ai[0 as core::ffi::c_int as usize];
                k = j.wrapping_add(BlockEnd);
                tr = ar[0 as core::ffi::c_int as usize]
                    * *RealOut.offset(k as isize) as core::ffi::c_double
                    - ai[0 as core::ffi::c_int as usize]
                        * *ImagOut.offset(k as isize) as core::ffi::c_double;
                ti = ar[0 as core::ffi::c_int as usize]
                    * *ImagOut.offset(k as isize) as core::ffi::c_double
                    + ai[0 as core::ffi::c_int as usize]
                        * *RealOut.offset(k as isize) as core::ffi::c_double;
                *RealOut.offset(k as isize) = (*RealOut.offset(j as isize)
                    as core::ffi::c_double - tr) as core::ffi::c_float;
                *ImagOut.offset(k as isize) = (*ImagOut.offset(j as isize)
                    as core::ffi::c_double - ti) as core::ffi::c_float;
                let ref mut fresh0 = *RealOut.offset(j as isize);
                *fresh0 = (*fresh0 as core::ffi::c_double + tr) as core::ffi::c_float;
                let ref mut fresh1 = *ImagOut.offset(j as isize);
                *fresh1 = (*fresh1 as core::ffi::c_double + ti) as core::ffi::c_float;
                j = j.wrapping_add(1);
                n = n.wrapping_add(1);
            }
            i = i.wrapping_add(BlockSize);
        }
        BlockEnd = BlockSize;
        BlockSize <<= 1 as core::ffi::c_int;
    }
    if InverseTransform != 0 {
        let mut denom: core::ffi::c_double = NumSamples as core::ffi::c_double;
        i = 0 as core::ffi::c_uint;
        while i < NumSamples {
            let ref mut fresh2 = *RealOut.offset(i as isize);
            *fresh2 = (*fresh2 as core::ffi::c_double / denom) as core::ffi::c_float;
            let ref mut fresh3 = *ImagOut.offset(i as isize);
            *fresh3 = (*fresh3 as core::ffi::c_double / denom) as core::ffi::c_float;
            i = i.wrapping_add(1);
        }
    }
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;

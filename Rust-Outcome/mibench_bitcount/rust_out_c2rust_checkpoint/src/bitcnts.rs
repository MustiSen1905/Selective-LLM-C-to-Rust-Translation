extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn puts(__s: *const core::ffi::c_char) -> core::ffi::c_int;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn rand() -> core::ffi::c_int;
    fn exit(__status: core::ffi::c_int) -> !;
    fn clock() -> clock_t;
    fn bit_count(x: core::ffi::c_long) -> core::ffi::c_int;
    fn bitcount(i: core::ffi::c_long) -> core::ffi::c_int;
    fn ntbl_bitcount(x: core::ffi::c_long) -> core::ffi::c_int;
    fn BW_btbl_bitcount(x: core::ffi::c_long) -> core::ffi::c_int;
    fn AR_btbl_bitcount(x: core::ffi::c_long) -> core::ffi::c_int;
    fn ntbl_bitcnt(x: core::ffi::c_long) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type __clock_t = core::ffi::c_long;
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
pub type clock_t = __clock_t;
pub const CHAR_BIT: core::ffi::c_int = __CHAR_BIT__;
pub const CLOCKS_PER_SEC: __clock_t = 1000000 as core::ffi::c_int as __clock_t;
pub const DBL_MAX: core::ffi::c_double = __DBL_MAX__;
pub const FUNCS: core::ffi::c_int = 7 as core::ffi::c_int;
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut start: clock_t = 0;
    let mut stop: clock_t = 0;
    let mut ct: core::ffi::c_double = 0.;
    let mut cmin: core::ffi::c_double = DBL_MAX;
    let mut cmax: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let mut i: core::ffi::c_int = 0;
    let mut cminix: core::ffi::c_int = 0;
    let mut cmaxix: core::ffi::c_int = 0;
    let mut j: core::ffi::c_long = 0;
    let mut n: core::ffi::c_long = 0;
    let mut seed: core::ffi::c_long = 0;
    let mut iterations: core::ffi::c_int = 0;
    static mut pBitCntFunc: [Option<
        unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
    >; 7] = unsafe {
        [
            Some(
                bit_count as unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
            ),
            Some(
                bitcount as unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
            ),
            Some(
                ntbl_bitcnt
                    as unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
            ),
            Some(
                ntbl_bitcount
                    as unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
            ),
            Some(
                BW_btbl_bitcount
                    as unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
            ),
            Some(
                AR_btbl_bitcount
                    as unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
            ),
            Some(
                bit_shifter
                    as unsafe extern "C" fn(core::ffi::c_long) -> core::ffi::c_int,
            ),
        ]
    };
    static mut text: [*mut core::ffi::c_char; 7] = [
        b"Optimized 1 bit/loop counter\0" as *const u8 as *const core::ffi::c_char
            as *mut core::ffi::c_char,
        b"Ratko's mystery algorithm\0" as *const u8 as *const core::ffi::c_char
            as *mut core::ffi::c_char,
        b"Recursive bit count by nybbles\0" as *const u8 as *const core::ffi::c_char
            as *mut core::ffi::c_char,
        b"Non-recursive bit count by nybbles\0" as *const u8 as *const core::ffi::c_char
            as *mut core::ffi::c_char,
        b"Non-recursive bit count by bytes (BW)\0" as *const u8
            as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"Non-recursive bit count by bytes (AR)\0" as *const u8
            as *const core::ffi::c_char as *mut core::ffi::c_char,
        b"Shift and count bits\0" as *const u8 as *const core::ffi::c_char
            as *mut core::ffi::c_char,
    ];
    if argc < 2 as core::ffi::c_int {
        fprintf(
            stderr,
            b"Usage: bitcnts <iterations>\n\0" as *const u8 as *const core::ffi::c_char,
        );
        exit(-(1 as core::ffi::c_int));
    }
    iterations = atoi(*argv.offset(1 as core::ffi::c_int as isize));
    puts(
        b"Bit counter algorithm benchmark\n\0" as *const u8 as *const core::ffi::c_char,
    );
    i = 0 as core::ffi::c_int;
    while i < FUNCS {
        start = clock();
        n = 0 as core::ffi::c_long;
        j = n;
        seed = rand() as core::ffi::c_long;
        while j < iterations as core::ffi::c_long {
            n
                += (pBitCntFunc[i as usize]).expect("non-null function pointer")(seed)
                    as core::ffi::c_long;
            j += 1;
            seed += 13 as core::ffi::c_long;
        }
        stop = clock();
        ct = (stop - start) as core::ffi::c_double
            / CLOCKS_PER_SEC as core::ffi::c_double;
        if ct < cmin {
            cmin = ct;
            cminix = i;
        }
        if ct > cmax {
            cmax = ct;
            cmaxix = i;
        }
        printf(
            b"%-38s> Time: %7.3f sec.; Bits: %ld\n\0" as *const u8
                as *const core::ffi::c_char,
            text[i as usize],
            ct,
            n,
        );
        i += 1;
    }
    printf(
        b"\nBest  > %s\n\0" as *const u8 as *const core::ffi::c_char,
        text[cminix as usize],
    );
    printf(
        b"Worst > %s\n\0" as *const u8 as *const core::ffi::c_char,
        text[cmaxix as usize],
    );
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn bit_shifter(mut x: core::ffi::c_long) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    n = 0 as core::ffi::c_int;
    i = n;
    while x != 0
        && (i as usize)
            < (::core::mem::size_of::<core::ffi::c_long>() as usize)
                .wrapping_mul(CHAR_BIT as usize)
    {
        n += (x & 1 as core::ffi::c_long) as core::ffi::c_int;
        i += 1;
        x >>= 1 as core::ffi::c_int;
    }
    return n;
}
pub const __CHAR_BIT__: core::ffi::c_int = 8 as core::ffi::c_int;
pub const __DBL_MAX__: core::ffi::c_double = 1.7976931348623157e+308f64;
pub fn main() {
    let mut args: Vec<*mut core::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}

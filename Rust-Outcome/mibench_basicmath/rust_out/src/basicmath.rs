// --- auto-generated Safe stub types ---
pub type Safe_IO_codecvt = ();
pub type Safe_IO_marker = ();
pub type Safe_IO_wide_data = ();
// --- end stubs ---

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Safeint_sqrt {
    pub sqrt: u32,
    pub frac: u32,
}

impl Safeint_sqrt {

}

#[derive(Debug, Clone)]
pub struct Safe_IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: String,
    pub _IO_read_end: String,
    pub _IO_read_base: String,
    pub _IO_write_base: String,
    pub _IO_write_ptr: String,
    pub _IO_write_end: String,
    pub _IO_buf_base: String,
    pub _IO_buf_end: String,
    pub _IO_save_base: String,
    pub _IO_backup_base: String,
    pub _IO_save_end: String,
    pub _markers: Box<Safe_IO_marker>,
    pub _chain: Box<Safe_IO_FILE>,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: isize,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: String,
    pub _lock: *const core::ffi::c_void,
    pub _offset: isize,
    pub _codecvt: Box<Safe_IO_codecvt>,
    pub _wide_data: Box<Safe_IO_wide_data>,
    pub _freeres_list: Box<Safe_IO_FILE>,
    pub _freeres_buf: *const core::ffi::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: String,
}

impl Safe_IO_FILE {
    unsafe fn from_ptr(_ptr: *const _IO_FILE) -> Self {
        unimplemented!("from_ptr stub")
    }
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atan(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn SolveCubic(
        a: core::ffi::c_double,
        b: core::ffi::c_double,
        c: core::ffi::c_double,
        d: core::ffi::c_double,
        solutions: *mut core::ffi::c_int,
        x: *mut core::ffi::c_double,
    );
    fn usqrt(x: core::ffi::c_ulong, q: *mut int_sqrt);
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn puts(__s: *const core::ffi::c_char) -> core::ffi::c_int;
}
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int_sqrt {
    pub sqrt: core::ffi::c_uint,
    pub frac: core::ffi::c_uint,
}
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
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    if argc < 2 as core::ffi::c_int {
        fprintf(
            stderr,
            b"ERROR: number of runs not provided!\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"USAGE: ./basicmath [RUNS]\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 1 as core::ffi::c_int;
    }
    let mut runs: core::ffi::c_int = atoi(*argv.offset(1 as core::ffi::c_int as isize));
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < runs {
        let mut a1: core::ffi::c_double = 1.0f64;
        let mut b1: core::ffi::c_double = -10.5f64;
        let mut c1: core::ffi::c_double = 32.0f64;
        let mut d1: core::ffi::c_double = -30.0f64;
        let mut x: [core::ffi::c_double; 3] = [0.; 3];
        let mut X: core::ffi::c_double = 0.;
        let mut solutions: core::ffi::c_int = 0;
        let mut i: core::ffi::c_int = 0;
        let mut l: core::ffi::c_ulong = 0x3fed0169 as core::ffi::c_ulong;
        let mut q: int_sqrt = int_sqrt { sqrt: 0, frac: 0 };
        let mut n: core::ffi::c_long = 0 as core::ffi::c_long;
        printf(
            b"********* CUBIC FUNCTIONS ***********\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = 1.0f64;
        b1 = -4.5f64;
        c1 = 17.0f64;
        d1 = -30.0f64;
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = 1.0f64;
        b1 = -3.5f64;
        c1 = 22.0f64;
        d1 = -31.0f64;
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = 1.0f64;
        b1 = -13.7f64;
        c1 = 1.0f64;
        d1 = -35.0f64;
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = 3.0f64;
        b1 = 12.34f64;
        c1 = 5.0f64;
        d1 = 12.0f64;
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = -8.0f64;
        b1 = -67.89f64;
        c1 = 6.0f64;
        d1 = -23.6f64;
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = 45.0f64;
        b1 = 8.67f64;
        c1 = 7.5f64;
        d1 = 34.0f64;
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = -12.0f64;
        b1 = -1.7f64;
        c1 = 5.3f64;
        d1 = 16.0f64;
        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
        i = 0 as core::ffi::c_int;
        while i < solutions {
            printf(b" %f\0" as *const u8 as *const core::ffi::c_char, x[i as usize]);
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        a1 = 1 as core::ffi::c_int as core::ffi::c_double;
        while a1 < 10 as core::ffi::c_int as core::ffi::c_double {
            b1 = 10 as core::ffi::c_int as core::ffi::c_double;
            while b1 > 0 as core::ffi::c_int as core::ffi::c_double {
                c1 = 5 as core::ffi::c_int as core::ffi::c_double;
                while c1 < 15 as core::ffi::c_int as core::ffi::c_double {
                    d1 = -(1 as core::ffi::c_int) as core::ffi::c_double;
                    while d1 > -(5 as core::ffi::c_int) as core::ffi::c_double {
                        SolveCubic(a1, b1, c1, d1, &mut solutions, x.as_mut_ptr());
                        printf(b"Solutions:\0" as *const u8 as *const core::ffi::c_char);
                        i = 0 as core::ffi::c_int;
                        while i < solutions {
                            printf(
                                b" %f\0" as *const u8 as *const core::ffi::c_char,
                                x[i as usize],
                            );
                            i += 1;
                        }
                        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
                        d1 -= 0.451f64;
                    }
                    c1 += 0.61f64;
                }
                b1 -= 0.25f64;
            }
            a1 += 1 as core::ffi::c_int as core::ffi::c_double;
        }
        printf(
            b"********* INTEGER SQR ROOTS ***********\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        i = 0 as core::ffi::c_int;
        while i < 100000 as core::ffi::c_int {
            usqrt(i as core::ffi::c_ulong, &mut q);
            printf(
                b"sqrt(%3d) = %2d\n\0" as *const u8 as *const core::ffi::c_char,
                i,
                q.sqrt,
            );
            i += 2 as core::ffi::c_int;
        }
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
        l = 0x3fed0169 as core::ffi::c_ulong;
        while l < 0x3fed4169 as core::ffi::c_ulong {
            usqrt(l, &mut q);
            printf(
                b"sqrt(%lX) = %X\n\0" as *const u8 as *const core::ffi::c_char,
                l,
                q.sqrt,
            );
            l = l.wrapping_add(1);
        }
        printf(
            b"********* ANGLE CONVERSION ***********\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        X = 0.0f64;
        while X <= 360.0f64 {
            printf(
                b"%3.0f degrees = %.12f radians\n\0" as *const u8
                    as *const core::ffi::c_char,
                X,
                X
                    * (4 as core::ffi::c_int as core::ffi::c_double
                        * atan(1 as core::ffi::c_int as core::ffi::c_double))
                    / 180 as core::ffi::c_int as core::ffi::c_double,
            );
            X += 0.001f64;
        }
        puts(b"\0" as *const u8 as *const core::ffi::c_char);
        X = 0.0f64;
        while X
            <= 2 as core::ffi::c_int as core::ffi::c_double
                * (4 as core::ffi::c_int as core::ffi::c_double
                    * atan(1 as core::ffi::c_int as core::ffi::c_double)) + 1e-6f64
        {
            printf(
                b"%.12f radians = %3.0f degrees\n\0" as *const u8
                    as *const core::ffi::c_char,
                X,
                X * 180 as core::ffi::c_int as core::ffi::c_double
                    / (4 as core::ffi::c_int as core::ffi::c_double
                        * atan(1 as core::ffi::c_int as core::ffi::c_double)),
            );
            X
                += 4 as core::ffi::c_int as core::ffi::c_double
                    * atan(1 as core::ffi::c_int as core::ffi::c_double)
                    / 5760 as core::ffi::c_int as core::ffi::c_double;
        }
        j += 1;
    }
    return 0 as core::ffi::c_int;
}
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

// --- auto-generated manual Default impls (raw-pointer structs) ---
impl Default for Safe_IO_FILE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

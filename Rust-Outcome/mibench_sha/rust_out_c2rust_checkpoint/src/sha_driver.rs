extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn sha_stream(_: *mut SHA_INFO, _: *mut FILE);
    fn sha_print(_: *mut SHA_INFO);
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
pub type LONG = core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA_INFO {
    pub digest: [LONG; 5],
    pub count_lo: LONG,
    pub count_hi: LONG,
    pub data: [LONG; 16],
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut fin: *mut FILE = 0 as *mut FILE;
    let mut sha_info: SHA_INFO = SHA_INFO {
        digest: [0; 5],
        count_lo: 0,
        count_hi: 0,
        data: [0; 16],
    };
    if argc < 2 as core::ffi::c_int {
        fin = stdin;
        sha_stream(&mut sha_info, fin);
        sha_print(&mut sha_info);
    } else {
        loop {
            argc -= 1;
            if !(argc != 0) {
                break;
            }
            argv = argv.offset(1);
            fin = fopen(*argv, b"rb\0" as *const u8 as *const core::ffi::c_char)
                as *mut FILE;
            if fin.is_null() {
                printf(
                    b"error opening %s for reading\n\0" as *const u8
                        as *const core::ffi::c_char,
                    *argv,
                );
            } else {
                sha_stream(&mut sha_info, fin);
                sha_print(&mut sha_info);
                fclose(fin);
            }
        }
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

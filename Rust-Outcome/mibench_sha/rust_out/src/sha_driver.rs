// --- auto-generated Safe stub types ---
pub type Safe_IO_codecvt = ();
pub type Safe_IO_marker = ();
pub type Safe_IO_wide_data = ();
// --- end stubs ---



impl Safe_IO_FILE {
    pub unsafe fn from_ptr(_ptr: *const _IO_FILE) -> Self { unimplemented!("from_ptr stub") }
}

impl SafeSHA_INFO {

}

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

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type LONG = core::ffi::c_ulong;

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
    args.push(std::ptr::null_mut());
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

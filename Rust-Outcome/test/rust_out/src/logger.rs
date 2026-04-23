#[derive(Debug, Clone)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: Option<String>,
    pub _IO_read_end: Option<String>,
    pub _IO_read_base: Option<String>,
    pub _IO_write_base: Option<String>,
    pub _IO_write_ptr: Option<String>,
    pub _IO_write_end: Option<String>,
    pub _IO_buf_base: Option<String>,
    pub _IO_buf_end: Option<String>,
    pub _IO_save_base: Option<String>,
    pub _IO_backup_base: Option<String>,
    pub _IO_save_end: Option<String>,
    pub _markers: Option<Box<_IO_marker>>,
    pub _chain: Option<Box<_IO_FILE>>,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [u8; 1],
    pub _lock: Option<Box<()>>,
    pub _offset: i64,
    pub _codecvt: Option<Box<_IO_codecvt>>,
    pub _wide_data: Option<Box<_IO_wide_data>>,
    pub _freeres_list: Option<Box<_IO_FILE>>,
    pub _freeres_buf: Option<Box<()>>,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [u8; 20],
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fscanf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn log_message(mut msg: *const core::ffi::c_char) {
    let mut f: *mut FILE = fopen(
        b"system.log\0" as *const u8 as *const core::ffi::c_char,
        b"a\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        return;
    }
    fprintf(f, msg);
    fprintf(f, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn read_log_unsafe(mut user_buffer: *mut core::ffi::c_char) {
    let mut f: *mut FILE = fopen(
        b"system.log\0" as *const u8 as *const core::ffi::c_char,
        b"r\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        return;
    }
    fscanf(f, b"%s\0" as *const u8 as *const core::ffi::c_char, user_buffer);
    fclose(f);
}

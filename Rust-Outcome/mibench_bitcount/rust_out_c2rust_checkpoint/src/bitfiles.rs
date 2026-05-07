extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(__ptr: *mut core::ffi::c_void);
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> core::ffi::c_int;
    fn fputc(__c: core::ffi::c_int, __stream: *mut FILE) -> core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfile {
    pub file: *mut FILE,
    pub rbuf: core::ffi::c_char,
    pub rcnt: core::ffi::c_char,
    pub wbuf: core::ffi::c_char,
    pub wcnt: core::ffi::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn bfread(mut bf: *mut bfile) -> core::ffi::c_int {
    if 0 as core::ffi::c_int == (*bf).rcnt as core::ffi::c_int {
        (*bf).rbuf = fgetc((*bf).file) as core::ffi::c_char;
        (*bf).rcnt = 8 as core::ffi::c_char;
    }
    (*bf).rcnt -= 1;
    return ((*bf).rbuf as core::ffi::c_int
        & (1 as core::ffi::c_int) << (*bf).rcnt as core::ffi::c_int
        != 0 as core::ffi::c_int) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bfwrite(mut bit: core::ffi::c_int, mut bf: *mut bfile) {
    if 8 as core::ffi::c_int == (*bf).wcnt as core::ffi::c_int {
        fputc((*bf).wbuf as core::ffi::c_int, (*bf).file);
        (*bf).wcnt = 0 as core::ffi::c_char;
    }
    (*bf).wcnt += 1;
    (*bf).wbuf = (((*bf).wbuf as core::ffi::c_int) << 1 as core::ffi::c_int)
        as core::ffi::c_char;
    (*bf).wbuf = ((*bf).wbuf as core::ffi::c_int | bit & 1 as core::ffi::c_int)
        as core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bfclose(mut bf: *mut bfile) {
    fclose((*bf).file);
    free(bf as *mut core::ffi::c_void);
}

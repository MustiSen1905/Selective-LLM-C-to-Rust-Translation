// --- auto-generated Safe stub types ---
pub type Safe_IO_codecvt = ();
pub type Safe_IO_marker = ();
pub type Safe_IO_wide_data = ();
// --- end stubs ---



#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Safefile {
    pub file: Box<Safe_IO_FILE>,
    pub rbuf: i8,
    pub rcnt: i8,
    pub wbuf: i8,
    pub wcnt: i8,
}

impl Safefile {

}

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
pub unsafe extern "C" fn bfwrite(bit: core::ffi::c_int, mut bfile_ptr: *mut bfile) {
    let safe_bfile = bfile_ptr;
    
    if 8 as core::ffi::c_int == (*safe_bfile).wcnt as core::ffi::c_int {
        fputc((*safe_bfile).wbuf as core::ffi::c_int, (*safe_bfile).file);
        (*safe_bfile).wcnt = 0 as core::ffi::c_char;
    }
    
    (*safe_bfile).wcnt += 1;
    (*safe_bfile).wbuf = ((((*safe_bfile).wbuf as core::ffi::c_int) << 1 as core::ffi::c_int) | bit & 1 as core::ffi::c_int) as core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bfclose(mut bf: *mut bfile) {
    if (*bf).wcnt as core::ffi::c_int != 0 as core::ffi::c_int {
        fputc((*bf).wbuf as core::ffi::c_int, (*bf).file);
    }
    fclose((*bf).file);
    free(bf as *mut core::ffi::c_void);
}

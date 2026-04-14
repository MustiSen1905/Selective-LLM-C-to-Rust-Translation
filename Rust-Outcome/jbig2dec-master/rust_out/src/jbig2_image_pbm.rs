extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _Jbig2Page;
    pub type _Jbig2Segment;
    static mut stderr: *mut FILE;
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
    fn sscanf(
        __s: *const core::ffi::c_char,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> core::ffi::c_int;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const core::ffi::c_ushort;
    fn jbig2_image_new(
        ctx: *mut Jbig2Ctx,
        width: uint32_t,
        height: uint32_t,
    ) -> *mut Jbig2Image;
    fn jbig2_image_release(ctx: *mut Jbig2Ctx, image: *mut Jbig2Image);
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type Jbig2Severity = core::ffi::c_uint;
pub const JBIG2_SEVERITY_FATAL: Jbig2Severity = 3;
pub const JBIG2_SEVERITY_WARNING: Jbig2Severity = 2;
pub const JBIG2_SEVERITY_INFO: Jbig2Severity = 1;
pub const JBIG2_SEVERITY_DEBUG: Jbig2Severity = 0;
pub type Jbig2Options = core::ffi::c_uint;
pub const JBIG2_OPTIONS_EMBEDDED: Jbig2Options = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2Allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut Jbig2Allocator, size_t) -> *mut core::ffi::c_void,
    >,
    pub free: Option<
        unsafe extern "C" fn(*mut Jbig2Allocator, *mut core::ffi::c_void) -> (),
    >,
    pub realloc: Option<
        unsafe extern "C" fn(
            *mut Jbig2Allocator,
            *mut core::ffi::c_void,
            size_t,
        ) -> *mut core::ffi::c_void,
    >,
}
pub type Jbig2Allocator = _Jbig2Allocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2Ctx {
    pub allocator: *mut Jbig2Allocator,
    pub options: Jbig2Options,
    pub global_ctx: *const Jbig2Ctx,
    pub error_callback: Jbig2ErrorCallback,
    pub error_callback_data: *mut core::ffi::c_void,
    pub buf: *mut byte,
    pub buf_size: size_t,
    pub buf_rd_ix: size_t,
    pub buf_wr_ix: size_t,
    pub state: Jbig2FileState,
    pub file_header_flags: uint8_t,
    pub n_pages: uint32_t,
    pub n_segments_max: uint32_t,
    pub segments: *mut *mut Jbig2Segment,
    pub n_segments: uint32_t,
    pub segment_index: uint32_t,
    pub current_page: uint32_t,
    pub max_page_index: uint32_t,
    pub pages: *mut Jbig2Page,
}
pub type Jbig2Page = _Jbig2Page;
pub type Jbig2Segment = _Jbig2Segment;
pub type Jbig2FileState = core::ffi::c_uint;
pub const JBIG2_FILE_EOF: Jbig2FileState = 5;
pub const JBIG2_FILE_RANDOM_BODIES: Jbig2FileState = 4;
pub const JBIG2_FILE_RANDOM_HEADERS: Jbig2FileState = 3;
pub const JBIG2_FILE_SEQUENTIAL_BODY: Jbig2FileState = 2;
pub const JBIG2_FILE_SEQUENTIAL_HEADER: Jbig2FileState = 1;
pub const JBIG2_FILE_HEADER: Jbig2FileState = 0;
pub type byte = uint8_t;
pub type Jbig2ErrorCallback = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_char,
        Jbig2Severity,
        uint32_t,
    ) -> (),
>;
pub type Jbig2Ctx = _Jbig2Ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2Image {
    pub width: uint32_t,
    pub height: uint32_t,
    pub stride: uint32_t,
    pub data: *mut uint8_t,
    pub refcount: core::ffi::c_int,
}
pub type Jbig2Image = _Jbig2Image;
pub const EOF: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_write_pbm_file(
    mut image: *mut Jbig2Image,
    mut filename: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut code: core::ffi::c_int = 0;
    out = fopen(filename, b"wb\0" as *const u8 as *const core::ffi::c_char) as *mut FILE;
    if out.is_null() {
        fprintf(
            stderr,
            b"unable to open '%s' for writing\0" as *const u8
                as *const core::ffi::c_char,
            filename,
        );
        return 1 as core::ffi::c_int;
    }
    code = jbig2_image_write_pbm(image, out);
    fclose(out);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_write_pbm(
    mut image: *mut Jbig2Image,
    mut out: *mut FILE,
) -> core::ffi::c_int {
    fprintf(
        out,
        b"P4\n%d %d\n\0" as *const u8 as *const core::ffi::c_char,
        (*image).width,
        (*image).height,
    );
    fwrite(
        (*image).data as *const core::ffi::c_void,
        1 as size_t,
        ((*image).height).wrapping_mul((*image).stride) as size_t,
        out,
    );
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_read_pbm_file(
    mut ctx: *mut Jbig2Ctx,
    mut filename: *mut core::ffi::c_char,
) -> *mut Jbig2Image {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut image: *mut Jbig2Image = 0 as *mut Jbig2Image;
    in_0 = fopen(filename, b"rb\0" as *const u8 as *const core::ffi::c_char)
        as *mut FILE;
    if in_0.is_null() {
        fprintf(
            stderr,
            b"unable to open '%s' for reading\n\0" as *const u8
                as *const core::ffi::c_char,
            filename,
        );
        return 0 as *mut Jbig2Image;
    }
    image = jbig2_image_read_pbm(ctx, in_0);
    fclose(in_0);
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_read_pbm(
    mut ctx: *mut Jbig2Ctx,
    mut in_0: *mut FILE,
) -> *mut Jbig2Image {
    let mut i: core::ffi::c_int = 0;
    let mut dim: [core::ffi::c_int; 2] = [0; 2];
    let mut done: core::ffi::c_int = 0;
    let mut image: *mut Jbig2Image = 0 as *mut Jbig2Image;
    let mut c: core::ffi::c_int = 0;
    let mut buf: [core::ffi::c_char; 32] = [0; 32];
    loop {
        c = fgetc(in_0);
        if !(c != 'P' as i32) {
            break;
        }
        if feof(in_0) != 0 {
            return 0 as *mut Jbig2Image;
        }
    }
    c = fgetc(in_0);
    if c != '4' as i32 {
        fprintf(
            stderr,
            b"not a binary pbm file.\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2Image;
    }
    done = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while done < 2 as core::ffi::c_int {
        c = fgetc(in_0);
        if c == ' ' as i32 || c == '\t' as i32 || c == '\r' as i32 || c == '\n' as i32 {
            continue;
        }
        if c == '#' as i32 {
            loop {
                c = fgetc(in_0);
                if !(c != '\n' as i32) {
                    break;
                }
            }
        } else {
            if c == EOF {
                fprintf(
                    stderr,
                    b"end-of-file parsing pbm header\n\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return 0 as *mut Jbig2Image;
            }
            if *(*__ctype_b_loc()).offset(c as isize) as core::ffi::c_int
                & _ISdigit as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
                != 0
            {
                let fresh0 = i;
                i = i + 1;
                buf[fresh0 as usize] = c as core::ffi::c_char;
                loop {
                    c = fgetc(in_0);
                    if !(*(*__ctype_b_loc()).offset(c as isize) as core::ffi::c_int
                        & _ISdigit as core::ffi::c_int as core::ffi::c_ushort
                            as core::ffi::c_int != 0)
                    {
                        break;
                    }
                    if i >= 32 as core::ffi::c_int {
                        fprintf(
                            stderr,
                            b"pbm parsing error\n\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                        return 0 as *mut Jbig2Image;
                    }
                    let fresh1 = i;
                    i = i + 1;
                    buf[fresh1 as usize] = c as core::ffi::c_char;
                }
                buf[i as usize] = '\0' as i32 as core::ffi::c_char;
                if sscanf(
                    buf.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &mut *dim.as_mut_ptr().offset(done as isize) as *mut core::ffi::c_int,
                ) != 1 as core::ffi::c_int
                {
                    fprintf(
                        stderr,
                        b"failed to read pbm image dimensions\n\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    return 0 as *mut Jbig2Image;
                }
                i = 0 as core::ffi::c_int;
                done += 1;
            }
        }
    }
    image = jbig2_image_new(
        ctx,
        dim[0 as core::ffi::c_int as usize] as uint32_t,
        dim[1 as core::ffi::c_int as usize] as uint32_t,
    );
    if image.is_null() {
        fprintf(
            stderr,
            b"failed to allocate %dx%d image for pbm file\n\0" as *const u8
                as *const core::ffi::c_char,
            dim[0 as core::ffi::c_int as usize],
            dim[1 as core::ffi::c_int as usize],
        );
        return 0 as *mut Jbig2Image;
    }
    fread(
        (*image).data as *mut core::ffi::c_void,
        1 as size_t,
        ((*image).height).wrapping_mul((*image).stride) as size_t,
        in_0,
    );
    if feof(in_0) != 0 {
        fprintf(
            stderr,
            b"unexpected end of pbm file.\n\0" as *const u8 as *const core::ffi::c_char,
        );
        jbig2_image_release(ctx, image);
        return 0 as *mut Jbig2Image;
    }
    return image;
}

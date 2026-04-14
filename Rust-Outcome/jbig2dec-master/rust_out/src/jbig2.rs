extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _Jbig2GlobalCtx;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn vsnprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strncpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
        __n: size_t,
    ) -> *mut core::ffi::c_char;
    fn jbig2_image_release(ctx: *mut Jbig2Ctx, image: *mut Jbig2Image);
    fn jbig2_parse_segment_header(
        ctx: *mut Jbig2Ctx,
        buf: *mut uint8_t,
        buf_size: size_t,
        p_header_size: *mut size_t,
    ) -> *mut Jbig2Segment;
    fn jbig2_parse_segment(
        ctx: *mut Jbig2Ctx,
        segment: *mut Jbig2Segment,
        segment_data: *const uint8_t,
    ) -> core::ffi::c_int;
    fn jbig2_free_segment(ctx: *mut Jbig2Ctx, segment: *mut Jbig2Segment);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2Page {
    pub state: Jbig2PageState,
    pub number: uint32_t,
    pub height: uint32_t,
    pub width: uint32_t,
    pub x_resolution: uint32_t,
    pub y_resolution: uint32_t,
    pub stripe_size: uint16_t,
    pub striped: core::ffi::c_int,
    pub end_row: uint32_t,
    pub flags: uint8_t,
    pub image: *mut Jbig2Image,
}
pub type Jbig2Image = _Jbig2Image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2Image {
    pub width: uint32_t,
    pub height: uint32_t,
    pub stride: uint32_t,
    pub data: *mut uint8_t,
    pub refcount: core::ffi::c_int,
}
pub type Jbig2PageState = core::ffi::c_uint;
pub const JBIG2_PAGE_RELEASED: Jbig2PageState = 4;
pub const JBIG2_PAGE_RETURNED: Jbig2PageState = 3;
pub const JBIG2_PAGE_COMPLETE: Jbig2PageState = 2;
pub const JBIG2_PAGE_NEW: Jbig2PageState = 1;
pub const JBIG2_PAGE_FREE: Jbig2PageState = 0;
pub type Jbig2Segment = _Jbig2Segment;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2Segment {
    pub number: uint32_t,
    pub flags: uint8_t,
    pub page_association: uint32_t,
    pub data_length: size_t,
    pub referred_to_segment_count: core::ffi::c_int,
    pub referred_to_segments: *mut uint32_t,
    pub rows: uint32_t,
    pub result: *mut core::ffi::c_void,
}
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
pub type Jbig2GlobalCtx = _Jbig2GlobalCtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2WordStream {
    pub get_next_word: Option<
        unsafe extern "C" fn(
            *mut Jbig2Ctx,
            *mut Jbig2WordStream,
            size_t,
            *mut uint32_t,
        ) -> core::ffi::c_int,
    >,
}
pub type Jbig2WordStream = _Jbig2WordStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Jbig2WordStreamBuf {
    pub super_0: Jbig2WordStream,
    pub data: *const byte,
    pub size: size_t,
}
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const SIZE_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const JBIG2_VERSION_MAJOR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const JBIG2_VERSION_MINOR: core::ffi::c_int = 20 as core::ffi::c_int;
pub const JBIG2_UNKNOWN_SEGMENT_NUMBER: core::ffi::c_uint = !(0 as core::ffi::c_uint);
unsafe extern "C" fn jbig2_default_alloc(
    mut allocator: *mut Jbig2Allocator,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    return malloc(size);
}
unsafe extern "C" fn jbig2_default_free(
    mut allocator: *mut Jbig2Allocator,
    mut p: *mut core::ffi::c_void,
) {
    free(p);
}
unsafe extern "C" fn jbig2_default_realloc(
    mut allocator: *mut Jbig2Allocator,
    mut p: *mut core::ffi::c_void,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    return realloc(p, size);
}
static mut jbig2_default_allocator: Jbig2Allocator = unsafe {
    {
        let mut init = _Jbig2Allocator {
            alloc: Some(
                jbig2_default_alloc
                    as unsafe extern "C" fn(
                        *mut Jbig2Allocator,
                        size_t,
                    ) -> *mut core::ffi::c_void,
            ),
            free: Some(
                jbig2_default_free
                    as unsafe extern "C" fn(
                        *mut Jbig2Allocator,
                        *mut core::ffi::c_void,
                    ) -> (),
            ),
            realloc: Some(
                jbig2_default_realloc
                    as unsafe extern "C" fn(
                        *mut Jbig2Allocator,
                        *mut core::ffi::c_void,
                        size_t,
                    ) -> *mut core::ffi::c_void,
            ),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn jbig2_alloc(
    mut allocator: *mut Jbig2Allocator,
    mut size: size_t,
    mut num: size_t,
) -> *mut core::ffi::c_void {
    if num > 0 as size_t && size > (SIZE_MAX as size_t).wrapping_div(num) {
        return NULL;
    }
    return ((*allocator).alloc)
        .expect("non-null function pointer")(allocator, size.wrapping_mul(num));
}
unsafe extern "C" fn jbig2_default_error(
    mut data: *mut core::ffi::c_void,
    mut msg: *const core::ffi::c_char,
    mut severity: Jbig2Severity,
    mut seg_idx: uint32_t,
) {
    if severity as core::ffi::c_uint
        == JBIG2_SEVERITY_FATAL as core::ffi::c_int as core::ffi::c_uint
    {
        fprintf(
            stderr,
            b"jbig2 decoder FATAL ERROR: %s\0" as *const u8 as *const core::ffi::c_char,
            msg,
        );
        if seg_idx != JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t {
            fprintf(
                stderr,
                b" (segment 0x%02x)\0" as *const u8 as *const core::ffi::c_char,
                seg_idx,
            );
        }
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        fflush(stderr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_error(
    mut ctx: *mut Jbig2Ctx,
    mut severity: Jbig2Severity,
    mut segment_number: uint32_t,
    mut fmt: *const core::ffi::c_char,
    mut args: ...
) -> core::ffi::c_int {
    let mut buf: [core::ffi::c_char; 1024] = [0; 1024];
    let mut ap: ::core::ffi::VaListImpl;
    let mut n: core::ffi::c_int = 0;
    ap = args.clone();
    n = vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[core::ffi::c_char; 1024]>() as size_t,
        fmt,
        ap.as_va_list(),
    );
    if n < 0 as core::ffi::c_int
        || n as usize == ::core::mem::size_of::<[core::ffi::c_char; 1024]>() as usize
    {
        strncpy(
            buf.as_mut_ptr(),
            b"failed to generate error string\0" as *const u8
                as *const core::ffi::c_char,
            ::core::mem::size_of::<[core::ffi::c_char; 1024]>() as size_t,
        );
    }
    ((*ctx).error_callback)
        .expect(
            "non-null function pointer",
        )((*ctx).error_callback_data, buf.as_mut_ptr(), severity, segment_number);
    return -(1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_ctx_new_imp(
    mut allocator: *mut Jbig2Allocator,
    mut options: Jbig2Options,
    mut global_ctx: *mut Jbig2GlobalCtx,
    mut error_callback: Jbig2ErrorCallback,
    mut error_callback_data: *mut core::ffi::c_void,
    mut jbig2_version_major: core::ffi::c_int,
    mut jbig2_version_minor: core::ffi::c_int,
) -> *mut Jbig2Ctx {
    let mut result: *mut Jbig2Ctx = 0 as *mut Jbig2Ctx;
    if jbig2_version_major != JBIG2_VERSION_MAJOR
        || jbig2_version_minor != JBIG2_VERSION_MINOR
    {
        let mut fakectx: Jbig2Ctx = _Jbig2Ctx {
            allocator: 0 as *mut Jbig2Allocator,
            options: 0 as Jbig2Options,
            global_ctx: 0 as *const Jbig2Ctx,
            error_callback: None,
            error_callback_data: 0 as *mut core::ffi::c_void,
            buf: 0 as *mut byte,
            buf_size: 0,
            buf_rd_ix: 0,
            buf_wr_ix: 0,
            state: JBIG2_FILE_HEADER,
            file_header_flags: 0,
            n_pages: 0,
            n_segments_max: 0,
            segments: 0 as *mut *mut Jbig2Segment,
            n_segments: 0,
            segment_index: 0,
            current_page: 0,
            max_page_index: 0,
            pages: 0 as *mut Jbig2Page,
        };
        fakectx.error_callback = error_callback;
        fakectx.error_callback_data = error_callback_data;
        jbig2_error(
            &mut fakectx as *mut Jbig2Ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"incompatible jbig2dec header (%d.%d) and library (%d.%d) versions\0"
                as *const u8 as *const core::ffi::c_char,
            jbig2_version_major,
            jbig2_version_minor,
            JBIG2_VERSION_MAJOR,
            JBIG2_VERSION_MINOR,
        );
        return 0 as *mut Jbig2Ctx;
    }
    if allocator.is_null() {
        allocator = &mut jbig2_default_allocator;
    }
    if error_callback.is_none() {
        error_callback = Some(
            jbig2_default_error
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *const core::ffi::c_char,
                    Jbig2Severity,
                    uint32_t,
                ) -> (),
        ) as Jbig2ErrorCallback;
    }
    result = jbig2_alloc(
        allocator,
        ::core::mem::size_of::<Jbig2Ctx>() as size_t,
        1 as size_t,
    ) as *mut Jbig2Ctx;
    if result.is_null() {
        error_callback
            .expect(
                "non-null function pointer",
            )(
            error_callback_data,
            b"failed to allocate initial context\0" as *const u8
                as *const core::ffi::c_char,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
        );
        return 0 as *mut Jbig2Ctx;
    }
    (*result).allocator = allocator;
    (*result).options = options;
    (*result).global_ctx = global_ctx as *const Jbig2Ctx;
    (*result).error_callback = error_callback;
    (*result).error_callback_data = error_callback_data;
    (*result).state = (if options as core::ffi::c_uint
        & JBIG2_OPTIONS_EMBEDDED as core::ffi::c_int as core::ffi::c_uint != 0
    {
        JBIG2_FILE_SEQUENTIAL_HEADER as core::ffi::c_int
    } else {
        JBIG2_FILE_HEADER as core::ffi::c_int
    }) as Jbig2FileState;
    (*result).buf = 0 as *mut byte;
    (*result).n_segments = 0 as uint32_t;
    (*result).n_segments_max = 16 as uint32_t;
    (*result).segments = jbig2_alloc(
        (*result).allocator,
        (*result).n_segments_max as size_t,
        ::core::mem::size_of::<*mut Jbig2Segment>() as size_t,
    ) as *mut *mut Jbig2Segment;
    if ((*result).segments).is_null() {
        error_callback
            .expect(
                "non-null function pointer",
            )(
            error_callback_data,
            b"failed to allocate initial segments\0" as *const u8
                as *const core::ffi::c_char,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
        );
        jbig2_free(allocator, result as *mut core::ffi::c_void);
        return 0 as *mut Jbig2Ctx;
    }
    (*result).segment_index = 0 as uint32_t;
    (*result).current_page = 0 as uint32_t;
    (*result).max_page_index = 4 as uint32_t;
    (*result).pages = jbig2_alloc(
        (*result).allocator,
        (*result).max_page_index as size_t,
        ::core::mem::size_of::<Jbig2Page>() as size_t,
    ) as *mut Jbig2Page;
    if ((*result).pages).is_null() {
        error_callback
            .expect(
                "non-null function pointer",
            )(
            error_callback_data,
            b"failed to allocated initial pages\0" as *const u8
                as *const core::ffi::c_char,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
        );
        jbig2_free(allocator, (*result).segments as *mut core::ffi::c_void);
        jbig2_free(allocator, result as *mut core::ffi::c_void);
        return 0 as *mut Jbig2Ctx;
    }
    let mut index: uint32_t = 0;
    index = 0 as uint32_t;
    while index < (*result).max_page_index {
        (*((*result).pages).offset(index as isize)).state = JBIG2_PAGE_FREE;
        (*((*result).pages).offset(index as isize)).number = 0 as uint32_t;
        (*((*result).pages).offset(index as isize)).width = 0 as uint32_t;
        (*((*result).pages).offset(index as isize)).height = 0xffffffff
            as core::ffi::c_uint as uint32_t;
        (*((*result).pages).offset(index as isize)).x_resolution = 0 as uint32_t;
        (*((*result).pages).offset(index as isize)).y_resolution = 0 as uint32_t;
        (*((*result).pages).offset(index as isize)).stripe_size = 0 as uint16_t;
        (*((*result).pages).offset(index as isize)).striped = 0 as core::ffi::c_int;
        (*((*result).pages).offset(index as isize)).end_row = 0 as uint32_t;
        (*((*result).pages).offset(index as isize)).flags = 0 as uint8_t;
        let ref mut fresh0 = (*((*result).pages).offset(index as isize)).image;
        *fresh0 = 0 as *mut Jbig2Image;
        index = index.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_get_int16(mut bptr: *const byte) -> int16_t {
    return ((((*bptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 8 as core::ffi::c_int
        | *bptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
        ^ 0x8000 as core::ffi::c_int) - 0x8000 as core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_get_uint16(mut bptr: *const byte) -> uint16_t {
    return ((*bptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 8 as core::ffi::c_int
        | *bptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_get_int32(mut bptr: *const byte) -> int32_t {
    return (((((*bptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 8 as core::ffi::c_int
        | *bptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
        ^ 0x8000 as core::ffi::c_int) - 0x8000 as core::ffi::c_int) as int32_t)
        << 16 as core::ffi::c_int
        | ((*bptr
            .offset(2 as core::ffi::c_int as isize)
            .offset(0 as core::ffi::c_int as isize) as int32_t) << 8 as core::ffi::c_int
            | *bptr
                .offset(2 as core::ffi::c_int as isize)
                .offset(1 as core::ffi::c_int as isize) as int32_t);
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_get_uint32(mut bptr: *const byte) -> uint32_t {
    return (((*bptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 8 as core::ffi::c_int
        | *bptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int) as uint32_t)
        << 16 as core::ffi::c_int
        | ((*bptr
            .offset(2 as core::ffi::c_int as isize)
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            << 8 as core::ffi::c_int
            | *bptr
                .offset(2 as core::ffi::c_int as isize)
                .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int) as uint32_t;
}
unsafe extern "C" fn jbig2_find_buffer_size(mut desired: size_t) -> size_t {
    let initial_buf_size: size_t = 1024 as size_t;
    let mut size: size_t = initial_buf_size;
    if desired == SIZE_MAX as size_t {
        return SIZE_MAX as size_t;
    }
    while size < desired {
        size <<= 1 as core::ffi::c_int;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_data_in(
    mut ctx: *mut Jbig2Ctx,
    mut data: *const core::ffi::c_uchar,
    mut size: size_t,
) -> core::ffi::c_int {
    if ((*ctx).buf).is_null() {
        let mut buf_size: size_t = jbig2_find_buffer_size(size);
        (*ctx).buf = jbig2_alloc(
            (*ctx).allocator,
            buf_size,
            ::core::mem::size_of::<byte>() as size_t,
        ) as *mut byte;
        if ((*ctx).buf).is_null() {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to allocate buffer when reading data\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        (*ctx).buf_size = buf_size;
        (*ctx).buf_rd_ix = 0 as size_t;
        (*ctx).buf_wr_ix = 0 as size_t;
    } else if size > ((*ctx).buf_size).wrapping_sub((*ctx).buf_wr_ix) {
        let mut already: size_t = ((*ctx).buf_wr_ix).wrapping_sub((*ctx).buf_rd_ix);
        if (*ctx).buf_rd_ix <= (*ctx).buf_size >> 1 as core::ffi::c_int
            && size <= ((*ctx).buf_size).wrapping_sub(already)
        {
            memmove(
                (*ctx).buf as *mut core::ffi::c_void,
                ((*ctx).buf).offset((*ctx).buf_rd_ix as isize)
                    as *const core::ffi::c_void,
                already,
            );
        } else {
            let mut buf: *mut byte = 0 as *mut byte;
            let mut buf_size_0: size_t = 0;
            if already > (SIZE_MAX as size_t).wrapping_sub(size) {
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_FATAL,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"read data causes buffer to grow too large\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            buf_size_0 = jbig2_find_buffer_size(size.wrapping_add(already));
            buf = jbig2_alloc(
                (*ctx).allocator,
                buf_size_0,
                ::core::mem::size_of::<byte>() as size_t,
            ) as *mut byte;
            if buf.is_null() {
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_FATAL,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to allocate bigger buffer when reading data\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            memcpy(
                buf as *mut core::ffi::c_void,
                ((*ctx).buf).offset((*ctx).buf_rd_ix as isize)
                    as *const core::ffi::c_void,
                already,
            );
            jbig2_free((*ctx).allocator, (*ctx).buf as *mut core::ffi::c_void);
            (*ctx).buf = buf;
            (*ctx).buf_size = buf_size_0;
        }
        (*ctx).buf_wr_ix = ((*ctx).buf_wr_ix).wrapping_sub((*ctx).buf_rd_ix);
        (*ctx).buf_rd_ix = 0 as size_t;
    }
    memcpy(
        ((*ctx).buf).offset((*ctx).buf_wr_ix as isize) as *mut core::ffi::c_void,
        data as *const core::ffi::c_void,
        size,
    );
    (*ctx).buf_wr_ix = ((*ctx).buf_wr_ix).wrapping_add(size);
    loop {
        let jbig2_id_string: [byte; 8] = [
            0x97 as core::ffi::c_int as byte,
            0x4a as core::ffi::c_int as byte,
            0x42 as core::ffi::c_int as byte,
            0x32 as core::ffi::c_int as byte,
            0xd as core::ffi::c_int as byte,
            0xa as core::ffi::c_int as byte,
            0x1a as core::ffi::c_int as byte,
            0xa as core::ffi::c_int as byte,
        ];
        let mut segment: *mut Jbig2Segment = 0 as *mut Jbig2Segment;
        let mut header_size: size_t = 0;
        let mut code: core::ffi::c_int = 0;
        match (*ctx).state as core::ffi::c_uint {
            0 => {
                if ((*ctx).buf_wr_ix).wrapping_sub((*ctx).buf_rd_ix) < 9 as size_t {
                    return 0 as core::ffi::c_int;
                }
                if memcmp(
                    ((*ctx).buf).offset((*ctx).buf_rd_ix as isize)
                        as *const core::ffi::c_void,
                    jbig2_id_string.as_ptr() as *const core::ffi::c_void,
                    8 as size_t,
                ) != 0
                {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_FATAL,
                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                        b"not a JBIG2 file header\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                (*ctx).file_header_flags = *((*ctx).buf)
                    .offset(((*ctx).buf_rd_ix).wrapping_add(8 as size_t) as isize)
                    as uint8_t;
                if (*ctx).file_header_flags as core::ffi::c_int & 0x4 as core::ffi::c_int
                    != 0
                {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_FATAL,
                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                        b"file header indicates use of 12 adaptive template pixels (NYI)\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                if (*ctx).file_header_flags as core::ffi::c_int & 0x8 as core::ffi::c_int
                    != 0
                {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_FATAL,
                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                        b"file header indicates use of colored region segments (NYI)\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                if (*ctx).file_header_flags as core::ffi::c_int
                    & 0xfc as core::ffi::c_int != 0
                {
                    jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                        b"reserved bits (2-7) of file header flags are not zero (0x%02x)\0"
                            as *const u8 as *const core::ffi::c_char,
                        (*ctx).file_header_flags as core::ffi::c_int,
                    );
                }
                if (*ctx).file_header_flags as core::ffi::c_int & 2 as core::ffi::c_int
                    == 0
                {
                    if ((*ctx).buf_wr_ix).wrapping_sub((*ctx).buf_rd_ix) < 13 as size_t {
                        return 0 as core::ffi::c_int;
                    }
                    (*ctx).n_pages = jbig2_get_uint32(
                        ((*ctx).buf)
                            .offset((*ctx).buf_rd_ix as isize)
                            .offset(9 as core::ffi::c_int as isize),
                    );
                    (*ctx).buf_rd_ix = ((*ctx).buf_rd_ix).wrapping_add(13 as size_t);
                    if (*ctx).n_pages == 1 as uint32_t {
                        jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_INFO,
                            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                            b"file header indicates a single page document\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {
                        jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_INFO,
                            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                            b"file header indicates a %d page document\0" as *const u8
                                as *const core::ffi::c_char,
                            (*ctx).n_pages,
                        );
                    }
                } else {
                    (*ctx).n_pages = 0 as uint32_t;
                    (*ctx).buf_rd_ix = ((*ctx).buf_rd_ix).wrapping_add(9 as size_t);
                }
                if (*ctx).file_header_flags as core::ffi::c_int & 1 as core::ffi::c_int
                    != 0
                {
                    (*ctx).state = JBIG2_FILE_SEQUENTIAL_HEADER;
                    jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_DEBUG,
                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                        b"file header indicates sequential organization\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {
                    (*ctx).state = JBIG2_FILE_RANDOM_HEADERS;
                    jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_DEBUG,
                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                        b"file header indicates random-access organization\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
            }
            1 | 3 => {
                segment = jbig2_parse_segment_header(
                    ctx,
                    ((*ctx).buf).offset((*ctx).buf_rd_ix as isize),
                    ((*ctx).buf_wr_ix).wrapping_sub((*ctx).buf_rd_ix),
                    &mut header_size,
                );
                if segment.is_null() {
                    return 0 as core::ffi::c_int;
                }
                (*ctx).buf_rd_ix = ((*ctx).buf_rd_ix).wrapping_add(header_size);
                if (*ctx).n_segments >= (*ctx).n_segments_max {
                    let mut segments: *mut *mut Jbig2Segment = 0
                        as *mut *mut Jbig2Segment;
                    if (*ctx).n_segments_max == UINT32_MAX as uint32_t {
                        (*ctx).state = JBIG2_FILE_EOF;
                        jbig2_free_segment(ctx, segment);
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_FATAL,
                            (*segment).number,
                            b"too many segments in jbig2 image\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    } else if (*ctx).n_segments_max
                        > UINT32_MAX as uint32_t >> 2 as core::ffi::c_int
                    {
                        (*ctx).n_segments_max = UINT32_MAX as uint32_t;
                    } else {
                        (*ctx).n_segments_max <<= 2 as core::ffi::c_int;
                    }
                    segments = jbig2_realloc(
                        (*ctx).allocator,
                        (*ctx).segments as *mut core::ffi::c_void,
                        (*ctx).n_segments_max as size_t,
                        ::core::mem::size_of::<*mut Jbig2Segment>() as size_t,
                    ) as *mut *mut Jbig2Segment;
                    if segments.is_null() {
                        (*ctx).state = JBIG2_FILE_EOF;
                        jbig2_free_segment(ctx, segment);
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_FATAL,
                            (*segment).number,
                            b"failed to allocate space for more segments\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    (*ctx).segments = segments;
                }
                let fresh1 = (*ctx).n_segments;
                (*ctx).n_segments = ((*ctx).n_segments).wrapping_add(1);
                let ref mut fresh2 = *((*ctx).segments).offset(fresh1 as isize);
                *fresh2 = segment;
                if (*ctx).state as core::ffi::c_uint
                    == JBIG2_FILE_RANDOM_HEADERS as core::ffi::c_int as core::ffi::c_uint
                {
                    if (*segment).flags as core::ffi::c_int & 63 as core::ffi::c_int
                        == 51 as core::ffi::c_int
                    {
                        (*ctx).state = JBIG2_FILE_RANDOM_BODIES;
                    }
                } else {
                    (*ctx).state = JBIG2_FILE_SEQUENTIAL_BODY;
                }
            }
            2 | 4 => {
                segment = *((*ctx).segments).offset((*ctx).segment_index as isize);
                if (*segment).data_length == 0xffffffff as size_t
                    && (*segment).flags as core::ffi::c_int & 63 as core::ffi::c_int
                        == 38 as core::ffi::c_int
                {
                    let mut s: *mut byte = 0 as *mut byte;
                    let mut e: *mut byte = 0 as *mut byte;
                    let mut p: *mut byte = 0 as *mut byte;
                    let mut mmr: core::ffi::c_int = 0;
                    let mut mmr_marker: [byte; 2] = [
                        0 as core::ffi::c_int as byte,
                        0 as core::ffi::c_int as byte,
                    ];
                    let mut arith_marker: [byte; 2] = [
                        0xff as core::ffi::c_int as byte,
                        0xac as core::ffi::c_int as byte,
                    ];
                    let mut desired_marker: *mut byte = 0 as *mut byte;
                    p = ((*ctx).buf).offset((*ctx).buf_rd_ix as isize);
                    s = p;
                    e = ((*ctx).buf).offset((*ctx).buf_wr_ix as isize);
                    if (e.offset_from(p) as core::ffi::c_long) < 18 as core::ffi::c_long
                    {
                        return 0 as core::ffi::c_int;
                    }
                    mmr = *p.offset(17 as core::ffi::c_int as isize) as core::ffi::c_int
                        & 1 as core::ffi::c_int;
                    p = p.offset(18 as core::ffi::c_int as isize);
                    desired_marker = if mmr != 0 {
                        mmr_marker.as_mut_ptr()
                    } else {
                        arith_marker.as_mut_ptr()
                    };
                    if (e.offset_from(p) as core::ffi::c_long) < 2 as core::ffi::c_long {
                        return 0 as core::ffi::c_int;
                    }
                    while *p.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        != *desired_marker.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int
                        || *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            != *desired_marker.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                    {
                        p = p.offset(1);
                        if (e.offset_from(p) as core::ffi::c_long)
                            < 2 as core::ffi::c_long
                        {
                            return 0 as core::ffi::c_int;
                        }
                    }
                    p = p.offset(2 as core::ffi::c_int as isize);
                    if (e.offset_from(p) as core::ffi::c_long) < 4 as core::ffi::c_long {
                        return 0 as core::ffi::c_int;
                    }
                    (*segment).rows = jbig2_get_uint32(p);
                    p = p.offset(4 as core::ffi::c_int as isize);
                    (*segment).data_length = p.offset_from(s) as core::ffi::c_long
                        as size_t;
                    jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_INFO,
                        (*segment).number,
                        b"unknown length determined to be %lu\0" as *const u8
                            as *const core::ffi::c_char,
                        (*segment).data_length as core::ffi::c_long,
                    );
                } else if (*segment).data_length
                    > ((*ctx).buf_wr_ix).wrapping_sub((*ctx).buf_rd_ix)
                {
                    return 0 as core::ffi::c_int
                }
                code = jbig2_parse_segment(
                    ctx,
                    segment,
                    ((*ctx).buf).offset((*ctx).buf_rd_ix as isize),
                );
                (*ctx).buf_rd_ix = ((*ctx).buf_rd_ix)
                    .wrapping_add((*segment).data_length);
                (*ctx).segment_index = ((*ctx).segment_index).wrapping_add(1);
                if (*ctx).state as core::ffi::c_uint
                    == JBIG2_FILE_RANDOM_BODIES as core::ffi::c_int as core::ffi::c_uint
                {
                    if (*ctx).segment_index == (*ctx).n_segments {
                        (*ctx).state = JBIG2_FILE_EOF;
                    }
                } else {
                    (*ctx).state = JBIG2_FILE_SEQUENTIAL_HEADER;
                }
                if code < 0 as core::ffi::c_int {
                    (*ctx).state = JBIG2_FILE_EOF;
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode; treating as end of file\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
            5 => {
                if (*ctx).buf_rd_ix == (*ctx).buf_wr_ix {
                    return 0 as core::ffi::c_int;
                }
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"garbage beyond end of file\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_ctx_free(mut ctx: *mut Jbig2Ctx) -> *mut Jbig2Allocator {
    let mut ca: *mut Jbig2Allocator = 0 as *mut Jbig2Allocator;
    let mut i: uint32_t = 0;
    if ctx.is_null() {
        return 0 as *mut Jbig2Allocator;
    }
    ca = (*ctx).allocator;
    jbig2_free(ca, (*ctx).buf as *mut core::ffi::c_void);
    if !((*ctx).segments).is_null() {
        i = 0 as uint32_t;
        while i < (*ctx).n_segments {
            jbig2_free_segment(ctx, *((*ctx).segments).offset(i as isize));
            i = i.wrapping_add(1);
        }
        jbig2_free(ca, (*ctx).segments as *mut core::ffi::c_void);
    }
    if !((*ctx).pages).is_null() {
        i = 0 as uint32_t;
        while i <= (*ctx).current_page {
            if !((*((*ctx).pages).offset(i as isize)).image).is_null() {
                jbig2_image_release(ctx, (*((*ctx).pages).offset(i as isize)).image);
            }
            i = i.wrapping_add(1);
        }
        jbig2_free(ca, (*ctx).pages as *mut core::ffi::c_void);
    }
    jbig2_free(ca, ctx as *mut core::ffi::c_void);
    return ca;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_make_global_ctx(
    mut ctx: *mut Jbig2Ctx,
) -> *mut Jbig2GlobalCtx {
    return ctx as *mut Jbig2GlobalCtx;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_global_ctx_free(
    mut global_ctx: *mut Jbig2GlobalCtx,
) -> *mut Jbig2Allocator {
    return jbig2_ctx_free(global_ctx as *mut Jbig2Ctx);
}
unsafe extern "C" fn jbig2_word_stream_buf_get_next_word(
    mut ctx: *mut Jbig2Ctx,
    mut self_0: *mut Jbig2WordStream,
    mut offset: size_t,
    mut word: *mut uint32_t,
) -> core::ffi::c_int {
    let mut z: *mut Jbig2WordStreamBuf = self_0 as *mut Jbig2WordStreamBuf;
    let mut val: uint32_t = 0 as uint32_t;
    let mut ret: core::ffi::c_int = 0 as core::ffi::c_int;
    if self_0.is_null() || word.is_null() {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to read next word of stream because stream or output missing\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    if offset >= (*z).size {
        *word = 0 as uint32_t;
        return 0 as core::ffi::c_int;
    }
    if offset < (*z).size {
        val = (*((*z).data).offset(offset as isize) as uint32_t)
            << 24 as core::ffi::c_int;
        ret += 1;
    }
    if offset.wrapping_add(1 as size_t) < (*z).size {
        val
            |= (*((*z).data).offset(offset.wrapping_add(1 as size_t) as isize)
                as uint32_t) << 16 as core::ffi::c_int;
        ret += 1;
    }
    if offset.wrapping_add(2 as size_t) < (*z).size {
        val
            |= (*((*z).data).offset(offset.wrapping_add(2 as size_t) as isize)
                as uint32_t) << 8 as core::ffi::c_int;
        ret += 1;
    }
    if offset.wrapping_add(3 as size_t) < (*z).size {
        val
            |= *((*z).data).offset(offset.wrapping_add(3 as size_t) as isize)
                as uint32_t;
        ret += 1;
    }
    *word = val;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_word_stream_buf_new(
    mut ctx: *mut Jbig2Ctx,
    mut data: *const byte,
    mut size: size_t,
) -> *mut Jbig2WordStream {
    let mut result: *mut Jbig2WordStreamBuf = jbig2_alloc(
        (*ctx).allocator,
        1 as size_t,
        ::core::mem::size_of::<Jbig2WordStreamBuf>() as size_t,
    ) as *mut Jbig2WordStreamBuf;
    if result.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate word stream\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2WordStream;
    }
    (*result).super_0.get_next_word = Some(
        jbig2_word_stream_buf_get_next_word
            as unsafe extern "C" fn(
                *mut Jbig2Ctx,
                *mut Jbig2WordStream,
                size_t,
                *mut uint32_t,
            ) -> core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut Jbig2Ctx,
                *mut Jbig2WordStream,
                size_t,
                *mut uint32_t,
            ) -> core::ffi::c_int,
        >;
    (*result).data = data;
    (*result).size = size;
    return &mut (*result).super_0;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_word_stream_buf_free(
    mut ctx: *mut Jbig2Ctx,
    mut ws: *mut Jbig2WordStream,
) {
    jbig2_free((*ctx).allocator, ws as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_free(
    mut allocator: *mut Jbig2Allocator,
    mut p: *mut core::ffi::c_void,
) {
    ((*allocator).free).expect("non-null function pointer")(allocator, p);
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_realloc(
    mut allocator: *mut Jbig2Allocator,
    mut p: *mut core::ffi::c_void,
    mut size: size_t,
    mut num: size_t,
) -> *mut core::ffi::c_void {
    if num > 0 as size_t && size >= (SIZE_MAX as size_t).wrapping_div(num) {
        return NULL;
    }
    return ((*allocator).realloc)
        .expect("non-null function pointer")(allocator, p, size.wrapping_mul(num));
}

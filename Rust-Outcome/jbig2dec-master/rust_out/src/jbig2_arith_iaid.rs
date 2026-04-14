extern "C" {
    pub type _Jbig2Page;
    pub type _Jbig2Segment;
    pub type _Jbig2ArithState;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn jbig2_alloc(
        allocator: *mut Jbig2Allocator,
        size: size_t,
        num: size_t,
    ) -> *mut core::ffi::c_void;
    fn jbig2_free(allocator: *mut Jbig2Allocator, p: *mut core::ffi::c_void);
    fn jbig2_error(
        ctx: *mut Jbig2Ctx,
        severity: Jbig2Severity,
        seg_idx: uint32_t,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn jbig2_arith_decode(
        ctx: *mut Jbig2Ctx,
        as_0: *mut Jbig2ArithState,
        pcx: *mut Jbig2ArithCx,
    ) -> core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
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
pub type Jbig2ArithState = _Jbig2ArithState;
pub type Jbig2ArithCx = core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2ArithIaidCtx {
    pub SBSYMCODELEN: uint8_t,
    pub IAIDx: *mut Jbig2ArithCx,
}
pub type Jbig2ArithIaidCtx = _Jbig2ArithIaidCtx;
pub const JBIG2_UNKNOWN_SEGMENT_NUMBER: core::ffi::c_uint = !(0 as core::ffi::c_uint);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn jbig2_arith_iaid_ctx_new(
    mut ctx: *mut Jbig2Ctx,
    mut SBSYMCODELEN: uint8_t,
) -> *mut Jbig2ArithIaidCtx {
    let mut result: *mut Jbig2ArithIaidCtx = 0 as *mut Jbig2ArithIaidCtx;
    let mut ctx_size: size_t = 0;
    if (::core::mem::size_of::<size_t>() as usize).wrapping_mul(8 as usize)
        <= SBSYMCODELEN as usize
    {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"requested IAID arithmetic coding state size too large\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2ArithIaidCtx;
    }
    ctx_size = (1 as core::ffi::c_uint as size_t) << SBSYMCODELEN as core::ffi::c_int;
    result = jbig2_alloc(
        (*ctx).allocator,
        1 as size_t,
        ::core::mem::size_of::<Jbig2ArithIaidCtx>() as size_t,
    ) as *mut Jbig2ArithIaidCtx;
    if result.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate IAID arithmetic coding state\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2ArithIaidCtx;
    }
    (*result).SBSYMCODELEN = SBSYMCODELEN;
    (*result).IAIDx = jbig2_alloc(
        (*ctx).allocator,
        ctx_size,
        ::core::mem::size_of::<Jbig2ArithCx>() as size_t,
    ) as *mut Jbig2ArithCx;
    if ((*result).IAIDx).is_null() {
        jbig2_free((*ctx).allocator, result as *mut core::ffi::c_void);
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate symbol ID in IAID arithmetic coding state\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2ArithIaidCtx;
    }
    memset((*result).IAIDx as *mut core::ffi::c_void, 0 as core::ffi::c_int, ctx_size);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_arith_iaid_decode(
    mut ctx: *mut Jbig2Ctx,
    mut actx: *mut Jbig2ArithIaidCtx,
    mut as_0: *mut Jbig2ArithState,
    mut p_result: *mut int32_t,
) -> core::ffi::c_int {
    let mut IAIDx: *mut Jbig2ArithCx = (*actx).IAIDx;
    let mut SBSYMCODELEN: uint8_t = (*actx).SBSYMCODELEN;
    let mut PREV: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut D: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < SBSYMCODELEN as core::ffi::c_int {
        D = jbig2_arith_decode(ctx, as_0, &mut *IAIDx.offset(PREV as isize));
        if D < 0 as core::ffi::c_int {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to decode IAIDx code\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        PREV = PREV << 1 as core::ffi::c_int | D;
        i += 1;
    }
    PREV -= (1 as core::ffi::c_int) << SBSYMCODELEN as core::ffi::c_int;
    *p_result = PREV as int32_t;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_arith_iaid_ctx_free(
    mut ctx: *mut Jbig2Ctx,
    mut iax: *mut Jbig2ArithIaidCtx,
) {
    if !iax.is_null() {
        jbig2_free((*ctx).allocator, (*iax).IAIDx as *mut core::ffi::c_void);
        jbig2_free((*ctx).allocator, iax as *mut core::ffi::c_void);
    }
}

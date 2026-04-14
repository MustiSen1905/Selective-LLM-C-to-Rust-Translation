extern "C" {
    pub type _Jbig2ArithState;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn jbig2_get_uint32(bptr: *const byte) -> uint32_t;
    fn jbig2_get_int32(buf: *const byte) -> int32_t;
    fn jbig2_get_uint16(bptr: *const byte) -> uint16_t;
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
    fn jbig2_word_stream_buf_new(
        ctx: *mut Jbig2Ctx,
        data: *const byte,
        size: size_t,
    ) -> *mut Jbig2WordStream;
    fn jbig2_word_stream_buf_free(ctx: *mut Jbig2Ctx, ws: *mut Jbig2WordStream);
    fn jbig2_arith_new(
        ctx: *mut Jbig2Ctx,
        ws: *mut Jbig2WordStream,
    ) -> *mut Jbig2ArithState;
    fn jbig2_generic_stats_size(
        ctx: *mut Jbig2Ctx,
        template: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn jbig2_decode_generic_region(
        ctx: *mut Jbig2Ctx,
        segment: *mut Jbig2Segment,
        params: *const Jbig2GenericRegionParams,
        as_0: *mut Jbig2ArithState,
        image: *mut Jbig2Image,
        GB_stats: *mut Jbig2ArithCx,
    ) -> core::ffi::c_int;
    fn jbig2_image_new(
        ctx: *mut Jbig2Ctx,
        width: uint32_t,
        height: uint32_t,
    ) -> *mut Jbig2Image;
    fn jbig2_image_release(ctx: *mut Jbig2Ctx, image: *mut Jbig2Image);
    fn jbig2_image_compose(
        ctx: *mut Jbig2Ctx,
        dst: *mut Jbig2Image,
        src: *mut Jbig2Image,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
        op: Jbig2ComposeOp,
    ) -> core::ffi::c_int;
    fn jbig2_image_get_pixel(
        image: *mut Jbig2Image,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn jbig2_image_set_pixel(
        image: *mut Jbig2Image,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
        value: core::ffi::c_int,
    );
    fn jbig2_decode_generic_mmr(
        ctx: *mut Jbig2Ctx,
        segment: *mut Jbig2Segment,
        params: *const Jbig2GenericRegionParams,
        data: *const byte,
        size: size_t,
        image: *mut Jbig2Image,
    ) -> core::ffi::c_int;
    fn jbig2_decode_halftone_mmr(
        ctx: *mut Jbig2Ctx,
        params: *const Jbig2GenericRegionParams,
        data: *const byte,
        size: size_t,
        image: *mut Jbig2Image,
        consumed_bytes: *mut size_t,
    ) -> core::ffi::c_int;
    fn jbig2_page_add_result(
        ctx: *mut Jbig2Ctx,
        page: *mut Jbig2Page,
        src: *mut Jbig2Image,
        x: uint32_t,
        y: uint32_t,
        op: Jbig2ComposeOp,
    ) -> core::ffi::c_int;
    fn jbig2_find_segment(ctx: *mut Jbig2Ctx, number: uint32_t) -> *mut Jbig2Segment;
    fn jbig2_get_region_segment_info(
        info: *mut Jbig2RegionSegmentInfo,
        segment_data: *const uint8_t,
    );
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type Jbig2ArithState = _Jbig2ArithState;
pub type Jbig2ArithCx = core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Jbig2GenericRegionParams {
    pub MMR: core::ffi::c_int,
    pub GBTEMPLATE: core::ffi::c_int,
    pub TPGDON: core::ffi::c_int,
    pub USESKIP: core::ffi::c_int,
    pub SKIP: *mut Jbig2Image,
    pub gbat: [int8_t; 8],
}
pub type Jbig2ComposeOp = core::ffi::c_uint;
pub const JBIG2_COMPOSE_REPLACE: Jbig2ComposeOp = 4;
pub const JBIG2_COMPOSE_XNOR: Jbig2ComposeOp = 3;
pub const JBIG2_COMPOSE_XOR: Jbig2ComposeOp = 2;
pub const JBIG2_COMPOSE_AND: Jbig2ComposeOp = 1;
pub const JBIG2_COMPOSE_OR: Jbig2ComposeOp = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Jbig2PatternDict {
    pub n_patterns: core::ffi::c_int,
    pub patterns: *mut *mut Jbig2Image,
    pub HPW: uint32_t,
    pub HPH: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Jbig2PatternDictParams {
    pub HDMMR: core::ffi::c_int,
    pub HDPW: uint32_t,
    pub HDPH: uint32_t,
    pub GRAYMAX: uint32_t,
    pub HDTEMPLATE: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Jbig2HalftoneRegionParams {
    pub flags: byte,
    pub HGW: uint32_t,
    pub HGH: uint32_t,
    pub HGX: int32_t,
    pub HGY: int32_t,
    pub HRX: uint16_t,
    pub HRY: uint16_t,
    pub HMMR: core::ffi::c_int,
    pub HTEMPLATE: core::ffi::c_int,
    pub HENABLESKIP: core::ffi::c_int,
    pub HCOMBOP: Jbig2ComposeOp,
    pub HDEFPIXEL: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Jbig2RegionSegmentInfo {
    pub width: uint32_t,
    pub height: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,
    pub op: Jbig2ComposeOp,
    pub flags: uint8_t,
}
pub const JBIG2_UNKNOWN_SEGMENT_NUMBER: core::ffi::c_uint = !(0 as core::ffi::c_uint);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
unsafe extern "C" fn jbig2_hd_new(
    mut ctx: *mut Jbig2Ctx,
    mut params: *const Jbig2PatternDictParams,
    mut image: *mut Jbig2Image,
) -> *mut Jbig2PatternDict {
    let mut new: *mut Jbig2PatternDict = 0 as *mut Jbig2PatternDict;
    let N: uint32_t = ((*params).GRAYMAX).wrapping_add(1 as uint32_t);
    let HPW: uint32_t = (*params).HDPW;
    let HPH: uint32_t = (*params).HDPH;
    let mut code: core::ffi::c_int = 0;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    if N == 0 as uint32_t {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"params->GRAYMAX out of range\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2PatternDict;
    }
    new = jbig2_alloc(
        (*ctx).allocator,
        1 as size_t,
        ::core::mem::size_of::<Jbig2PatternDict>() as size_t,
    ) as *mut Jbig2PatternDict;
    if !new.is_null() {
        (*new).patterns = jbig2_alloc(
            (*ctx).allocator,
            N as size_t,
            ::core::mem::size_of::<*mut Jbig2Image>() as size_t,
        ) as *mut *mut Jbig2Image;
        if ((*new).patterns).is_null() {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to allocate pattern in collective bitmap dictionary\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            jbig2_free((*ctx).allocator, new as *mut core::ffi::c_void);
            return 0 as *mut Jbig2PatternDict;
        }
        (*new).n_patterns = N as core::ffi::c_int;
        (*new).HPW = HPW;
        (*new).HPH = HPH;
        i = 0 as uint32_t;
        while i < N {
            let ref mut fresh4 = *((*new).patterns).offset(i as isize);
            *fresh4 = jbig2_image_new(ctx, HPW, HPH);
            if (*((*new).patterns).offset(i as isize)).is_null() {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to allocate pattern element image\0" as *const u8
                        as *const core::ffi::c_char,
                );
                j = 0 as uint32_t;
                while j < i {
                    jbig2_image_release(ctx, *((*new).patterns).offset(j as isize));
                    j = j.wrapping_add(1);
                }
                jbig2_free((*ctx).allocator, (*new).patterns as *mut core::ffi::c_void);
                jbig2_free((*ctx).allocator, new as *mut core::ffi::c_void);
                return 0 as *mut Jbig2PatternDict;
            }
            code = jbig2_image_compose(
                ctx,
                *((*new).patterns).offset(i as isize),
                image,
                i.wrapping_neg().wrapping_mul(HPW as int32_t as uint32_t)
                    as core::ffi::c_int,
                0 as core::ffi::c_int,
                JBIG2_COMPOSE_REPLACE,
            );
            if code < 0 as core::ffi::c_int {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to compose image into collective bitmap dictionary\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                j = 0 as uint32_t;
                while j <= i {
                    jbig2_image_release(ctx, *((*new).patterns).offset(j as isize));
                    j = j.wrapping_add(1);
                }
                jbig2_free((*ctx).allocator, (*new).patterns as *mut core::ffi::c_void);
                jbig2_free((*ctx).allocator, new as *mut core::ffi::c_void);
                return 0 as *mut Jbig2PatternDict;
            }
            i = i.wrapping_add(1);
        }
    } else {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate collective bitmap dictionary\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_hd_release(
    mut ctx: *mut Jbig2Ctx,
    mut dict: *mut Jbig2PatternDict,
) {
    let mut i: core::ffi::c_int = 0;
    if dict.is_null() {
        return;
    }
    if !((*dict).patterns).is_null() {
        i = 0 as core::ffi::c_int;
        while i < (*dict).n_patterns {
            jbig2_image_release(ctx, *((*dict).patterns).offset(i as isize));
            i += 1;
        }
    }
    jbig2_free((*ctx).allocator, (*dict).patterns as *mut core::ffi::c_void);
    jbig2_free((*ctx).allocator, dict as *mut core::ffi::c_void);
}
unsafe extern "C" fn jbig2_decode_pattern_dict(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2PatternDictParams,
    mut data: *const byte,
    size: size_t,
    mut GB_stats: *mut Jbig2ArithCx,
) -> *mut Jbig2PatternDict {
    let mut hd: *mut Jbig2PatternDict = 0 as *mut Jbig2PatternDict;
    let mut image: *mut Jbig2Image = 0 as *mut Jbig2Image;
    let mut rparams: Jbig2GenericRegionParams = Jbig2GenericRegionParams {
        MMR: 0,
        GBTEMPLATE: 0,
        TPGDON: 0,
        USESKIP: 0,
        SKIP: 0 as *mut Jbig2Image,
        gbat: [0; 8],
    };
    let mut code: core::ffi::c_int = 0 as core::ffi::c_int;
    image = jbig2_image_new(
        ctx,
        ((*params).HDPW).wrapping_mul(((*params).GRAYMAX).wrapping_add(1 as uint32_t)),
        (*params).HDPH,
    );
    if image.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"failed to allocate collective bitmap for halftone dictionary\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2PatternDict;
    }
    rparams.MMR = (*params).HDMMR;
    rparams.GBTEMPLATE = (*params).HDTEMPLATE;
    rparams.TPGDON = 0 as core::ffi::c_int;
    rparams.USESKIP = 0 as core::ffi::c_int;
    rparams.gbat[0 as core::ffi::c_int as usize] = -((*params).HDPW as int8_t
        as core::ffi::c_int) as int8_t;
    rparams.gbat[1 as core::ffi::c_int as usize] = 0 as int8_t;
    rparams.gbat[2 as core::ffi::c_int as usize] = -(3 as core::ffi::c_int) as int8_t;
    rparams.gbat[3 as core::ffi::c_int as usize] = -(1 as core::ffi::c_int) as int8_t;
    rparams.gbat[4 as core::ffi::c_int as usize] = 2 as int8_t;
    rparams.gbat[5 as core::ffi::c_int as usize] = -(2 as core::ffi::c_int) as int8_t;
    rparams.gbat[6 as core::ffi::c_int as usize] = -(2 as core::ffi::c_int) as int8_t;
    rparams.gbat[7 as core::ffi::c_int as usize] = -(2 as core::ffi::c_int) as int8_t;
    if (*params).HDMMR != 0 {
        code = jbig2_decode_generic_mmr(ctx, segment, &mut rparams, data, size, image);
    } else {
        let mut ws: *mut Jbig2WordStream = jbig2_word_stream_buf_new(ctx, data, size);
        if !ws.is_null() {
            let mut as_0: *mut Jbig2ArithState = jbig2_arith_new(ctx, ws);
            if !as_0.is_null() {
                code = jbig2_decode_generic_region(
                    ctx,
                    segment,
                    &mut rparams,
                    as_0,
                    image,
                    GB_stats,
                );
            } else {
                code = jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"failed to allocate arithmetic coding state when handling halftone dictionary\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            jbig2_free((*ctx).allocator, as_0 as *mut core::ffi::c_void);
            jbig2_word_stream_buf_free(ctx, ws);
        } else {
            code = jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to allocate word stream when handling halftone dictionary\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    if code == 0 as core::ffi::c_int {
        hd = jbig2_hd_new(ctx, params, image);
    } else {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"failed to decode immediate generic region\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    jbig2_image_release(ctx, image);
    return hd;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_pattern_dictionary(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut segment_data: *const byte,
) -> core::ffi::c_int {
    let mut params: Jbig2PatternDictParams = Jbig2PatternDictParams {
        HDMMR: 0,
        HDPW: 0,
        HDPH: 0,
        GRAYMAX: 0,
        HDTEMPLATE: 0,
    };
    let mut GB_stats: *mut Jbig2ArithCx = 0 as *mut Jbig2ArithCx;
    let mut flags: byte = 0;
    let mut offset: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*segment).data_length < 7 as size_t {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"segment too short\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    flags = *segment_data.offset(0 as core::ffi::c_int as isize);
    params.HDMMR = flags as core::ffi::c_int & 1 as core::ffi::c_int;
    params.HDTEMPLATE = (flags as core::ffi::c_int & 6 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    params.HDPW = *segment_data.offset(1 as core::ffi::c_int as isize) as uint32_t;
    params.HDPH = *segment_data.offset(2 as core::ffi::c_int as isize) as uint32_t;
    params.GRAYMAX = jbig2_get_uint32(
        segment_data.offset(3 as core::ffi::c_int as isize),
    );
    offset += 7 as core::ffi::c_int;
    jbig2_error(
        ctx,
        JBIG2_SEVERITY_INFO,
        (*segment).number,
        b"pattern dictionary, flags=%02x, %d grays (%dx%d cell)\0" as *const u8
            as *const core::ffi::c_char,
        flags as core::ffi::c_int,
        (params.GRAYMAX).wrapping_add(1 as uint32_t),
        params.HDPW,
        params.HDPH,
    );
    if params.HDMMR != 0 && params.HDTEMPLATE != 0 {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"HDTEMPLATE is %d when HDMMR is %d, contrary to spec\0" as *const u8
                as *const core::ffi::c_char,
            params.HDTEMPLATE,
            params.HDMMR,
        );
    }
    if flags as core::ffi::c_int & 0xf8 as core::ffi::c_int != 0 {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"reserved flag bits non-zero\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if params.HDMMR == 0 {
        let mut stats_size: core::ffi::c_int = jbig2_generic_stats_size(
            ctx,
            params.HDTEMPLATE,
        );
        GB_stats = jbig2_alloc(
            (*ctx).allocator,
            stats_size as size_t,
            ::core::mem::size_of::<Jbig2ArithCx>() as size_t,
        ) as *mut Jbig2ArithCx;
        if GB_stats.is_null() {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to allocate arithmetic coding state when handling pattern dictionary\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        memset(
            GB_stats as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            stats_size as size_t,
        );
    }
    (*segment).result = jbig2_decode_pattern_dict(
        ctx,
        segment,
        &mut params,
        segment_data.offset(offset as isize),
        ((*segment).data_length).wrapping_sub(offset as size_t),
        GB_stats,
    ) as *mut core::ffi::c_void;
    if params.HDMMR == 0 {
        jbig2_free((*ctx).allocator, GB_stats as *mut core::ffi::c_void);
    }
    return if !((*segment).result).is_null() {
        0 as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
}
unsafe extern "C" fn jbig2_decode_gray_scale_image(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut data: *const byte,
    size: size_t,
    mut GSMMR: core::ffi::c_int,
    mut GSW: uint32_t,
    mut GSH: uint32_t,
    mut GSBPP: uint32_t,
    mut GSUSESKIP: core::ffi::c_int,
    mut GSKIP: *mut Jbig2Image,
    mut GSTEMPLATE: core::ffi::c_int,
    mut GB_stats: *mut Jbig2ArithCx,
) -> *mut *mut uint16_t {
    let mut current_block: u64;
    let mut GSVALS: *mut *mut uint16_t = 0 as *mut *mut uint16_t;
    let mut consumed_bytes: size_t = 0 as size_t;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut stride: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut code: core::ffi::c_int = 0;
    let mut GSPLANES: *mut *mut Jbig2Image = 0 as *mut *mut Jbig2Image;
    let mut rparams: Jbig2GenericRegionParams = Jbig2GenericRegionParams {
        MMR: 0,
        GBTEMPLATE: 0,
        TPGDON: 0,
        USESKIP: 0,
        SKIP: 0 as *mut Jbig2Image,
        gbat: [0; 8],
    };
    let mut ws: *mut Jbig2WordStream = 0 as *mut Jbig2WordStream;
    let mut as_0: *mut Jbig2ArithState = 0 as *mut Jbig2ArithState;
    GSPLANES = jbig2_alloc(
        (*ctx).allocator,
        GSBPP as size_t,
        ::core::mem::size_of::<*mut Jbig2Image>() as size_t,
    ) as *mut *mut Jbig2Image;
    if GSPLANES.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"failed to allocate %d bytes for GSPLANES\0" as *const u8
                as *const core::ffi::c_char,
            GSBPP,
        );
        return 0 as *mut *mut uint16_t;
    }
    i = 0 as uint32_t;
    while i < GSBPP {
        let ref mut fresh0 = *GSPLANES.offset(i as isize);
        *fresh0 = jbig2_image_new(ctx, GSW, GSH);
        if (*GSPLANES.offset(i as isize)).is_null() {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to allocate %dx%d image for GSPLANES\0" as *const u8
                    as *const core::ffi::c_char,
                GSW,
                GSH,
            );
            j = i;
            while j > 0 as uint32_t {
                j = j.wrapping_sub(1);
                jbig2_image_release(ctx, *GSPLANES.offset(j as isize));
            }
            jbig2_free((*ctx).allocator, GSPLANES as *mut core::ffi::c_void);
            return 0 as *mut *mut uint16_t;
        }
        i = i.wrapping_add(1);
    }
    rparams.MMR = GSMMR;
    rparams.GBTEMPLATE = GSTEMPLATE;
    rparams.TPGDON = 0 as core::ffi::c_int;
    rparams.USESKIP = GSUSESKIP;
    rparams.SKIP = GSKIP;
    rparams.gbat[0 as core::ffi::c_int as usize] = (if GSTEMPLATE
        <= 1 as core::ffi::c_int
    {
        3 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    }) as int8_t;
    rparams.gbat[1 as core::ffi::c_int as usize] = -(1 as core::ffi::c_int) as int8_t;
    rparams.gbat[2 as core::ffi::c_int as usize] = -(3 as core::ffi::c_int) as int8_t;
    rparams.gbat[3 as core::ffi::c_int as usize] = -(1 as core::ffi::c_int) as int8_t;
    rparams.gbat[4 as core::ffi::c_int as usize] = 2 as int8_t;
    rparams.gbat[5 as core::ffi::c_int as usize] = -(2 as core::ffi::c_int) as int8_t;
    rparams.gbat[6 as core::ffi::c_int as usize] = -(2 as core::ffi::c_int) as int8_t;
    rparams.gbat[7 as core::ffi::c_int as usize] = -(2 as core::ffi::c_int) as int8_t;
    if GSMMR != 0 {
        code = jbig2_decode_halftone_mmr(
            ctx,
            &mut rparams,
            data,
            size,
            *GSPLANES.offset(GSBPP.wrapping_sub(1 as uint32_t) as isize),
            &mut consumed_bytes,
        );
        current_block = 3275366147856559585;
    } else {
        ws = jbig2_word_stream_buf_new(ctx, data, size);
        if ws.is_null() {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to allocate word stream when decoding gray scale image\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            current_block = 9342823757441581283;
        } else {
            as_0 = jbig2_arith_new(ctx, ws);
            if as_0.is_null() {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"failed to allocate arithmetic coding state when decoding gray scale image\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                current_block = 9342823757441581283;
            } else {
                code = jbig2_decode_generic_region(
                    ctx,
                    segment,
                    &mut rparams,
                    as_0,
                    *GSPLANES.offset(GSBPP.wrapping_sub(1 as uint32_t) as isize),
                    GB_stats,
                );
                current_block = 3275366147856559585;
            }
        }
    }
    match current_block {
        3275366147856559585 => {
            if code < 0 as core::ffi::c_int {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"error decoding GSPLANES for halftone image\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {
                j = GSBPP.wrapping_sub(1 as uint32_t);
                loop {
                    if !(j > 0 as uint32_t) {
                        current_block = 1345366029464561491;
                        break;
                    }
                    j = j.wrapping_sub(1);
                    if GSMMR != 0 {
                        code = jbig2_decode_halftone_mmr(
                            ctx,
                            &mut rparams,
                            data.offset(consumed_bytes as isize),
                            size.wrapping_sub(consumed_bytes),
                            *GSPLANES.offset(j as isize),
                            &mut consumed_bytes,
                        );
                    } else {
                        code = jbig2_decode_generic_region(
                            ctx,
                            segment,
                            &mut rparams,
                            as_0,
                            *GSPLANES.offset(j as isize),
                            GB_stats,
                        );
                    }
                    if code < 0 as core::ffi::c_int {
                        jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to decode GSPLANES for halftone image\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                        current_block = 9342823757441581283;
                        break;
                    } else {
                        stride = (**GSPLANES.offset(j as isize)).stride;
                        i = 0 as uint32_t;
                        while i < stride.wrapping_mul(GSH) {
                            let ref mut fresh1 = *((**GSPLANES.offset(j as isize)).data)
                                .offset(i as isize);
                            *fresh1 = (*fresh1 as core::ffi::c_int
                                ^ *((**GSPLANES
                                    .offset(j.wrapping_add(1 as uint32_t) as isize))
                                    .data)
                                    .offset(i as isize) as core::ffi::c_int) as uint8_t;
                            i = i.wrapping_add(1);
                        }
                    }
                }
                match current_block {
                    9342823757441581283 => {}
                    _ => {
                        GSVALS = jbig2_alloc(
                            (*ctx).allocator,
                            GSW as size_t,
                            ::core::mem::size_of::<*mut uint16_t>() as size_t,
                        ) as *mut *mut uint16_t;
                        if GSVALS.is_null() {
                            jbig2_error(
                                ctx,
                                JBIG2_SEVERITY_FATAL,
                                (*segment).number,
                                b"failed to allocate GSVALS: %d bytes\0" as *const u8
                                    as *const core::ffi::c_char,
                                GSW,
                            );
                        } else {
                            i = 0 as uint32_t;
                            loop {
                                if !(i < GSW) {
                                    current_block = 6560072651652764009;
                                    break;
                                }
                                let ref mut fresh2 = *GSVALS.offset(i as isize);
                                *fresh2 = jbig2_alloc(
                                    (*ctx).allocator,
                                    GSH as size_t,
                                    ::core::mem::size_of::<uint16_t>() as size_t,
                                ) as *mut uint16_t;
                                if (*GSVALS.offset(i as isize)).is_null() {
                                    jbig2_error(
                                        ctx,
                                        JBIG2_SEVERITY_FATAL,
                                        (*segment).number,
                                        b"failed to allocate GSVALS: %d bytes\0" as *const u8
                                            as *const core::ffi::c_char,
                                        GSH.wrapping_mul(GSW),
                                    );
                                    j = i;
                                    while j > 0 as uint32_t {
                                        j = j.wrapping_sub(1);
                                        jbig2_free(
                                            (*ctx).allocator,
                                            *GSVALS.offset(j as isize) as *mut core::ffi::c_void,
                                        );
                                    }
                                    jbig2_free(
                                        (*ctx).allocator,
                                        GSVALS as *mut core::ffi::c_void,
                                    );
                                    GSVALS = 0 as *mut *mut uint16_t;
                                    current_block = 9342823757441581283;
                                    break;
                                } else {
                                    i = i.wrapping_add(1);
                                }
                            }
                            match current_block {
                                9342823757441581283 => {}
                                _ => {
                                    x = 0 as uint32_t;
                                    while x < GSW {
                                        y = 0 as uint32_t;
                                        while y < GSH {
                                            *(*GSVALS.offset(x as isize)).offset(y as isize) = 0
                                                as uint16_t;
                                            j = 0 as uint32_t;
                                            while j < GSBPP {
                                                let ref mut fresh3 = *(*GSVALS.offset(x as isize))
                                                    .offset(y as isize);
                                                *fresh3 = (*fresh3 as core::ffi::c_int
                                                    + (jbig2_image_get_pixel(
                                                        *GSPLANES.offset(j as isize),
                                                        x as core::ffi::c_int,
                                                        y as core::ffi::c_int,
                                                    ) << j)) as uint16_t;
                                                j = j.wrapping_add(1);
                                            }
                                            y = y.wrapping_add(1);
                                        }
                                        x = x.wrapping_add(1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if GSMMR == 0 {
        jbig2_free((*ctx).allocator, as_0 as *mut core::ffi::c_void);
        jbig2_word_stream_buf_free(ctx, ws);
    }
    i = 0 as uint32_t;
    while i < GSBPP {
        jbig2_image_release(ctx, *GSPLANES.offset(i as isize));
        i = i.wrapping_add(1);
    }
    jbig2_free((*ctx).allocator, GSPLANES as *mut core::ffi::c_void);
    return GSVALS;
}
unsafe extern "C" fn jbig2_decode_ht_region_get_hpats(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
) -> *mut Jbig2PatternDict {
    let mut index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut pattern_dict: *mut Jbig2PatternDict = 0 as *mut Jbig2PatternDict;
    let mut rsegment: *mut Jbig2Segment = 0 as *mut Jbig2Segment;
    while pattern_dict.is_null() && (*segment).referred_to_segment_count > index {
        rsegment = jbig2_find_segment(
            ctx,
            *((*segment).referred_to_segments).offset(index as isize),
        );
        if !rsegment.is_null() {
            if (*rsegment).flags as core::ffi::c_int & 0x3f as core::ffi::c_int
                == 16 as core::ffi::c_int && !((*rsegment).result).is_null()
            {
                pattern_dict = (*rsegment).result as *mut Jbig2PatternDict;
                return pattern_dict;
            }
        }
        index += 1;
    }
    return pattern_dict;
}
unsafe extern "C" fn jbig2_decode_halftone_region(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *mut Jbig2HalftoneRegionParams,
    mut data: *const byte,
    size: size_t,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let mut HBPP: uint32_t = 0;
    let mut HNUMPATS: uint32_t = 0;
    let mut GI: *mut *mut uint16_t = 0 as *mut *mut uint16_t;
    let mut HSKIP: *mut Jbig2Image = 0 as *mut Jbig2Image;
    let mut HPATS: *mut Jbig2PatternDict = 0 as *mut Jbig2PatternDict;
    let mut i: uint32_t = 0;
    let mut mg: uint32_t = 0;
    let mut ng: uint32_t = 0;
    let mut gray_val: uint16_t = 0;
    let mut code: core::ffi::c_int = 0 as core::ffi::c_int;
    HPATS = jbig2_decode_ht_region_get_hpats(ctx, segment);
    if HPATS.is_null() {
        code = jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"no pattern dictionary found, skipping halftone image\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {
        memset(
            (*image).data as *mut core::ffi::c_void,
            (*params).HDEFPIXEL,
            ((*image).stride).wrapping_mul((*image).height) as size_t,
        );
        if (*params).HENABLESKIP == 1 as core::ffi::c_int {
            HSKIP = jbig2_image_new(ctx, (*params).HGW, (*params).HGH);
            if HSKIP.is_null() {
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"failed to allocate skip image\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            mg = 0 as uint32_t;
            while mg < (*params).HGH {
                ng = 0 as uint32_t;
                while ng < (*params).HGW {
                    let mut x: int64_t = (*params).HGX as int64_t
                        + mg.wrapping_mul((*params).HRY as uint32_t) as int64_t
                        + ng.wrapping_mul((*params).HRX as uint32_t) as int64_t
                        >> 8 as core::ffi::c_int;
                    let mut y: int64_t = (*params).HGY as int64_t
                        + mg.wrapping_mul((*params).HRX as uint32_t) as int64_t
                        - ng.wrapping_mul((*params).HRY as uint32_t) as int64_t
                        >> 8 as core::ffi::c_int;
                    if x + (*HPATS).HPW as int64_t <= 0 as int64_t
                        || x >= (*image).width as int64_t
                        || y + (*HPATS).HPH as int64_t <= 0 as int64_t
                        || y >= (*image).height as int64_t
                    {
                        jbig2_image_set_pixel(
                            HSKIP,
                            ng as core::ffi::c_int,
                            mg as core::ffi::c_int,
                            1 as core::ffi::c_int,
                        );
                    } else {
                        jbig2_image_set_pixel(
                            HSKIP,
                            ng as core::ffi::c_int,
                            mg as core::ffi::c_int,
                            0 as core::ffi::c_int,
                        );
                    }
                    ng = ng.wrapping_add(1);
                }
                mg = mg.wrapping_add(1);
            }
        }
        HNUMPATS = (*HPATS).n_patterns as uint32_t;
        HBPP = 0 as uint32_t;
        loop {
            HBPP = HBPP.wrapping_add(1);
            if !(HNUMPATS > (1 as uint32_t) << HBPP) {
                break;
            }
        }
        if HBPP > 16 as uint32_t {
            code = jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                (*segment).number,
                b"HBPP is larger than supported (%u)\0" as *const u8
                    as *const core::ffi::c_char,
                HBPP,
            );
        } else {
            GI = jbig2_decode_gray_scale_image(
                ctx,
                segment,
                data,
                size,
                (*params).HMMR,
                (*params).HGW,
                (*params).HGH,
                HBPP,
                (*params).HENABLESKIP,
                HSKIP,
                (*params).HTEMPLATE,
                GB_stats,
            );
            if GI.is_null() {
                code = jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"unable to acquire gray-scale image, skipping halftone image\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {
                mg = 0 as uint32_t;
                's_147: while mg < (*params).HGH {
                    ng = 0 as uint32_t;
                    while ng < (*params).HGW {
                        let mut x_0: int64_t = (*params).HGX as int64_t
                            + mg.wrapping_mul((*params).HRY as uint32_t) as int64_t
                            + ng.wrapping_mul((*params).HRX as uint32_t) as int64_t
                            >> 8 as core::ffi::c_int;
                        let mut y_0: int64_t = (*params).HGY as int64_t
                            + mg.wrapping_mul((*params).HRX as uint32_t) as int64_t
                            - ng.wrapping_mul((*params).HRY as uint32_t) as int64_t
                            >> 8 as core::ffi::c_int;
                        gray_val = *(*GI.offset(ng as isize)).offset(mg as isize);
                        if gray_val as uint32_t >= HNUMPATS {
                            jbig2_error(
                                ctx,
                                JBIG2_SEVERITY_WARNING,
                                (*segment).number,
                                b"gray-scale index %d out of range, using largest index\0"
                                    as *const u8 as *const core::ffi::c_char,
                                gray_val as core::ffi::c_int,
                            );
                            gray_val = HNUMPATS.wrapping_sub(1 as uint32_t) as uint16_t;
                        }
                        code = jbig2_image_compose(
                            ctx,
                            image,
                            *((*HPATS).patterns).offset(gray_val as isize),
                            x_0 as core::ffi::c_int,
                            y_0 as core::ffi::c_int,
                            (*params).HCOMBOP,
                        );
                        if code < 0 as core::ffi::c_int {
                            code = jbig2_error(
                                ctx,
                                JBIG2_SEVERITY_WARNING,
                                (*segment).number,
                                b"failed to compose pattern with gray-scale image\0"
                                    as *const u8 as *const core::ffi::c_char,
                            );
                            break 's_147;
                        } else {
                            ng = ng.wrapping_add(1);
                        }
                    }
                    mg = mg.wrapping_add(1);
                }
            }
        }
    }
    if !GI.is_null() {
        i = 0 as uint32_t;
        while i < (*params).HGW {
            jbig2_free(
                (*ctx).allocator,
                *GI.offset(i as isize) as *mut core::ffi::c_void,
            );
            i = i.wrapping_add(1);
        }
    }
    jbig2_free((*ctx).allocator, GI as *mut core::ffi::c_void);
    jbig2_image_release(ctx, HSKIP);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_halftone_region(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut segment_data: *const byte,
) -> core::ffi::c_int {
    let mut offset: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut region_info: Jbig2RegionSegmentInfo = Jbig2RegionSegmentInfo {
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        op: JBIG2_COMPOSE_OR,
        flags: 0,
    };
    let mut params: Jbig2HalftoneRegionParams = Jbig2HalftoneRegionParams {
        flags: 0,
        HGW: 0,
        HGH: 0,
        HGX: 0,
        HGY: 0,
        HRX: 0,
        HRY: 0,
        HMMR: 0,
        HTEMPLATE: 0,
        HENABLESKIP: 0,
        HCOMBOP: JBIG2_COMPOSE_OR,
        HDEFPIXEL: 0,
    };
    let mut image: *mut Jbig2Image = 0 as *mut Jbig2Image;
    let mut GB_stats: *mut Jbig2ArithCx = 0 as *mut Jbig2ArithCx;
    let mut code: core::ffi::c_int = 0 as core::ffi::c_int;
    if !((*segment).data_length < 17 as size_t) {
        jbig2_get_region_segment_info(&mut region_info, segment_data as *const uint8_t);
        offset += 17 as core::ffi::c_int;
        if !((*segment).data_length < 18 as size_t) {
            params.flags = *segment_data.offset(offset as isize);
            params.HMMR = params.flags as core::ffi::c_int & 1 as core::ffi::c_int;
            params.HTEMPLATE = (params.flags as core::ffi::c_int & 6 as core::ffi::c_int)
                >> 1 as core::ffi::c_int;
            params.HENABLESKIP = (params.flags as core::ffi::c_int
                & 8 as core::ffi::c_int) >> 3 as core::ffi::c_int;
            params.HCOMBOP = ((params.flags as core::ffi::c_int
                & 0x70 as core::ffi::c_int) >> 4 as core::ffi::c_int) as Jbig2ComposeOp;
            params.HDEFPIXEL = (params.flags as core::ffi::c_int
                & 0x80 as core::ffi::c_int) >> 7 as core::ffi::c_int;
            offset += 1 as core::ffi::c_int;
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_INFO,
                (*segment).number,
                b"halftone region: %u x %u @ (%u, %u), flags = %02x\0" as *const u8
                    as *const core::ffi::c_char,
                region_info.width,
                region_info.height,
                region_info.x,
                region_info.y,
                params.flags as core::ffi::c_int,
            );
            if params.HMMR != 0 && params.HTEMPLATE != 0 {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"HTEMPLATE is %d when HMMR is %d, contrary to spec\0" as *const u8
                        as *const core::ffi::c_char,
                    params.HTEMPLATE,
                    params.HMMR,
                );
            }
            if params.HMMR != 0 && params.HENABLESKIP != 0 {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"HENABLESKIP is %d when HMMR is %d, contrary to spec\0" as *const u8
                        as *const core::ffi::c_char,
                    params.HENABLESKIP,
                    params.HMMR,
                );
            }
            if !(((*segment).data_length).wrapping_sub(offset as size_t) < 16 as size_t)
            {
                params.HGW = jbig2_get_uint32(segment_data.offset(offset as isize));
                params.HGH = jbig2_get_uint32(
                    segment_data
                        .offset(offset as isize)
                        .offset(4 as core::ffi::c_int as isize),
                );
                params.HGX = jbig2_get_int32(
                    segment_data
                        .offset(offset as isize)
                        .offset(8 as core::ffi::c_int as isize),
                );
                params.HGY = jbig2_get_int32(
                    segment_data
                        .offset(offset as isize)
                        .offset(12 as core::ffi::c_int as isize),
                );
                offset += 16 as core::ffi::c_int;
                if !(((*segment).data_length).wrapping_sub(offset as size_t)
                    < 4 as size_t)
                {
                    params.HRX = jbig2_get_uint16(segment_data.offset(offset as isize));
                    params.HRY = jbig2_get_uint16(
                        segment_data
                            .offset(offset as isize)
                            .offset(2 as core::ffi::c_int as isize),
                    );
                    offset += 4 as core::ffi::c_int;
                    jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_INFO,
                        (*segment).number,
                        b"grid %d x %d @ (%d.%d,%d.%d) vector (%d.%d,%d.%d)\0"
                            as *const u8 as *const core::ffi::c_char,
                        params.HGW,
                        params.HGH,
                        params.HGX >> 8 as core::ffi::c_int,
                        params.HGX & 0xff as int32_t,
                        params.HGY >> 8 as core::ffi::c_int,
                        params.HGY & 0xff as int32_t,
                        params.HRX as core::ffi::c_int >> 8 as core::ffi::c_int,
                        params.HRX as core::ffi::c_int & 0xff as core::ffi::c_int,
                        params.HRY as core::ffi::c_int >> 8 as core::ffi::c_int,
                        params.HRY as core::ffi::c_int & 0xff as core::ffi::c_int,
                    );
                    if params.HMMR == 0 {
                        let mut stats_size: core::ffi::c_int = jbig2_generic_stats_size(
                            ctx,
                            params.HTEMPLATE,
                        );
                        GB_stats = jbig2_alloc(
                            (*ctx).allocator,
                            stats_size as size_t,
                            ::core::mem::size_of::<Jbig2ArithCx>() as size_t,
                        ) as *mut Jbig2ArithCx;
                        if GB_stats.is_null() {
                            return jbig2_error(
                                ctx,
                                JBIG2_SEVERITY_FATAL,
                                (*segment).number,
                                b"failed to allocate arithmetic decoder states in halftone region\0"
                                    as *const u8 as *const core::ffi::c_char,
                            );
                        }
                        memset(
                            GB_stats as *mut core::ffi::c_void,
                            0 as core::ffi::c_int,
                            stats_size as size_t,
                        );
                    }
                    image = jbig2_image_new(ctx, region_info.width, region_info.height);
                    if image.is_null() {
                        jbig2_free((*ctx).allocator, GB_stats as *mut core::ffi::c_void);
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to allocate halftone image\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    code = jbig2_decode_halftone_region(
                        ctx,
                        segment,
                        &mut params,
                        segment_data.offset(offset as isize),
                        ((*segment).data_length).wrapping_sub(offset as size_t),
                        image,
                        GB_stats,
                    );
                    if code < 0 as core::ffi::c_int {
                        jbig2_image_release(ctx, image);
                        jbig2_free((*ctx).allocator, GB_stats as *mut core::ffi::c_void);
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to decode halftone region\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if params.HMMR == 0 {
                        jbig2_free((*ctx).allocator, GB_stats as *mut core::ffi::c_void);
                    }
                    code = jbig2_page_add_result(
                        ctx,
                        &mut *((*ctx).pages).offset((*ctx).current_page as isize),
                        image,
                        region_info.x,
                        region_info.y,
                        region_info.op,
                    );
                    if code < 0 as core::ffi::c_int {
                        jbig2_image_release(ctx, image);
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"unable to add halftone region to page\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    jbig2_image_release(ctx, image);
                    return code;
                }
            }
        }
    }
    return jbig2_error(
        ctx,
        JBIG2_SEVERITY_FATAL,
        (*segment).number,
        b"segment too short\0" as *const u8 as *const core::ffi::c_char,
    );
}

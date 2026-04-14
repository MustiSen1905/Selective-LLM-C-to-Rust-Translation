extern "C" {
    pub type _Jbig2ArithState;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
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
    fn jbig2_arith_decode(
        ctx: *mut Jbig2Ctx,
        as_0: *mut Jbig2ArithState,
        pcx: *mut Jbig2ArithCx,
    ) -> core::ffi::c_int;
    fn jbig2_image_new(
        ctx: *mut Jbig2Ctx,
        width: uint32_t,
        height: uint32_t,
    ) -> *mut Jbig2Image;
    fn jbig2_image_release(ctx: *mut Jbig2Ctx, image: *mut Jbig2Image);
    fn jbig2_image_get_pixel(
        image: *mut Jbig2Image,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn jbig2_decode_generic_mmr(
        ctx: *mut Jbig2Ctx,
        segment: *mut Jbig2Segment,
        params: *const Jbig2GenericRegionParams,
        data: *const byte,
        size: size_t,
        image: *mut Jbig2Image,
    ) -> core::ffi::c_int;
    fn jbig2_page_add_result(
        ctx: *mut Jbig2Ctx,
        page: *mut Jbig2Page,
        src: *mut Jbig2Image,
        x: uint32_t,
        y: uint32_t,
        op: Jbig2ComposeOp,
    ) -> core::ffi::c_int;
    fn jbig2_get_region_segment_info(
        info: *mut Jbig2RegionSegmentInfo,
        segment_data: *const uint8_t,
    );
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int8_t = __int8_t;
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
pub struct Jbig2RegionSegmentInfo {
    pub width: uint32_t,
    pub height: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,
    pub op: Jbig2ComposeOp,
    pub flags: uint8_t,
}
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
unsafe extern "C" fn jbig2_image_get_pixel_fast(
    mut image: *mut Jbig2Image,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
) -> core::ffi::c_int {
    let byte: core::ffi::c_int = ((x >> 3 as core::ffi::c_int) as uint32_t)
        .wrapping_add((y as uint32_t).wrapping_mul((*image).stride)) as core::ffi::c_int;
    let bit: core::ffi::c_int = 7 as core::ffi::c_int - (x & 7 as core::ffi::c_int);
    return *((*image).data).offset(byte as isize) as core::ffi::c_int >> bit
        & 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_generic_stats_size(
    mut ctx: *mut Jbig2Ctx,
    mut template: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut stats_size: core::ffi::c_int = if template == 0 as core::ffi::c_int {
        (1 as core::ffi::c_int) << 16 as core::ffi::c_int
    } else if template == 1 as core::ffi::c_int {
        (1 as core::ffi::c_int) << 13 as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 10 as core::ffi::c_int
    };
    return stats_size;
}
unsafe extern "C" fn jbig2_decode_generic_template0(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let rowstride: uint32_t = (*image).stride;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut line2: *mut byte = 0 as *mut byte;
    let mut line1: *mut byte = 0 as *mut byte;
    let mut gbreg_line: *mut byte = (*image).data as *mut byte;
    if GBW <= 0 as uint32_t {
        return 0 as core::ffi::c_int;
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut CONTEXT: uint32_t = 0;
        let mut line_m1: uint32_t = 0;
        let mut line_m2: uint32_t = 0;
        let mut padded_width: uint32_t = GBW.wrapping_add(7 as uint32_t)
            & -(8 as core::ffi::c_int) as uint32_t;
        line_m1 = (if !line1.is_null() {
            *line1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        line_m2 = (if !line2.is_null() {
            (*line2.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                << 6 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        CONTEXT = line_m1 & 0x7f0 as uint32_t | line_m2 & 0xf800 as uint32_t;
        x = 0 as uint32_t;
        while x < padded_width {
            let mut result: byte = 0 as byte;
            let mut x_minor: core::ffi::c_int = 0;
            let mut minor_width: core::ffi::c_int = (if GBW.wrapping_sub(x)
                > 8 as uint32_t
            {
                8 as uint32_t
            } else {
                GBW.wrapping_sub(x)
            }) as core::ffi::c_int;
            if !line1.is_null() {
                line_m1 = line_m1 << 8 as core::ffi::c_int
                    | (if x.wrapping_add(8 as uint32_t) < GBW {
                        *line1
                            .offset(
                                (x >> 3 as core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t;
            }
            if !line2.is_null() {
                line_m2 = line_m2 << 8 as core::ffi::c_int
                    | (if x.wrapping_add(8 as uint32_t) < GBW {
                        (*line2
                            .offset(
                                (x >> 3 as core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) as core::ffi::c_int) << 6 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t;
            }
            x_minor = 0 as core::ffi::c_int;
            while x_minor < minor_width {
                let mut bit: core::ffi::c_int = 0;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template0 optimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                result = (result as core::ffi::c_int
                    | bit << 7 as core::ffi::c_int - x_minor) as byte;
                CONTEXT = (CONTEXT & 0x7bf7 as uint32_t) << 1 as core::ffi::c_int
                    | bit as uint32_t
                    | line_m1 >> 7 as core::ffi::c_int - x_minor & 0x10 as uint32_t
                    | line_m2 >> 7 as core::ffi::c_int - x_minor & 0x800 as uint32_t;
                x_minor += 1;
            }
            *gbreg_line.offset((x >> 3 as core::ffi::c_int) as isize) = result;
            x = x.wrapping_add(8 as uint32_t);
        }
        line2 = line1;
        line1 = gbreg_line;
        gbreg_line = gbreg_line.offset(rowstride as isize);
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template0_unopt(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut bit: core::ffi::c_int = 0;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
        || (((*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
            || (*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            || ((*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int)
                < -(128 as core::ffi::c_int)
            || ((*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int)
                < 0 as core::ffi::c_int
                && (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int
                    > 127 as core::ffi::c_int
            || (*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int
                    >= 0 as core::ffi::c_int)
        || (((*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
            || (*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            || ((*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int)
                < -(128 as core::ffi::c_int)
            || ((*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int)
                < 0 as core::ffi::c_int
                && (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int
                    > 127 as core::ffi::c_int
            || (*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int
                    >= 0 as core::ffi::c_int)
        || (((*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
            || (*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            || ((*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int)
                < -(128 as core::ffi::c_int)
            || ((*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int)
                < 0 as core::ffi::c_int
                && (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int
                    > 127 as core::ffi::c_int
            || (*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int
                    >= 0 as core::ffi::c_int)
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut out_byte: uint32_t = 0 as uint32_t;
        let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
        let mut d: *mut uint8_t = &mut *((*image).data)
            .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
        let mut pd: uint32_t = 0 as uint32_t;
        let mut ppd: uint32_t = 0 as uint32_t;
        let mut pline: *mut uint8_t = 0 as *mut uint8_t;
        let mut ppline: *mut uint8_t = 0 as *mut uint8_t;
        if y >= 1 as uint32_t {
            pline = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let fresh15 = pline;
            pline = pline.offset(1);
            pd = ((*fresh15 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint32_t;
            if GBW > 8 as uint32_t {
                let fresh16 = pline;
                pline = pline.offset(1);
                pd |= *fresh16 as uint32_t;
            }
        }
        if y >= 2 as uint32_t {
            ppline = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(2 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let fresh17 = ppline;
            ppline = ppline.offset(1);
            ppd = ((*fresh17 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint32_t;
            if GBW > 8 as uint32_t {
                let fresh18 = ppline;
                ppline = ppline.offset(1);
                ppd |= *fresh18 as uint32_t;
            }
        }
        x = 0 as uint32_t;
        while x < GBW {
            if (*params).USESKIP != 0
                && jbig2_image_get_pixel(
                    (*params).SKIP,
                    x as core::ffi::c_int,
                    y as core::ffi::c_int,
                ) != 0
            {
                bit = 0 as core::ffi::c_int;
            } else {
                CONTEXT = out_byte & 0xf as uint32_t;
                CONTEXT
                    |= (jbig2_image_get_pixel(
                        image,
                        x
                            .wrapping_add(
                                (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                        y
                            .wrapping_add(
                                (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                    ) << 4 as core::ffi::c_int) as uint32_t;
                CONTEXT |= pd >> 8 as core::ffi::c_int & 0x3e0 as uint32_t;
                CONTEXT
                    |= (jbig2_image_get_pixel(
                        image,
                        x
                            .wrapping_add(
                                (*params).gbat[2 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                        y
                            .wrapping_add(
                                (*params).gbat[3 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                    ) << 10 as core::ffi::c_int) as uint32_t;
                CONTEXT
                    |= (jbig2_image_get_pixel(
                        image,
                        x
                            .wrapping_add(
                                (*params).gbat[4 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                        y
                            .wrapping_add(
                                (*params).gbat[5 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                    ) << 11 as core::ffi::c_int) as uint32_t;
                CONTEXT |= ppd >> 2 as core::ffi::c_int & 0x7000 as uint32_t;
                CONTEXT
                    |= (jbig2_image_get_pixel(
                        image,
                        x
                            .wrapping_add(
                                (*params).gbat[6 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                        y
                            .wrapping_add(
                                (*params).gbat[7 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                    ) << 15 as core::ffi::c_int) as uint32_t;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template0 unoptimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
            }
            pd = pd << 1 as core::ffi::c_int;
            ppd = ppd << 1 as core::ffi::c_int;
            out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
            out_bits_to_go_in_byte -= 1;
            *d = (out_byte << out_bits_to_go_in_byte) as uint8_t;
            if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                d = d.offset(1);
                if x.wrapping_add(9 as uint32_t) < GBW && !pline.is_null() {
                    let fresh19 = pline;
                    pline = pline.offset(1);
                    pd |= *fresh19 as uint32_t;
                    if !ppline.is_null() {
                        let fresh20 = ppline;
                        ppline = ppline.offset(1);
                        ppd |= *fresh20 as uint32_t;
                    }
                }
            }
            x = x.wrapping_add(1);
        }
        if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
            *d = ((out_byte as uint8_t as core::ffi::c_int) << out_bits_to_go_in_byte)
                as uint8_t;
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template1_unopt(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut bit: core::ffi::c_int = 0;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut out_byte: uint32_t = 0 as uint32_t;
        let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
        let mut d: *mut uint8_t = &mut *((*image).data)
            .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
        let mut pd: uint32_t = 0 as uint32_t;
        let mut ppd: uint32_t = 0 as uint32_t;
        let mut pline: *mut uint8_t = 0 as *mut uint8_t;
        let mut ppline: *mut uint8_t = 0 as *mut uint8_t;
        if y >= 1 as uint32_t {
            pline = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let fresh9 = pline;
            pline = pline.offset(1);
            pd = ((*fresh9 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint32_t;
            if GBW > 8 as uint32_t {
                let fresh10 = pline;
                pline = pline.offset(1);
                pd |= *fresh10 as uint32_t;
            }
        }
        if y >= 2 as uint32_t {
            ppline = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(2 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let fresh11 = ppline;
            ppline = ppline.offset(1);
            ppd = ((*fresh11 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint32_t;
            if GBW > 8 as uint32_t {
                let fresh12 = ppline;
                ppline = ppline.offset(1);
                ppd |= *fresh12 as uint32_t;
            }
        }
        x = 0 as uint32_t;
        while x < GBW {
            if (*params).USESKIP != 0
                && jbig2_image_get_pixel(
                    (*params).SKIP,
                    x as core::ffi::c_int,
                    y as core::ffi::c_int,
                ) != 0
            {
                bit = 0 as core::ffi::c_int;
            } else {
                CONTEXT = out_byte & 0x7 as uint32_t;
                CONTEXT
                    |= (jbig2_image_get_pixel(
                        image,
                        x
                            .wrapping_add(
                                (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                        y
                            .wrapping_add(
                                (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                    ) << 3 as core::ffi::c_int) as uint32_t;
                CONTEXT |= pd >> 9 as core::ffi::c_int & 0x1f0 as uint32_t;
                CONTEXT |= ppd >> 4 as core::ffi::c_int & 0x1e00 as uint32_t;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template1 unoptimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
            }
            pd = pd << 1 as core::ffi::c_int;
            ppd = ppd << 1 as core::ffi::c_int;
            out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
            out_bits_to_go_in_byte -= 1;
            *d = (out_byte << out_bits_to_go_in_byte) as uint8_t;
            if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                d = d.offset(1);
                if x.wrapping_add(9 as uint32_t) < GBW && !pline.is_null() {
                    let fresh13 = pline;
                    pline = pline.offset(1);
                    pd |= *fresh13 as uint32_t;
                    if !ppline.is_null() {
                        let fresh14 = ppline;
                        ppline = ppline.offset(1);
                        ppd |= *fresh14 as uint32_t;
                    }
                }
            }
            x = x.wrapping_add(1);
        }
        if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
            *d = ((out_byte as uint8_t as core::ffi::c_int) << out_bits_to_go_in_byte)
                as uint8_t;
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template1(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let rowstride: uint32_t = (*image).stride;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut line2: *mut byte = 0 as *mut byte;
    let mut line1: *mut byte = 0 as *mut byte;
    let mut gbreg_line: *mut byte = (*image).data as *mut byte;
    if GBW <= 0 as uint32_t {
        return 0 as core::ffi::c_int;
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut CONTEXT: uint32_t = 0;
        let mut line_m1: uint32_t = 0;
        let mut line_m2: uint32_t = 0;
        let mut padded_width: uint32_t = GBW.wrapping_add(7 as uint32_t)
            & -(8 as core::ffi::c_int) as uint32_t;
        line_m1 = (if !line1.is_null() {
            *line1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        line_m2 = (if !line2.is_null() {
            (*line2.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                << 5 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        CONTEXT = line_m1 >> 1 as core::ffi::c_int & 0x1f8 as uint32_t
            | line_m2 >> 1 as core::ffi::c_int & 0x1e00 as uint32_t;
        x = 0 as uint32_t;
        while x < padded_width {
            let mut result: byte = 0 as byte;
            let mut x_minor: core::ffi::c_int = 0;
            let mut minor_width: core::ffi::c_int = (if GBW.wrapping_sub(x)
                > 8 as uint32_t
            {
                8 as uint32_t
            } else {
                GBW.wrapping_sub(x)
            }) as core::ffi::c_int;
            if !line1.is_null() {
                line_m1 = line_m1 << 8 as core::ffi::c_int
                    | (if x.wrapping_add(8 as uint32_t) < GBW {
                        *line1
                            .offset(
                                (x >> 3 as core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t;
            }
            if !line2.is_null() {
                line_m2 = line_m2 << 8 as core::ffi::c_int
                    | (if x.wrapping_add(8 as uint32_t) < GBW {
                        (*line2
                            .offset(
                                (x >> 3 as core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) as core::ffi::c_int) << 5 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t;
            }
            x_minor = 0 as core::ffi::c_int;
            while x_minor < minor_width {
                let mut bit: core::ffi::c_int = 0;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template1 optimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                result = (result as core::ffi::c_int
                    | bit << 7 as core::ffi::c_int - x_minor) as byte;
                CONTEXT = (CONTEXT & 0xefb as uint32_t) << 1 as core::ffi::c_int
                    | bit as uint32_t
                    | line_m1 >> 8 as core::ffi::c_int - x_minor & 0x8 as uint32_t
                    | line_m2 >> 8 as core::ffi::c_int - x_minor & 0x200 as uint32_t;
                x_minor += 1;
            }
            *gbreg_line.offset((x >> 3 as core::ffi::c_int) as isize) = result;
            x = x.wrapping_add(8 as uint32_t);
        }
        line2 = line1;
        line1 = gbreg_line;
        gbreg_line = gbreg_line.offset(rowstride as isize);
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template2_unopt(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut bit: core::ffi::c_int = 0;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut out_byte: uint32_t = 0 as uint32_t;
        let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
        let mut d: *mut uint8_t = &mut *((*image).data)
            .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
        let mut pline: *mut uint8_t = &mut *((*image).data)
            .offset(
                ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t)) as isize,
            ) as *mut uint8_t;
        let mut ppline: *mut uint8_t = &mut *((*image).data)
            .offset(
                ((*image).stride).wrapping_mul(y.wrapping_sub(2 as uint32_t)) as isize,
            ) as *mut uint8_t;
        let mut pd: uint32_t = 0 as uint32_t;
        let mut ppd: uint32_t = 0 as uint32_t;
        if y > 0 as uint32_t {
            let fresh3 = pline;
            pline = pline.offset(1);
            pd = ((*fresh3 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint32_t;
            if GBW > 8 as uint32_t {
                let fresh4 = pline;
                pline = pline.offset(1);
                pd |= *fresh4 as uint32_t;
            }
            if y > 1 as uint32_t {
                let fresh5 = ppline;
                ppline = ppline.offset(1);
                ppd = ((*fresh5 as core::ffi::c_int) << 8 as core::ffi::c_int)
                    as uint32_t;
                if GBW > 8 as uint32_t {
                    let fresh6 = ppline;
                    ppline = ppline.offset(1);
                    ppd |= *fresh6 as uint32_t;
                }
            }
        }
        x = 0 as uint32_t;
        while x < GBW {
            if (*params).USESKIP != 0
                && jbig2_image_get_pixel(
                    (*params).SKIP,
                    x as core::ffi::c_int,
                    y as core::ffi::c_int,
                ) != 0
            {
                bit = 0 as core::ffi::c_int;
            } else {
                CONTEXT = out_byte & 0x3 as uint32_t;
                CONTEXT
                    |= (jbig2_image_get_pixel(
                        image,
                        x
                            .wrapping_add(
                                (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                        y
                            .wrapping_add(
                                (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                    ) << 2 as core::ffi::c_int) as uint32_t;
                CONTEXT |= pd >> 11 as core::ffi::c_int & 0x78 as uint32_t;
                CONTEXT |= ppd >> 7 as core::ffi::c_int & 0x380 as uint32_t;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template2 unoptimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
            }
            pd = pd << 1 as core::ffi::c_int;
            ppd = ppd << 1 as core::ffi::c_int;
            out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
            out_bits_to_go_in_byte -= 1;
            *d = ((out_byte as uint8_t as core::ffi::c_int) << out_bits_to_go_in_byte)
                as uint8_t;
            if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                d = d.offset(1);
                if x.wrapping_add(9 as uint32_t) < GBW && y > 0 as uint32_t {
                    let fresh7 = pline;
                    pline = pline.offset(1);
                    pd |= *fresh7 as uint32_t;
                    if y > 1 as uint32_t {
                        let fresh8 = ppline;
                        ppline = ppline.offset(1);
                        ppd |= *fresh8 as uint32_t;
                    }
                }
            }
            x = x.wrapping_add(1);
        }
        if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
            *d = ((out_byte as uint8_t as core::ffi::c_int) << out_bits_to_go_in_byte)
                as uint8_t;
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template2(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let rowstride: uint32_t = (*image).stride;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut line2: *mut byte = 0 as *mut byte;
    let mut line1: *mut byte = 0 as *mut byte;
    let mut gbreg_line: *mut byte = (*image).data as *mut byte;
    if GBW <= 0 as uint32_t {
        return 0 as core::ffi::c_int;
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut CONTEXT: uint32_t = 0;
        let mut line_m1: uint32_t = 0;
        let mut line_m2: uint32_t = 0;
        let mut padded_width: uint32_t = GBW.wrapping_add(7 as uint32_t)
            & -(8 as core::ffi::c_int) as uint32_t;
        line_m1 = (if !line1.is_null() {
            *line1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        line_m2 = (if !line2.is_null() {
            (*line2.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                << 4 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        CONTEXT = line_m1 >> 3 as core::ffi::c_int & 0x7c as uint32_t
            | line_m2 >> 3 as core::ffi::c_int & 0x380 as uint32_t;
        x = 0 as uint32_t;
        while x < padded_width {
            let mut result: byte = 0 as byte;
            let mut x_minor: core::ffi::c_int = 0;
            let mut minor_width: core::ffi::c_int = (if GBW.wrapping_sub(x)
                > 8 as uint32_t
            {
                8 as uint32_t
            } else {
                GBW.wrapping_sub(x)
            }) as core::ffi::c_int;
            if !line1.is_null() {
                line_m1 = line_m1 << 8 as core::ffi::c_int
                    | (if x.wrapping_add(8 as uint32_t) < GBW {
                        *line1
                            .offset(
                                (x >> 3 as core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t;
            }
            if !line2.is_null() {
                line_m2 = line_m2 << 8 as core::ffi::c_int
                    | (if x.wrapping_add(8 as uint32_t) < GBW {
                        (*line2
                            .offset(
                                (x >> 3 as core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) as core::ffi::c_int) << 4 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t;
            }
            x_minor = 0 as core::ffi::c_int;
            while x_minor < minor_width {
                let mut bit: core::ffi::c_int = 0;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template2 optimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                result = (result as core::ffi::c_int
                    | bit << 7 as core::ffi::c_int - x_minor) as byte;
                CONTEXT = (CONTEXT & 0x1bd as uint32_t) << 1 as core::ffi::c_int
                    | bit as uint32_t
                    | line_m1 >> 10 as core::ffi::c_int - x_minor & 0x4 as uint32_t
                    | line_m2 >> 10 as core::ffi::c_int - x_minor & 0x80 as uint32_t;
                x_minor += 1;
            }
            *gbreg_line.offset((x >> 3 as core::ffi::c_int) as isize) = result;
            x = x.wrapping_add(8 as uint32_t);
        }
        line2 = line1;
        line1 = gbreg_line;
        gbreg_line = gbreg_line.offset(rowstride as isize);
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template3(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let rowstride: uint32_t = (*image).stride;
    let mut line1: *mut byte = 0 as *mut byte;
    let mut gbreg_line: *mut byte = (*image).data as *mut byte;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    if GBW <= 0 as uint32_t {
        return 0 as core::ffi::c_int;
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut CONTEXT: uint32_t = 0;
        let mut line_m1: uint32_t = 0;
        let mut padded_width: uint32_t = GBW.wrapping_add(7 as uint32_t)
            & -(8 as core::ffi::c_int) as uint32_t;
        line_m1 = (if !line1.is_null() {
            *line1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        CONTEXT = line_m1 >> 1 as core::ffi::c_int & 0x3f0 as uint32_t;
        x = 0 as uint32_t;
        while x < padded_width {
            let mut result: byte = 0 as byte;
            let mut x_minor: core::ffi::c_int = 0;
            let mut minor_width: core::ffi::c_int = (if GBW.wrapping_sub(x)
                > 8 as uint32_t
            {
                8 as uint32_t
            } else {
                GBW.wrapping_sub(x)
            }) as core::ffi::c_int;
            if !line1.is_null() {
                line_m1 = line_m1 << 8 as core::ffi::c_int
                    | (if x.wrapping_add(8 as uint32_t) < GBW {
                        *line1
                            .offset(
                                (x >> 3 as core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t;
            }
            x_minor = 0 as core::ffi::c_int;
            while x_minor < minor_width {
                let mut bit: core::ffi::c_int = 0;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template3 optimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                result = (result as core::ffi::c_int
                    | bit << 7 as core::ffi::c_int - x_minor) as byte;
                CONTEXT = (CONTEXT & 0x1f7 as uint32_t) << 1 as core::ffi::c_int
                    | bit as uint32_t
                    | line_m1 >> 8 as core::ffi::c_int - x_minor & 0x10 as uint32_t;
                x_minor += 1;
            }
            *gbreg_line.offset((x >> 3 as core::ffi::c_int) as isize) = result;
            x = x.wrapping_add(8 as uint32_t);
        }
        line1 = gbreg_line;
        gbreg_line = gbreg_line.offset(rowstride as isize);
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template3_unopt(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut bit: core::ffi::c_int = 0;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut out_byte: uint32_t = 0 as uint32_t;
        let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
        let mut d: *mut uint8_t = &mut *((*image).data)
            .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
        let mut pline: *mut uint8_t = &mut *((*image).data)
            .offset(
                ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t)) as isize,
            ) as *mut uint8_t;
        let mut pd: uint32_t = 0 as uint32_t;
        if y > 0 as uint32_t {
            let fresh0 = pline;
            pline = pline.offset(1);
            pd = ((*fresh0 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint32_t;
            if GBW > 8 as uint32_t {
                let fresh1 = pline;
                pline = pline.offset(1);
                pd |= *fresh1 as uint32_t;
            }
        }
        x = 0 as uint32_t;
        while x < GBW {
            if (*params).USESKIP != 0
                && jbig2_image_get_pixel(
                    (*params).SKIP,
                    x as core::ffi::c_int,
                    y as core::ffi::c_int,
                ) != 0
            {
                bit = 0 as core::ffi::c_int;
            } else {
                CONTEXT = out_byte & 0xf as uint32_t;
                CONTEXT
                    |= (jbig2_image_get_pixel(
                        image,
                        x
                            .wrapping_add(
                                (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                        y
                            .wrapping_add(
                                (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                            ) as core::ffi::c_int,
                    ) << 4 as core::ffi::c_int) as uint32_t;
                CONTEXT |= pd >> 9 as core::ffi::c_int & 0x3e0 as uint32_t;
                bit = jbig2_arith_decode(
                    ctx,
                    as_0,
                    &mut *GB_stats.offset(CONTEXT as isize),
                );
                if bit < 0 as core::ffi::c_int {
                    return jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to decode arithmetic code when handling generic template3 unoptimized\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
            }
            pd = pd << 1 as core::ffi::c_int;
            out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
            out_bits_to_go_in_byte -= 1;
            *d = ((out_byte as uint8_t as core::ffi::c_int) << out_bits_to_go_in_byte)
                as uint8_t;
            if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                d = d.offset(1);
                if x.wrapping_add(9 as uint32_t) < GBW && y > 0 as uint32_t {
                    let fresh2 = pline;
                    pline = pline.offset(1);
                    pd |= *fresh2 as uint32_t;
                }
            }
            x = x.wrapping_add(1);
        }
        if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
            *d = ((out_byte as uint8_t as core::ffi::c_int) << out_bits_to_go_in_byte)
                as uint8_t;
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn copy_prev_row(
    mut image: *mut Jbig2Image,
    mut row: core::ffi::c_int,
) {
    if row == 0 {
        memset(
            (*image).data as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (*image).stride as size_t,
        );
    } else {
        let mut src: *mut uint8_t = ((*image).data)
            .offset(
                ((row - 1 as core::ffi::c_int) as uint32_t).wrapping_mul((*image).stride)
                    as isize,
            );
        memcpy(
            src.offset((*image).stride as isize) as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            (*image).stride as size_t,
        );
    };
}
unsafe extern "C" fn jbig2_decode_generic_template0_TPGDON(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut LTP: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut gmin: core::ffi::c_int = 0;
    let mut gmax: core::ffi::c_int = 0;
    let mut left: uint32_t = 0;
    let mut right: uint32_t = 0;
    let mut top: uint32_t = 0;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
        || (((*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
            || (*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            || ((*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int)
                < -(128 as core::ffi::c_int)
            || ((*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int)
                < 0 as core::ffi::c_int
                && (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int
                    > 127 as core::ffi::c_int
            || (*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int
                    >= 0 as core::ffi::c_int)
        || (((*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
            || (*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            || ((*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int)
                < -(128 as core::ffi::c_int)
            || ((*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int)
                < 0 as core::ffi::c_int
                && (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int
                    > 127 as core::ffi::c_int
            || (*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int
                    >= 0 as core::ffi::c_int)
        || (((*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
            || (*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            || ((*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int)
                < -(128 as core::ffi::c_int)
            || ((*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int)
                < 0 as core::ffi::c_int
                && (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int
                    > 127 as core::ffi::c_int
            || (*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int
                    >= 0 as core::ffi::c_int)
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
        == 3 as core::ffi::c_int
        && (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == -(1 as core::ffi::c_int)
        && (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int
            == -(3 as core::ffi::c_int)
        && (*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int
            == -(1 as core::ffi::c_int)
        && (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int
            == 2 as core::ffi::c_int
        && (*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int
            == -(2 as core::ffi::c_int)
        && (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int
            == -(2 as core::ffi::c_int)
        && (*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int
            == -(2 as core::ffi::c_int)
    {
        y = 0 as uint32_t;
        while y < GBH {
            let mut bit: core::ffi::c_int = jbig2_arith_decode(
                ctx,
                as_0,
                &mut *GB_stats.offset(0x9b25 as core::ffi::c_int as isize),
            );
            if bit < 0 as core::ffi::c_int {
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"failed to decode arithmetic code when handling generic template0 TPGDON1\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            LTP ^= bit;
            if LTP == 0 {
                let mut out_byte: uint32_t = 0 as uint32_t;
                let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
                let mut d: *mut uint8_t = &mut *((*image).data)
                    .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
                let mut pline: *mut uint8_t = &mut *((*image).data)
                    .offset(
                        ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t))
                            as isize,
                    ) as *mut uint8_t;
                let mut ppline: *mut uint8_t = &mut *((*image).data)
                    .offset(
                        ((*image).stride).wrapping_mul(y.wrapping_sub(2 as uint32_t))
                            as isize,
                    ) as *mut uint8_t;
                let mut pd: uint32_t = 0 as uint32_t;
                let mut ppd: uint32_t = 0 as uint32_t;
                if y > 0 as uint32_t {
                    let fresh36 = pline;
                    pline = pline.offset(1);
                    pd = ((*fresh36 as core::ffi::c_int) << 8 as core::ffi::c_int)
                        as uint32_t;
                    if GBW > 8 as uint32_t {
                        let fresh37 = pline;
                        pline = pline.offset(1);
                        pd |= *fresh37 as uint32_t;
                    }
                    if y > 1 as uint32_t {
                        let fresh38 = ppline;
                        ppline = ppline.offset(1);
                        ppd = ((*fresh38 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            as uint32_t;
                        if GBW > 8 as uint32_t {
                            let fresh39 = ppline;
                            ppline = ppline.offset(1);
                            ppd |= *fresh39 as uint32_t;
                        }
                    }
                }
                x = 0 as uint32_t;
                while x < GBW {
                    if (*params).USESKIP != 0
                        && jbig2_image_get_pixel(
                            (*params).SKIP,
                            x as core::ffi::c_int,
                            y as core::ffi::c_int,
                        ) != 0
                    {
                        bit = 0 as core::ffi::c_int;
                    } else {
                        CONTEXT = out_byte & 0xf as uint32_t;
                        CONTEXT |= pd >> 8 as core::ffi::c_int & 0x7f0 as uint32_t;
                        CONTEXT |= ppd >> 2 as core::ffi::c_int & 0xf800 as uint32_t;
                        bit = jbig2_arith_decode(
                            ctx,
                            as_0,
                            &mut *GB_stats.offset(CONTEXT as isize),
                        );
                        if bit < 0 as core::ffi::c_int {
                            return jbig2_error(
                                ctx,
                                JBIG2_SEVERITY_WARNING,
                                (*segment).number,
                                b"failed to decode arithmetic code when handling generic template0 TPGDON2\0"
                                    as *const u8 as *const core::ffi::c_char,
                            );
                        }
                    }
                    pd = pd << 1 as core::ffi::c_int;
                    ppd = ppd << 1 as core::ffi::c_int;
                    out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
                    out_bits_to_go_in_byte -= 1;
                    if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                        out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                        let fresh40 = d;
                        d = d.offset(1);
                        *fresh40 = out_byte as uint8_t;
                        if x.wrapping_add(9 as uint32_t) < GBW && y > 0 as uint32_t {
                            let fresh41 = pline;
                            pline = pline.offset(1);
                            pd |= *fresh41 as uint32_t;
                            if y > 1 as uint32_t {
                                let fresh42 = ppline;
                                ppline = ppline.offset(1);
                                ppd |= *fresh42 as uint32_t;
                            }
                        }
                    }
                    x = x.wrapping_add(1);
                }
                if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
                    *d = ((out_byte as uint8_t as core::ffi::c_int)
                        << out_bits_to_go_in_byte) as uint8_t;
                }
            } else {
                copy_prev_row(image, y as core::ffi::c_int);
            }
            y = y.wrapping_add(1);
        }
        return 0 as core::ffi::c_int;
    }
    left = 4 as uint32_t;
    right = 2 as uint32_t;
    gmax = (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int;
    gmin = gmax;
    if ((*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int) < gmin {
        gmin = (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if gmax < (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int {
        gmax = (*params).gbat[2 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if ((*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int) < gmin {
        gmin = (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if gmax < (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int {
        gmax = (*params).gbat[4 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if ((*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int) < gmin {
        gmin = (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if gmax < (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int {
        gmax = (*params).gbat[6 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if (left as core::ffi::c_int) < -gmin {
        left = -gmin as uint32_t;
    }
    if (right as core::ffi::c_int) < gmax {
        right = gmax as uint32_t;
    }
    if right > GBW {
        right = GBW;
    }
    right = GBW.wrapping_sub(right);
    top = 2 as uint32_t;
    gmin = (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int;
    if ((*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int) < gmin {
        gmin = (*params).gbat[3 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if ((*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int) < gmin {
        gmin = (*params).gbat[5 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if ((*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int) < gmin {
        gmin = (*params).gbat[7 as core::ffi::c_int as usize] as core::ffi::c_int;
    }
    if (top as core::ffi::c_int) < -gmin {
        top = -gmin as uint32_t;
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut bit_0: core::ffi::c_int = jbig2_arith_decode(
            ctx,
            as_0,
            &mut *GB_stats.offset(0x9b25 as core::ffi::c_int as isize),
        );
        if bit_0 < 0 as core::ffi::c_int {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to decode arithmetic code when handling generic template0 TPGDON1\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        LTP ^= bit_0;
        if LTP == 0 {
            let mut out_byte_0: uint32_t = 0 as uint32_t;
            let mut out_bits_to_go_in_byte_0: core::ffi::c_int = 8 as core::ffi::c_int;
            let mut d_0: *mut uint8_t = &mut *((*image).data)
                .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
            let mut pline_0: *mut uint8_t = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let mut ppline_0: *mut uint8_t = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(2 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let mut pd_0: uint32_t = 0 as uint32_t;
            let mut ppd_0: uint32_t = 0 as uint32_t;
            if y > 0 as uint32_t {
                let fresh43 = pline_0;
                pline_0 = pline_0.offset(1);
                pd_0 = ((*fresh43 as core::ffi::c_int) << 8 as core::ffi::c_int)
                    as uint32_t;
                if GBW > 8 as uint32_t {
                    let fresh44 = pline_0;
                    pline_0 = pline_0.offset(1);
                    pd_0 |= *fresh44 as uint32_t;
                }
                if y > 1 as uint32_t {
                    let fresh45 = ppline_0;
                    ppline_0 = ppline_0.offset(1);
                    ppd_0 = ((*fresh45 as core::ffi::c_int) << 8 as core::ffi::c_int)
                        as uint32_t;
                    if GBW > 8 as uint32_t {
                        let fresh46 = ppline_0;
                        ppline_0 = ppline_0.offset(1);
                        ppd_0 |= *fresh46 as uint32_t;
                    }
                }
            }
            x = 0 as uint32_t;
            while x < GBW {
                if (*params).USESKIP != 0
                    && jbig2_image_get_pixel(
                        (*params).SKIP,
                        x as core::ffi::c_int,
                        y as core::ffi::c_int,
                    ) != 0
                {
                    bit_0 = 0 as core::ffi::c_int;
                } else {
                    CONTEXT = out_byte_0 & 0xf as uint32_t;
                    CONTEXT |= pd_0 >> 8 as core::ffi::c_int & 0x3e0 as uint32_t;
                    CONTEXT |= ppd_0 >> 2 as core::ffi::c_int & 0x7000 as uint32_t;
                    if y >= top && x >= left && x < right {
                        CONTEXT
                            |= (jbig2_image_get_pixel_fast(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 4 as core::ffi::c_int) as uint32_t;
                        CONTEXT
                            |= (jbig2_image_get_pixel_fast(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[2 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[3 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 10 as core::ffi::c_int) as uint32_t;
                        CONTEXT
                            |= (jbig2_image_get_pixel_fast(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[4 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[5 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 11 as core::ffi::c_int) as uint32_t;
                        CONTEXT
                            |= (jbig2_image_get_pixel_fast(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[6 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[7 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 15 as core::ffi::c_int) as uint32_t;
                    } else {
                        CONTEXT
                            |= (jbig2_image_get_pixel(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 4 as core::ffi::c_int) as uint32_t;
                        CONTEXT
                            |= (jbig2_image_get_pixel(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[2 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[3 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 10 as core::ffi::c_int) as uint32_t;
                        CONTEXT
                            |= (jbig2_image_get_pixel(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[4 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[5 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 11 as core::ffi::c_int) as uint32_t;
                        CONTEXT
                            |= (jbig2_image_get_pixel(
                                image,
                                x
                                    .wrapping_add(
                                        (*params).gbat[6 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                                y
                                    .wrapping_add(
                                        (*params).gbat[7 as core::ffi::c_int as usize] as uint32_t,
                                    ) as core::ffi::c_int,
                            ) << 15 as core::ffi::c_int) as uint32_t;
                    }
                    bit_0 = jbig2_arith_decode(
                        ctx,
                        as_0,
                        &mut *GB_stats.offset(CONTEXT as isize),
                    );
                    if bit_0 < 0 as core::ffi::c_int {
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to decode arithmetic code when handling generic template0 TPGDON2\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    }
                }
                pd_0 = pd_0 << 1 as core::ffi::c_int;
                ppd_0 = ppd_0 << 1 as core::ffi::c_int;
                out_byte_0 = out_byte_0 << 1 as core::ffi::c_int | bit_0 as uint32_t;
                out_bits_to_go_in_byte_0 -= 1;
                *d_0 = ((out_byte_0 as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte_0) as uint8_t;
                if out_bits_to_go_in_byte_0 == 0 as core::ffi::c_int {
                    out_bits_to_go_in_byte_0 = 8 as core::ffi::c_int;
                    d_0 = d_0.offset(1);
                    if x.wrapping_add(9 as uint32_t) < GBW && y > 0 as uint32_t {
                        let fresh47 = pline_0;
                        pline_0 = pline_0.offset(1);
                        pd_0 |= *fresh47 as uint32_t;
                        if y > 1 as uint32_t {
                            let fresh48 = ppline_0;
                            ppline_0 = ppline_0.offset(1);
                            ppd_0 |= *fresh48 as uint32_t;
                        }
                    }
                }
                x = x.wrapping_add(1);
            }
            if out_bits_to_go_in_byte_0 != 8 as core::ffi::c_int {
                *d_0 = ((out_byte_0 as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte_0) as uint8_t;
            }
        } else {
            copy_prev_row(image, y as core::ffi::c_int);
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template1_TPGDON(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut LTP: core::ffi::c_int = 0 as core::ffi::c_int;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut bit: core::ffi::c_int = jbig2_arith_decode(
            ctx,
            as_0,
            &mut *GB_stats.offset(0x795 as core::ffi::c_int as isize),
        );
        if bit < 0 as core::ffi::c_int {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to decode arithmetic code when handling generic template1 TPGDON1\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        LTP ^= bit;
        if LTP == 0 {
            let mut out_byte: uint32_t = 0 as uint32_t;
            let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
            let mut d: *mut uint8_t = &mut *((*image).data)
                .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
            let mut pline: *mut uint8_t = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let mut ppline: *mut uint8_t = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(2 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let mut pd: uint32_t = 0 as uint32_t;
            let mut ppd: uint32_t = 0 as uint32_t;
            if y > 0 as uint32_t {
                let fresh30 = pline;
                pline = pline.offset(1);
                pd = ((*fresh30 as core::ffi::c_int) << 8 as core::ffi::c_int)
                    as uint32_t;
                if GBW > 8 as uint32_t {
                    let fresh31 = pline;
                    pline = pline.offset(1);
                    pd |= *fresh31 as uint32_t;
                }
                if y > 1 as uint32_t {
                    let fresh32 = ppline;
                    ppline = ppline.offset(1);
                    ppd = ((*fresh32 as core::ffi::c_int) << 8 as core::ffi::c_int)
                        as uint32_t;
                    if GBW > 8 as uint32_t {
                        let fresh33 = ppline;
                        ppline = ppline.offset(1);
                        ppd |= *fresh33 as uint32_t;
                    }
                }
            }
            x = 0 as uint32_t;
            while x < GBW {
                if (*params).USESKIP != 0
                    && jbig2_image_get_pixel(
                        (*params).SKIP,
                        x as core::ffi::c_int,
                        y as core::ffi::c_int,
                    ) != 0
                {
                    bit = 0 as core::ffi::c_int;
                } else {
                    CONTEXT = out_byte & 0x7 as uint32_t;
                    CONTEXT
                        |= (jbig2_image_get_pixel(
                            image,
                            x
                                .wrapping_add(
                                    (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                                ) as core::ffi::c_int,
                            y
                                .wrapping_add(
                                    (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                                ) as core::ffi::c_int,
                        ) << 3 as core::ffi::c_int) as uint32_t;
                    CONTEXT |= pd >> 9 as core::ffi::c_int & 0x1f0 as uint32_t;
                    CONTEXT |= ppd >> 4 as core::ffi::c_int & 0x1e00 as uint32_t;
                    bit = jbig2_arith_decode(
                        ctx,
                        as_0,
                        &mut *GB_stats.offset(CONTEXT as isize),
                    );
                    if bit < 0 as core::ffi::c_int {
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to decode arithmetic code when handling generic template1 TPGDON2\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    }
                }
                pd = pd << 1 as core::ffi::c_int;
                ppd = ppd << 1 as core::ffi::c_int;
                out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
                out_bits_to_go_in_byte -= 1;
                *d = ((out_byte as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte) as uint8_t;
                if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                    out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                    d = d.offset(1);
                    if x.wrapping_add(9 as uint32_t) < GBW && y > 0 as uint32_t {
                        let fresh34 = pline;
                        pline = pline.offset(1);
                        pd |= *fresh34 as uint32_t;
                        if y > 1 as uint32_t {
                            let fresh35 = ppline;
                            ppline = ppline.offset(1);
                            ppd |= *fresh35 as uint32_t;
                        }
                    }
                }
                x = x.wrapping_add(1);
            }
            if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
                *d = ((out_byte as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte) as uint8_t;
            }
        } else {
            copy_prev_row(image, y as core::ffi::c_int);
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template2_TPGDON(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut LTP: core::ffi::c_int = 0 as core::ffi::c_int;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut bit: core::ffi::c_int = jbig2_arith_decode(
            ctx,
            as_0,
            &mut *GB_stats.offset(0xe5 as core::ffi::c_int as isize),
        );
        if bit < 0 as core::ffi::c_int {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to decode arithmetic code when handling generic template2 TPGDON1\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        LTP ^= bit;
        if LTP == 0 {
            let mut out_byte: uint32_t = 0 as uint32_t;
            let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
            let mut d: *mut uint8_t = &mut *((*image).data)
                .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
            let mut pline: *mut uint8_t = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let mut ppline: *mut uint8_t = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(2 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let mut pd: uint32_t = 0 as uint32_t;
            let mut ppd: uint32_t = 0 as uint32_t;
            if y > 0 as uint32_t {
                let fresh24 = pline;
                pline = pline.offset(1);
                pd = ((*fresh24 as core::ffi::c_int) << 8 as core::ffi::c_int)
                    as uint32_t;
                if GBW > 8 as uint32_t {
                    let fresh25 = pline;
                    pline = pline.offset(1);
                    pd |= *fresh25 as uint32_t;
                }
                if y > 1 as uint32_t {
                    let fresh26 = ppline;
                    ppline = ppline.offset(1);
                    ppd = ((*fresh26 as core::ffi::c_int) << 8 as core::ffi::c_int)
                        as uint32_t;
                    if GBW > 8 as uint32_t {
                        let fresh27 = ppline;
                        ppline = ppline.offset(1);
                        ppd |= *fresh27 as uint32_t;
                    }
                }
            }
            x = 0 as uint32_t;
            while x < GBW {
                if (*params).USESKIP != 0
                    && jbig2_image_get_pixel(
                        (*params).SKIP,
                        x as core::ffi::c_int,
                        y as core::ffi::c_int,
                    ) != 0
                {
                    bit = 0 as core::ffi::c_int;
                } else {
                    CONTEXT = out_byte & 0x3 as uint32_t;
                    CONTEXT
                        |= (jbig2_image_get_pixel(
                            image,
                            x
                                .wrapping_add(
                                    (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                                ) as core::ffi::c_int,
                            y
                                .wrapping_add(
                                    (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                                ) as core::ffi::c_int,
                        ) << 2 as core::ffi::c_int) as uint32_t;
                    CONTEXT |= pd >> 11 as core::ffi::c_int & 0x78 as uint32_t;
                    CONTEXT |= ppd >> 7 as core::ffi::c_int & 0x380 as uint32_t;
                    bit = jbig2_arith_decode(
                        ctx,
                        as_0,
                        &mut *GB_stats.offset(CONTEXT as isize),
                    );
                    if bit < 0 as core::ffi::c_int {
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to decode arithmetic code when handling generic template2 TPGDON2\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    }
                }
                pd = pd << 1 as core::ffi::c_int;
                ppd = ppd << 1 as core::ffi::c_int;
                out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
                out_bits_to_go_in_byte -= 1;
                *d = ((out_byte as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte) as uint8_t;
                if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                    out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                    d = d.offset(1);
                    if x.wrapping_add(9 as uint32_t) < GBW && y > 0 as uint32_t {
                        let fresh28 = pline;
                        pline = pline.offset(1);
                        pd |= *fresh28 as uint32_t;
                        if y > 1 as uint32_t {
                            let fresh29 = ppline;
                            ppline = ppline.offset(1);
                            ppd |= *fresh29 as uint32_t;
                        }
                    }
                }
                x = x.wrapping_add(1);
            }
            if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
                *d = ((out_byte as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte) as uint8_t;
            }
        } else {
            copy_prev_row(image, y as core::ffi::c_int);
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_template3_TPGDON(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let GBW: uint32_t = (*image).width;
    let GBH: uint32_t = (*image).height;
    let mut CONTEXT: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut LTP: core::ffi::c_int = 0 as core::ffi::c_int;
    if ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
        < -(128 as core::ffi::c_int)
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
        || ((*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            < -(128 as core::ffi::c_int)
        || ((*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            < 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 127 as core::ffi::c_int
        || (*params).gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == 0 as core::ffi::c_int
            && (*params).gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int
                >= 0 as core::ffi::c_int
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"adaptive template pixel is out of field\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    y = 0 as uint32_t;
    while y < GBH {
        let mut bit: core::ffi::c_int = jbig2_arith_decode(
            ctx,
            as_0,
            &mut *GB_stats.offset(0x195 as core::ffi::c_int as isize),
        );
        if bit < 0 as core::ffi::c_int {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to decode arithmetic code when handling generic template3 TPGDON1\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        LTP ^= bit;
        if LTP == 0 {
            let mut out_byte: uint32_t = 0 as uint32_t;
            let mut out_bits_to_go_in_byte: core::ffi::c_int = 8 as core::ffi::c_int;
            let mut d: *mut uint8_t = &mut *((*image).data)
                .offset(((*image).stride).wrapping_mul(y) as isize) as *mut uint8_t;
            let mut pline: *mut uint8_t = &mut *((*image).data)
                .offset(
                    ((*image).stride).wrapping_mul(y.wrapping_sub(1 as uint32_t))
                        as isize,
                ) as *mut uint8_t;
            let mut pd: uint32_t = 0 as uint32_t;
            if y > 0 as uint32_t {
                let fresh21 = pline;
                pline = pline.offset(1);
                pd = ((*fresh21 as core::ffi::c_int) << 8 as core::ffi::c_int)
                    as uint32_t;
                if GBW > 8 as uint32_t {
                    let fresh22 = pline;
                    pline = pline.offset(1);
                    pd |= *fresh22 as uint32_t;
                }
            }
            x = 0 as uint32_t;
            while x < GBW {
                if (*params).USESKIP != 0
                    && jbig2_image_get_pixel(
                        (*params).SKIP,
                        x as core::ffi::c_int,
                        y as core::ffi::c_int,
                    ) != 0
                {
                    bit = 0 as core::ffi::c_int;
                } else {
                    CONTEXT = out_byte & 0xf as uint32_t;
                    CONTEXT
                        |= (jbig2_image_get_pixel(
                            image,
                            x
                                .wrapping_add(
                                    (*params).gbat[0 as core::ffi::c_int as usize] as uint32_t,
                                ) as core::ffi::c_int,
                            y
                                .wrapping_add(
                                    (*params).gbat[1 as core::ffi::c_int as usize] as uint32_t,
                                ) as core::ffi::c_int,
                        ) << 4 as core::ffi::c_int) as uint32_t;
                    CONTEXT |= pd >> 9 as core::ffi::c_int & 0x3e0 as uint32_t;
                    bit = jbig2_arith_decode(
                        ctx,
                        as_0,
                        &mut *GB_stats.offset(CONTEXT as isize),
                    );
                    if bit < 0 as core::ffi::c_int {
                        return jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to decode arithmetic code when handling generic template3 TPGDON2\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    }
                }
                pd = pd << 1 as core::ffi::c_int;
                out_byte = out_byte << 1 as core::ffi::c_int | bit as uint32_t;
                out_bits_to_go_in_byte -= 1;
                *d = ((out_byte as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte) as uint8_t;
                if out_bits_to_go_in_byte == 0 as core::ffi::c_int {
                    out_bits_to_go_in_byte = 8 as core::ffi::c_int;
                    d = d.offset(1);
                    if x.wrapping_add(9 as uint32_t) < GBW && y > 0 as uint32_t {
                        let fresh23 = pline;
                        pline = pline.offset(1);
                        pd |= *fresh23 as uint32_t;
                    }
                }
                x = x.wrapping_add(1);
            }
            if out_bits_to_go_in_byte != 8 as core::ffi::c_int {
                *d = ((out_byte as uint8_t as core::ffi::c_int)
                    << out_bits_to_go_in_byte) as uint8_t;
            }
        } else {
            copy_prev_row(image, y as core::ffi::c_int);
        }
        y = y.wrapping_add(1);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn jbig2_decode_generic_region_TPGDON(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    match (*params).GBTEMPLATE {
        0 => {
            return jbig2_decode_generic_template0_TPGDON(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            );
        }
        1 => {
            return jbig2_decode_generic_template1_TPGDON(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            );
        }
        2 => {
            return jbig2_decode_generic_template2_TPGDON(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            );
        }
        3 => {
            return jbig2_decode_generic_template3_TPGDON(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            );
        }
        _ => {}
    }
    return jbig2_error(
        ctx,
        JBIG2_SEVERITY_FATAL,
        (*segment).number,
        b"unsupported GBTEMPLATE (%d)\0" as *const u8 as *const core::ffi::c_char,
        (*params).GBTEMPLATE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_decode_generic_region(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut params: *const Jbig2GenericRegionParams,
    mut as_0: *mut Jbig2ArithState,
    mut image: *mut Jbig2Image,
    mut GB_stats: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let mut gbat: *const int8_t = ((*params).gbat).as_ptr();
    if (*params).MMR == 0 && (*params).TPGDON != 0 {
        return jbig2_decode_generic_region_TPGDON(
            ctx,
            segment,
            params,
            as_0,
            image,
            GB_stats,
        );
    }
    if (*params).MMR == 0 && (*params).GBTEMPLATE == 0 as core::ffi::c_int {
        if (*params).USESKIP == 0
            && *gbat.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == 3 as core::ffi::c_int
            && *gbat.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(1 as core::ffi::c_int)
            && *gbat.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(3 as core::ffi::c_int)
            && *gbat.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(1 as core::ffi::c_int)
            && *gbat.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int
                == 2 as core::ffi::c_int
            && *gbat.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(2 as core::ffi::c_int)
            && *gbat.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(2 as core::ffi::c_int)
            && *gbat.offset(7 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(2 as core::ffi::c_int)
        {
            return jbig2_decode_generic_template0(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        } else {
            return jbig2_decode_generic_template0_unopt(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        }
    } else if (*params).MMR == 0 && (*params).GBTEMPLATE == 1 as core::ffi::c_int {
        if (*params).USESKIP == 0
            && *gbat.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == 3 as core::ffi::c_int
            && *gbat.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(1 as core::ffi::c_int)
        {
            return jbig2_decode_generic_template1(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        } else {
            return jbig2_decode_generic_template1_unopt(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        }
    } else if (*params).MMR == 0 && (*params).GBTEMPLATE == 2 as core::ffi::c_int {
        if (*params).USESKIP == 0
            && *gbat.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == 2 as core::ffi::c_int
            && *gbat.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(1 as core::ffi::c_int)
        {
            return jbig2_decode_generic_template2(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        } else {
            return jbig2_decode_generic_template2_unopt(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        }
    } else if (*params).MMR == 0 && (*params).GBTEMPLATE == 3 as core::ffi::c_int {
        if (*params).USESKIP == 0
            && *gbat.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == 2 as core::ffi::c_int
            && *gbat.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                == -(1 as core::ffi::c_int)
        {
            return jbig2_decode_generic_template3(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        } else {
            return jbig2_decode_generic_template3_unopt(
                ctx,
                segment,
                params,
                as_0,
                image,
                GB_stats,
            )
        }
    }
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_DEBUG,
            (*segment).number,
            b"gbat[%d] = %d\0" as *const u8 as *const core::ffi::c_char,
            i,
            (*params).gbat[i as usize] as core::ffi::c_int,
        );
        i += 1;
    }
    return jbig2_error(
        ctx,
        JBIG2_SEVERITY_FATAL,
        (*segment).number,
        b"unsupported generic region (MMR=%d, GBTEMPLATE=%d)\0" as *const u8
            as *const core::ffi::c_char,
        (*params).MMR,
        (*params).GBTEMPLATE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_immediate_generic_region(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut segment_data: *const byte,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut rsi: Jbig2RegionSegmentInfo = Jbig2RegionSegmentInfo {
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        op: JBIG2_COMPOSE_OR,
        flags: 0,
    };
    let mut seg_flags: byte = 0;
    let mut gbat: [int8_t; 8] = [0; 8];
    let mut offset: core::ffi::c_int = 0;
    let mut gbat_bytes: uint32_t = 0 as uint32_t;
    let mut params: Jbig2GenericRegionParams = Jbig2GenericRegionParams {
        MMR: 0,
        GBTEMPLATE: 0,
        TPGDON: 0,
        USESKIP: 0,
        SKIP: 0 as *mut Jbig2Image,
        gbat: [0; 8],
    };
    let mut code: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut image: *mut Jbig2Image = 0 as *mut Jbig2Image;
    let mut ws: *mut Jbig2WordStream = 0 as *mut Jbig2WordStream;
    let mut as_0: *mut Jbig2ArithState = 0 as *mut Jbig2ArithState;
    let mut GB_stats: *mut Jbig2ArithCx = 0 as *mut Jbig2ArithCx;
    let mut height: uint32_t = 0;
    let mut page: *mut Jbig2Page = &mut *((*ctx).pages)
        .offset((*ctx).current_page as isize) as *mut Jbig2Page;
    if (*segment).data_length < 18 as size_t {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"segment too short\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    jbig2_get_region_segment_info(&mut rsi, segment_data as *const uint8_t);
    jbig2_error(
        ctx,
        JBIG2_SEVERITY_INFO,
        (*segment).number,
        b"generic region: %u x %u @ (%u, %u), flags = %02x\0" as *const u8
            as *const core::ffi::c_char,
        rsi.width,
        rsi.height,
        rsi.x,
        rsi.y,
        rsi.flags as core::ffi::c_int,
    );
    height = rsi.height;
    if (*segment).rows != UINT32_MAX as uint32_t {
        if (*segment).rows > rsi.height {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                (*segment).number,
                b"segment contains more rows than stated in header\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        height = (*segment).rows;
    }
    seg_flags = *segment_data.offset(17 as core::ffi::c_int as isize);
    jbig2_error(
        ctx,
        JBIG2_SEVERITY_INFO,
        (*segment).number,
        b"segment flags = %02x\0" as *const u8 as *const core::ffi::c_char,
        seg_flags as core::ffi::c_int,
    );
    if seg_flags as core::ffi::c_int & 1 as core::ffi::c_int != 0
        && seg_flags as core::ffi::c_int & 6 as core::ffi::c_int != 0
    {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"MMR is 1, but GBTEMPLATE is not 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if seg_flags as core::ffi::c_int & 1 as core::ffi::c_int == 0 {
        gbat_bytes = (if seg_flags as core::ffi::c_int & 6 as core::ffi::c_int != 0 {
            2 as core::ffi::c_int
        } else {
            8 as core::ffi::c_int
        }) as uint32_t;
        if (18 as uint32_t).wrapping_add(gbat_bytes) as size_t > (*segment).data_length {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                (*segment).number,
                b"segment too short\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        memcpy(
            gbat.as_mut_ptr() as *mut core::ffi::c_void,
            segment_data.offset(18 as core::ffi::c_int as isize)
                as *const core::ffi::c_void,
            gbat_bytes as size_t,
        );
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_INFO,
            (*segment).number,
            b"gbat: %d, %d\0" as *const u8 as *const core::ffi::c_char,
            gbat[0 as core::ffi::c_int as usize] as core::ffi::c_int,
            gbat[1 as core::ffi::c_int as usize] as core::ffi::c_int,
        );
    }
    offset = (18 as uint32_t).wrapping_add(gbat_bytes) as core::ffi::c_int;
    if seg_flags as core::ffi::c_int >> 5 as core::ffi::c_int & 1 as core::ffi::c_int
        != 0
    {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            (*segment).number,
            b"segment uses 12 adaptive template pixels (NYI)\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    params.MMR = seg_flags as core::ffi::c_int & 1 as core::ffi::c_int;
    params.GBTEMPLATE = (seg_flags as core::ffi::c_int & 6 as core::ffi::c_int)
        >> 1 as core::ffi::c_int;
    params.TPGDON = (seg_flags as core::ffi::c_int & 8 as core::ffi::c_int)
        >> 3 as core::ffi::c_int;
    params.USESKIP = 0 as core::ffi::c_int;
    memcpy(
        (params.gbat).as_mut_ptr() as *mut core::ffi::c_void,
        gbat.as_mut_ptr() as *const core::ffi::c_void,
        gbat_bytes as size_t,
    );
    if (*page).height == 0xffffffff as uint32_t && (*page).striped != 0
        && (*page).stripe_size as core::ffi::c_int > 0 as core::ffi::c_int
    {
        if rsi.y >= ((*page).end_row).wrapping_add((*page).stripe_size as uint32_t) {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"ignoring %u x %u region at (%u, %u) outside of stripe at row %u covering %u rows, on page of height %u\0"
                    as *const u8 as *const core::ffi::c_char,
                rsi.width,
                rsi.height,
                rsi.x,
                rsi.y,
                (*page).end_row,
                (*page).stripe_size as core::ffi::c_int,
                (*(*page).image).height,
            );
            return 0 as core::ffi::c_int;
        }
        if height > ((*page).end_row).wrapping_add((*page).stripe_size as uint32_t) {
            height = ((*page).end_row).wrapping_add((*page).stripe_size as uint32_t);
        }
    } else {
        if rsi.y >= (*page).height {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"ignoring %u x %u region at (%u, %u) outside of page of height %u\0"
                    as *const u8 as *const core::ffi::c_char,
                rsi.width,
                rsi.height,
                rsi.x,
                rsi.y,
                (*page).height,
            );
            return 0 as core::ffi::c_int;
        }
        if height > ((*page).height).wrapping_sub(rsi.y) {
            height = ((*page).height).wrapping_sub(rsi.y);
        }
    }
    if height == 0 as uint32_t {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"nothing remains of region, ignoring\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    image = jbig2_image_new(ctx, rsi.width, height);
    if image.is_null() {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            (*segment).number,
            b"failed to allocate generic image\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    jbig2_error(
        ctx,
        JBIG2_SEVERITY_DEBUG,
        (*segment).number,
        b"allocated %d x %d image buffer for region decode results\0" as *const u8
            as *const core::ffi::c_char,
        rsi.width,
        height,
    );
    if params.MMR != 0 {
        code = jbig2_decode_generic_mmr(
            ctx,
            segment,
            &mut params,
            segment_data.offset(offset as isize),
            ((*segment).data_length).wrapping_sub(offset as size_t),
            image,
        );
        if code < 0 as core::ffi::c_int {
            code = jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                (*segment).number,
                b"failed to decode MMR-coded generic region\0" as *const u8
                    as *const core::ffi::c_char,
            );
            current_block = 6151874488942633193;
        } else {
            current_block = 168769493162332264;
        }
    } else {
        let mut stats_size: core::ffi::c_int = jbig2_generic_stats_size(
            ctx,
            params.GBTEMPLATE,
        );
        GB_stats = jbig2_alloc(
            (*ctx).allocator,
            stats_size as size_t,
            ::core::mem::size_of::<Jbig2ArithCx>() as size_t,
        ) as *mut Jbig2ArithCx;
        if GB_stats.is_null() {
            code = jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                (*segment).number,
                b"failed to allocate arithmetic decoder states when handling immediate generic region\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            current_block = 6151874488942633193;
        } else {
            memset(
                GB_stats as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                stats_size as size_t,
            );
            ws = jbig2_word_stream_buf_new(
                ctx,
                segment_data.offset(offset as isize),
                ((*segment).data_length).wrapping_sub(offset as size_t),
            );
            if ws.is_null() {
                code = jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"failed to allocated word stream when handling immediate generic region\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                current_block = 6151874488942633193;
            } else {
                as_0 = jbig2_arith_new(ctx, ws);
                if as_0.is_null() {
                    code = jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_WARNING,
                        (*segment).number,
                        b"failed to allocate arithmetic coding state when handling immediate generic region\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                    current_block = 6151874488942633193;
                } else {
                    code = jbig2_decode_generic_region(
                        ctx,
                        segment,
                        &mut params,
                        as_0,
                        image,
                        GB_stats,
                    );
                    if code < 0 as core::ffi::c_int {
                        code = jbig2_error(
                            ctx,
                            JBIG2_SEVERITY_WARNING,
                            (*segment).number,
                            b"failed to decode immediate generic region\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                        current_block = 6151874488942633193;
                    } else {
                        current_block = 168769493162332264;
                    }
                }
            }
        }
    }
    match current_block {
        168769493162332264 => {
            code = jbig2_page_add_result(
                ctx,
                &mut *((*ctx).pages).offset((*ctx).current_page as isize),
                image,
                rsi.x,
                rsi.y,
                rsi.op,
            );
            if code < 0 as core::ffi::c_int {
                code = jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    (*segment).number,
                    b"unable to add result to page\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    jbig2_free((*ctx).allocator, as_0 as *mut core::ffi::c_void);
    jbig2_word_stream_buf_free(ctx, ws);
    jbig2_free((*ctx).allocator, GB_stats as *mut core::ffi::c_void);
    jbig2_image_release(ctx, image);
    return code;
}

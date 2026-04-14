extern "C" {
    pub type _Jbig2Page;
    pub type _Jbig2Segment;
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
    fn jbig2_realloc(
        allocator: *mut Jbig2Allocator,
        p: *mut core::ffi::c_void,
        size: size_t,
        num: size_t,
    ) -> *mut core::ffi::c_void;
    fn jbig2_error(
        ctx: *mut Jbig2Ctx,
        severity: Jbig2Severity,
        seg_idx: uint32_t,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
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
pub type Jbig2ComposeOp = core::ffi::c_uint;
pub const JBIG2_COMPOSE_REPLACE: Jbig2ComposeOp = 4;
pub const JBIG2_COMPOSE_XNOR: Jbig2ComposeOp = 3;
pub const JBIG2_COMPOSE_XOR: Jbig2ComposeOp = 2;
pub const JBIG2_COMPOSE_AND: Jbig2ComposeOp = 1;
pub const JBIG2_COMPOSE_OR: Jbig2ComposeOp = 0;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const JBIG2_UNKNOWN_SEGMENT_NUMBER: core::ffi::c_uint = !(0 as core::ffi::c_uint);
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_new(
    mut ctx: *mut Jbig2Ctx,
    mut width: uint32_t,
    mut height: uint32_t,
) -> *mut Jbig2Image {
    let mut image: *mut Jbig2Image = 0 as *mut Jbig2Image;
    let mut stride: uint32_t = 0;
    if width == 0 as uint32_t || height == 0 as uint32_t {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to create zero sized image\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2Image;
    }
    image = jbig2_alloc(
        (*ctx).allocator,
        1 as size_t,
        ::core::mem::size_of::<Jbig2Image>() as size_t,
    ) as *mut Jbig2Image;
    if image.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate image\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2Image;
    }
    stride = (width.wrapping_sub(1 as uint32_t) >> 3 as core::ffi::c_int)
        .wrapping_add(1 as uint32_t);
    if height > (INT32_MAX as uint32_t).wrapping_div(stride) {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"integer multiplication overflow (stride=%u, height=%u)\0" as *const u8
                as *const core::ffi::c_char,
            stride,
            height,
        );
        jbig2_free((*ctx).allocator, image as *mut core::ffi::c_void);
        return 0 as *mut Jbig2Image;
    }
    (*image).data = jbig2_alloc(
        (*ctx).allocator,
        (height as size_t).wrapping_mul(stride as size_t),
        ::core::mem::size_of::<uint8_t>() as size_t,
    ) as *mut uint8_t;
    if ((*image).data).is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate image data buffer (stride=%u, height=%u)\0" as *const u8
                as *const core::ffi::c_char,
            stride,
            height,
        );
        jbig2_free((*ctx).allocator, image as *mut core::ffi::c_void);
        return 0 as *mut Jbig2Image;
    }
    (*image).width = width;
    (*image).height = height;
    (*image).stride = stride;
    (*image).refcount = 1 as core::ffi::c_int;
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_reference(
    mut ctx: *mut Jbig2Ctx,
    mut image: *mut Jbig2Image,
) -> *mut Jbig2Image {
    if !image.is_null() {
        (*image).refcount += 1;
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_release(
    mut ctx: *mut Jbig2Ctx,
    mut image: *mut Jbig2Image,
) {
    if image.is_null() {
        return;
    }
    (*image).refcount -= 1;
    if (*image).refcount == 0 as core::ffi::c_int {
        jbig2_image_free(ctx, image);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_free(
    mut ctx: *mut Jbig2Ctx,
    mut image: *mut Jbig2Image,
) {
    if !image.is_null() {
        jbig2_free((*ctx).allocator, (*image).data as *mut core::ffi::c_void);
        jbig2_free((*ctx).allocator, image as *mut core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_resize(
    mut ctx: *mut Jbig2Ctx,
    mut image: *mut Jbig2Image,
    mut width: uint32_t,
    mut height: uint32_t,
    mut value: core::ffi::c_int,
) -> *mut Jbig2Image {
    if width == (*image).width {
        let mut data: *mut uint8_t = 0 as *mut uint8_t;
        if (*image).height > (INT32_MAX as uint32_t).wrapping_div((*image).stride) {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"integer multiplication overflow during resize (stride=%u, height=%u)\0"
                    as *const u8 as *const core::ffi::c_char,
                (*image).stride,
                height,
            );
            return 0 as *mut Jbig2Image;
        }
        data = jbig2_realloc(
            (*ctx).allocator,
            (*image).data as *mut core::ffi::c_void,
            (height as size_t).wrapping_mul((*image).stride as size_t),
            ::core::mem::size_of::<uint8_t>() as size_t,
        ) as *mut uint8_t;
        if data.is_null() {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to reallocate image\0" as *const u8 as *const core::ffi::c_char,
            );
            return 0 as *mut Jbig2Image;
        }
        (*image).data = data;
        if height > (*image).height {
            let fill: uint8_t = (if value != 0 {
                0xff as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as uint8_t;
            memset(
                ((*image).data)
                    .offset(
                        ((*image).height as size_t)
                            .wrapping_mul((*image).stride as size_t) as isize,
                    ) as *mut core::ffi::c_void,
                fill as core::ffi::c_int,
                (height as size_t)
                    .wrapping_sub((*image).height as size_t)
                    .wrapping_mul((*image).stride as size_t),
            );
        }
        (*image).height = height;
    } else {
        let mut newimage: *mut Jbig2Image = 0 as *mut Jbig2Image;
        let mut code: core::ffi::c_int = 0;
        newimage = jbig2_image_new(ctx, width, height);
        if newimage.is_null() {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to allocate resized image\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return 0 as *mut Jbig2Image;
        }
        jbig2_image_clear(ctx, newimage, value);
        code = jbig2_image_compose(
            ctx,
            newimage,
            image,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            JBIG2_COMPOSE_REPLACE,
        );
        if code < 0 as core::ffi::c_int {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to compose image buffers when resizing\0" as *const u8
                    as *const core::ffi::c_char,
            );
            jbig2_image_release(ctx, newimage);
            return 0 as *mut Jbig2Image;
        }
        jbig2_free((*ctx).allocator, (*image).data as *mut core::ffi::c_void);
        (*image).width = (*newimage).width;
        (*image).height = (*newimage).height;
        (*image).stride = (*newimage).stride;
        (*image).data = (*newimage).data;
        jbig2_free((*ctx).allocator, newimage as *mut core::ffi::c_void);
    }
    return image;
}
unsafe extern "C" fn template_image_compose_opt(
    mut ss: *const uint8_t,
    mut dd: *mut uint8_t,
    mut early: core::ffi::c_int,
    mut late: core::ffi::c_int,
    mut leftmask: uint8_t,
    mut rightmask: uint8_t,
    mut bytewidth_: uint32_t,
    mut h: uint32_t,
    mut shift: uint32_t,
    mut dstride: uint32_t,
    mut sstride: uint32_t,
    mut op: Jbig2ComposeOp,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: uint32_t = 0;
    let mut bytewidth: core::ffi::c_int = bytewidth_ as core::ffi::c_int;
    if bytewidth == 1 as core::ffi::c_int {
        j = 0 as uint32_t;
        while j < h {
            let mut v: uint8_t = (((if early != 0 {
                0 as core::ffi::c_int
            } else {
                (*ss.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                    << 8 as core::ffi::c_int
            })
                | (if late != 0 {
                    0 as core::ffi::c_int
                } else {
                    *ss.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                })) >> shift) as uint8_t;
            if op as core::ffi::c_uint
                == JBIG2_COMPOSE_OR as core::ffi::c_int as core::ffi::c_uint
            {
                *dd = (*dd as core::ffi::c_int
                    | v as core::ffi::c_int & leftmask as core::ffi::c_int) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_AND as core::ffi::c_int as core::ffi::c_uint
            {
                *dd = (*dd as core::ffi::c_int
                    & (v as core::ffi::c_int & leftmask as core::ffi::c_int
                        | !(leftmask as core::ffi::c_int))) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XOR as core::ffi::c_int as core::ffi::c_uint
            {
                *dd = (*dd as core::ffi::c_int
                    ^ v as core::ffi::c_int & leftmask as core::ffi::c_int) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XNOR as core::ffi::c_int as core::ffi::c_uint
            {
                *dd = (*dd as core::ffi::c_int
                    ^ !(v as core::ffi::c_int) & leftmask as core::ffi::c_int)
                    as uint8_t;
            } else {
                *dd = (v as core::ffi::c_int & leftmask as core::ffi::c_int
                    | *dd as core::ffi::c_int & !(leftmask as core::ffi::c_int))
                    as uint8_t;
            }
            dd = dd.offset(dstride as isize);
            ss = ss.offset(sstride as isize);
            j = j.wrapping_add(1);
        }
        return;
    }
    bytewidth -= 2 as core::ffi::c_int;
    if shift == 0 as uint32_t {
        ss = ss.offset(1);
        j = 0 as uint32_t;
        while j < h {
            let mut s: *const uint8_t = ss;
            let mut d: *mut uint8_t = dd;
            if op as core::ffi::c_uint
                == JBIG2_COMPOSE_OR as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh0 = s;
                s = s.offset(1);
                let fresh1 = d;
                d = d.offset(1);
                *fresh1 = (*fresh1 as core::ffi::c_int
                    | *fresh0 as core::ffi::c_int & leftmask as core::ffi::c_int)
                    as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_AND as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh2 = s;
                s = s.offset(1);
                let fresh3 = d;
                d = d.offset(1);
                *fresh3 = (*fresh3 as core::ffi::c_int
                    & (*fresh2 as core::ffi::c_int & leftmask as core::ffi::c_int
                        | !(leftmask as core::ffi::c_int))) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XOR as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh4 = s;
                s = s.offset(1);
                let fresh5 = d;
                d = d.offset(1);
                *fresh5 = (*fresh5 as core::ffi::c_int
                    ^ *fresh4 as core::ffi::c_int & leftmask as core::ffi::c_int)
                    as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XNOR as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh6 = s;
                s = s.offset(1);
                let fresh7 = d;
                d = d.offset(1);
                *fresh7 = (*fresh7 as core::ffi::c_int
                    ^ !(*fresh6 as core::ffi::c_int) & leftmask as core::ffi::c_int)
                    as uint8_t;
            } else {
                let fresh8 = s;
                s = s.offset(1);
                *d = (*fresh8 as core::ffi::c_int & leftmask as core::ffi::c_int
                    | *d as core::ffi::c_int & !(leftmask as core::ffi::c_int))
                    as uint8_t;
                d = d.offset(1);
            }
            i = bytewidth;
            while i != 0 as core::ffi::c_int {
                if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_OR as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh9 = s;
                    s = s.offset(1);
                    let fresh10 = d;
                    d = d.offset(1);
                    *fresh10 = (*fresh10 as core::ffi::c_int
                        | *fresh9 as core::ffi::c_int) as uint8_t;
                } else if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_AND as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh11 = s;
                    s = s.offset(1);
                    let fresh12 = d;
                    d = d.offset(1);
                    *fresh12 = (*fresh12 as core::ffi::c_int
                        & *fresh11 as core::ffi::c_int) as uint8_t;
                } else if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_XOR as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh13 = s;
                    s = s.offset(1);
                    let fresh14 = d;
                    d = d.offset(1);
                    *fresh14 = (*fresh14 as core::ffi::c_int
                        ^ *fresh13 as core::ffi::c_int) as uint8_t;
                } else if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_XNOR as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh15 = s;
                    s = s.offset(1);
                    let fresh16 = d;
                    d = d.offset(1);
                    *fresh16 = (*fresh16 as core::ffi::c_int
                        ^ !(*fresh15 as core::ffi::c_int)) as uint8_t;
                } else {
                    let fresh17 = s;
                    s = s.offset(1);
                    let fresh18 = d;
                    d = d.offset(1);
                    *fresh18 = *fresh17;
                }
                i -= 1;
            }
            if op as core::ffi::c_uint
                == JBIG2_COMPOSE_OR as core::ffi::c_int as core::ffi::c_uint
            {
                *d = (*d as core::ffi::c_int
                    | *s as core::ffi::c_int & rightmask as core::ffi::c_int) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_AND as core::ffi::c_int as core::ffi::c_uint
            {
                *d = (*d as core::ffi::c_int
                    & (*s as core::ffi::c_int & rightmask as core::ffi::c_int
                        | !(rightmask as core::ffi::c_int))) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XOR as core::ffi::c_int as core::ffi::c_uint
            {
                *d = (*d as core::ffi::c_int
                    ^ *s as core::ffi::c_int & rightmask as core::ffi::c_int) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XNOR as core::ffi::c_int as core::ffi::c_uint
            {
                *d = (*d as core::ffi::c_int
                    ^ !(*s as core::ffi::c_int) & rightmask as core::ffi::c_int)
                    as uint8_t;
            } else {
                *d = (*s as core::ffi::c_int & rightmask as core::ffi::c_int
                    | *d as core::ffi::c_int & !(rightmask as core::ffi::c_int))
                    as uint8_t;
            }
            dd = dd.offset(dstride as isize);
            ss = ss.offset(sstride as isize);
            j = j.wrapping_add(1);
        }
    } else {
        j = 0 as uint32_t;
        while j < h {
            let mut s_0: *const uint8_t = ss;
            let mut d_0: *mut uint8_t = dd;
            let mut s0: uint8_t = 0;
            let mut s1: uint8_t = 0;
            let mut v_0: uint8_t = 0;
            s0 = (if early != 0 {
                0 as core::ffi::c_int
            } else {
                *s_0 as core::ffi::c_int
            }) as uint8_t;
            s_0 = s_0.offset(1);
            let fresh19 = s_0;
            s_0 = s_0.offset(1);
            s1 = *fresh19;
            v_0 = (((s0 as core::ffi::c_int) << 8 as core::ffi::c_int
                | s1 as core::ffi::c_int) >> shift) as uint8_t;
            if op as core::ffi::c_uint
                == JBIG2_COMPOSE_OR as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh20 = d_0;
                d_0 = d_0.offset(1);
                *fresh20 = (*fresh20 as core::ffi::c_int
                    | v_0 as core::ffi::c_int & leftmask as core::ffi::c_int) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_AND as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh21 = d_0;
                d_0 = d_0.offset(1);
                *fresh21 = (*fresh21 as core::ffi::c_int
                    & (v_0 as core::ffi::c_int & leftmask as core::ffi::c_int
                        | !(leftmask as core::ffi::c_int))) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XOR as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh22 = d_0;
                d_0 = d_0.offset(1);
                *fresh22 = (*fresh22 as core::ffi::c_int
                    ^ v_0 as core::ffi::c_int & leftmask as core::ffi::c_int) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XNOR as core::ffi::c_int as core::ffi::c_uint
            {
                let fresh23 = d_0;
                d_0 = d_0.offset(1);
                *fresh23 = (*fresh23 as core::ffi::c_int
                    ^ !(v_0 as core::ffi::c_int) & leftmask as core::ffi::c_int)
                    as uint8_t;
            } else {
                *d_0 = (v_0 as core::ffi::c_int & leftmask as core::ffi::c_int
                    | *d_0 as core::ffi::c_int & !(leftmask as core::ffi::c_int))
                    as uint8_t;
                d_0 = d_0.offset(1);
            }
            i = bytewidth;
            while i > 0 as core::ffi::c_int {
                s0 = s1;
                let fresh24 = s_0;
                s_0 = s_0.offset(1);
                s1 = *fresh24;
                v_0 = (((s0 as core::ffi::c_int) << 8 as core::ffi::c_int
                    | s1 as core::ffi::c_int) >> shift) as uint8_t;
                if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_OR as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh25 = d_0;
                    d_0 = d_0.offset(1);
                    *fresh25 = (*fresh25 as core::ffi::c_int | v_0 as core::ffi::c_int)
                        as uint8_t;
                } else if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_AND as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh26 = d_0;
                    d_0 = d_0.offset(1);
                    *fresh26 = (*fresh26 as core::ffi::c_int & v_0 as core::ffi::c_int)
                        as uint8_t;
                } else if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_XOR as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh27 = d_0;
                    d_0 = d_0.offset(1);
                    *fresh27 = (*fresh27 as core::ffi::c_int ^ v_0 as core::ffi::c_int)
                        as uint8_t;
                } else if op as core::ffi::c_uint
                    == JBIG2_COMPOSE_XNOR as core::ffi::c_int as core::ffi::c_uint
                {
                    let fresh28 = d_0;
                    d_0 = d_0.offset(1);
                    *fresh28 = (*fresh28 as core::ffi::c_int
                        ^ !(v_0 as core::ffi::c_int)) as uint8_t;
                } else {
                    let fresh29 = d_0;
                    d_0 = d_0.offset(1);
                    *fresh29 = v_0;
                }
                i -= 1;
            }
            s0 = s1;
            s1 = (if late != 0 {
                0 as core::ffi::c_int
            } else {
                *s_0 as core::ffi::c_int
            }) as uint8_t;
            v_0 = (((s0 as core::ffi::c_int) << 8 as core::ffi::c_int
                | s1 as core::ffi::c_int) >> shift) as uint8_t;
            if op as core::ffi::c_uint
                == JBIG2_COMPOSE_OR as core::ffi::c_int as core::ffi::c_uint
            {
                *d_0 = (*d_0 as core::ffi::c_int
                    | v_0 as core::ffi::c_int & rightmask as core::ffi::c_int)
                    as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_AND as core::ffi::c_int as core::ffi::c_uint
            {
                *d_0 = (*d_0 as core::ffi::c_int
                    & (v_0 as core::ffi::c_int & rightmask as core::ffi::c_int
                        | !(rightmask as core::ffi::c_int))) as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XOR as core::ffi::c_int as core::ffi::c_uint
            {
                *d_0 = (*d_0 as core::ffi::c_int
                    ^ v_0 as core::ffi::c_int & rightmask as core::ffi::c_int)
                    as uint8_t;
            } else if op as core::ffi::c_uint
                == JBIG2_COMPOSE_XNOR as core::ffi::c_int as core::ffi::c_uint
            {
                *d_0 = (*d_0 as core::ffi::c_int
                    ^ !(v_0 as core::ffi::c_int) & rightmask as core::ffi::c_int)
                    as uint8_t;
            } else {
                *d_0 = (v_0 as core::ffi::c_int & rightmask as core::ffi::c_int
                    | *d_0 as core::ffi::c_int & !(rightmask as core::ffi::c_int))
                    as uint8_t;
            }
            dd = dd.offset(dstride as isize);
            ss = ss.offset(sstride as isize);
            j = j.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn jbig2_image_compose_opt_OR(
    mut s: *const uint8_t,
    mut d: *mut uint8_t,
    mut early: core::ffi::c_int,
    mut late: core::ffi::c_int,
    mut mask: uint8_t,
    mut rightmask: uint8_t,
    mut bytewidth: uint32_t,
    mut h: uint32_t,
    mut shift: uint32_t,
    mut dstride: uint32_t,
    mut sstride: uint32_t,
) {
    if early != 0 || late != 0 {
        template_image_compose_opt(
            s,
            d,
            early,
            late,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_OR,
        );
    } else {
        template_image_compose_opt(
            s,
            d,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_OR,
        );
    };
}
unsafe extern "C" fn jbig2_image_compose_opt_AND(
    mut s: *const uint8_t,
    mut d: *mut uint8_t,
    mut early: core::ffi::c_int,
    mut late: core::ffi::c_int,
    mut mask: uint8_t,
    mut rightmask: uint8_t,
    mut bytewidth: uint32_t,
    mut h: uint32_t,
    mut shift: uint32_t,
    mut dstride: uint32_t,
    mut sstride: uint32_t,
) {
    if early != 0 || late != 0 {
        template_image_compose_opt(
            s,
            d,
            early,
            late,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_AND,
        );
    } else {
        template_image_compose_opt(
            s,
            d,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_AND,
        );
    };
}
unsafe extern "C" fn jbig2_image_compose_opt_XOR(
    mut s: *const uint8_t,
    mut d: *mut uint8_t,
    mut early: core::ffi::c_int,
    mut late: core::ffi::c_int,
    mut mask: uint8_t,
    mut rightmask: uint8_t,
    mut bytewidth: uint32_t,
    mut h: uint32_t,
    mut shift: uint32_t,
    mut dstride: uint32_t,
    mut sstride: uint32_t,
) {
    if early != 0 || late != 0 {
        template_image_compose_opt(
            s,
            d,
            early,
            late,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_XOR,
        );
    } else {
        template_image_compose_opt(
            s,
            d,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_XOR,
        );
    };
}
unsafe extern "C" fn jbig2_image_compose_opt_XNOR(
    mut s: *const uint8_t,
    mut d: *mut uint8_t,
    mut early: core::ffi::c_int,
    mut late: core::ffi::c_int,
    mut mask: uint8_t,
    mut rightmask: uint8_t,
    mut bytewidth: uint32_t,
    mut h: uint32_t,
    mut shift: uint32_t,
    mut dstride: uint32_t,
    mut sstride: uint32_t,
) {
    if early != 0 || late != 0 {
        template_image_compose_opt(
            s,
            d,
            early,
            late,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_XNOR,
        );
    } else {
        template_image_compose_opt(
            s,
            d,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_XNOR,
        );
    };
}
unsafe extern "C" fn jbig2_image_compose_opt_REPLACE(
    mut s: *const uint8_t,
    mut d: *mut uint8_t,
    mut early: core::ffi::c_int,
    mut late: core::ffi::c_int,
    mut mask: uint8_t,
    mut rightmask: uint8_t,
    mut bytewidth: uint32_t,
    mut h: uint32_t,
    mut shift: uint32_t,
    mut dstride: uint32_t,
    mut sstride: uint32_t,
) {
    if early != 0 || late != 0 {
        template_image_compose_opt(
            s,
            d,
            early,
            late,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_REPLACE,
        );
    } else {
        template_image_compose_opt(
            s,
            d,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            mask,
            rightmask,
            bytewidth,
            h,
            shift,
            dstride,
            sstride,
            JBIG2_COMPOSE_REPLACE,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_compose(
    mut ctx: *mut Jbig2Ctx,
    mut dst: *mut Jbig2Image,
    mut src: *mut Jbig2Image,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut op: Jbig2ComposeOp,
) -> core::ffi::c_int {
    let mut w: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut shift: uint32_t = 0;
    let mut leftbyte: uint32_t = 0;
    let mut ss: *mut uint8_t = 0 as *mut uint8_t;
    let mut dd: *mut uint8_t = 0 as *mut uint8_t;
    let mut leftmask: uint8_t = 0;
    let mut rightmask: uint8_t = 0;
    let mut early: core::ffi::c_int = (x >= 0 as core::ffi::c_int) as core::ffi::c_int;
    let mut late: core::ffi::c_int = 0;
    let mut bytewidth: uint32_t = 0;
    let mut syoffset: uint32_t = 0 as uint32_t;
    if src.is_null() {
        return 0 as core::ffi::c_int;
    }
    if (UINT32_MAX as uint32_t).wrapping_sub((*src).width)
        < (if x > 0 as core::ffi::c_int { x } else { -x }) as uint32_t
        || (UINT32_MAX as uint32_t).wrapping_sub((*src).height)
            < (if y > 0 as core::ffi::c_int { y } else { -y }) as uint32_t
    {
        return 0 as core::ffi::c_int;
    }
    w = (*src).width;
    h = (*src).height;
    shift = (x & 7 as core::ffi::c_int) as uint32_t;
    ss = ((*src).data).offset(-(early as isize));
    if x < 0 as core::ffi::c_int {
        if w < -x as uint32_t {
            w = 0 as uint32_t;
        } else {
            w = w.wrapping_add(x as uint32_t);
        }
        ss = ss.offset((-x - 1 as core::ffi::c_int >> 3 as core::ffi::c_int) as isize);
        x = 0 as core::ffi::c_int;
    }
    if y < 0 as core::ffi::c_int {
        if h < -y as uint32_t {
            h = 0 as uint32_t;
        } else {
            h = h.wrapping_add(y as uint32_t);
        }
        syoffset = (-y as uint32_t).wrapping_mul((*src).stride);
        y = 0 as core::ffi::c_int;
    }
    if (x as uint32_t).wrapping_add(w) > (*dst).width {
        if (*dst).width < x as uint32_t {
            w = 0 as uint32_t;
        } else {
            w = ((*dst).width).wrapping_sub(x as uint32_t);
        }
    }
    if (y as uint32_t).wrapping_add(h) > (*dst).height {
        if (*dst).height < y as uint32_t {
            h = 0 as uint32_t;
        } else {
            h = ((*dst).height).wrapping_sub(y as uint32_t);
        }
    }
    if w <= 0 as uint32_t || h <= 0 as uint32_t {
        return 0 as core::ffi::c_int;
    }
    leftbyte = x as uint32_t >> 3 as core::ffi::c_int;
    dd = ((*dst).data)
        .offset((y as uint32_t).wrapping_mul((*dst).stride) as isize)
        .offset(leftbyte as isize);
    bytewidth = ((x as uint32_t).wrapping_add(w).wrapping_sub(1 as uint32_t)
        >> 3 as core::ffi::c_int)
        .wrapping_sub(leftbyte)
        .wrapping_add(1 as uint32_t);
    leftmask = (255 as core::ffi::c_int >> (x & 7 as core::ffi::c_int)) as uint8_t;
    rightmask = (if (x as uint32_t).wrapping_add(w) & 7 as uint32_t == 0 as uint32_t {
        255 as core::ffi::c_int
    } else {
        !(255 as core::ffi::c_int >> ((x as uint32_t).wrapping_add(w) & 7 as uint32_t))
    }) as uint8_t;
    if bytewidth == 1 as uint32_t {
        leftmask = (leftmask as core::ffi::c_int & rightmask as core::ffi::c_int)
            as uint8_t;
    }
    late = (ss.offset(bytewidth as isize)
        >= ((*src).data)
            .offset(
                (((*src).width).wrapping_add(7 as uint32_t) >> 3 as core::ffi::c_int)
                    as isize,
            )) as core::ffi::c_int;
    ss = ss.offset(syoffset as isize);
    match op as core::ffi::c_uint {
        0 => {
            jbig2_image_compose_opt_OR(
                ss,
                dd,
                early,
                late,
                leftmask,
                rightmask,
                bytewidth,
                h,
                shift,
                (*dst).stride,
                (*src).stride,
            );
        }
        1 => {
            jbig2_image_compose_opt_AND(
                ss,
                dd,
                early,
                late,
                leftmask,
                rightmask,
                bytewidth,
                h,
                shift,
                (*dst).stride,
                (*src).stride,
            );
        }
        2 => {
            jbig2_image_compose_opt_XOR(
                ss,
                dd,
                early,
                late,
                leftmask,
                rightmask,
                bytewidth,
                h,
                shift,
                (*dst).stride,
                (*src).stride,
            );
        }
        3 => {
            jbig2_image_compose_opt_XNOR(
                ss,
                dd,
                early,
                late,
                leftmask,
                rightmask,
                bytewidth,
                h,
                shift,
                (*dst).stride,
                (*src).stride,
            );
        }
        4 => {
            jbig2_image_compose_opt_REPLACE(
                ss,
                dd,
                early,
                late,
                leftmask,
                rightmask,
                bytewidth,
                h,
                shift,
                (*dst).stride,
                (*src).stride,
            );
        }
        _ => {}
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_clear(
    mut ctx: *mut Jbig2Ctx,
    mut image: *mut Jbig2Image,
    mut value: core::ffi::c_int,
) {
    let fill: uint8_t = (if value != 0 {
        0xff as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    }) as uint8_t;
    memset(
        (*image).data as *mut core::ffi::c_void,
        fill as core::ffi::c_int,
        ((*image).stride).wrapping_mul((*image).height) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_get_pixel(
    mut image: *mut Jbig2Image,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
) -> core::ffi::c_int {
    let w: core::ffi::c_int = (*image).width as core::ffi::c_int;
    let h: core::ffi::c_int = (*image).height as core::ffi::c_int;
    let byte: core::ffi::c_int = ((x >> 3 as core::ffi::c_int) as uint32_t)
        .wrapping_add((y as uint32_t).wrapping_mul((*image).stride)) as core::ffi::c_int;
    let bit: core::ffi::c_int = 7 as core::ffi::c_int - (x & 7 as core::ffi::c_int);
    if x < 0 as core::ffi::c_int || x >= w {
        return 0 as core::ffi::c_int;
    }
    if y < 0 as core::ffi::c_int || y >= h {
        return 0 as core::ffi::c_int;
    }
    return *((*image).data).offset(byte as isize) as core::ffi::c_int >> bit
        & 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_image_set_pixel(
    mut image: *mut Jbig2Image,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut value: core::ffi::c_int,
) {
    let w: core::ffi::c_int = (*image).width as core::ffi::c_int;
    let h: core::ffi::c_int = (*image).height as core::ffi::c_int;
    let mut scratch: core::ffi::c_int = 0;
    let mut mask: core::ffi::c_int = 0;
    let mut bit: core::ffi::c_int = 0;
    let mut byte: core::ffi::c_int = 0;
    if x < 0 as core::ffi::c_int || x >= w {
        return;
    }
    if y < 0 as core::ffi::c_int || y >= h {
        return;
    }
    byte = ((x >> 3 as core::ffi::c_int) as uint32_t)
        .wrapping_add((y as uint32_t).wrapping_mul((*image).stride)) as core::ffi::c_int;
    bit = 7 as core::ffi::c_int - (x & 7 as core::ffi::c_int);
    mask = (1 as core::ffi::c_int) << bit ^ 0xff as core::ffi::c_int;
    scratch = *((*image).data).offset(byte as isize) as core::ffi::c_int & mask;
    *((*image).data).offset(byte as isize) = (scratch | value << bit) as uint8_t;
}

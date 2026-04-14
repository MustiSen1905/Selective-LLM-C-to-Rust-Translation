extern "C" {
    pub type _Jbig2Page;
    pub type _Jbig2Segment;
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
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
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
pub struct _Jbig2ArithState {
    pub C: uint32_t,
    pub A: uint32_t,
    pub CT: core::ffi::c_int,
    pub next_word: uint32_t,
    pub next_word_bytes: size_t,
    pub err: core::ffi::c_int,
    pub ws: *mut Jbig2WordStream,
    pub offset: size_t,
}
pub type Jbig2ArithState = _Jbig2ArithState;
pub type Jbig2ArithCx = core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Jbig2ArithQe {
    pub Qe: uint16_t,
    pub mps_xor: byte,
    pub lps_xor: byte,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const JBIG2_UNKNOWN_SEGMENT_NUMBER: core::ffi::c_uint = !(0 as core::ffi::c_uint);
unsafe extern "C" fn jbig2_arith_bytein(
    mut ctx: *mut Jbig2Ctx,
    mut as_0: *mut Jbig2ArithState,
) -> core::ffi::c_int {
    let mut B: byte = 0;
    if (*as_0).err != 0 as core::ffi::c_int {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to read from underlying stream during arithmetic decoding\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return -(1 as core::ffi::c_int);
    }
    if (*as_0).next_word_bytes == 0 as size_t {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to read beyond end of underlying stream during arithmetic decoding\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return -(1 as core::ffi::c_int);
    }
    B = ((*as_0).next_word >> 24 as core::ffi::c_int & 0xff as uint32_t) as byte;
    if B as core::ffi::c_int == 0xff as core::ffi::c_int {
        let mut B1: byte = 0;
        if (*as_0).next_word_bytes <= 1 as size_t {
            let mut ret: core::ffi::c_int = ((*(*as_0).ws).get_next_word)
                .expect(
                    "non-null function pointer",
                )(ctx, (*as_0).ws, (*as_0).offset, &mut (*as_0).next_word);
            if ret < 0 as core::ffi::c_int {
                (*as_0).err = 1 as core::ffi::c_int;
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to check for marker code due to failure in underlying stream during arithmetic decoding\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            (*as_0).next_word_bytes = ret as size_t;
            if (*as_0).next_word_bytes == 0 as size_t {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to read end of possible terminating marker code, assuming terminating marker code\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                (*as_0).next_word = 0xff900000 as core::ffi::c_uint as uint32_t;
                (*as_0).next_word_bytes = 2 as size_t;
                (*as_0).C = ((*as_0).C).wrapping_add(0xff00 as uint32_t);
                (*as_0).CT = 8 as core::ffi::c_int;
                return 0 as core::ffi::c_int;
            }
            (*as_0).offset = ((*as_0).offset).wrapping_add((*as_0).next_word_bytes);
            B1 = ((*as_0).next_word >> 24 as core::ffi::c_int & 0xff as uint32_t)
                as byte;
            if B1 as core::ffi::c_int > 0x8f as core::ffi::c_int {
                (*as_0).CT = 8 as core::ffi::c_int;
                (*as_0).next_word = 0xff000000 as uint32_t
                    | (*as_0).next_word >> 8 as core::ffi::c_int;
                (*as_0).next_word_bytes = 2 as size_t;
                (*as_0).offset = ((*as_0).offset).wrapping_sub(1);
            } else {
                (*as_0).C = ((*as_0).C)
                    .wrapping_add(
                        (0xfe00 as core::ffi::c_int
                            - ((B1 as core::ffi::c_int) << 9 as core::ffi::c_int))
                            as uint32_t,
                    );
                (*as_0).CT = 7 as core::ffi::c_int;
            }
        } else {
            B1 = ((*as_0).next_word >> 16 as core::ffi::c_int & 0xff as uint32_t)
                as byte;
            if B1 as core::ffi::c_int > 0x8f as core::ffi::c_int {
                (*as_0).CT = 8 as core::ffi::c_int;
            } else {
                (*as_0).next_word_bytes = ((*as_0).next_word_bytes).wrapping_sub(1);
                (*as_0).next_word <<= 8 as core::ffi::c_int;
                (*as_0).C = ((*as_0).C)
                    .wrapping_add(
                        (0xfe00 as core::ffi::c_int
                            - ((B1 as core::ffi::c_int) << 9 as core::ffi::c_int))
                            as uint32_t,
                    );
                (*as_0).CT = 7 as core::ffi::c_int;
            }
        }
    } else {
        (*as_0).next_word <<= 8 as core::ffi::c_int;
        (*as_0).next_word_bytes = ((*as_0).next_word_bytes).wrapping_sub(1);
        if (*as_0).next_word_bytes == 0 as size_t {
            let mut ret_0: core::ffi::c_int = ((*(*as_0).ws).get_next_word)
                .expect(
                    "non-null function pointer",
                )(ctx, (*as_0).ws, (*as_0).offset, &mut (*as_0).next_word);
            if ret_0 < 0 as core::ffi::c_int {
                (*as_0).err = 1 as core::ffi::c_int;
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to read from underlying stream during arithmetic decoding\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            (*as_0).next_word_bytes = ret_0 as size_t;
            if (*as_0).next_word_bytes == 0 as size_t {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to find terminating marker code before end of underlying stream, assuming terminating marker code\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                (*as_0).next_word = 0xff900000 as core::ffi::c_uint as uint32_t;
                (*as_0).next_word_bytes = 2 as size_t;
                (*as_0).C = ((*as_0).C).wrapping_add(0xff00 as uint32_t);
                (*as_0).CT = 8 as core::ffi::c_int;
                return 0 as core::ffi::c_int;
            }
            (*as_0).offset = ((*as_0).offset).wrapping_add((*as_0).next_word_bytes);
        }
        B = ((*as_0).next_word >> 24 as core::ffi::c_int & 0xff as uint32_t) as byte;
        (*as_0).C = ((*as_0).C)
            .wrapping_add(
                (0xff00 as core::ffi::c_int
                    - ((B as core::ffi::c_int) << 8 as core::ffi::c_int)) as uint32_t,
            );
        (*as_0).CT = 8 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_arith_new(
    mut ctx: *mut Jbig2Ctx,
    mut ws: *mut Jbig2WordStream,
) -> *mut Jbig2ArithState {
    let mut result: *mut Jbig2ArithState = 0 as *mut Jbig2ArithState;
    let mut ret: core::ffi::c_int = 0;
    result = jbig2_alloc(
        (*ctx).allocator,
        1 as size_t,
        ::core::mem::size_of::<Jbig2ArithState>() as size_t,
    ) as *mut Jbig2ArithState;
    if result.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate arithmetic coding state\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2ArithState;
    }
    (*result).err = 0 as core::ffi::c_int;
    (*result).ws = ws;
    (*result).offset = 0 as size_t;
    ret = ((*(*result).ws).get_next_word)
        .expect(
            "non-null function pointer",
        )(ctx, (*result).ws, (*result).offset, &mut (*result).next_word);
    if ret < 0 as core::ffi::c_int {
        jbig2_free((*ctx).allocator, result as *mut core::ffi::c_void);
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to initialize underlying stream of arithmetic decoder\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2ArithState;
    }
    (*result).next_word_bytes = ret as size_t;
    if (*result).next_word_bytes == 0 as size_t {
        jbig2_free((*ctx).allocator, result as *mut core::ffi::c_void);
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to read first byte from underlying stream when initializing arithmetic decoder\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2ArithState;
    }
    (*result).offset = ((*result).offset).wrapping_add((*result).next_word_bytes);
    (*result).C = !((*result).next_word >> 8 as core::ffi::c_int) & 0xff0000 as uint32_t;
    if jbig2_arith_bytein(ctx, result) < 0 as core::ffi::c_int {
        jbig2_free((*ctx).allocator, result as *mut core::ffi::c_void);
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_WARNING,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to read second byte from underlying stream when initializing arithmetic decoder\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2ArithState;
    }
    (*result).C <<= 7 as core::ffi::c_int;
    (*result).CT -= 7 as core::ffi::c_int;
    (*result).A = 0x8000 as uint32_t;
    return result;
}
pub const MAX_QE_ARRAY_SIZE: core::ffi::c_int = 47 as core::ffi::c_int;
static mut jbig2_arith_Qe: [Jbig2ArithQe; 47] = [
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5601 as uint16_t,
            mps_xor: (0 as core::ffi::c_int ^ 1 as core::ffi::c_int) as byte,
            lps_xor: (0 as core::ffi::c_int ^ 1 as core::ffi::c_int
                ^ (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x3401 as uint16_t,
            mps_xor: (1 as core::ffi::c_int ^ 2 as core::ffi::c_int) as byte,
            lps_xor: (1 as core::ffi::c_int ^ 6 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1801 as uint16_t,
            mps_xor: (2 as core::ffi::c_int ^ 3 as core::ffi::c_int) as byte,
            lps_xor: (2 as core::ffi::c_int ^ 9 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0xac1 as uint16_t,
            mps_xor: (3 as core::ffi::c_int ^ 4 as core::ffi::c_int) as byte,
            lps_xor: (3 as core::ffi::c_int ^ 12 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x521 as uint16_t,
            mps_xor: (4 as core::ffi::c_int ^ 5 as core::ffi::c_int) as byte,
            lps_xor: (4 as core::ffi::c_int ^ 29 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x221 as uint16_t,
            mps_xor: (5 as core::ffi::c_int ^ 38 as core::ffi::c_int) as byte,
            lps_xor: (5 as core::ffi::c_int ^ 33 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5601 as uint16_t,
            mps_xor: (6 as core::ffi::c_int ^ 7 as core::ffi::c_int) as byte,
            lps_xor: (6 as core::ffi::c_int ^ 6 as core::ffi::c_int
                ^ (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5401 as uint16_t,
            mps_xor: (7 as core::ffi::c_int ^ 8 as core::ffi::c_int) as byte,
            lps_xor: (7 as core::ffi::c_int ^ 14 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x4801 as uint16_t,
            mps_xor: (8 as core::ffi::c_int ^ 9 as core::ffi::c_int) as byte,
            lps_xor: (8 as core::ffi::c_int ^ 14 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x3801 as uint16_t,
            mps_xor: (9 as core::ffi::c_int ^ 10 as core::ffi::c_int) as byte,
            lps_xor: (9 as core::ffi::c_int ^ 14 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x3001 as uint16_t,
            mps_xor: (10 as core::ffi::c_int ^ 11 as core::ffi::c_int) as byte,
            lps_xor: (10 as core::ffi::c_int ^ 17 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x2401 as uint16_t,
            mps_xor: (11 as core::ffi::c_int ^ 12 as core::ffi::c_int) as byte,
            lps_xor: (11 as core::ffi::c_int ^ 18 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1c01 as uint16_t,
            mps_xor: (12 as core::ffi::c_int ^ 13 as core::ffi::c_int) as byte,
            lps_xor: (12 as core::ffi::c_int ^ 20 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1601 as uint16_t,
            mps_xor: (13 as core::ffi::c_int ^ 29 as core::ffi::c_int) as byte,
            lps_xor: (13 as core::ffi::c_int ^ 21 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5601 as uint16_t,
            mps_xor: (14 as core::ffi::c_int ^ 15 as core::ffi::c_int) as byte,
            lps_xor: (14 as core::ffi::c_int ^ 14 as core::ffi::c_int
                ^ (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5401 as uint16_t,
            mps_xor: (15 as core::ffi::c_int ^ 16 as core::ffi::c_int) as byte,
            lps_xor: (15 as core::ffi::c_int ^ 14 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5101 as uint16_t,
            mps_xor: (16 as core::ffi::c_int ^ 17 as core::ffi::c_int) as byte,
            lps_xor: (16 as core::ffi::c_int ^ 15 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x4801 as uint16_t,
            mps_xor: (17 as core::ffi::c_int ^ 18 as core::ffi::c_int) as byte,
            lps_xor: (17 as core::ffi::c_int ^ 16 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x3801 as uint16_t,
            mps_xor: (18 as core::ffi::c_int ^ 19 as core::ffi::c_int) as byte,
            lps_xor: (18 as core::ffi::c_int ^ 17 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x3401 as uint16_t,
            mps_xor: (19 as core::ffi::c_int ^ 20 as core::ffi::c_int) as byte,
            lps_xor: (19 as core::ffi::c_int ^ 18 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x3001 as uint16_t,
            mps_xor: (20 as core::ffi::c_int ^ 21 as core::ffi::c_int) as byte,
            lps_xor: (20 as core::ffi::c_int ^ 19 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x2801 as uint16_t,
            mps_xor: (21 as core::ffi::c_int ^ 22 as core::ffi::c_int) as byte,
            lps_xor: (21 as core::ffi::c_int ^ 19 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x2401 as uint16_t,
            mps_xor: (22 as core::ffi::c_int ^ 23 as core::ffi::c_int) as byte,
            lps_xor: (22 as core::ffi::c_int ^ 20 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x2201 as uint16_t,
            mps_xor: (23 as core::ffi::c_int ^ 24 as core::ffi::c_int) as byte,
            lps_xor: (23 as core::ffi::c_int ^ 21 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1c01 as uint16_t,
            mps_xor: (24 as core::ffi::c_int ^ 25 as core::ffi::c_int) as byte,
            lps_xor: (24 as core::ffi::c_int ^ 22 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1801 as uint16_t,
            mps_xor: (25 as core::ffi::c_int ^ 26 as core::ffi::c_int) as byte,
            lps_xor: (25 as core::ffi::c_int ^ 23 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1601 as uint16_t,
            mps_xor: (26 as core::ffi::c_int ^ 27 as core::ffi::c_int) as byte,
            lps_xor: (26 as core::ffi::c_int ^ 24 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1401 as uint16_t,
            mps_xor: (27 as core::ffi::c_int ^ 28 as core::ffi::c_int) as byte,
            lps_xor: (27 as core::ffi::c_int ^ 25 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1201 as uint16_t,
            mps_xor: (28 as core::ffi::c_int ^ 29 as core::ffi::c_int) as byte,
            lps_xor: (28 as core::ffi::c_int ^ 26 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1101 as uint16_t,
            mps_xor: (29 as core::ffi::c_int ^ 30 as core::ffi::c_int) as byte,
            lps_xor: (29 as core::ffi::c_int ^ 27 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0xac1 as uint16_t,
            mps_xor: (30 as core::ffi::c_int ^ 31 as core::ffi::c_int) as byte,
            lps_xor: (30 as core::ffi::c_int ^ 28 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x9c1 as uint16_t,
            mps_xor: (31 as core::ffi::c_int ^ 32 as core::ffi::c_int) as byte,
            lps_xor: (31 as core::ffi::c_int ^ 29 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x8a1 as uint16_t,
            mps_xor: (32 as core::ffi::c_int ^ 33 as core::ffi::c_int) as byte,
            lps_xor: (32 as core::ffi::c_int ^ 30 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x521 as uint16_t,
            mps_xor: (33 as core::ffi::c_int ^ 34 as core::ffi::c_int) as byte,
            lps_xor: (33 as core::ffi::c_int ^ 31 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x441 as uint16_t,
            mps_xor: (34 as core::ffi::c_int ^ 35 as core::ffi::c_int) as byte,
            lps_xor: (34 as core::ffi::c_int ^ 32 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x2a1 as uint16_t,
            mps_xor: (35 as core::ffi::c_int ^ 36 as core::ffi::c_int) as byte,
            lps_xor: (35 as core::ffi::c_int ^ 33 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x221 as uint16_t,
            mps_xor: (36 as core::ffi::c_int ^ 37 as core::ffi::c_int) as byte,
            lps_xor: (36 as core::ffi::c_int ^ 34 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x141 as uint16_t,
            mps_xor: (37 as core::ffi::c_int ^ 38 as core::ffi::c_int) as byte,
            lps_xor: (37 as core::ffi::c_int ^ 35 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x111 as uint16_t,
            mps_xor: (38 as core::ffi::c_int ^ 39 as core::ffi::c_int) as byte,
            lps_xor: (38 as core::ffi::c_int ^ 36 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x85 as uint16_t,
            mps_xor: (39 as core::ffi::c_int ^ 40 as core::ffi::c_int) as byte,
            lps_xor: (39 as core::ffi::c_int ^ 37 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x49 as uint16_t,
            mps_xor: (40 as core::ffi::c_int ^ 41 as core::ffi::c_int) as byte,
            lps_xor: (40 as core::ffi::c_int ^ 38 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x25 as uint16_t,
            mps_xor: (41 as core::ffi::c_int ^ 42 as core::ffi::c_int) as byte,
            lps_xor: (41 as core::ffi::c_int ^ 39 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x15 as uint16_t,
            mps_xor: (42 as core::ffi::c_int ^ 43 as core::ffi::c_int) as byte,
            lps_xor: (42 as core::ffi::c_int ^ 40 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x9 as uint16_t,
            mps_xor: (43 as core::ffi::c_int ^ 44 as core::ffi::c_int) as byte,
            lps_xor: (43 as core::ffi::c_int ^ 41 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5 as uint16_t,
            mps_xor: (44 as core::ffi::c_int ^ 45 as core::ffi::c_int) as byte,
            lps_xor: (44 as core::ffi::c_int ^ 42 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x1 as uint16_t,
            mps_xor: (45 as core::ffi::c_int ^ 45 as core::ffi::c_int) as byte,
            lps_xor: (45 as core::ffi::c_int ^ 43 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
    {
        let mut init = Jbig2ArithQe {
            Qe: 0x5601 as uint16_t,
            mps_xor: (46 as core::ffi::c_int ^ 46 as core::ffi::c_int) as byte,
            lps_xor: (46 as core::ffi::c_int ^ 46 as core::ffi::c_int
                ^ (0 as core::ffi::c_int) << 7 as core::ffi::c_int) as byte,
        };
        init
    },
];
unsafe extern "C" fn jbig2_arith_renormd(
    mut ctx: *mut Jbig2Ctx,
    mut as_0: *mut Jbig2ArithState,
) -> core::ffi::c_int {
    loop {
        if (*as_0).CT == 0 as core::ffi::c_int
            && jbig2_arith_bytein(ctx, as_0) < 0 as core::ffi::c_int
        {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to read byte from compressed data stream\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        (*as_0).A <<= 1 as core::ffi::c_int;
        (*as_0).C <<= 1 as core::ffi::c_int;
        (*as_0).CT -= 1;
        if !((*as_0).A & 0x8000 as uint32_t == 0 as uint32_t) {
            break;
        }
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_arith_decode(
    mut ctx: *mut Jbig2Ctx,
    mut as_0: *mut Jbig2ArithState,
    mut pcx: *mut Jbig2ArithCx,
) -> core::ffi::c_int {
    let mut cx: Jbig2ArithCx = *pcx;
    let mut pqe: *const Jbig2ArithQe = 0 as *const Jbig2ArithQe;
    let mut index: core::ffi::c_uint = (cx as core::ffi::c_int
        & 0x7f as core::ffi::c_int) as core::ffi::c_uint;
    let mut D: core::ffi::c_int = 0;
    if index >= MAX_QE_ARRAY_SIZE as core::ffi::c_uint {
        return jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to determine probability estimate because index out of range\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    pqe = &*jbig2_arith_Qe.as_ptr().offset(index as isize) as *const Jbig2ArithQe;
    (*as_0).A = ((*as_0).A).wrapping_sub((*pqe).Qe as uint32_t);
    if ((*as_0).C >> 16 as core::ffi::c_int) < (*as_0).A {
        if (*as_0).A & 0x8000 as uint32_t == 0 as uint32_t {
            if (*as_0).A < (*pqe).Qe as uint32_t {
                D = 1 as core::ffi::c_int
                    - (cx as core::ffi::c_int >> 7 as core::ffi::c_int);
                *pcx = (*pcx as core::ffi::c_int ^ (*pqe).lps_xor as core::ffi::c_int)
                    as Jbig2ArithCx;
            } else {
                D = cx as core::ffi::c_int >> 7 as core::ffi::c_int;
                *pcx = (*pcx as core::ffi::c_int ^ (*pqe).mps_xor as core::ffi::c_int)
                    as Jbig2ArithCx;
            }
            if jbig2_arith_renormd(ctx, as_0) < 0 as core::ffi::c_int {
                return jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to renormalize decoder\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            return D;
        } else {
            return cx as core::ffi::c_int >> 7 as core::ffi::c_int
        }
    } else {
        (*as_0).C = ((*as_0).C).wrapping_sub((*as_0).A << 16 as core::ffi::c_int);
        if (*as_0).A < (*pqe).Qe as uint32_t {
            (*as_0).A = (*pqe).Qe as uint32_t;
            D = cx as core::ffi::c_int >> 7 as core::ffi::c_int;
            *pcx = (*pcx as core::ffi::c_int ^ (*pqe).mps_xor as core::ffi::c_int)
                as Jbig2ArithCx;
        } else {
            (*as_0).A = (*pqe).Qe as uint32_t;
            D = 1 as core::ffi::c_int
                - (cx as core::ffi::c_int >> 7 as core::ffi::c_int);
            *pcx = (*pcx as core::ffi::c_int ^ (*pqe).lps_xor as core::ffi::c_int)
                as Jbig2ArithCx;
        }
        if jbig2_arith_renormd(ctx, as_0) < 0 as core::ffi::c_int {
            return jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to renormalize decoder\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        return D;
    };
}

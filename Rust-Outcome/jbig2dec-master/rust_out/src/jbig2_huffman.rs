extern "C" {
    pub type _Jbig2Page;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn jbig2_get_int32(buf: *const byte) -> int32_t;
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
    fn jbig2_find_segment(ctx: *mut Jbig2Ctx, number: uint32_t) -> *mut Jbig2Segment;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2HuffmanEntry {
    pub u: C2RustUnnamed,
    pub PREFLEN: byte,
    pub RANGELEN: byte,
    pub flags: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RANGELOW: int32_t,
    pub ext_table: *mut Jbig2HuffmanTable,
}
pub type Jbig2HuffmanTable = _Jbig2HuffmanTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2HuffmanTable {
    pub log_table_size: core::ffi::c_int,
    pub entries: *mut Jbig2HuffmanEntry,
}
pub type Jbig2HuffmanEntry = _Jbig2HuffmanEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2HuffmanState {
    pub this_word: uint32_t,
    pub next_word: uint32_t,
    pub offset_bits: uint32_t,
    pub offset: uint32_t,
    pub offset_limit: uint32_t,
    pub ws: *mut Jbig2WordStream,
    pub ctx: *mut Jbig2Ctx,
}
pub type Jbig2HuffmanState = _Jbig2HuffmanState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2HuffmanParams {
    pub HTOOB: core::ffi::c_int,
    pub n_lines: core::ffi::c_int,
    pub lines: *const Jbig2HuffmanLine,
}
pub type Jbig2HuffmanLine = _Jbig2HuffmanLine;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Jbig2HuffmanLine {
    pub PREFLEN: core::ffi::c_int,
    pub RANGELEN: core::ffi::c_int,
    pub RANGELOW: core::ffi::c_int,
}
pub type Jbig2HuffmanParams = _Jbig2HuffmanParams;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const JBIG2_UNKNOWN_SEGMENT_NUMBER: core::ffi::c_uint = !(0 as core::ffi::c_uint);
pub const JBIG2_HUFFMAN_FLAGS_ISOOB: core::ffi::c_int = 1 as core::ffi::c_int;
pub const JBIG2_HUFFMAN_FLAGS_ISLOW: core::ffi::c_int = 2 as core::ffi::c_int;
pub const JBIG2_HUFFMAN_FLAGS_ISEXT: core::ffi::c_int = 4 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn jbig2_huffman_new(
    mut ctx: *mut Jbig2Ctx,
    mut ws: *mut Jbig2WordStream,
) -> *mut Jbig2HuffmanState {
    let mut result: *mut Jbig2HuffmanState = 0 as *mut Jbig2HuffmanState;
    let mut code: core::ffi::c_int = 0;
    result = jbig2_alloc(
        (*ctx).allocator,
        1 as size_t,
        ::core::mem::size_of::<Jbig2HuffmanState>() as size_t,
    ) as *mut Jbig2HuffmanState;
    if !result.is_null() {
        (*result).offset = 0 as uint32_t;
        (*result).offset_bits = 0 as uint32_t;
        (*result).offset_limit = 0 as uint32_t;
        (*result).ws = ws;
        (*result).ctx = ctx;
        code = ((*(*result).ws).get_next_word)
            .expect(
                "non-null function pointer",
            )((*result).ctx, (*result).ws, 0 as size_t, &mut (*result).this_word);
        if code < 0 as core::ffi::c_int {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to read first huffman word\0" as *const u8
                    as *const core::ffi::c_char,
            );
            jbig2_huffman_free(ctx, result);
            return 0 as *mut Jbig2HuffmanState;
        }
        code = ((*(*result).ws).get_next_word)
            .expect(
                "non-null function pointer",
            )((*result).ctx, (*result).ws, 4 as size_t, &mut (*result).next_word);
        if code < 0 as core::ffi::c_int {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to read second huffman word\0" as *const u8
                    as *const core::ffi::c_char,
            );
            jbig2_huffman_free(ctx, result);
            return 0 as *mut Jbig2HuffmanState;
        }
    } else {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate new huffman coding state\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2HuffmanState;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_huffman_free(
    mut ctx: *mut Jbig2Ctx,
    mut hs: *mut Jbig2HuffmanState,
) {
    jbig2_free((*ctx).allocator, hs as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_huffman_skip(
    mut hs: *mut Jbig2HuffmanState,
) -> core::ffi::c_int {
    let mut bits: uint32_t = (*hs).offset_bits & 7 as uint32_t;
    let mut code: core::ffi::c_int = 0;
    if bits != 0 {
        bits = (8 as uint32_t).wrapping_sub(bits);
        (*hs).offset_bits = ((*hs).offset_bits).wrapping_add(bits);
        (*hs).this_word = (*hs).this_word << bits
            | (*hs).next_word >> (32 as uint32_t).wrapping_sub((*hs).offset_bits);
    }
    if (*hs).offset_bits >= 32 as uint32_t {
        (*hs).this_word = (*hs).next_word;
        (*hs).offset = ((*hs).offset).wrapping_add(4 as uint32_t);
        code = ((*(*hs).ws).get_next_word)
            .expect(
                "non-null function pointer",
            )(
            (*hs).ctx,
            (*hs).ws,
            ((*hs).offset).wrapping_add(4 as uint32_t) as size_t,
            &mut (*hs).next_word,
        );
        if code < 0 as core::ffi::c_int {
            return jbig2_error(
                (*hs).ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to read next huffman word when skipping\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        (*hs).offset_bits = ((*hs).offset_bits).wrapping_sub(32 as uint32_t);
        if (*hs).offset_bits != 0 {
            (*hs).this_word = (*hs).this_word << (*hs).offset_bits
                | (*hs).next_word >> (32 as uint32_t).wrapping_sub((*hs).offset_bits);
        }
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_huffman_advance(
    mut hs: *mut Jbig2HuffmanState,
    mut advance: size_t,
) -> core::ffi::c_int {
    let mut code: core::ffi::c_int = 0;
    (*hs).offset = ((*hs).offset as size_t)
        .wrapping_add(advance & !(3 as core::ffi::c_int) as size_t) as uint32_t
        as uint32_t;
    (*hs).offset_bits = ((*hs).offset_bits as size_t)
        .wrapping_add((advance & 3 as size_t) << 3 as core::ffi::c_int) as uint32_t
        as uint32_t;
    if (*hs).offset_bits >= 32 as uint32_t {
        (*hs).offset = ((*hs).offset).wrapping_add(4 as uint32_t);
        (*hs).offset_bits = ((*hs).offset_bits).wrapping_sub(32 as uint32_t);
    }
    code = ((*(*hs).ws).get_next_word)
        .expect(
            "non-null function pointer",
        )((*hs).ctx, (*hs).ws, (*hs).offset as size_t, &mut (*hs).this_word);
    if code < 0 as core::ffi::c_int {
        return jbig2_error(
            (*hs).ctx,
            JBIG2_SEVERITY_WARNING,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to get first huffman word after advancing\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    code = ((*(*hs).ws).get_next_word)
        .expect(
            "non-null function pointer",
        )(
        (*hs).ctx,
        (*hs).ws,
        ((*hs).offset).wrapping_add(4 as uint32_t) as size_t,
        &mut (*hs).next_word,
    );
    if code < 0 as core::ffi::c_int {
        return jbig2_error(
            (*hs).ctx,
            JBIG2_SEVERITY_WARNING,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to get second huffman word after advancing\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if (*hs).offset_bits > 0 as uint32_t {
        (*hs).this_word = (*hs).this_word << (*hs).offset_bits
            | (*hs).next_word >> (32 as uint32_t).wrapping_sub((*hs).offset_bits);
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_huffman_offset(
    mut hs: *mut Jbig2HuffmanState,
) -> uint32_t {
    return ((*hs).offset).wrapping_add((*hs).offset_bits >> 3 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_huffman_get_bits(
    mut hs: *mut Jbig2HuffmanState,
    bits: core::ffi::c_int,
    mut err: *mut core::ffi::c_int,
) -> int32_t {
    let mut this_word: uint32_t = (*hs).this_word;
    let mut result: int32_t = 0;
    let mut code: core::ffi::c_int = 0;
    if (*hs).offset_limit != 0 && (*hs).offset >= (*hs).offset_limit {
        *err = -(1 as core::ffi::c_int);
        return jbig2_error(
            (*hs).ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"end of jbig2 buffer reached at offset %d\0" as *const u8
                as *const core::ffi::c_char,
            (*hs).offset,
        ) as int32_t;
    }
    result = (this_word >> 32 as core::ffi::c_int - bits) as int32_t;
    (*hs).offset_bits = ((*hs).offset_bits).wrapping_add(bits as uint32_t);
    if (*hs).offset_bits >= 32 as uint32_t {
        (*hs).offset = ((*hs).offset).wrapping_add(4 as uint32_t);
        (*hs).offset_bits = ((*hs).offset_bits).wrapping_sub(32 as uint32_t);
        (*hs).this_word = (*hs).next_word;
        code = ((*(*hs).ws).get_next_word)
            .expect(
                "non-null function pointer",
            )(
            (*hs).ctx,
            (*hs).ws,
            ((*hs).offset).wrapping_add(4 as uint32_t) as size_t,
            &mut (*hs).next_word,
        );
        if code < 0 as core::ffi::c_int {
            return jbig2_error(
                (*hs).ctx,
                JBIG2_SEVERITY_WARNING,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"failed to get next huffman word\0" as *const u8
                    as *const core::ffi::c_char,
            ) as int32_t;
        }
        if (*hs).offset_bits != 0 {
            (*hs).this_word = (*hs).this_word << (*hs).offset_bits
                | (*hs).next_word >> (32 as uint32_t).wrapping_sub((*hs).offset_bits);
        } else {
            (*hs).this_word = (*hs).this_word << (*hs).offset_bits;
        }
    } else {
        (*hs).this_word = this_word << bits
            | (*hs).next_word >> (32 as uint32_t).wrapping_sub((*hs).offset_bits);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_huffman_get(
    mut hs: *mut Jbig2HuffmanState,
    mut table: *const Jbig2HuffmanTable,
    mut oob: *mut core::ffi::c_int,
) -> int32_t {
    let mut entry: *mut Jbig2HuffmanEntry = 0 as *mut Jbig2HuffmanEntry;
    let mut flags: byte = 0;
    let mut offset_bits: core::ffi::c_int = (*hs).offset_bits as core::ffi::c_int;
    let mut this_word: uint32_t = (*hs).this_word;
    let mut next_word: uint32_t = 0;
    let mut RANGELEN: core::ffi::c_int = 0;
    let mut result: int32_t = 0;
    if (*hs).offset_limit != 0 && (*hs).offset >= (*hs).offset_limit {
        if !oob.is_null() {
            *oob = -(1 as core::ffi::c_int);
        }
        return jbig2_error(
            (*hs).ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"end of Jbig2WordStream reached at offset %d\0" as *const u8
                as *const core::ffi::c_char,
            (*hs).offset,
        ) as int32_t;
    }
    loop {
        let mut log_table_size: core::ffi::c_int = (*table).log_table_size;
        let mut PREFLEN: core::ffi::c_int = 0;
        let mut code: core::ffi::c_int = 0;
        entry = &mut *((*table).entries)
            .offset(
                (if log_table_size > 0 as core::ffi::c_int {
                    this_word >> 32 as core::ffi::c_int - log_table_size
                } else {
                    0 as uint32_t
                }) as isize,
            ) as *mut Jbig2HuffmanEntry;
        flags = (*entry).flags;
        PREFLEN = (*entry).PREFLEN as core::ffi::c_int;
        if flags as core::ffi::c_int
            == -(1 as core::ffi::c_int) as byte as core::ffi::c_int
            || PREFLEN == -(1 as core::ffi::c_int) as byte as core::ffi::c_int
        {
            if !oob.is_null() {
                *oob = -(1 as core::ffi::c_int);
            }
            return jbig2_error(
                (*hs).ctx,
                JBIG2_SEVERITY_FATAL,
                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                b"encountered unpopulated huffman table entry\0" as *const u8
                    as *const core::ffi::c_char,
            ) as int32_t;
        }
        next_word = (*hs).next_word;
        offset_bits += PREFLEN;
        if offset_bits >= 32 as core::ffi::c_int {
            this_word = next_word;
            (*hs).offset = ((*hs).offset).wrapping_add(4 as uint32_t);
            code = ((*(*hs).ws).get_next_word)
                .expect(
                    "non-null function pointer",
                )(
                (*hs).ctx,
                (*hs).ws,
                ((*hs).offset).wrapping_add(4 as uint32_t) as size_t,
                &mut next_word,
            );
            if code < 0 as core::ffi::c_int {
                return jbig2_error(
                    (*hs).ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to get next huffman word\0" as *const u8
                        as *const core::ffi::c_char,
                ) as int32_t;
            }
            offset_bits -= 32 as core::ffi::c_int;
            (*hs).next_word = next_word;
            PREFLEN = offset_bits;
        }
        if PREFLEN != 0 {
            this_word = this_word << PREFLEN
                | next_word >> 32 as core::ffi::c_int - offset_bits;
        }
        if !(flags as core::ffi::c_int & JBIG2_HUFFMAN_FLAGS_ISEXT != 0) {
            break;
        }
        table = (*entry).u.ext_table;
    }
    result = (*entry).u.RANGELOW;
    RANGELEN = (*entry).RANGELEN as core::ffi::c_int;
    if RANGELEN > 0 as core::ffi::c_int {
        let mut HTOFFSET: int32_t = 0;
        let mut code_0: core::ffi::c_int = 0;
        HTOFFSET = (this_word >> 32 as core::ffi::c_int - RANGELEN) as int32_t;
        if flags as core::ffi::c_int & JBIG2_HUFFMAN_FLAGS_ISLOW != 0 {
            result -= HTOFFSET;
        } else {
            result += HTOFFSET;
        }
        offset_bits += RANGELEN;
        if offset_bits >= 32 as core::ffi::c_int {
            this_word = next_word;
            (*hs).offset = ((*hs).offset).wrapping_add(4 as uint32_t);
            code_0 = ((*(*hs).ws).get_next_word)
                .expect(
                    "non-null function pointer",
                )(
                (*hs).ctx,
                (*hs).ws,
                ((*hs).offset).wrapping_add(4 as uint32_t) as size_t,
                &mut next_word,
            );
            if code_0 < 0 as core::ffi::c_int {
                return jbig2_error(
                    (*hs).ctx,
                    JBIG2_SEVERITY_WARNING,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"failed to get next huffman word\0" as *const u8
                        as *const core::ffi::c_char,
                ) as int32_t;
            }
            offset_bits -= 32 as core::ffi::c_int;
            (*hs).next_word = next_word;
            RANGELEN = offset_bits;
        }
        if RANGELEN != 0 {
            this_word = this_word << RANGELEN
                | next_word >> 32 as core::ffi::c_int - offset_bits;
        }
    }
    (*hs).this_word = this_word;
    (*hs).offset_bits = offset_bits as uint32_t;
    if !oob.is_null() {
        *oob = flags as core::ffi::c_int & JBIG2_HUFFMAN_FLAGS_ISOOB;
    }
    return result;
}
pub const LOG_TABLE_SIZE_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn jbig2_build_huffman_table(
    mut ctx: *mut Jbig2Ctx,
    mut params: *const Jbig2HuffmanParams,
) -> *mut Jbig2HuffmanTable {
    let mut LENCOUNT: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut LENMAX: core::ffi::c_int = -(1 as core::ffi::c_int);
    let lencountcount: core::ffi::c_int = 256 as core::ffi::c_int;
    let mut lines: *const Jbig2HuffmanLine = (*params).lines;
    let mut n_lines: core::ffi::c_int = (*params).n_lines;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut max_j: uint32_t = 0;
    let mut log_table_size: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut result: *mut Jbig2HuffmanTable = 0 as *mut Jbig2HuffmanTable;
    let mut entries: *mut Jbig2HuffmanEntry = 0 as *mut Jbig2HuffmanEntry;
    let mut CURLEN: core::ffi::c_int = 0;
    let mut firstcode: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut CURCODE: core::ffi::c_int = 0;
    let mut CURTEMP: core::ffi::c_int = 0;
    LENCOUNT = jbig2_alloc(
        (*ctx).allocator,
        lencountcount as size_t,
        ::core::mem::size_of::<core::ffi::c_int>() as size_t,
    ) as *mut core::ffi::c_int;
    if LENCOUNT.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate huffman histogram\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut Jbig2HuffmanTable;
    }
    memset(
        LENCOUNT as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(lencountcount as size_t),
    );
    i = 0 as core::ffi::c_int;
    while i < (*params).n_lines {
        let mut PREFLEN: core::ffi::c_int = (*lines.offset(i as isize)).PREFLEN;
        let mut lts: core::ffi::c_int = 0;
        if PREFLEN > LENMAX {
            j = LENMAX + 1 as core::ffi::c_int;
            while j < PREFLEN + 1 as core::ffi::c_int {
                *LENCOUNT.offset(j as isize) = 0 as core::ffi::c_int;
                j += 1;
            }
            LENMAX = PREFLEN;
        }
        let ref mut fresh0 = *LENCOUNT.offset(PREFLEN as isize);
        *fresh0 += 1;
        lts = PREFLEN + (*lines.offset(i as isize)).RANGELEN;
        if lts > LOG_TABLE_SIZE_MAX {
            lts = PREFLEN;
        }
        if lts <= LOG_TABLE_SIZE_MAX && log_table_size < lts {
            log_table_size = lts;
        }
        i += 1;
    }
    jbig2_error(
        ctx,
        JBIG2_SEVERITY_DEBUG,
        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
        b"constructing huffman table log size %d\0" as *const u8
            as *const core::ffi::c_char,
        log_table_size,
    );
    max_j = ((1 as core::ffi::c_int) << log_table_size) as uint32_t;
    result = jbig2_alloc(
        (*ctx).allocator,
        1 as size_t,
        ::core::mem::size_of::<Jbig2HuffmanTable>() as size_t,
    ) as *mut Jbig2HuffmanTable;
    if result.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate result\0" as *const u8 as *const core::ffi::c_char,
        );
        jbig2_free((*ctx).allocator, LENCOUNT as *mut core::ffi::c_void);
        return 0 as *mut Jbig2HuffmanTable;
    }
    (*result).log_table_size = log_table_size;
    entries = jbig2_alloc(
        (*ctx).allocator,
        max_j as size_t,
        ::core::mem::size_of::<Jbig2HuffmanEntry>() as size_t,
    ) as *mut Jbig2HuffmanEntry;
    if entries.is_null() {
        jbig2_error(
            ctx,
            JBIG2_SEVERITY_FATAL,
            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
            b"failed to allocate result entries\0" as *const u8
                as *const core::ffi::c_char,
        );
        jbig2_free((*ctx).allocator, result as *mut core::ffi::c_void);
        jbig2_free((*ctx).allocator, LENCOUNT as *mut core::ffi::c_void);
        return 0 as *mut Jbig2HuffmanTable;
    }
    memset(
        entries as *mut core::ffi::c_void,
        0xff as core::ffi::c_int,
        (::core::mem::size_of::<Jbig2HuffmanEntry>() as size_t)
            .wrapping_mul(max_j as size_t),
    );
    (*result).entries = entries;
    *LENCOUNT.offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
    CURLEN = 1 as core::ffi::c_int;
    while CURLEN <= LENMAX {
        let mut shift: core::ffi::c_int = log_table_size - CURLEN;
        firstcode = firstcode
            + *LENCOUNT.offset((CURLEN - 1 as core::ffi::c_int) as isize)
            << 1 as core::ffi::c_int;
        CURCODE = firstcode;
        CURTEMP = 0 as core::ffi::c_int;
        while CURTEMP < n_lines {
            let mut PREFLEN_0: core::ffi::c_int = (*lines.offset(CURTEMP as isize))
                .PREFLEN;
            if PREFLEN_0 == CURLEN {
                let mut RANGELEN: core::ffi::c_int = (*lines.offset(CURTEMP as isize))
                    .RANGELEN;
                let mut start_j: uint32_t = (CURCODE << shift) as uint32_t;
                let mut end_j: uint32_t = ((CURCODE + 1 as core::ffi::c_int) << shift)
                    as uint32_t;
                let mut cur_j: uint32_t = 0;
                let mut eflags: byte = 0 as byte;
                if end_j > max_j {
                    jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_FATAL,
                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                        b"ran off the end of the entries table! (%d >= %d)\0"
                            as *const u8 as *const core::ffi::c_char,
                        end_j,
                        max_j,
                    );
                    jbig2_free(
                        (*ctx).allocator,
                        (*result).entries as *mut core::ffi::c_void,
                    );
                    jbig2_free((*ctx).allocator, result as *mut core::ffi::c_void);
                    jbig2_free((*ctx).allocator, LENCOUNT as *mut core::ffi::c_void);
                    return 0 as *mut Jbig2HuffmanTable;
                }
                if (*params).HTOOB != 0 && CURTEMP == n_lines - 1 as core::ffi::c_int {
                    eflags = (eflags as core::ffi::c_int | JBIG2_HUFFMAN_FLAGS_ISOOB)
                        as byte;
                }
                if CURTEMP
                    == n_lines
                        - (if (*params).HTOOB != 0 {
                            3 as core::ffi::c_int
                        } else {
                            2 as core::ffi::c_int
                        })
                {
                    eflags = (eflags as core::ffi::c_int | JBIG2_HUFFMAN_FLAGS_ISLOW)
                        as byte;
                }
                if PREFLEN_0 + RANGELEN > LOG_TABLE_SIZE_MAX {
                    cur_j = start_j;
                    while cur_j < end_j {
                        (*entries.offset(cur_j as isize)).u.RANGELOW = (*lines
                            .offset(CURTEMP as isize))
                            .RANGELOW as int32_t;
                        (*entries.offset(cur_j as isize)).PREFLEN = PREFLEN_0 as byte;
                        (*entries.offset(cur_j as isize)).RANGELEN = RANGELEN as byte;
                        (*entries.offset(cur_j as isize)).flags = eflags;
                        cur_j = cur_j.wrapping_add(1);
                    }
                } else {
                    cur_j = start_j;
                    while cur_j < end_j {
                        let mut HTOFFSET: int32_t = (cur_j >> shift - RANGELEN
                            & (((1 as core::ffi::c_int) << RANGELEN)
                                - 1 as core::ffi::c_int) as uint32_t) as int32_t;
                        if eflags as core::ffi::c_int & JBIG2_HUFFMAN_FLAGS_ISLOW != 0 {
                            (*entries.offset(cur_j as isize)).u.RANGELOW = (*lines
                                .offset(CURTEMP as isize))
                                .RANGELOW as int32_t - HTOFFSET;
                        } else {
                            (*entries.offset(cur_j as isize)).u.RANGELOW = (*lines
                                .offset(CURTEMP as isize))
                                .RANGELOW as int32_t + HTOFFSET;
                        }
                        (*entries.offset(cur_j as isize)).PREFLEN = (PREFLEN_0
                            + RANGELEN) as byte;
                        (*entries.offset(cur_j as isize)).RANGELEN = 0 as byte;
                        (*entries.offset(cur_j as isize)).flags = eflags;
                        cur_j = cur_j.wrapping_add(1);
                    }
                }
                CURCODE += 1;
            }
            CURTEMP += 1;
        }
        CURLEN += 1;
    }
    jbig2_free((*ctx).allocator, LENCOUNT as *mut core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_release_huffman_table(
    mut ctx: *mut Jbig2Ctx,
    mut table: *mut Jbig2HuffmanTable,
) {
    if !table.is_null() {
        jbig2_free((*ctx).allocator, (*table).entries as *mut core::ffi::c_void);
        jbig2_free((*ctx).allocator, table as *mut core::ffi::c_void);
    }
}
unsafe extern "C" fn jbig2_table_read_bits(
    mut data: *const byte,
    mut bitoffset: *mut size_t,
    bitlen: core::ffi::c_int,
) -> uint32_t {
    let mut result: uint32_t = 0 as uint32_t;
    let mut byte_offset: uint32_t = (*bitoffset).wrapping_div(8 as size_t) as uint32_t;
    let endbit: core::ffi::c_int = (*bitoffset & 7 as size_t)
        .wrapping_add(bitlen as size_t) as core::ffi::c_int;
    let n_proc_bytes: core::ffi::c_int = (endbit + 7 as core::ffi::c_int)
        / 8 as core::ffi::c_int;
    let rshift: core::ffi::c_int = n_proc_bytes * 8 as core::ffi::c_int - endbit;
    let mut i: core::ffi::c_int = 0;
    i = n_proc_bytes - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let fresh1 = byte_offset;
        byte_offset = byte_offset.wrapping_add(1);
        let mut d: uint32_t = *data.offset(fresh1 as isize) as uint32_t;
        let nshift: core::ffi::c_int = i * 8 as core::ffi::c_int - rshift;
        if nshift > 0 as core::ffi::c_int {
            d <<= nshift;
        } else if nshift < 0 as core::ffi::c_int {
            d >>= -nshift;
        }
        result |= d;
        i -= 1;
    }
    result &= !(-(1 as core::ffi::c_int) << bitlen) as uint32_t;
    *bitoffset = (*bitoffset).wrapping_add(bitlen as size_t);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_table(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut segment_data: *const byte,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut params: *mut Jbig2HuffmanParams = 0 as *mut Jbig2HuffmanParams;
    let mut line: *mut Jbig2HuffmanLine = 0 as *mut Jbig2HuffmanLine;
    (*segment).result = NULL;
    if (*segment).data_length < 10 as size_t {
        current_block = 6434727510068703521;
    } else {
        let code_table_flags: core::ffi::c_int = *segment_data
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
        let HTOOB: core::ffi::c_int = code_table_flags & 0x1 as core::ffi::c_int;
        let HTPS: core::ffi::c_int = (code_table_flags >> 1 as core::ffi::c_int
            & 0x7 as core::ffi::c_int) + 1 as core::ffi::c_int;
        let HTRS: core::ffi::c_int = (code_table_flags >> 4 as core::ffi::c_int
            & 0x7 as core::ffi::c_int) + 1 as core::ffi::c_int;
        let HTLOW: int32_t = jbig2_get_int32(
            segment_data.offset(1 as core::ffi::c_int as isize),
        ) as int32_t;
        let HTHIGH: int32_t = jbig2_get_int32(
            segment_data.offset(5 as core::ffi::c_int as isize),
        ) as int32_t;
        let lines_max: size_t = ((*segment).data_length)
            .wrapping_mul(8 as size_t)
            .wrapping_sub(
                (HTPS
                    * (if HTOOB != 0 {
                        3 as core::ffi::c_int
                    } else {
                        2 as core::ffi::c_int
                    })) as size_t,
            )
            .wrapping_div((HTPS + HTRS) as size_t)
            .wrapping_add(
                (if HTOOB != 0 { 3 as core::ffi::c_int } else { 2 as core::ffi::c_int })
                    as size_t,
            );
        let mut lines_data: *const byte = segment_data
            .offset(9 as core::ffi::c_int as isize);
        let lines_data_bitlen: size_t = ((*segment).data_length)
            .wrapping_sub(9 as size_t)
            .wrapping_mul(8 as size_t);
        let mut boffset: size_t = 0 as size_t;
        let mut CURRANGELOW: int32_t = HTLOW;
        let mut NTEMP: size_t = 0 as size_t;
        if HTLOW >= HTHIGH {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                (*segment).number,
                b"invalid Huffman Table range\0" as *const u8 as *const core::ffi::c_char,
            );
            current_block = 1104916756744550168;
        } else {
            params = jbig2_alloc(
                (*ctx).allocator,
                1 as size_t,
                ::core::mem::size_of::<Jbig2HuffmanParams>() as size_t,
            ) as *mut Jbig2HuffmanParams;
            if params.is_null() {
                jbig2_error(
                    ctx,
                    JBIG2_SEVERITY_FATAL,
                    (*segment).number,
                    b"failed to allocate Huffman Table Parameter\0" as *const u8
                        as *const core::ffi::c_char,
                );
                current_block = 1104916756744550168;
            } else {
                line = jbig2_alloc(
                    (*ctx).allocator,
                    lines_max,
                    ::core::mem::size_of::<Jbig2HuffmanLine>() as size_t,
                ) as *mut Jbig2HuffmanLine;
                if line.is_null() {
                    jbig2_error(
                        ctx,
                        JBIG2_SEVERITY_FATAL,
                        (*segment).number,
                        b"failed to allocate huffman table lines\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    current_block = 1104916756744550168;
                } else {
                    loop {
                        if !(CURRANGELOW < HTHIGH) {
                            current_block = 224731115979188411;
                            break;
                        }
                        if boffset.wrapping_add(HTPS as size_t) >= lines_data_bitlen {
                            current_block = 6434727510068703521;
                            break;
                        }
                        (*line.offset(NTEMP as isize)).PREFLEN = jbig2_table_read_bits(
                            lines_data,
                            &mut boffset,
                            HTPS,
                        ) as core::ffi::c_int;
                        if boffset.wrapping_add(HTRS as size_t) >= lines_data_bitlen {
                            current_block = 6434727510068703521;
                            break;
                        }
                        (*line.offset(NTEMP as isize)).RANGELEN = jbig2_table_read_bits(
                            lines_data,
                            &mut boffset,
                            HTRS,
                        ) as core::ffi::c_int;
                        (*line.offset(NTEMP as isize)).RANGELOW = CURRANGELOW
                            as core::ffi::c_int;
                        CURRANGELOW = (CURRANGELOW as core::ffi::c_int
                            + ((1 as core::ffi::c_int)
                                << (*line.offset(NTEMP as isize)).RANGELEN)) as int32_t;
                        NTEMP = NTEMP.wrapping_add(1);
                    }
                    match current_block {
                        6434727510068703521 => {}
                        _ => {
                            if boffset.wrapping_add(HTPS as size_t) >= lines_data_bitlen
                            {
                                current_block = 6434727510068703521;
                            } else {
                                (*line.offset(NTEMP as isize)).PREFLEN = jbig2_table_read_bits(
                                    lines_data,
                                    &mut boffset,
                                    HTPS,
                                ) as core::ffi::c_int;
                                (*line.offset(NTEMP as isize)).RANGELEN = 32
                                    as core::ffi::c_int;
                                (*line.offset(NTEMP as isize)).RANGELOW = (HTLOW
                                    - 1 as int32_t) as core::ffi::c_int;
                                NTEMP = NTEMP.wrapping_add(1);
                                if boffset.wrapping_add(HTPS as size_t) >= lines_data_bitlen
                                {
                                    current_block = 6434727510068703521;
                                } else {
                                    (*line.offset(NTEMP as isize)).PREFLEN = jbig2_table_read_bits(
                                        lines_data,
                                        &mut boffset,
                                        HTPS,
                                    ) as core::ffi::c_int;
                                    (*line.offset(NTEMP as isize)).RANGELEN = 32
                                        as core::ffi::c_int;
                                    (*line.offset(NTEMP as isize)).RANGELOW = HTHIGH
                                        as core::ffi::c_int;
                                    NTEMP = NTEMP.wrapping_add(1);
                                    if HTOOB != 0 {
                                        if boffset.wrapping_add(HTPS as size_t) >= lines_data_bitlen
                                        {
                                            current_block = 6434727510068703521;
                                        } else {
                                            (*line.offset(NTEMP as isize)).PREFLEN = jbig2_table_read_bits(
                                                lines_data,
                                                &mut boffset,
                                                HTPS,
                                            ) as core::ffi::c_int;
                                            (*line.offset(NTEMP as isize)).RANGELEN = 0
                                                as core::ffi::c_int;
                                            (*line.offset(NTEMP as isize)).RANGELOW = 0
                                                as core::ffi::c_int;
                                            NTEMP = NTEMP.wrapping_add(1);
                                            current_block = 18377268871191777778;
                                        }
                                    } else {
                                        current_block = 18377268871191777778;
                                    }
                                    match current_block {
                                        6434727510068703521 => {}
                                        _ => {
                                            if NTEMP != lines_max {
                                                let mut new_line: *mut Jbig2HuffmanLine = jbig2_realloc(
                                                    (*ctx).allocator,
                                                    line as *mut core::ffi::c_void,
                                                    NTEMP,
                                                    ::core::mem::size_of::<Jbig2HuffmanLine>() as size_t,
                                                ) as *mut Jbig2HuffmanLine;
                                                if new_line.is_null() {
                                                    jbig2_error(
                                                        ctx,
                                                        JBIG2_SEVERITY_FATAL,
                                                        (*segment).number,
                                                        b"failed to reallocate huffman table lines\0" as *const u8
                                                            as *const core::ffi::c_char,
                                                    );
                                                    current_block = 1104916756744550168;
                                                } else {
                                                    line = new_line;
                                                    current_block = 8180496224585318153;
                                                }
                                            } else {
                                                current_block = 8180496224585318153;
                                            }
                                            match current_block {
                                                1104916756744550168 => {}
                                                _ => {
                                                    (*params).HTOOB = HTOOB;
                                                    (*params).n_lines = NTEMP as core::ffi::c_int;
                                                    (*params).lines = line;
                                                    (*segment).result = params as *mut core::ffi::c_void;
                                                    return 0 as core::ffi::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        6434727510068703521 => {
            jbig2_error(
                ctx,
                JBIG2_SEVERITY_FATAL,
                (*segment).number,
                b"segment too short\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        _ => {}
    }
    jbig2_free((*ctx).allocator, line as *mut core::ffi::c_void);
    jbig2_free((*ctx).allocator, params as *mut core::ffi::c_void);
    return -(1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_table_free(
    mut ctx: *mut Jbig2Ctx,
    mut params: *mut Jbig2HuffmanParams,
) {
    if !params.is_null() {
        jbig2_free((*ctx).allocator, (*params).lines as *mut core::ffi::c_void);
        jbig2_free((*ctx).allocator, params as *mut core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jbig2_find_table(
    mut ctx: *mut Jbig2Ctx,
    mut segment: *mut Jbig2Segment,
    mut index: core::ffi::c_int,
) -> *const Jbig2HuffmanParams {
    let mut i: core::ffi::c_int = 0;
    let mut table_index: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < (*segment).referred_to_segment_count {
        let rsegment: *const Jbig2Segment = jbig2_find_segment(
            ctx,
            *((*segment).referred_to_segments).offset(i as isize),
        );
        if !rsegment.is_null()
            && (*rsegment).flags as core::ffi::c_int & 63 as core::ffi::c_int
                == 53 as core::ffi::c_int
        {
            if table_index == index {
                return (*rsegment).result as *const Jbig2HuffmanParams;
            }
            table_index += 1;
        }
        i += 1;
    }
    jbig2_error(
        ctx,
        JBIG2_SEVERITY_FATAL,
        (*segment).number,
        b"huffman table not found (%d)\0" as *const u8 as *const core::ffi::c_char,
        index,
    );
    return 0 as *const Jbig2HuffmanParams;
}

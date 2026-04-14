extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _Jbig2Page;
    pub type _Jbig2Segment;
    pub type _Jbig2GlobalCtx;
    static mut stdout: *mut FILE;
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
    fn snprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn sscanf(
        __s: *const core::ffi::c_char,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strdup(__s: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    fn strrchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    static mut optarg: *mut core::ffi::c_char;
    static mut optind: core::ffi::c_int;
    fn getopt_long(
        __argc: core::ffi::c_int,
        __argv: *const *mut core::ffi::c_char,
        __shortopts: *const core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn SHA1_Init(context: *mut SHA1_CTX);
    fn SHA1_Update(context: *mut SHA1_CTX, data: *const uint8_t, len: size_t);
    fn SHA1_Final(context: *mut SHA1_CTX, digest: *mut uint8_t);
    fn jbig2_ctx_new_imp(
        allocator: *mut Jbig2Allocator,
        options: Jbig2Options,
        global_ctx: *mut Jbig2GlobalCtx,
        error_callback_0: Jbig2ErrorCallback,
        error_callback_data: *mut core::ffi::c_void,
        jbig2_version_major: core::ffi::c_int,
        jbig2_version_minor: core::ffi::c_int,
    ) -> *mut Jbig2Ctx;
    fn jbig2_ctx_free(ctx: *mut Jbig2Ctx) -> *mut Jbig2Allocator;
    fn jbig2_make_global_ctx(ctx: *mut Jbig2Ctx) -> *mut Jbig2GlobalCtx;
    fn jbig2_global_ctx_free(global_ctx: *mut Jbig2GlobalCtx) -> *mut Jbig2Allocator;
    fn jbig2_data_in(
        ctx: *mut Jbig2Ctx,
        data: *const core::ffi::c_uchar,
        size: size_t,
    ) -> core::ffi::c_int;
    fn jbig2_page_out(ctx: *mut Jbig2Ctx) -> *mut Jbig2Image;
    fn jbig2_release_page(ctx: *mut Jbig2Ctx, image: *mut Jbig2Image);
    fn jbig2_complete_page(ctx: *mut Jbig2Ctx) -> core::ffi::c_int;
    fn jbig2_error(
        ctx: *mut Jbig2Ctx,
        severity: Jbig2Severity,
        seg_idx: uint32_t,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn jbig2_image_write_pbm(image: *mut Jbig2Image, out: *mut FILE) -> core::ffi::c_int;
    fn jbig2dec_alloc(
        allocator_: *mut Jbig2Allocator,
        size: size_t,
    ) -> *mut core::ffi::c_void;
    fn jbig2dec_realloc(
        allocator_: *mut Jbig2Allocator,
        p: *mut core::ffi::c_void,
        size: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
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
pub struct option {
    pub name: *const core::ffi::c_char,
    pub has_arg: core::ffi::c_int,
    pub flag: *mut core::ffi::c_int,
    pub val: core::ffi::c_int,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1_CTX {
    pub state: [uint32_t; 5],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
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
pub type Jbig2GlobalCtx = _Jbig2GlobalCtx;
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
pub type jbig2dec_mode = core::ffi::c_uint;
pub const render: jbig2dec_mode = 2;
pub const dump: jbig2dec_mode = 1;
pub const usage: jbig2dec_mode = 0;
pub type jbig2dec_format = core::ffi::c_uint;
pub const jbig2dec_format_pbm: jbig2dec_format = 2;
pub const jbig2dec_format_jbig2: jbig2dec_format = 1;
pub const jbig2dec_format_none: jbig2dec_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jbig2dec_params_t {
    pub mode: jbig2dec_mode,
    pub verbose: core::ffi::c_int,
    pub hash: core::ffi::c_int,
    pub embedded: core::ffi::c_int,
    pub hash_ctx: *mut SHA1_CTX,
    pub output_filename: *mut core::ffi::c_char,
    pub output_format: jbig2dec_format,
    pub memory_limit: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jbig2dec_error_callback_state_t {
    pub verbose: core::ffi::c_int,
    pub last_message: *mut core::ffi::c_char,
    pub severity: Jbig2Severity,
    pub type_0: *mut core::ffi::c_char,
    pub repeats: core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jbig2dec_allocator_t {
    pub super_0: Jbig2Allocator,
    pub ctx: *mut Jbig2Ctx,
    pub memory_limit: size_t,
    pub memory_used: size_t,
    pub memory_peak: size_t,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const SHA1_DIGEST_SIZE: core::ffi::c_int = 20 as core::ffi::c_int;
pub const JBIG2_VERSION_MAJOR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const JBIG2_VERSION_MINOR: core::ffi::c_int = 20 as core::ffi::c_int;
pub const JBIG2_UNKNOWN_SEGMENT_NUMBER: core::ffi::c_uint = !(0 as core::ffi::c_uint);
pub const ALIGNMENT: core::ffi::c_int = 16 as core::ffi::c_int;
pub const KBYTE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const MBYTE: core::ffi::c_int = 1024 as core::ffi::c_int * KBYTE;
unsafe extern "C" fn jbig2dec_free(
    mut allocator_: *mut Jbig2Allocator,
    mut p: *mut core::ffi::c_void,
) {
    let mut allocator: *mut jbig2dec_allocator_t = allocator_
        as *mut jbig2dec_allocator_t;
    let mut size: size_t = 0;
    if p.is_null() {
        return;
    }
    memcpy(
        &mut size as *mut size_t as *mut core::ffi::c_void,
        (p as *mut core::ffi::c_uchar).offset(-(ALIGNMENT as isize))
            as *const core::ffi::c_void,
        ::core::mem::size_of::<size_t>() as size_t,
    );
    (*allocator).memory_used = ((*allocator).memory_used)
        .wrapping_sub(size.wrapping_add(ALIGNMENT as size_t));
    free(
        (p as *mut core::ffi::c_uchar).offset(-(ALIGNMENT as isize))
            as *mut core::ffi::c_void,
    );
}
unsafe extern "C" fn hash_init(mut params: *mut jbig2dec_params_t) {
    (*params).hash_ctx = malloc(::core::mem::size_of::<SHA1_CTX>() as size_t)
        as *mut SHA1_CTX;
    if ((*params).hash_ctx).is_null() {
        fprintf(
            stderr,
            b"unable to allocate hash state\n\0" as *const u8 as *const core::ffi::c_char,
        );
        (*params).hash = 0 as core::ffi::c_int;
        return;
    } else {
        SHA1_Init((*params).hash_ctx);
    };
}
unsafe extern "C" fn hash_image(
    mut params: *mut jbig2dec_params_t,
    mut image: *mut Jbig2Image,
) {
    let mut N: core::ffi::c_uint = ((*image).stride as core::ffi::c_uint)
        .wrapping_mul((*image).height as core::ffi::c_uint);
    SHA1_Update((*params).hash_ctx, (*image).data, N as size_t);
}
unsafe extern "C" fn hash_print(mut params: *mut jbig2dec_params_t, mut out: *mut FILE) {
    let mut md: [core::ffi::c_uchar; 20] = [0; 20];
    let mut digest: [core::ffi::c_char; 41] = [0; 41];
    let mut i: core::ffi::c_int = 0;
    SHA1_Final((*params).hash_ctx, md.as_mut_ptr() as *mut uint8_t);
    i = 0 as core::ffi::c_int;
    while i < SHA1_DIGEST_SIZE {
        snprintf(
            &mut *digest.as_mut_ptr().offset((2 as core::ffi::c_int * i) as isize)
                as *mut core::ffi::c_char,
            3 as size_t,
            b"%02x\0" as *const u8 as *const core::ffi::c_char,
            md[i as usize] as core::ffi::c_int,
        );
        i += 1;
    }
    fprintf(out, b"%s\0" as *const u8 as *const core::ffi::c_char, digest.as_mut_ptr());
}
unsafe extern "C" fn hash_free(mut params: *mut jbig2dec_params_t) {
    free((*params).hash_ctx as *mut core::ffi::c_void);
    (*params).hash_ctx = 0 as *mut SHA1_CTX;
}
unsafe extern "C" fn set_output_format(
    mut params: *mut jbig2dec_params_t,
    mut format: *const core::ffi::c_char,
) -> core::ffi::c_int {
    (*params).output_format = jbig2dec_format_pbm;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn parse_options(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
    mut params: *mut jbig2dec_params_t,
) -> core::ffi::c_int {
    static mut long_options: [option; 10] = [
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 0 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 0 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 0 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"verbose\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 2 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dump\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 0 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"hash\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 0 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"output\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 1 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"format\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 1 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"embedded\0" as *const u8 as *const core::ffi::c_char,
                has_arg: 0 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const core::ffi::c_char,
                has_arg: 0 as core::ffi::c_int,
                flag: 0 as *const core::ffi::c_int as *mut core::ffi::c_int,
                val: 0 as core::ffi::c_int,
            };
            init
        },
    ];
    let mut option_idx: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut option: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int = 0;
    loop {
        option = getopt_long(
            argc,
            argv as *const *mut core::ffi::c_char,
            b"Vh?qv:do:t:eM:\0" as *const u8 as *const core::ffi::c_char,
            long_options.as_mut_ptr(),
            &mut option_idx,
        );
        if option == -(1 as core::ffi::c_int) {
            break;
        }
        match option {
            0 => {
                if (*params).verbose == 0 {
                    fprintf(
                        stdout,
                        b"unrecognized option: --%s\n\0" as *const u8
                            as *const core::ffi::c_char,
                        long_options[option_idx as usize].name,
                    );
                }
            }
            113 => {
                (*params).verbose = 0 as core::ffi::c_int;
            }
            118 => {
                if !optarg.is_null() {
                    (*params).verbose = atoi(optarg);
                } else {
                    (*params).verbose = 2 as core::ffi::c_int;
                }
            }
            104 | 63 => {
                (*params).mode = usage;
            }
            86 => {
                print_version();
                exit(0 as core::ffi::c_int);
            }
            100 => {
                (*params).mode = dump;
            }
            109 => {
                (*params).hash = 1 as core::ffi::c_int;
            }
            111 => {
                (*params).output_filename = strdup(optarg);
            }
            116 => {
                set_output_format(params, optarg);
            }
            101 => {
                (*params).embedded = 1 as core::ffi::c_int;
            }
            77 => {
                ret = sscanf(
                    optarg,
                    b"%zu\0" as *const u8 as *const core::ffi::c_char,
                    &mut (*params).memory_limit as *mut size_t,
                );
                if ret != 1 as core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"could not parse memory limit argument\n\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            }
            _ => {
                if (*params).verbose == 0 {
                    fprintf(
                        stderr,
                        b"unrecognized option: -%c\n\0" as *const u8
                            as *const core::ffi::c_char,
                        option,
                    );
                }
            }
        }
    }
    return optind;
}
unsafe extern "C" fn print_version() -> core::ffi::c_int {
    fprintf(
        stdout,
        b"jbig2dec %d.%d\n\0" as *const u8 as *const core::ffi::c_char,
        JBIG2_VERSION_MAJOR,
        JBIG2_VERSION_MINOR,
    );
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn print_usage() -> core::ffi::c_int {
    fprintf(
        stderr,
        b"Usage: jbig2dec [options] <file.jbig2>\n   or  jbig2dec [options] <global_stream> <page_stream>\n\n  When invoked with a single file, it attempts to parse it as\n  a normal jbig2 file. Invoked with two files, it treats the\n  first as the global segments, and the second as the segment\n  stream for a particular page. This is useful for examining\n  embedded streams.\n\n  available options:\n    -h --help       this usage summary\n    -q --quiet      suppress diagnostic output\n    -v --verbose    set the verbosity level\n    -d --dump       print the structure of the jbig2 file\n                    rather than explicitly decoding\n    -V --version    program name and version information\n    -m --hash       print a hash of the decoded document\n    -e --embedded   expect embedded bit stream without file header\n    -M <limit>      memory limit expressed in bytes\n    -o <file>\n    --output <file> send decoded output to <file>\n                    Defaults to the the input with a different\n                    extension. Pass '-' for stdout.\n    -t <type>\n    --format <type> force a particular output file format\n                    the only supported option is 'pbm'\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    return 1 as core::ffi::c_int;
}
unsafe extern "C" fn error_callback(
    mut error_callback_data: *mut core::ffi::c_void,
    mut message: *const core::ffi::c_char,
    mut severity: Jbig2Severity,
    mut seg_idx: uint32_t,
) {
    let mut current_block: u64;
    let mut state: *mut jbig2dec_error_callback_state_t = error_callback_data
        as *mut jbig2dec_error_callback_state_t;
    let mut type_0: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut ret: core::ffi::c_int = 0;
    match severity as core::ffi::c_uint {
        0 => {
            if (*state).verbose < 3 as core::ffi::c_int {
                return;
            }
            type_0 = b"DEBUG\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char;
        }
        1 => {
            if (*state).verbose < 2 as core::ffi::c_int {
                return;
            }
            type_0 = b"info\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char;
        }
        2 => {
            if (*state).verbose < 1 as core::ffi::c_int {
                return;
            }
            type_0 = b"WARNING\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char;
        }
        3 => {
            type_0 = b"FATAL ERROR\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char;
        }
        _ => {
            type_0 = b"unknown message\0" as *const u8 as *const core::ffi::c_char
                as *mut core::ffi::c_char;
        }
    }
    if !((*state).last_message).is_null() && strcmp(message, (*state).last_message) == 0
        && (*state).severity as core::ffi::c_uint == severity as core::ffi::c_uint
        && (*state).type_0 == type_0
    {
        (*state).repeats += 1;
        if (*state).repeats % 1000000 as core::ffi::c_long == 0 as core::ffi::c_long {
            ret = fprintf(
                stderr,
                b"jbig2dec %s last message repeated %ld times so far\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*state).type_0,
                (*state).repeats,
            );
            if ret < 0 as core::ffi::c_int {
                current_block = 15937355050609159083;
            } else {
                current_block = 14136749492126903395;
            }
        } else {
            current_block = 14136749492126903395;
        }
    } else {
        if (*state).repeats > 1 as core::ffi::c_long {
            ret = fprintf(
                stderr,
                b"jbig2dec %s last message repeated %ld times\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*state).type_0,
                (*state).repeats,
            );
            if ret < 0 as core::ffi::c_int {
                current_block = 15937355050609159083;
            } else {
                current_block = 4495394744059808450;
            }
        } else {
            current_block = 4495394744059808450;
        }
        match current_block {
            15937355050609159083 => {}
            _ => {
                if seg_idx == JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t {
                    ret = fprintf(
                        stderr,
                        b"jbig2dec %s %s\n\0" as *const u8 as *const core::ffi::c_char,
                        type_0,
                        message,
                    );
                } else {
                    ret = fprintf(
                        stderr,
                        b"jbig2dec %s %s (segment 0x%08x)\n\0" as *const u8
                            as *const core::ffi::c_char,
                        type_0,
                        message,
                        seg_idx,
                    );
                }
                if ret < 0 as core::ffi::c_int {
                    current_block = 15937355050609159083;
                } else {
                    (*state).repeats = 0 as core::ffi::c_long;
                    (*state).severity = severity;
                    (*state).type_0 = type_0;
                    free((*state).last_message as *mut core::ffi::c_void);
                    (*state).last_message = 0 as *mut core::ffi::c_char;
                    if !message.is_null() {
                        (*state).last_message = strdup(message);
                        if ((*state).last_message).is_null() {
                            ret = fprintf(
                                stderr,
                                b"jbig2dec WARNING could not duplicate message\n\0"
                                    as *const u8 as *const core::ffi::c_char,
                            );
                            if ret < 0 as core::ffi::c_int {
                                current_block = 15937355050609159083;
                            } else {
                                current_block = 14136749492126903395;
                            }
                        } else {
                            current_block = 14136749492126903395;
                        }
                    } else {
                        current_block = 14136749492126903395;
                    }
                }
            }
        }
    }
    match current_block {
        14136749492126903395 => return,
        _ => {
            fprintf(
                stderr,
                b"jbig2dec WARNING could not print message\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            (*state).repeats = 0 as core::ffi::c_long;
            free((*state).last_message as *mut core::ffi::c_void);
            (*state).last_message = 0 as *mut core::ffi::c_char;
            return;
        }
    };
}
unsafe extern "C" fn flush_errors(mut state: *mut jbig2dec_error_callback_state_t) {
    if (*state).repeats > 1 as core::ffi::c_long {
        fprintf(
            stderr,
            b"jbig2dec last message repeated %ld times\n\0" as *const u8
                as *const core::ffi::c_char,
            (*state).repeats,
        );
    }
}
unsafe extern "C" fn make_output_filename(
    mut input_filename: *const core::ffi::c_char,
    mut extension: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    let mut output_filename: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut c: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    let mut e: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    let mut extlen: size_t = 0;
    let mut len: size_t = 0;
    if extension.is_null() {
        fprintf(
            stderr,
            b"no filename extension; cannot create output filename!\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        exit(1 as core::ffi::c_int);
    }
    if input_filename.is_null() {
        c = b"out\0" as *const u8 as *const core::ffi::c_char;
    } else {
        c = strrchr(input_filename, '/' as i32);
        if c.is_null() {
            c = strrchr(input_filename, '\\' as i32);
        }
        if !c.is_null() {
            c = c.offset(1);
        } else {
            c = input_filename;
        }
    }
    if *c as core::ffi::c_int == '\0' as i32 {
        c = b"out\0" as *const u8 as *const core::ffi::c_char;
    }
    len = strlen(c);
    e = strrchr(c, '.' as i32);
    if !e.is_null() {
        len = len.wrapping_sub(strlen(e));
    }
    extlen = strlen(extension);
    output_filename = malloc(len.wrapping_add(extlen).wrapping_add(1 as size_t))
        as *mut core::ffi::c_char;
    if output_filename.is_null() {
        fprintf(
            stderr,
            b"failed to allocate memory for output filename\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        exit(1 as core::ffi::c_int);
    }
    memcpy(
        output_filename as *mut core::ffi::c_void,
        c as *const core::ffi::c_void,
        len,
    );
    memcpy(
        output_filename.offset(len as isize) as *mut core::ffi::c_void,
        extension as *const core::ffi::c_void,
        extlen,
    );
    *output_filename.offset(len as isize).offset(extlen as isize) = '\0' as i32
        as core::ffi::c_char;
    return output_filename;
}
unsafe extern "C" fn write_page_image(
    mut params: *mut jbig2dec_params_t,
    mut out: *mut FILE,
    mut image: *mut Jbig2Image,
) -> core::ffi::c_int {
    match (*params).output_format as core::ffi::c_uint {
        2 => return jbig2_image_write_pbm(image, out),
        _ => {
            fprintf(
                stderr,
                b"unsupported output format.\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return 1 as core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn write_document_hash(
    mut params: *mut jbig2dec_params_t,
) -> core::ffi::c_int {
    let mut out: *mut FILE = 0 as *mut FILE;
    if strncmp(
        (*params).output_filename,
        b"-\0" as *const u8 as *const core::ffi::c_char,
        2 as size_t,
    ) == 0
    {
        out = stderr;
    } else {
        out = stdout;
    }
    fprintf(
        out,
        b"Hash of decoded document: \0" as *const u8 as *const core::ffi::c_char,
    );
    hash_print(params, out);
    fprintf(out, b"\n\0" as *const u8 as *const core::ffi::c_char);
    return 0 as core::ffi::c_int;
}
unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut params: jbig2dec_params_t = jbig2dec_params_t {
        mode: usage,
        verbose: 0,
        hash: 0,
        embedded: 0,
        hash_ctx: 0 as *mut SHA1_CTX,
        output_filename: 0 as *mut core::ffi::c_char,
        output_format: jbig2dec_format_none,
        memory_limit: 0,
    };
    let mut error_callback_state: jbig2dec_error_callback_state_t = jbig2dec_error_callback_state_t {
        verbose: 0,
        last_message: 0 as *mut core::ffi::c_char,
        severity: JBIG2_SEVERITY_DEBUG,
        type_0: 0 as *mut core::ffi::c_char,
        repeats: 0,
    };
    let mut allocator_: jbig2dec_allocator_t = {
        let mut init = jbig2dec_allocator_t {
            super_0: {
                let mut init = _Jbig2Allocator {
                    alloc: None,
                    free: None,
                    realloc: None,
                };
                init
            },
            ctx: 0 as *mut Jbig2Ctx,
            memory_limit: 0,
            memory_used: 0,
            memory_peak: 0,
        };
        init
    };
    let mut allocator: *mut jbig2dec_allocator_t = &mut allocator_;
    let mut ctx: *mut Jbig2Ctx = 0 as *mut Jbig2Ctx;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut f_page: *mut FILE = 0 as *mut FILE;
    let mut buf: [uint8_t; 4096] = [0; 4096];
    let mut filearg: core::ffi::c_int = 0;
    let mut result: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut code: core::ffi::c_int = 0;
    params.mode = render;
    params.verbose = 1 as core::ffi::c_int;
    params.hash = 0 as core::ffi::c_int;
    params.output_filename = 0 as *mut core::ffi::c_char;
    params.output_format = jbig2dec_format_none;
    params.embedded = 0 as core::ffi::c_int;
    params.memory_limit = 0 as size_t;
    filearg = parse_options(argc, argv as *mut *mut core::ffi::c_char, &mut params);
    error_callback_state.verbose = params.verbose;
    error_callback_state.severity = JBIG2_SEVERITY_DEBUG;
    error_callback_state.type_0 = 0 as *mut core::ffi::c_char;
    error_callback_state.last_message = 0 as *mut core::ffi::c_char;
    error_callback_state.repeats = 0 as core::ffi::c_long;
    if params.hash != 0 {
        hash_init(&mut params);
    }
    match params.mode as core::ffi::c_uint {
        0 => {
            print_usage();
            exit(0 as core::ffi::c_int);
        }
        1 => {
            fprintf(
                stderr,
                b"Sorry, segment dump not yet implemented\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            current_block = 6040267449472925966;
        }
        2 => {
            if argc - filearg == 1 as core::ffi::c_int {
                let mut fn_0: *mut core::ffi::c_char = *argv.offset(filearg as isize);
                f = fopen(fn_0, b"rb\0" as *const u8 as *const core::ffi::c_char)
                    as *mut FILE;
                if f.is_null() {
                    fprintf(
                        stderr,
                        b"error opening %s\n\0" as *const u8 as *const core::ffi::c_char,
                        fn_0,
                    );
                    current_block = 14406871155163651824;
                } else {
                    current_block = 15897653523371991391;
                }
            } else if argc - filearg == 2 as core::ffi::c_int {
                let mut fn_1: *mut core::ffi::c_char = *argv.offset(filearg as isize);
                let mut fn_page: *mut core::ffi::c_char = *argv
                    .offset((filearg + 1 as core::ffi::c_int) as isize);
                f = fopen(fn_1, b"rb\0" as *const u8 as *const core::ffi::c_char)
                    as *mut FILE;
                if f.is_null() {
                    fprintf(
                        stderr,
                        b"error opening %s\n\0" as *const u8 as *const core::ffi::c_char,
                        fn_1,
                    );
                    current_block = 14406871155163651824;
                } else {
                    f_page = fopen(
                        fn_page,
                        b"rb\0" as *const u8 as *const core::ffi::c_char,
                    ) as *mut FILE;
                    if f_page.is_null() {
                        fclose(f);
                        fprintf(
                            stderr,
                            b"error opening %s\n\0" as *const u8
                                as *const core::ffi::c_char,
                            fn_page,
                        );
                        current_block = 14406871155163651824;
                    } else {
                        current_block = 15897653523371991391;
                    }
                }
            } else {
                result = print_usage();
                current_block = 14406871155163651824;
            }
            match current_block {
                14406871155163651824 => {}
                _ => {
                    if params.memory_limit == 0 as size_t {
                        allocator = 0 as *mut jbig2dec_allocator_t;
                    } else {
                        (*allocator).super_0.alloc = Some(
                            jbig2dec_alloc
                                as unsafe extern "C" fn(
                                    *mut Jbig2Allocator,
                                    size_t,
                                ) -> *mut core::ffi::c_void,
                        )
                            as Option<
                                unsafe extern "C" fn(
                                    *mut Jbig2Allocator,
                                    size_t,
                                ) -> *mut core::ffi::c_void,
                            >;
                        (*allocator).super_0.free = Some(
                            jbig2dec_free
                                as unsafe extern "C" fn(
                                    *mut Jbig2Allocator,
                                    *mut core::ffi::c_void,
                                ) -> (),
                        )
                            as Option<
                                unsafe extern "C" fn(
                                    *mut Jbig2Allocator,
                                    *mut core::ffi::c_void,
                                ) -> (),
                            >;
                        (*allocator).super_0.realloc = Some(
                            jbig2dec_realloc
                                as unsafe extern "C" fn(
                                    *mut Jbig2Allocator,
                                    *mut core::ffi::c_void,
                                    size_t,
                                ) -> *mut core::ffi::c_void,
                        )
                            as Option<
                                unsafe extern "C" fn(
                                    *mut Jbig2Allocator,
                                    *mut core::ffi::c_void,
                                    size_t,
                                ) -> *mut core::ffi::c_void,
                            >;
                        (*allocator).ctx = 0 as *mut Jbig2Ctx;
                        (*allocator).memory_limit = params.memory_limit;
                        (*allocator).memory_used = 0 as size_t;
                        (*allocator).memory_peak = 0 as size_t;
                    }
                    ctx = jbig2_ctx_new_imp(
                        allocator as *mut Jbig2Allocator,
                        (if !f_page.is_null() || params.embedded != 0 {
                            JBIG2_OPTIONS_EMBEDDED as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        }) as Jbig2Options,
                        0 as *mut Jbig2GlobalCtx,
                        Some(
                            error_callback
                                as unsafe extern "C" fn(
                                    *mut core::ffi::c_void,
                                    *const core::ffi::c_char,
                                    Jbig2Severity,
                                    uint32_t,
                                ) -> (),
                        ),
                        &mut error_callback_state as *mut jbig2dec_error_callback_state_t
                            as *mut core::ffi::c_void,
                        JBIG2_VERSION_MAJOR,
                        JBIG2_VERSION_MINOR,
                    );
                    if ctx.is_null() {
                        fclose(f);
                        if !f_page.is_null() {
                            fclose(f_page);
                        }
                        current_block = 14406871155163651824;
                    } else {
                        if !allocator.is_null() {
                            (*allocator).ctx = ctx;
                        }
                        loop {
                            let mut n_bytes: core::ffi::c_int = fread(
                                buf.as_mut_ptr() as *mut core::ffi::c_void,
                                1 as size_t,
                                ::core::mem::size_of::<[uint8_t; 4096]>() as size_t,
                                f,
                            ) as core::ffi::c_int;
                            if n_bytes < 0 as core::ffi::c_int {
                                if !f_page.is_null() {
                                    jbig2_error(
                                        ctx,
                                        JBIG2_SEVERITY_WARNING,
                                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                                        b"unable to read jbig2 global stream\0" as *const u8
                                            as *const core::ffi::c_char,
                                    );
                                } else {
                                    jbig2_error(
                                        ctx,
                                        JBIG2_SEVERITY_WARNING,
                                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                                        b"unable to read jbig2 page stream\0" as *const u8
                                            as *const core::ffi::c_char,
                                    );
                                }
                            }
                            if n_bytes <= 0 as core::ffi::c_int {
                                break;
                            }
                            if !(jbig2_data_in(ctx, buf.as_mut_ptr(), n_bytes as size_t)
                                < 0 as core::ffi::c_int)
                            {
                                continue;
                            }
                            if !f_page.is_null() {
                                jbig2_error(
                                    ctx,
                                    JBIG2_SEVERITY_WARNING,
                                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                                    b"unable to process jbig2 global stream\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            } else {
                                jbig2_error(
                                    ctx,
                                    JBIG2_SEVERITY_WARNING,
                                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                                    b"unable to process jbig2 page stream\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                            break;
                        }
                        fclose(f);
                        if !f_page.is_null() {
                            let mut global_ctx: *mut Jbig2GlobalCtx = jbig2_make_global_ctx(
                                ctx,
                            );
                            ctx = jbig2_ctx_new_imp(
                                allocator as *mut Jbig2Allocator,
                                JBIG2_OPTIONS_EMBEDDED,
                                global_ctx,
                                Some(
                                    error_callback
                                        as unsafe extern "C" fn(
                                            *mut core::ffi::c_void,
                                            *const core::ffi::c_char,
                                            Jbig2Severity,
                                            uint32_t,
                                        ) -> (),
                                ),
                                &mut error_callback_state
                                    as *mut jbig2dec_error_callback_state_t
                                    as *mut core::ffi::c_void,
                                JBIG2_VERSION_MAJOR,
                                JBIG2_VERSION_MINOR,
                            );
                            if !ctx.is_null() {
                                if !allocator.is_null() {
                                    (*allocator).ctx = ctx;
                                }
                                loop {
                                    let mut n_bytes_0: core::ffi::c_int = fread(
                                        buf.as_mut_ptr() as *mut core::ffi::c_void,
                                        1 as size_t,
                                        ::core::mem::size_of::<[uint8_t; 4096]>() as size_t,
                                        f_page,
                                    ) as core::ffi::c_int;
                                    if n_bytes_0 < 0 as core::ffi::c_int {
                                        jbig2_error(
                                            ctx,
                                            JBIG2_SEVERITY_WARNING,
                                            JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                                            b"unable to read jbig2 page stream\0" as *const u8
                                                as *const core::ffi::c_char,
                                        );
                                    }
                                    if n_bytes_0 <= 0 as core::ffi::c_int {
                                        break;
                                    }
                                    if !(jbig2_data_in(
                                        ctx,
                                        buf.as_mut_ptr(),
                                        n_bytes_0 as size_t,
                                    ) < 0 as core::ffi::c_int)
                                    {
                                        continue;
                                    }
                                    jbig2_error(
                                        ctx,
                                        JBIG2_SEVERITY_WARNING,
                                        JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                                        b"unable to process jbig2 page stream\0" as *const u8
                                            as *const core::ffi::c_char,
                                    );
                                    break;
                                }
                            }
                            fclose(f_page);
                            jbig2_global_ctx_free(global_ctx);
                        }
                        let mut image: *mut Jbig2Image = 0 as *mut Jbig2Image;
                        let mut out: *mut FILE = 0 as *mut FILE;
                        code = jbig2_complete_page(ctx);
                        if code < 0 as core::ffi::c_int {
                            jbig2_error(
                                ctx,
                                JBIG2_SEVERITY_WARNING,
                                JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                                b"unable to complete page\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                            current_block = 14406871155163651824;
                        } else {
                            if (params.output_filename).is_null() {
                                match params.output_format as core::ffi::c_uint {
                                    2 => {
                                        params.output_filename = make_output_filename(
                                            *argv.offset(filearg as isize),
                                            b".pbm\0" as *const u8 as *const core::ffi::c_char,
                                        );
                                        current_block = 18383263831861166299;
                                    }
                                    _ => {
                                        fprintf(
                                            stderr,
                                            b"unsupported output format: %d\n\0" as *const u8
                                                as *const core::ffi::c_char,
                                            params.output_format as core::ffi::c_uint,
                                        );
                                        current_block = 14406871155163651824;
                                    }
                                }
                            } else {
                                let mut len: core::ffi::c_int = strlen(
                                    params.output_filename,
                                ) as core::ffi::c_int;
                                if len >= 3 as core::ffi::c_int
                                    && params.output_format as core::ffi::c_uint
                                        == jbig2dec_format_none as core::ffi::c_int
                                            as core::ffi::c_uint
                                {
                                    set_output_format(
                                        &mut params,
                                        (params.output_filename)
                                            .offset(len as isize)
                                            .offset(-(3 as core::ffi::c_int as isize)),
                                    );
                                }
                                current_block = 18383263831861166299;
                            }
                            match current_block {
                                14406871155163651824 => {}
                                _ => {
                                    if strncmp(
                                        params.output_filename,
                                        b"-\0" as *const u8 as *const core::ffi::c_char,
                                        2 as size_t,
                                    ) == 0
                                    {
                                        out = stdout;
                                        current_block = 10261677128829721533;
                                    } else {
                                        if params.verbose > 1 as core::ffi::c_int {
                                            fprintf(
                                                stderr,
                                                b"saving decoded page as '%s'\n\0" as *const u8
                                                    as *const core::ffi::c_char,
                                                params.output_filename,
                                            );
                                        }
                                        out = fopen(
                                            params.output_filename,
                                            b"wb\0" as *const u8 as *const core::ffi::c_char,
                                        ) as *mut FILE;
                                        if out.is_null() {
                                            fprintf(
                                                stderr,
                                                b"unable to open '%s' for writing\n\0" as *const u8
                                                    as *const core::ffi::c_char,
                                                params.output_filename,
                                            );
                                            current_block = 14406871155163651824;
                                        } else {
                                            current_block = 10261677128829721533;
                                        }
                                    }
                                    match current_block {
                                        14406871155163651824 => {}
                                        _ => {
                                            loop {
                                                image = jbig2_page_out(ctx);
                                                if image.is_null() {
                                                    break;
                                                }
                                                write_page_image(&mut params, out, image);
                                                if params.hash != 0 {
                                                    hash_image(&mut params, image);
                                                }
                                                jbig2_release_page(ctx, image);
                                            }
                                            if out != stdout {
                                                fclose(out);
                                            }
                                            if params.hash != 0 {
                                                write_document_hash(&mut params);
                                            }
                                            current_block = 6040267449472925966;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            current_block = 6040267449472925966;
        }
    }
    match current_block {
        6040267449472925966 => {
            if !allocator.is_null() && !((*allocator).ctx).is_null() {
                let mut limit_mb: size_t = ((*allocator).memory_limit)
                    .wrapping_div(MBYTE as size_t);
                let mut peak_mb: size_t = ((*allocator).memory_peak)
                    .wrapping_div(MBYTE as size_t);
                jbig2_error(
                    (*allocator).ctx,
                    JBIG2_SEVERITY_DEBUG,
                    JBIG2_UNKNOWN_SEGMENT_NUMBER as uint32_t,
                    b"memory: limit: %lu Mbyte peak usage: %lu Mbyte\0" as *const u8
                        as *const core::ffi::c_char,
                    limit_mb,
                    peak_mb,
                );
            }
            result = 0 as core::ffi::c_int;
        }
        _ => {}
    }
    flush_errors(&mut error_callback_state);
    jbig2_ctx_free(ctx);
    if !(params.output_filename).is_null() {
        free(params.output_filename as *mut core::ffi::c_void);
    }
    if !(error_callback_state.last_message).is_null() {
        free(error_callback_state.last_message as *mut core::ffi::c_void);
    }
    if params.hash != 0 {
        hash_free(&mut params);
    }
    return result;
}
pub fn main() {
    let mut args: Vec<*mut core::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}

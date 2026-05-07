extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn sscanf(
        __s: *const core::ffi::c_char,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fgets(
        __s: *mut core::ffi::c_char,
        __n: core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut core::ffi::c_char;
    fn perror(__s: *const core::ffi::c_char);
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn exit(__status: core::ffi::c_int) -> !;
    fn bzero(__s: *mut core::ffi::c_void, __n: size_t);
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn pat_insert(n: *mut ptree, head: *mut ptree) -> *mut ptree;
    fn pat_search(key: core::ffi::c_ulong, head: *mut ptree) -> *mut ptree;
}
pub type size_t = usize;
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
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptree_mask {
    pub pm_mask: core::ffi::c_ulong,
    pub pm_data: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptree {
    pub p_key: core::ffi::c_ulong,
    pub p_m: *mut ptree_mask,
    pub p_mlen: core::ffi::c_uchar,
    pub p_b: core::ffi::c_char,
    pub p_left: *mut ptree,
    pub p_right: *mut ptree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MyNode {
    pub foo: core::ffi::c_int,
    pub bar: core::ffi::c_double,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut phead: *mut ptree = 0 as *mut ptree;
    let mut p: *mut ptree = 0 as *mut ptree;
    let mut pfind: *mut ptree = 0 as *mut ptree;
    let mut pm: *mut ptree_mask = 0 as *mut ptree_mask;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: [core::ffi::c_char; 128] = [0; 128];
    let mut addr_str: [core::ffi::c_char; 16] = [0; 16];
    let mut addr: in_addr = in_addr { s_addr: 0 };
    let mut mask: core::ffi::c_ulong = 0xffffffff as core::ffi::c_ulong;
    let mut time: core::ffi::c_float = 0.;
    if argc < 2 as core::ffi::c_int {
        printf(
            b"Usage: %s <TCP stream>\n\0" as *const u8 as *const core::ffi::c_char,
            *argv.offset(0 as core::ffi::c_int as isize),
        );
        exit(-(1 as core::ffi::c_int));
    }
    fp = fopen(
        *argv.offset(1 as core::ffi::c_int as isize),
        b"r\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        printf(
            b"File %s doesn't seem to exist\n\0" as *const u8
                as *const core::ffi::c_char,
            *argv.offset(1 as core::ffi::c_int as isize),
        );
        exit(1 as core::ffi::c_int);
    }
    phead = malloc(::core::mem::size_of::<ptree>() as size_t) as *mut ptree;
    if phead.is_null() {
        perror(b"Allocating p-trie node\0" as *const u8 as *const core::ffi::c_char);
        exit(1 as core::ffi::c_int);
    }
    bzero(phead as *mut core::ffi::c_void, ::core::mem::size_of::<ptree>() as size_t);
    (*phead).p_m = malloc(::core::mem::size_of::<ptree_mask>() as size_t)
        as *mut ptree_mask;
    if ((*phead).p_m).is_null() {
        perror(
            b"Allocating p-trie mask data\0" as *const u8 as *const core::ffi::c_char,
        );
        exit(1 as core::ffi::c_int);
    }
    bzero(
        (*phead).p_m as *mut core::ffi::c_void,
        ::core::mem::size_of::<ptree_mask>() as size_t,
    );
    pm = (*phead).p_m;
    (*pm).pm_data = malloc(::core::mem::size_of::<MyNode>() as size_t) as *mut MyNode
        as *mut core::ffi::c_void;
    if ((*pm).pm_data).is_null() {
        perror(
            b"Allocating p-trie mask's node data\0" as *const u8
                as *const core::ffi::c_char,
        );
        exit(1 as core::ffi::c_int);
    }
    bzero((*pm).pm_data, ::core::mem::size_of::<()>() as size_t);
    (*phead).p_mlen = 1 as core::ffi::c_uchar;
    (*phead).p_right = phead;
    (*phead).p_left = (*phead).p_right;
    while !(fgets(line.as_mut_ptr(), 128 as core::ffi::c_int, fp)).is_null() {
        sscanf(
            line.as_mut_ptr(),
            b"%f %d\0" as *const u8 as *const core::ffi::c_char,
            &mut time as *mut core::ffi::c_float,
            &mut addr as *mut in_addr as *mut core::ffi::c_uint,
        );
        p = malloc(::core::mem::size_of::<ptree>() as size_t) as *mut ptree;
        if p.is_null() {
            perror(b"Allocating p-trie node\0" as *const u8 as *const core::ffi::c_char);
            exit(1 as core::ffi::c_int);
        }
        bzero(p as *mut core::ffi::c_void, ::core::mem::size_of::<ptree>() as size_t);
        (*p).p_m = malloc(::core::mem::size_of::<ptree_mask>() as size_t)
            as *mut ptree_mask;
        if ((*p).p_m).is_null() {
            perror(
                b"Allocating p-trie mask data\0" as *const u8 as *const core::ffi::c_char,
            );
            exit(1 as core::ffi::c_int);
        }
        bzero(
            (*p).p_m as *mut core::ffi::c_void,
            ::core::mem::size_of::<ptree_mask>() as size_t,
        );
        pm = (*p).p_m;
        (*pm).pm_data = malloc(::core::mem::size_of::<MyNode>() as size_t) as *mut MyNode
            as *mut core::ffi::c_void;
        if ((*pm).pm_data).is_null() {
            perror(
                b"Allocating p-trie mask's node data\0" as *const u8
                    as *const core::ffi::c_char,
            );
            exit(1 as core::ffi::c_int);
        }
        bzero((*pm).pm_data, ::core::mem::size_of::<()>() as size_t);
        (*p).p_key = addr.s_addr as core::ffi::c_ulong;
        (*(*p).p_m).pm_mask = htonl(mask as uint32_t) as core::ffi::c_ulong;
        pfind = pat_search(addr.s_addr as core::ffi::c_ulong, phead);
        if (*pfind).p_key == addr.s_addr as core::ffi::c_ulong {
            printf(
                b"%f %08x: \0" as *const u8 as *const core::ffi::c_char,
                time as core::ffi::c_double,
                addr.s_addr,
            );
            printf(b"Found.\n\0" as *const u8 as *const core::ffi::c_char);
        } else {
            p = pat_insert(p, phead);
        }
        if p.is_null() {
            fprintf(
                stderr,
                b"Failed on pat_insert\n\0" as *const u8 as *const core::ffi::c_char,
            );
            exit(1 as core::ffi::c_int);
        }
    }
    exit(0 as core::ffi::c_int);
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

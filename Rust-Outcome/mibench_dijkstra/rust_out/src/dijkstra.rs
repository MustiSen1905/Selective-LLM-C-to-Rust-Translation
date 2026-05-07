#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Safefrac {
    pub numerator: i8,
    pub denominator: i8,
}
impl _IO_FILE {}
impl _NODE {}
impl _QITEM {}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> core::ffi::c_int;
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
    fn fscanf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    fn qcount() -> core::ffi::c_int;
}
pub type size_t = usize;
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
pub struct _NODE {
    pub iDist: core::ffi::c_int,
    pub iPrev: core::ffi::c_int,
}
pub type NODE = _NODE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QITEM {
    pub iNode: core::ffi::c_int,
    pub iDist: core::ffi::c_int,
    pub iPrev: core::ffi::c_int,
    pub qNext: *mut _QITEM,
}
pub type QITEM = _QITEM;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const NONE: core::ffi::c_int = 9999 as core::ffi::c_int;
#[no_mangle]
pub static mut qHead: *mut QITEM = 0 as *const QITEM as *mut QITEM;
#[no_mangle]
pub static mut num_nodes: core::ffi::c_int = 0;
#[no_mangle]
pub static mut AdjMatrix: *mut *mut core::ffi::c_int = 0 as *const *mut core::ffi::c_int
    as *mut *mut core::ffi::c_int;
#[no_mangle]
pub static mut g_qCount: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub static mut rgnNodes: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut ch: core::ffi::c_int = 0;
#[no_mangle]
pub static mut iPrev: core::ffi::c_int = 0;
#[no_mangle]
pub static mut iNode: core::ffi::c_int = 0;
#[no_mangle]
pub static mut i: core::ffi::c_int = 0;
#[no_mangle]
pub static mut iCost: core::ffi::c_int = 0;
#[no_mangle]
pub static mut iDist: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn print_path(
    mut rgnNodes_0: *mut NODE,
    mut chNode: core::ffi::c_int,
) {
    if (*rgnNodes_0.offset(chNode as isize)).iPrev != NONE {
        print_path(rgnNodes_0, (*rgnNodes_0.offset(chNode as isize)).iPrev);
    }
    printf(b" %d\0" as *const u8 as *const core::ffi::c_char, chNode);
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn enqueue(
    mut iNode_0: core::ffi::c_int,
    mut iDist_0: core::ffi::c_int,
    mut iPrev_0: core::ffi::c_int,
) {
    let mut qNew: *mut QITEM = std::mem::MaybeUninit::<QITEM>::uninit().as_mut_ptr();
    qNew = malloc(std::mem::size_of::<QITEM>() as size_t) as *mut QITEM;
    
    let mut qLast: *mut QITEM = qHead;
    if qNew.is_null() {
        fprintf(stderr, b"Out of memory.\n\0" as *const u8 as *const core::ffi::c_char);
        exit(1 as core::ffi::c_int);
     }
     
    (*qNew).iNode = iNode_0;
    (*qNew).iDist = iDist_0;
    (*qNew).iPrev = iPrev_0;
    (*qNew).qNext = 0 as *mut _QITEM;
    
    if qLast.is_null() {
        qHead = qNew;
    } else {
        while !((*qLast).qNext).is_null() {
            qLast = (*qLast).qNext as *mut QITEM;
         }
        (*qLast).qNext = qNew as *mut _QITEM;
     }
    g_qCount += 1;
}
#[no_mangle]
pub unsafe extern "C" fn dequeue(
    mut piNode: *mut core::ffi::c_int,
    mut piDist: *mut core::ffi::c_int,
    mut piPrev: *mut core::ffi::c_int,
) {
    if !qHead.is_null()  {
        let safe_qhead = qHead;
        
        // 100% Safe Rust logic here: use safe wrapper and no `unsafe` keyword
        *piNode = (*safe_qhead).iNode;
        *piDist = (*safe_qhead).iDist;
        *piPrev = (*safe_qhead).iPrev;
        
        qHead = (*safe_qhead).qNext as  *mut QITEM;
        g_qCount -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dijkstra(
    mut chStart: core::ffi::c_int,
    mut chEnd: core::ffi::c_int,
) -> core::ffi::c_int {
    ch = 0 as core::ffi::c_int;
    while ch < num_nodes {
        (*rgnNodes.offset(ch as isize)).iDist = NONE;
        (*rgnNodes.offset(ch as isize)).iPrev = NONE;
        ch += 1;
    }
    if chStart == chEnd {
        printf(
            b"Shortest path is 0 in cost. Just stay where you are.\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {
        (*rgnNodes.offset(chStart as isize)).iDist = 0 as core::ffi::c_int;
        (*rgnNodes.offset(chStart as isize)).iPrev = NONE;
        enqueue(chStart, 0 as core::ffi::c_int, NONE);
        while qcount() > 0 as core::ffi::c_int {
            dequeue(&mut iNode, &mut iDist, &mut iPrev);
            i = 0 as core::ffi::c_int;
            while i < num_nodes {
                iCost = *(*AdjMatrix.offset(iNode as isize)).offset(i as isize);
                if iCost != NONE {
                    if NONE == (*rgnNodes.offset(i as isize)).iDist
                        || (*rgnNodes.offset(i as isize)).iDist > iCost + iDist
                    {
                        (*rgnNodes.offset(i as isize)).iDist = iDist + iCost;
                        (*rgnNodes.offset(i as isize)).iPrev = iNode;
                        enqueue(i, iDist + iCost, iNode);
                    }
                }
                i += 1;
            }
        }
        printf(
            b"Shortest path is %d in cost. \0" as *const u8 as *const core::ffi::c_char,
            (*rgnNodes.offset(chEnd as isize)).iDist,
        );
        printf(b"Path is: \0" as *const u8 as *const core::ffi::c_char);
        print_path(rgnNodes, chEnd);
        printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
    }
    panic!("Reached end of non-void function without returning");
}
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut i_0: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if argc < 3 as core::ffi::c_int {
        fprintf(
            stderr,
            b"Usage: dijkstra <NUM_NODES> <INPUT_FILE>\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    num_nodes = atoi(*argv.offset(1 as core::ffi::c_int as isize));
    rgnNodes = malloc(
        (num_nodes as size_t).wrapping_mul(::core::mem::size_of::<NODE>() as size_t),
    ) as *mut NODE;
    AdjMatrix = malloc(
        (num_nodes as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut core::ffi::c_int>() as size_t),
    ) as *mut *mut core::ffi::c_int;
    i_0 = 0 as core::ffi::c_int;
    while i_0 < num_nodes {
        let ref mut fresh0 = *AdjMatrix.offset(i_0 as isize);
        *fresh0 = malloc(
            (num_nodes as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        ) as *mut core::ffi::c_int;
        i_0 += 1;
    }
    fp = fopen(
        *argv.offset(2 as core::ffi::c_int as isize),
        b"r\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    i_0 = 0 as core::ffi::c_int;
    while i_0 < num_nodes {
        j = 0 as core::ffi::c_int;
        while j < num_nodes {
            fscanf(
                fp,
                b"%d\0" as *const u8 as *const core::ffi::c_char,
                &mut k as *mut core::ffi::c_int,
            );
            *(*AdjMatrix.offset(i_0 as isize)).offset(j as isize) = k;
            j += 1;
        }
        i_0 += 1;
    }
    i_0 = 0 as core::ffi::c_int;
    j = num_nodes / 2 as core::ffi::c_int;
    while i_0 < 20 as core::ffi::c_int {
        j = j % num_nodes;
        dijkstra(i_0, j);
        i_0 += 1;
        j += 1;
    }
    free(rgnNodes as *mut core::ffi::c_void);
    i_0 = 0 as core::ffi::c_int;
    while i_0 < num_nodes {
        free(*AdjMatrix.offset(i_0 as isize) as *mut core::ffi::c_void);
        i_0 += 1;
    }
    free(AdjMatrix as *mut core::ffi::c_void);
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
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}
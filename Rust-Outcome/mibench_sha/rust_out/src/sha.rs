// --- auto-generated Safe stub types ---
pub type Safe_IO_codecvt = ();
pub type Safe_IO_marker = ();
pub type Safe_IO_wide_data = ();
// --- end stubs ---

#[derive(Debug, Clone)]
pub struct Safe_IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: Option<String>,
    pub _IO_read_end: Option<String>,
    pub _IO_read_base: Option<String>,
    pub _IO_write_base: Option<String>,
    pub _IO_write_ptr: Option<String>,
    pub _IO_write_end: Option<String>,
    pub _IO_buf_base: Option<String>,
    pub _IO_buf_end: Option<String>,
    pub _IO_save_base: Option<String>,
    pub _IO_backup_base: Option<String>,
    pub _IO_save_end: Option<String>,
    pub _markers: *mut  Safe_IO_marker,
    pub _chain: *mut  Safe_IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut (),
    pub _offset: off64_t,
    pub _codecvt: *mut Safe_IO_codecvt,
    pub _wide_data: *mut Safe_IO_wide_data,
    pub _freeres_list: *mut Safe_IO_FILE,
    pub _freeres_buf: *mut (),
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
impl Safe_IO_FILE {
    unsafe fn from_ptr(ptr: *const _IO_FILE) -> Self {
        if ptr.is_null() {
            return Safe_IO_FILE {
                _flags: unimplemented!("from_ptr stub"),
                _IO_read_ptr: None,
                _IO_read_end: None,

                _unused2: [unimplemented!("from_ptr stub"); 20],
            ..Default::default()
        };
        }
        let orig = &*ptr;
        Safe_IO_FILE {
            _flags: orig._flags,
            _IO_read_ptr: std::ffi::CStr::from_ptr(orig._IO_read_ptr as *const i8).to_string_lossy().into_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SafeSHA_INFO {
    pub digest: [i32; 5],
    pub count_lo: i32,
    pub count_hi: i32,
    pub data: [i32; 16],
}
impl SafeSHA_INFO {

}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
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
pub type BYTE = core::ffi::c_uchar;
pub type LONG = core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA_INFO {
    pub digest: [LONG; 5],
    pub count_lo: LONG,
    pub count_hi: LONG,
    pub data: [LONG; 16],
}
pub const SHA_BLOCKSIZE: core::ffi::c_int = 64 as core::ffi::c_int;
pub const CONST1: LONG = 0x5a827999 as LONG;
pub const CONST2: LONG = 0x6ed9eba1 as LONG;
pub const CONST3: LONG = 0x8f1bbcdc as LONG;
pub const CONST4: LONG = 0xca62c1d6 as LONG;
#[no_mangle]
pub unsafe extern "C" fn sha_transform(mut sha_info: *mut SHA_INFO) {
    let mut i: core::ffi::c_int = 0;
    let mut temp: LONG = 0;
    let mut A: LONG = 0;
    let mut B: LONG = 0;
    let mut C: LONG = 0;
    let mut D: LONG = 0;
    let mut E: LONG = 0;
    let mut W: [LONG; 80] = [0; 80];
    i = 0 as core::ffi::c_int;
    while i < 16 as core::ffi::c_int {
        W[i as usize] = (*sha_info).data[i as usize];
        i += 1;
    }
    i = 16 as core::ffi::c_int;
    while i < 80 as core::ffi::c_int {
        W[i as usize] = W[(i - 3 as core::ffi::c_int) as usize]
            ^ W[(i - 8 as core::ffi::c_int) as usize]
            ^ W[(i - 14 as core::ffi::c_int) as usize]
            ^ W[(i - 16 as core::ffi::c_int) as usize];
        i += 1;
    }
    A = (*sha_info).digest[0 as core::ffi::c_int as usize];
    B = (*sha_info).digest[1 as core::ffi::c_int as usize];
    C = (*sha_info).digest[2 as core::ffi::c_int as usize];
    D = (*sha_info).digest[3 as core::ffi::c_int as usize];
    E = (*sha_info).digest[4 as core::ffi::c_int as usize];
    i = 0 as core::ffi::c_int;
    while i < 20 as core::ffi::c_int {
        temp = (A << 5 as core::ffi::c_int
            | A >> 32 as core::ffi::c_int - 5 as core::ffi::c_int)
            .wrapping_add(B & C | !B & D)
            .wrapping_add(E as usize)
            .wrapping_add(W[i as usize])
            .wrapping_add(CONST1 as LONG);
        E = D;
        D = C;
        C = B << 30 as core::ffi::c_int
            | B >> 32 as core::ffi::c_int - 30 as core::ffi::c_int;
        B = A;
        A = temp;
        i += 1;
    }
    i = 20 as core::ffi::c_int;
    while i < 40 as core::ffi::c_int {
        temp = (A << 5 as core::ffi::c_int
            | A >> 32 as core::ffi::c_int - 5 as core::ffi::c_int)
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(E as usize)
            .wrapping_add(W[i as usize])
            .wrapping_add(CONST2 as LONG);
        E = D;
        D = C;
        C = B << 30 as core::ffi::c_int
            | B >> 32 as core::ffi::c_int - 30 as core::ffi::c_int;
        B = A;
        A = temp;
        i += 1;
    }
    i = 40 as core::ffi::c_int;
    while i < 60 as core::ffi::c_int {
        temp = (A << 5 as core::ffi::c_int
            | A >> 32 as core::ffi::c_int - 5 as core::ffi::c_int)
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(E as usize)
            .wrapping_add(W[i as usize])
            .wrapping_add(CONST3 as LONG);
        E = D;
        D = C;
        C = B << 30 as core::ffi::c_int
            | B >> 32 as core::ffi::c_int - 30 as core::ffi::c_int;
        B = A;
        A = temp;
        i += 1;
    }
    i = 60 as core::ffi::c_int;
    while i < 80 as core::ffi::c_int {
        temp = (A << 5 as core::ffi::c_int
            | A >> 32 as core::ffi::c_int - 5 as core::ffi::c_int)
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(E as usize)
            .wrapping_add(W[i as usize])
            .wrapping_add(CONST4 as LONG);
        E = D;
        D = C;
        C = B << 30 as core::ffi::c_int
            | B >> 32 as core::ffi::c_int - 30 as core::ffi::c_int;
        B = A;
        A = temp;
        i += 1;
    }
    (*sha_info).digest[0 as core::ffi::c_int as usize] = ((*sha_info)
        .digest[0 as core::ffi::c_int as usize])
        .wrapping_add(A as usize);
    (*sha_info).digest[1 as core::ffi::c_int as usize] = ((*sha_info)
        .digest[1 as core::ffi::c_int as usize])
        .wrapping_add(B as usize);
    (*sha_info).digest[2 as core::ffi::c_int as usize] = ((*sha_info)
        .digest[2 as core::ffi::c_int as usize])
        .wrapping_add(C as usize);
    (*sha_info).digest[3 as core::ffi::c_int as usize] = ((*sha_info)
        .digest[3 as core::ffi::c_int as usize])
        .wrapping_add(D as usize);
    (*sha_info).digest[4 as core::ffi::c_int as usize] = ((*sha_info)
        .digest[4 as core::ffi::c_int as usize])
        .wrapping_add(E as usize);
}
#[no_mangle]
pub unsafe extern "C" fn byte_reverse(
    mut buffer: *mut LONG,
    mut count: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut ct: [BYTE; 4] = [0; 4];
    let mut cp: *mut BYTE = 0 as *mut BYTE;
    count = (count as core::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<LONG>() as usize as core::ffi::c_ulong)
        as core::ffi::c_int as core::ffi::c_int;
    cp = buffer as *mut BYTE;
    i = 0 as core::ffi::c_int;
    while i < count {
        ct[0 as core::ffi::c_int as usize] = *cp.offset(0 as core::ffi::c_int as isize);
        ct[1 as core::ffi::c_int as usize] = *cp.offset(1 as core::ffi::c_int as isize);
        ct[2 as core::ffi::c_int as usize] = *cp.offset(2 as core::ffi::c_int as isize);
        ct[3 as core::ffi::c_int as usize] = *cp.offset(3 as core::ffi::c_int as isize);
        *cp.offset(0 as core::ffi::c_int as isize) = ct[3 as core::ffi::c_int as usize];
        *cp.offset(1 as core::ffi::c_int as isize) = ct[2 as core::ffi::c_int as usize];
        *cp.offset(2 as core::ffi::c_int as isize) = ct[1 as core::ffi::c_int as usize];
        *cp.offset(3 as core::ffi::c_int as isize) = ct[0 as core::ffi::c_int as usize];
        cp = cp.offset(::core::mem::size_of::<LONG>() as usize as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sha_init(mut sha_info_ptr: *mut SHA_INFO) {
    let mut sha_info = SafeSHA_INFO::from_ptr(sha_info_ptr);
    
    sha_info.digest[0] = 0x67452301;
    sha_info.digest[1] = 0xefcdab89;
    sha_info.digest[2] = 0x98badcfe;
    sha_info.digest[3] = 0x10325476;
    sha_info.digest[4] = 0xc3d2e1f0;
    
    sha_info.count_lo = 0;
    sha_info.count_hi = 0;
}
#[no_mangle]
pub unsafe extern "C" fn sha_update(
    mut sha_info: *mut SHA_INFO,
    mut buffer: *mut BYTE,
    mut count: core::ffi::c_int,
) {
    if ((*sha_info).count_lo).wrapping_add((count as LONG) << 3 as core::ffi::c_int)
        < (*sha_info).count_lo
    {
        (*sha_info).count_hi = ((*sha_info).count_hi).wrapping_add(1);
    }
    (*sha_info).count_lo = ((*sha_info).count_lo)
        .wrapping_add((count as LONG) << 3 as core::ffi::c_int);
    (*sha_info).count_hi = ((*sha_info).count_hi)
        .wrapping_add(count as LONG >> 29 as core::ffi::c_int);
    while count >= SHA_BLOCKSIZE {
        memcpy(
            ((*sha_info).data).as_mut_ptr() as *mut core::ffi::c_void,
            buffer as *const core::ffi::c_void,
            SHA_BLOCKSIZE as size_t,
        );
        byte_reverse(((*sha_info).data).as_mut_ptr(), SHA_BLOCKSIZE);
        sha_transform(sha_info);
        buffer = buffer.offset(SHA_BLOCKSIZE as isize);
        count -= SHA_BLOCKSIZE;
    }
    memcpy(
        ((*sha_info).data).as_mut_ptr() as *mut core::ffi::c_void,
        buffer as *const core::ffi::c_void,
        count as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sha_final(mut sha_info: *mut SHA_INFO) {
    let mut count: core::ffi::c_int = 0;
    let mut lo_bit_count: LONG = 0;
    let mut hi_bit_count: LONG = 0;
    lo_bit_count = (*sha_info).count_lo;
    hi_bit_count = (*sha_info).count_hi;
    count = (lo_bit_count >> 3 as core::ffi::c_int & 0x3f as LONG) as core::ffi::c_int;
    let fresh0 = count;
    count = count + 1;
    *(((*sha_info).data).as_mut_ptr() as *mut BYTE).offset(fresh0 as isize) = 0x80
        as BYTE;
    if count > 56 as core::ffi::c_int {
        memset(
            (&mut (*sha_info).data as *mut [LONG; 16] as *mut BYTE)
                .offset(count as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (64 as core::ffi::c_int - count) as size_t,
        );
        byte_reverse(((*sha_info).data).as_mut_ptr(), SHA_BLOCKSIZE);
        sha_transform(sha_info);
        memset(
            &mut (*sha_info).data as *mut [LONG; 16] as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            56 as size_t,
        );
    } else {
        memset(
            (&mut (*sha_info).data as *mut [LONG; 16] as *mut BYTE)
                .offset(count as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (56 as core::ffi::c_int - count) as size_t,
        );
    }
    byte_reverse(((*sha_info).data).as_mut_ptr(), SHA_BLOCKSIZE);
    (*sha_info).data[14 as core::ffi::c_int as usize] = hi_bit_count;
    (*sha_info).data[15 as core::ffi::c_int as usize] = lo_bit_count;
    sha_transform(sha_info);
}
pub const BLOCK_SIZE: core::ffi::c_int = 8192 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sha_stream(mut sha_info: *mut SHA_INFO, mut fin: *mut FILE) {
    let mut i: core::ffi::c_int = 0;
    let mut data: [BYTE; 8192] = [0; 8192];
    sha_init(sha_info);
    loop {
        i = fread(
            data.as_mut_ptr() as *mut core::ffi::c_void,
            1 as size_t,
            BLOCK_SIZE as size_t,
            fin,
        ) as core::ffi::c_int;
        if !(i > 0 as core::ffi::c_int) {
            break;
        }
        sha_update(sha_info, data.as_mut_ptr(), i);
    }
    sha_final(sha_info);
}
#[no_mangle]
pub extern "C" fn sha_print(sha_info: *mut SHA_INFO) {
    let safe_sha = unsafe { SafeSHA_INFO::from_ptr(sha_info) };
    println!("{:08x} {:08x} {:08x} {:08x} {:08x}", 
        safe_sha.digest[0], safe_sha.digest[1], safe_sha.digest[2], safe_sha.digest[3], safe_sha.digest[4]
    );
}

// --- auto-generated manual Default impls (raw-pointer structs) ---
impl Default for Safe_IO_FILE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

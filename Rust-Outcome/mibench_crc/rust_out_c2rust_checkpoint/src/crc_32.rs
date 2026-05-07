extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> core::ffi::c_int;
    fn perror(__s: *const core::ffi::c_char);
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
pub type Boolean_T = core::ffi::c_int;
pub const True_: Boolean_T = 1;
pub const False_: Boolean_T = 0;
pub const Success_: Boolean_T = 0;
pub const Error_: Boolean_T = -1;
pub type BYTE = core::ffi::c_uchar;
pub type DWORD = core::ffi::c_ulong;
pub type UNS_32_BITS = DWORD;
pub const EOF: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut crc_32_tab: [UNS_32_BITS; 256] = [
    0 as core::ffi::c_int as UNS_32_BITS,
    0x77073096 as core::ffi::c_int as UNS_32_BITS,
    0xee0e612c as core::ffi::c_uint as UNS_32_BITS,
    0x990951ba as core::ffi::c_uint as UNS_32_BITS,
    0x76dc419 as core::ffi::c_int as UNS_32_BITS,
    0x706af48f as core::ffi::c_int as UNS_32_BITS,
    0xe963a535 as core::ffi::c_uint as UNS_32_BITS,
    0x9e6495a3 as core::ffi::c_uint as UNS_32_BITS,
    0xedb8832 as core::ffi::c_int as UNS_32_BITS,
    0x79dcb8a4 as core::ffi::c_int as UNS_32_BITS,
    0xe0d5e91e as core::ffi::c_uint as UNS_32_BITS,
    0x97d2d988 as core::ffi::c_uint as UNS_32_BITS,
    0x9b64c2b as core::ffi::c_int as UNS_32_BITS,
    0x7eb17cbd as core::ffi::c_int as UNS_32_BITS,
    0xe7b82d07 as core::ffi::c_uint as UNS_32_BITS,
    0x90bf1d91 as core::ffi::c_uint as UNS_32_BITS,
    0x1db71064 as core::ffi::c_int as UNS_32_BITS,
    0x6ab020f2 as core::ffi::c_int as UNS_32_BITS,
    0xf3b97148 as core::ffi::c_uint as UNS_32_BITS,
    0x84be41de as core::ffi::c_uint as UNS_32_BITS,
    0x1adad47d as core::ffi::c_int as UNS_32_BITS,
    0x6ddde4eb as core::ffi::c_int as UNS_32_BITS,
    0xf4d4b551 as core::ffi::c_uint as UNS_32_BITS,
    0x83d385c7 as core::ffi::c_uint as UNS_32_BITS,
    0x136c9856 as core::ffi::c_int as UNS_32_BITS,
    0x646ba8c0 as core::ffi::c_int as UNS_32_BITS,
    0xfd62f97a as core::ffi::c_uint as UNS_32_BITS,
    0x8a65c9ec as core::ffi::c_uint as UNS_32_BITS,
    0x14015c4f as core::ffi::c_int as UNS_32_BITS,
    0x63066cd9 as core::ffi::c_int as UNS_32_BITS,
    0xfa0f3d63 as core::ffi::c_uint as UNS_32_BITS,
    0x8d080df5 as core::ffi::c_uint as UNS_32_BITS,
    0x3b6e20c8 as core::ffi::c_int as UNS_32_BITS,
    0x4c69105e as core::ffi::c_int as UNS_32_BITS,
    0xd56041e4 as core::ffi::c_uint as UNS_32_BITS,
    0xa2677172 as core::ffi::c_uint as UNS_32_BITS,
    0x3c03e4d1 as core::ffi::c_int as UNS_32_BITS,
    0x4b04d447 as core::ffi::c_int as UNS_32_BITS,
    0xd20d85fd as core::ffi::c_uint as UNS_32_BITS,
    0xa50ab56b as core::ffi::c_uint as UNS_32_BITS,
    0x35b5a8fa as core::ffi::c_int as UNS_32_BITS,
    0x42b2986c as core::ffi::c_int as UNS_32_BITS,
    0xdbbbc9d6 as core::ffi::c_uint as UNS_32_BITS,
    0xacbcf940 as core::ffi::c_uint as UNS_32_BITS,
    0x32d86ce3 as core::ffi::c_int as UNS_32_BITS,
    0x45df5c75 as core::ffi::c_int as UNS_32_BITS,
    0xdcd60dcf as core::ffi::c_uint as UNS_32_BITS,
    0xabd13d59 as core::ffi::c_uint as UNS_32_BITS,
    0x26d930ac as core::ffi::c_int as UNS_32_BITS,
    0x51de003a as core::ffi::c_int as UNS_32_BITS,
    0xc8d75180 as core::ffi::c_uint as UNS_32_BITS,
    0xbfd06116 as core::ffi::c_uint as UNS_32_BITS,
    0x21b4f4b5 as core::ffi::c_int as UNS_32_BITS,
    0x56b3c423 as core::ffi::c_int as UNS_32_BITS,
    0xcfba9599 as core::ffi::c_uint as UNS_32_BITS,
    0xb8bda50f as core::ffi::c_uint as UNS_32_BITS,
    0x2802b89e as core::ffi::c_int as UNS_32_BITS,
    0x5f058808 as core::ffi::c_int as UNS_32_BITS,
    0xc60cd9b2 as core::ffi::c_uint as UNS_32_BITS,
    0xb10be924 as core::ffi::c_uint as UNS_32_BITS,
    0x2f6f7c87 as core::ffi::c_int as UNS_32_BITS,
    0x58684c11 as core::ffi::c_int as UNS_32_BITS,
    0xc1611dab as core::ffi::c_uint as UNS_32_BITS,
    0xb6662d3d as core::ffi::c_uint as UNS_32_BITS,
    0x76dc4190 as core::ffi::c_int as UNS_32_BITS,
    0x1db7106 as core::ffi::c_int as UNS_32_BITS,
    0x98d220bc as core::ffi::c_uint as UNS_32_BITS,
    0xefd5102a as core::ffi::c_uint as UNS_32_BITS,
    0x71b18589 as core::ffi::c_int as UNS_32_BITS,
    0x6b6b51f as core::ffi::c_int as UNS_32_BITS,
    0x9fbfe4a5 as core::ffi::c_uint as UNS_32_BITS,
    0xe8b8d433 as core::ffi::c_uint as UNS_32_BITS,
    0x7807c9a2 as core::ffi::c_int as UNS_32_BITS,
    0xf00f934 as core::ffi::c_int as UNS_32_BITS,
    0x9609a88e as core::ffi::c_uint as UNS_32_BITS,
    0xe10e9818 as core::ffi::c_uint as UNS_32_BITS,
    0x7f6a0dbb as core::ffi::c_int as UNS_32_BITS,
    0x86d3d2d as core::ffi::c_int as UNS_32_BITS,
    0x91646c97 as core::ffi::c_uint as UNS_32_BITS,
    0xe6635c01 as core::ffi::c_uint as UNS_32_BITS,
    0x6b6b51f4 as core::ffi::c_int as UNS_32_BITS,
    0x1c6c6162 as core::ffi::c_int as UNS_32_BITS,
    0x856530d8 as core::ffi::c_uint as UNS_32_BITS,
    0xf262004e as core::ffi::c_uint as UNS_32_BITS,
    0x6c0695ed as core::ffi::c_int as UNS_32_BITS,
    0x1b01a57b as core::ffi::c_int as UNS_32_BITS,
    0x8208f4c1 as core::ffi::c_uint as UNS_32_BITS,
    0xf50fc457 as core::ffi::c_uint as UNS_32_BITS,
    0x65b0d9c6 as core::ffi::c_int as UNS_32_BITS,
    0x12b7e950 as core::ffi::c_int as UNS_32_BITS,
    0x8bbeb8ea as core::ffi::c_uint as UNS_32_BITS,
    0xfcb9887c as core::ffi::c_uint as UNS_32_BITS,
    0x62dd1ddf as core::ffi::c_int as UNS_32_BITS,
    0x15da2d49 as core::ffi::c_int as UNS_32_BITS,
    0x8cd37cf3 as core::ffi::c_uint as UNS_32_BITS,
    0xfbd44c65 as core::ffi::c_uint as UNS_32_BITS,
    0x4db26158 as core::ffi::c_int as UNS_32_BITS,
    0x3ab551ce as core::ffi::c_int as UNS_32_BITS,
    0xa3bc0074 as core::ffi::c_uint as UNS_32_BITS,
    0xd4bb30e2 as core::ffi::c_uint as UNS_32_BITS,
    0x4adfa541 as core::ffi::c_int as UNS_32_BITS,
    0x3dd895d7 as core::ffi::c_int as UNS_32_BITS,
    0xa4d1c46d as core::ffi::c_uint as UNS_32_BITS,
    0xd3d6f4fb as core::ffi::c_uint as UNS_32_BITS,
    0x4369e96a as core::ffi::c_int as UNS_32_BITS,
    0x346ed9fc as core::ffi::c_int as UNS_32_BITS,
    0xad678846 as core::ffi::c_uint as UNS_32_BITS,
    0xda60b8d0 as core::ffi::c_uint as UNS_32_BITS,
    0x44042d73 as core::ffi::c_int as UNS_32_BITS,
    0x33031de5 as core::ffi::c_int as UNS_32_BITS,
    0xaa0a4c5f as core::ffi::c_uint as UNS_32_BITS,
    0xdd0d7cc9 as core::ffi::c_uint as UNS_32_BITS,
    0x5005713c as core::ffi::c_int as UNS_32_BITS,
    0x270241aa as core::ffi::c_int as UNS_32_BITS,
    0xbe0b1010 as core::ffi::c_uint as UNS_32_BITS,
    0xc90c2086 as core::ffi::c_uint as UNS_32_BITS,
    0x5768b525 as core::ffi::c_int as UNS_32_BITS,
    0x206f85b3 as core::ffi::c_int as UNS_32_BITS,
    0xb966d409 as core::ffi::c_uint as UNS_32_BITS,
    0xce61e49f as core::ffi::c_uint as UNS_32_BITS,
    0x5edef90e as core::ffi::c_int as UNS_32_BITS,
    0x29d9c998 as core::ffi::c_int as UNS_32_BITS,
    0xb0d09822 as core::ffi::c_uint as UNS_32_BITS,
    0xc7d7a8b4 as core::ffi::c_uint as UNS_32_BITS,
    0x59b33d17 as core::ffi::c_int as UNS_32_BITS,
    0x2eb40d81 as core::ffi::c_int as UNS_32_BITS,
    0xb7bd5c3b as core::ffi::c_uint as UNS_32_BITS,
    0xc0ba6cad as core::ffi::c_uint as UNS_32_BITS,
    0xedb88320 as core::ffi::c_uint as UNS_32_BITS,
    0x9abfb3b6 as core::ffi::c_uint as UNS_32_BITS,
    0x3b6e20c as core::ffi::c_int as UNS_32_BITS,
    0x74b1d29a as core::ffi::c_int as UNS_32_BITS,
    0xead54739 as core::ffi::c_uint as UNS_32_BITS,
    0x9dd277af as core::ffi::c_uint as UNS_32_BITS,
    0x4db2615 as core::ffi::c_int as UNS_32_BITS,
    0x73dc1683 as core::ffi::c_int as UNS_32_BITS,
    0xe3630b12 as core::ffi::c_uint as UNS_32_BITS,
    0x94643b84 as core::ffi::c_uint as UNS_32_BITS,
    0xd6d6a3e as core::ffi::c_int as UNS_32_BITS,
    0x7a6a5aa8 as core::ffi::c_int as UNS_32_BITS,
    0xe40ecf0b as core::ffi::c_uint as UNS_32_BITS,
    0x9309ff9d as core::ffi::c_uint as UNS_32_BITS,
    0xa00ae27 as core::ffi::c_int as UNS_32_BITS,
    0x7d079eb1 as core::ffi::c_int as UNS_32_BITS,
    0xf00f9344 as core::ffi::c_uint as UNS_32_BITS,
    0x8708a3d2 as core::ffi::c_uint as UNS_32_BITS,
    0x1e01f268 as core::ffi::c_int as UNS_32_BITS,
    0x6906c2fe as core::ffi::c_int as UNS_32_BITS,
    0xf762575d as core::ffi::c_uint as UNS_32_BITS,
    0x806567cb as core::ffi::c_uint as UNS_32_BITS,
    0x196c3671 as core::ffi::c_int as UNS_32_BITS,
    0x6e6b06e7 as core::ffi::c_int as UNS_32_BITS,
    0xfed41b76 as core::ffi::c_uint as UNS_32_BITS,
    0x89d32be0 as core::ffi::c_uint as UNS_32_BITS,
    0x10da7a5a as core::ffi::c_int as UNS_32_BITS,
    0x67dd4acc as core::ffi::c_int as UNS_32_BITS,
    0xf9b9df6f as core::ffi::c_uint as UNS_32_BITS,
    0x8ebeeff9 as core::ffi::c_uint as UNS_32_BITS,
    0x17b7be43 as core::ffi::c_int as UNS_32_BITS,
    0x60b08ed5 as core::ffi::c_int as UNS_32_BITS,
    0xd6d6a3e8 as core::ffi::c_uint as UNS_32_BITS,
    0xa1d1937e as core::ffi::c_uint as UNS_32_BITS,
    0x38d8c2c4 as core::ffi::c_int as UNS_32_BITS,
    0x4fdff252 as core::ffi::c_int as UNS_32_BITS,
    0xd1bb67f1 as core::ffi::c_uint as UNS_32_BITS,
    0xa6bc5767 as core::ffi::c_uint as UNS_32_BITS,
    0x3fb506dd as core::ffi::c_int as UNS_32_BITS,
    0x48b2364b as core::ffi::c_int as UNS_32_BITS,
    0xd80d2bda as core::ffi::c_uint as UNS_32_BITS,
    0xaf0a1b4c as core::ffi::c_uint as UNS_32_BITS,
    0x36034af6 as core::ffi::c_int as UNS_32_BITS,
    0x41047a60 as core::ffi::c_int as UNS_32_BITS,
    0xdf60efc3 as core::ffi::c_uint as UNS_32_BITS,
    0xa867df55 as core::ffi::c_uint as UNS_32_BITS,
    0x316e8eef as core::ffi::c_int as UNS_32_BITS,
    0x4669be79 as core::ffi::c_int as UNS_32_BITS,
    0xcb61b38c as core::ffi::c_uint as UNS_32_BITS,
    0xbc66831a as core::ffi::c_uint as UNS_32_BITS,
    0x256fd2a0 as core::ffi::c_int as UNS_32_BITS,
    0x5268e236 as core::ffi::c_int as UNS_32_BITS,
    0xcc0c7795 as core::ffi::c_uint as UNS_32_BITS,
    0xbb0b4703 as core::ffi::c_uint as UNS_32_BITS,
    0x220216b9 as core::ffi::c_int as UNS_32_BITS,
    0x5505262f as core::ffi::c_int as UNS_32_BITS,
    0xc5ba3bbe as core::ffi::c_uint as UNS_32_BITS,
    0xb2bd0b28 as core::ffi::c_uint as UNS_32_BITS,
    0x2bb45a92 as core::ffi::c_int as UNS_32_BITS,
    0x5cb36a04 as core::ffi::c_int as UNS_32_BITS,
    0xc2d7ffa7 as core::ffi::c_uint as UNS_32_BITS,
    0xb5d0cf31 as core::ffi::c_uint as UNS_32_BITS,
    0x2cd99e8b as core::ffi::c_int as UNS_32_BITS,
    0x5bdeae1d as core::ffi::c_int as UNS_32_BITS,
    0x9b64c2b0 as core::ffi::c_uint as UNS_32_BITS,
    0xec63f226 as core::ffi::c_uint as UNS_32_BITS,
    0x756aa39c as core::ffi::c_int as UNS_32_BITS,
    0x26d930a as core::ffi::c_int as UNS_32_BITS,
    0x9c0906a9 as core::ffi::c_uint as UNS_32_BITS,
    0xeb0e363f as core::ffi::c_uint as UNS_32_BITS,
    0x72076785 as core::ffi::c_int as UNS_32_BITS,
    0x5005713 as core::ffi::c_int as UNS_32_BITS,
    0x95bf4a82 as core::ffi::c_uint as UNS_32_BITS,
    0xe2b87a14 as core::ffi::c_uint as UNS_32_BITS,
    0x7bb12bae as core::ffi::c_int as UNS_32_BITS,
    0xcb61b38 as core::ffi::c_int as UNS_32_BITS,
    0x92d28e9b as core::ffi::c_uint as UNS_32_BITS,
    0xe5d5be0d as core::ffi::c_uint as UNS_32_BITS,
    0x7cdcefb7 as core::ffi::c_int as UNS_32_BITS,
    0xbdbdf21 as core::ffi::c_int as UNS_32_BITS,
    0x86d3d2d4 as core::ffi::c_uint as UNS_32_BITS,
    0xf1d4e242 as core::ffi::c_uint as UNS_32_BITS,
    0x68ddb3f8 as core::ffi::c_int as UNS_32_BITS,
    0x1fda836e as core::ffi::c_int as UNS_32_BITS,
    0x81be16cd as core::ffi::c_uint as UNS_32_BITS,
    0xf6b9265b as core::ffi::c_uint as UNS_32_BITS,
    0x6fb077e1 as core::ffi::c_int as UNS_32_BITS,
    0x18b74777 as core::ffi::c_int as UNS_32_BITS,
    0x88085ae6 as core::ffi::c_uint as UNS_32_BITS,
    0xff0f6a70 as core::ffi::c_uint as UNS_32_BITS,
    0x66063bca as core::ffi::c_int as UNS_32_BITS,
    0x11010b5c as core::ffi::c_int as UNS_32_BITS,
    0x8f659eff as core::ffi::c_uint as UNS_32_BITS,
    0xf862ae69 as core::ffi::c_uint as UNS_32_BITS,
    0x616bffd3 as core::ffi::c_int as UNS_32_BITS,
    0x166ccf45 as core::ffi::c_int as UNS_32_BITS,
    0xa00ae278 as core::ffi::c_uint as UNS_32_BITS,
    0xd70dd2ee as core::ffi::c_uint as UNS_32_BITS,
    0x4e048354 as core::ffi::c_int as UNS_32_BITS,
    0x3903b3c2 as core::ffi::c_int as UNS_32_BITS,
    0xa7672661 as core::ffi::c_uint as UNS_32_BITS,
    0xd06016f7 as core::ffi::c_uint as UNS_32_BITS,
    0x4969474d as core::ffi::c_int as UNS_32_BITS,
    0x3e6e77db as core::ffi::c_int as UNS_32_BITS,
    0xaed16a4a as core::ffi::c_uint as UNS_32_BITS,
    0xd9d65adc as core::ffi::c_uint as UNS_32_BITS,
    0x40df0b66 as core::ffi::c_int as UNS_32_BITS,
    0x37d83bf0 as core::ffi::c_int as UNS_32_BITS,
    0xa9bcae53 as core::ffi::c_uint as UNS_32_BITS,
    0xdebb9ec5 as core::ffi::c_uint as UNS_32_BITS,
    0x47b2cf7f as core::ffi::c_int as UNS_32_BITS,
    0x30b5ffe9 as core::ffi::c_int as UNS_32_BITS,
    0xbdbdf21c as core::ffi::c_uint as UNS_32_BITS,
    0xcabac28a as core::ffi::c_uint as UNS_32_BITS,
    0x53b39330 as core::ffi::c_int as UNS_32_BITS,
    0x24b4a3a6 as core::ffi::c_int as UNS_32_BITS,
    0xbad03605 as core::ffi::c_uint as UNS_32_BITS,
    0xcdd70693 as core::ffi::c_uint as UNS_32_BITS,
    0x54de5729 as core::ffi::c_int as UNS_32_BITS,
    0x23d967bf as core::ffi::c_int as UNS_32_BITS,
    0xb3667a2e as core::ffi::c_uint as UNS_32_BITS,
    0xc4614ab8 as core::ffi::c_uint as UNS_32_BITS,
    0x5d681b02 as core::ffi::c_int as UNS_32_BITS,
    0x2a6f2b94 as core::ffi::c_int as UNS_32_BITS,
    0xb40bbe37 as core::ffi::c_uint as UNS_32_BITS,
    0xc30c8ea1 as core::ffi::c_uint as UNS_32_BITS,
    0x5a05df1b as core::ffi::c_int as UNS_32_BITS,
    0x2d02ef8d as core::ffi::c_int as UNS_32_BITS,
];
#[no_mangle]
pub unsafe extern "C" fn crc32file(
    mut name: *mut core::ffi::c_char,
    mut crc: *mut DWORD,
    mut charcnt: *mut core::ffi::c_long,
) -> Boolean_T {
    let mut fin: *mut FILE = 0 as *mut FILE;
    let mut oldcrc32: DWORD = 0;
    let mut c: core::ffi::c_int = 0;
    oldcrc32 = 0xffffffff as DWORD;
    *charcnt = 0 as core::ffi::c_long;
    fin = fopen(name, b"r\0" as *const u8 as *const core::ffi::c_char) as *mut FILE;
    if fin.is_null() {
        perror(name);
        return Error_;
    }
    loop {
        c = getc(fin);
        if !(c != EOF) {
            break;
        }
        *charcnt += 1;
        oldcrc32 = crc_32_tab[((oldcrc32 ^ c as BYTE as DWORD) & 0xff as DWORD) as usize]
            ^ oldcrc32 >> 8 as core::ffi::c_int;
    }
    if ferror(fin) != 0 {
        perror(name);
        *charcnt = -(1 as core::ffi::c_int) as core::ffi::c_long;
    }
    fclose(fin);
    oldcrc32 = !oldcrc32;
    *crc = oldcrc32;
    return Success_;
}
#[no_mangle]
pub unsafe extern "C" fn crc32buf(
    mut buf: *mut core::ffi::c_char,
    mut len: size_t,
) -> DWORD {
    let mut oldcrc32: DWORD = 0;
    oldcrc32 = 0xffffffff as DWORD;
    while len != 0 {
        oldcrc32 = crc_32_tab[((oldcrc32 ^ *buf as BYTE as DWORD) & 0xff as DWORD)
            as usize] ^ oldcrc32 >> 8 as core::ffi::c_int;
        len = len.wrapping_sub(1);
        buf = buf.offset(1);
    }
    return !oldcrc32;
}
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut crc: DWORD = 0;
    let mut charcnt: core::ffi::c_long = 0;
    let mut errors: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        argc -= 1;
        if !(argc > 0 as core::ffi::c_int) {
            break;
        }
        argv = argv.offset(1);
        errors |= crc32file(*argv, &mut crc, &mut charcnt) as core::ffi::c_int;
        printf(
            b"%08lX %7ld %s\n\0" as *const u8 as *const core::ffi::c_char,
            crc,
            charcnt,
            *argv,
        );
    }
    return (errors != 0 as core::ffi::c_int) as core::ffi::c_int;
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

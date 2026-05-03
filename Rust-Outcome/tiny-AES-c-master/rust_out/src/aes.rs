#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SafeAES_ctx {
    pub RoundKey: Vec<u8>,
    pub Iv: Vec<u8>,
}

impl SafeAES_ctx  {
    pub unsafe fn from_ptr(ptr: *const AES_ctx) -> Self {
        if ptr.is_null() {
            return SafeAES_ctx {
                RoundKey: vec![0; 176],
                Iv: vec![0; 16]
            };
        }

        let orig = core::ptr::read(ptr);
        
        let mut roundkey_vec = Vec::new();
        for &item in &orig.RoundKey {
            roundkey_vec.push(item);
        }

        let mut iv_vec = Vec::new();
        for &item in &orig.Iv {
            iv_vec.push(item);
        }

        SafeAES_ctx { RoundKey: roundkey_vec, Iv: iv_vec,
            ..Default::default()
        }
    }
}

extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AES_ctx {
    pub RoundKey: [uint8_t; 176],
    pub Iv: [uint8_t; 16],
}
pub type state_t = [[uint8_t; 4]; 4];
pub const AES_BLOCKLEN: core::ffi::c_int = 16 as core::ffi::c_int;
pub const Nb: core::ffi::c_int = 4 as core::ffi::c_int;
pub const Nk: core::ffi::c_int = 4 as core::ffi::c_int;
pub const Nr: core::ffi::c_int = 10 as core::ffi::c_int;
static mut sbox: [uint8_t; 256] = [
    0x63 as core::ffi::c_int as uint8_t,
    0x7c as core::ffi::c_int as uint8_t,
    0x77 as core::ffi::c_int as uint8_t,
    0x7b as core::ffi::c_int as uint8_t,
    0xf2 as core::ffi::c_int as uint8_t,
    0x6b as core::ffi::c_int as uint8_t,
    0x6f as core::ffi::c_int as uint8_t,
    0xc5 as core::ffi::c_int as uint8_t,
    0x30 as core::ffi::c_int as uint8_t,
    0x1 as core::ffi::c_int as uint8_t,
    0x67 as core::ffi::c_int as uint8_t,
    0x2b as core::ffi::c_int as uint8_t,
    0xfe as core::ffi::c_int as uint8_t,
    0xd7 as core::ffi::c_int as uint8_t,
    0xab as core::ffi::c_int as uint8_t,
    0x76 as core::ffi::c_int as uint8_t,
    0xca as core::ffi::c_int as uint8_t,
    0x82 as core::ffi::c_int as uint8_t,
    0xc9 as core::ffi::c_int as uint8_t,
    0x7d as core::ffi::c_int as uint8_t,
    0xfa as core::ffi::c_int as uint8_t,
    0x59 as core::ffi::c_int as uint8_t,
    0x47 as core::ffi::c_int as uint8_t,
    0xf0 as core::ffi::c_int as uint8_t,
    0xad as core::ffi::c_int as uint8_t,
    0xd4 as core::ffi::c_int as uint8_t,
    0xa2 as core::ffi::c_int as uint8_t,
    0xaf as core::ffi::c_int as uint8_t,
    0x9c as core::ffi::c_int as uint8_t,
    0xa4 as core::ffi::c_int as uint8_t,
    0x72 as core::ffi::c_int as uint8_t,
    0xc0 as core::ffi::c_int as uint8_t,
    0xb7 as core::ffi::c_int as uint8_t,
    0xfd as core::ffi::c_int as uint8_t,
    0x93 as core::ffi::c_int as uint8_t,
    0x26 as core::ffi::c_int as uint8_t,
    0x36 as core::ffi::c_int as uint8_t,
    0x3f as core::ffi::c_int as uint8_t,
    0xf7 as core::ffi::c_int as uint8_t,
    0xcc as core::ffi::c_int as uint8_t,
    0x34 as core::ffi::c_int as uint8_t,
    0xa5 as core::ffi::c_int as uint8_t,
    0xe5 as core::ffi::c_int as uint8_t,
    0xf1 as core::ffi::c_int as uint8_t,
    0x71 as core::ffi::c_int as uint8_t,
    0xd8 as core::ffi::c_int as uint8_t,
    0x31 as core::ffi::c_int as uint8_t,
    0x15 as core::ffi::c_int as uint8_t,
    0x4 as core::ffi::c_int as uint8_t,
    0xc7 as core::ffi::c_int as uint8_t,
    0x23 as core::ffi::c_int as uint8_t,
    0xc3 as core::ffi::c_int as uint8_t,
    0x18 as core::ffi::c_int as uint8_t,
    0x96 as core::ffi::c_int as uint8_t,
    0x5 as core::ffi::c_int as uint8_t,
    0x9a as core::ffi::c_int as uint8_t,
    0x7 as core::ffi::c_int as uint8_t,
    0x12 as core::ffi::c_int as uint8_t,
    0x80 as core::ffi::c_int as uint8_t,
    0xe2 as core::ffi::c_int as uint8_t,
    0xeb as core::ffi::c_int as uint8_t,
    0x27 as core::ffi::c_int as uint8_t,
    0xb2 as core::ffi::c_int as uint8_t,
    0x75 as core::ffi::c_int as uint8_t,
    0x9 as core::ffi::c_int as uint8_t,
    0x83 as core::ffi::c_int as uint8_t,
    0x2c as core::ffi::c_int as uint8_t,
    0x1a as core::ffi::c_int as uint8_t,
    0x1b as core::ffi::c_int as uint8_t,
    0x6e as core::ffi::c_int as uint8_t,
    0x5a as core::ffi::c_int as uint8_t,
    0xa0 as core::ffi::c_int as uint8_t,
    0x52 as core::ffi::c_int as uint8_t,
    0x3b as core::ffi::c_int as uint8_t,
    0xd6 as core::ffi::c_int as uint8_t,
    0xb3 as core::ffi::c_int as uint8_t,
    0x29 as core::ffi::c_int as uint8_t,
    0xe3 as core::ffi::c_int as uint8_t,
    0x2f as core::ffi::c_int as uint8_t,
    0x84 as core::ffi::c_int as uint8_t,
    0x53 as core::ffi::c_int as uint8_t,
    0xd1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0xed as core::ffi::c_int as uint8_t,
    0x20 as core::ffi::c_int as uint8_t,
    0xfc as core::ffi::c_int as uint8_t,
    0xb1 as core::ffi::c_int as uint8_t,
    0x5b as core::ffi::c_int as uint8_t,
    0x6a as core::ffi::c_int as uint8_t,
    0xcb as core::ffi::c_int as uint8_t,
    0xbe as core::ffi::c_int as uint8_t,
    0x39 as core::ffi::c_int as uint8_t,
    0x4a as core::ffi::c_int as uint8_t,
    0x4c as core::ffi::c_int as uint8_t,
    0x58 as core::ffi::c_int as uint8_t,
    0xcf as core::ffi::c_int as uint8_t,
    0xd0 as core::ffi::c_int as uint8_t,
    0xef as core::ffi::c_int as uint8_t,
    0xaa as core::ffi::c_int as uint8_t,
    0xfb as core::ffi::c_int as uint8_t,
    0x43 as core::ffi::c_int as uint8_t,
    0x4d as core::ffi::c_int as uint8_t,
    0x33 as core::ffi::c_int as uint8_t,
    0x85 as core::ffi::c_int as uint8_t,
    0x45 as core::ffi::c_int as uint8_t,
    0xf9 as core::ffi::c_int as uint8_t,
    0x2 as core::ffi::c_int as uint8_t,
    0x7f as core::ffi::c_int as uint8_t,
    0x50 as core::ffi::c_int as uint8_t,
    0x3c as core::ffi::c_int as uint8_t,
    0x9f as core::ffi::c_int as uint8_t,
    0xa8 as core::ffi::c_int as uint8_t,
    0x51 as core::ffi::c_int as uint8_t,
    0xa3 as core::ffi::c_int as uint8_t,
    0x40 as core::ffi::c_int as uint8_t,
    0x8f as core::ffi::c_int as uint8_t,
    0x92 as core::ffi::c_int as uint8_t,
    0x9d as core::ffi::c_int as uint8_t,
    0x38 as core::ffi::c_int as uint8_t,
    0xf5 as core::ffi::c_int as uint8_t,
    0xbc as core::ffi::c_int as uint8_t,
    0xb6 as core::ffi::c_int as uint8_t,
    0xda as core::ffi::c_int as uint8_t,
    0x21 as core::ffi::c_int as uint8_t,
    0x10 as core::ffi::c_int as uint8_t,
    0xff as core::ffi::c_int as uint8_t,
    0xf3 as core::ffi::c_int as uint8_t,
    0xd2 as core::ffi::c_int as uint8_t,
    0xcd as core::ffi::c_int as uint8_t,
    0xc as core::ffi::c_int as uint8_t,
    0x13 as core::ffi::c_int as uint8_t,
    0xec as core::ffi::c_int as uint8_t,
    0x5f as core::ffi::c_int as uint8_t,
    0x97 as core::ffi::c_int as uint8_t,
    0x44 as core::ffi::c_int as uint8_t,
    0x17 as core::ffi::c_int as uint8_t,
    0xc4 as core::ffi::c_int as uint8_t,
    0xa7 as core::ffi::c_int as uint8_t,
    0x7e as core::ffi::c_int as uint8_t,
    0x3d as core::ffi::c_int as uint8_t,
    0x64 as core::ffi::c_int as uint8_t,
    0x5d as core::ffi::c_int as uint8_t,
    0x19 as core::ffi::c_int as uint8_t,
    0x73 as core::ffi::c_int as uint8_t,
    0x60 as core::ffi::c_int as uint8_t,
    0x81 as core::ffi::c_int as uint8_t,
    0x4f as core::ffi::c_int as uint8_t,
    0xdc as core::ffi::c_int as uint8_t,
    0x22 as core::ffi::c_int as uint8_t,
    0x2a as core::ffi::c_int as uint8_t,
    0x90 as core::ffi::c_int as uint8_t,
    0x88 as core::ffi::c_int as uint8_t,
    0x46 as core::ffi::c_int as uint8_t,
    0xee as core::ffi::c_int as uint8_t,
    0xb8 as core::ffi::c_int as uint8_t,
    0x14 as core::ffi::c_int as uint8_t,
    0xde as core::ffi::c_int as uint8_t,
    0x5e as core::ffi::c_int as uint8_t,
    0xb as core::ffi::c_int as uint8_t,
    0xdb as core::ffi::c_int as uint8_t,
    0xe0 as core::ffi::c_int as uint8_t,
    0x32 as core::ffi::c_int as uint8_t,
    0x3a as core::ffi::c_int as uint8_t,
    0xa as core::ffi::c_int as uint8_t,
    0x49 as core::ffi::c_int as uint8_t,
    0x6 as core::ffi::c_int as uint8_t,
    0x24 as core::ffi::c_int as uint8_t,
    0x5c as core::ffi::c_int as uint8_t,
    0xc2 as core::ffi::c_int as uint8_t,
    0xd3 as core::ffi::c_int as uint8_t,
    0xac as core::ffi::c_int as uint8_t,
    0x62 as core::ffi::c_int as uint8_t,
    0x91 as core::ffi::c_int as uint8_t,
    0x95 as core::ffi::c_int as uint8_t,
    0xe4 as core::ffi::c_int as uint8_t,
    0x79 as core::ffi::c_int as uint8_t,
    0xe7 as core::ffi::c_int as uint8_t,
    0xc8 as core::ffi::c_int as uint8_t,
    0x37 as core::ffi::c_int as uint8_t,
    0x6d as core::ffi::c_int as uint8_t,
    0x8d as core::ffi::c_int as uint8_t,
    0xd5 as core::ffi::c_int as uint8_t,
    0x4e as core::ffi::c_int as uint8_t,
    0xa9 as core::ffi::c_int as uint8_t,
    0x6c as core::ffi::c_int as uint8_t,
    0x56 as core::ffi::c_int as uint8_t,
    0xf4 as core::ffi::c_int as uint8_t,
    0xea as core::ffi::c_int as uint8_t,
    0x65 as core::ffi::c_int as uint8_t,
    0x7a as core::ffi::c_int as uint8_t,
    0xae as core::ffi::c_int as uint8_t,
    0x8 as core::ffi::c_int as uint8_t,
    0xba as core::ffi::c_int as uint8_t,
    0x78 as core::ffi::c_int as uint8_t,
    0x25 as core::ffi::c_int as uint8_t,
    0x2e as core::ffi::c_int as uint8_t,
    0x1c as core::ffi::c_int as uint8_t,
    0xa6 as core::ffi::c_int as uint8_t,
    0xb4 as core::ffi::c_int as uint8_t,
    0xc6 as core::ffi::c_int as uint8_t,
    0xe8 as core::ffi::c_int as uint8_t,
    0xdd as core::ffi::c_int as uint8_t,
    0x74 as core::ffi::c_int as uint8_t,
    0x1f as core::ffi::c_int as uint8_t,
    0x4b as core::ffi::c_int as uint8_t,
    0xbd as core::ffi::c_int as uint8_t,
    0x8b as core::ffi::c_int as uint8_t,
    0x8a as core::ffi::c_int as uint8_t,
    0x70 as core::ffi::c_int as uint8_t,
    0x3e as core::ffi::c_int as uint8_t,
    0xb5 as core::ffi::c_int as uint8_t,
    0x66 as core::ffi::c_int as uint8_t,
    0x48 as core::ffi::c_int as uint8_t,
    0x3 as core::ffi::c_int as uint8_t,
    0xf6 as core::ffi::c_int as uint8_t,
    0xe as core::ffi::c_int as uint8_t,
    0x61 as core::ffi::c_int as uint8_t,
    0x35 as core::ffi::c_int as uint8_t,
    0x57 as core::ffi::c_int as uint8_t,
    0xb9 as core::ffi::c_int as uint8_t,
    0x86 as core::ffi::c_int as uint8_t,
    0xc1 as core::ffi::c_int as uint8_t,
    0x1d as core::ffi::c_int as uint8_t,
    0x9e as core::ffi::c_int as uint8_t,
    0xe1 as core::ffi::c_int as uint8_t,
    0xf8 as core::ffi::c_int as uint8_t,
    0x98 as core::ffi::c_int as uint8_t,
    0x11 as core::ffi::c_int as uint8_t,
    0x69 as core::ffi::c_int as uint8_t,
    0xd9 as core::ffi::c_int as uint8_t,
    0x8e as core::ffi::c_int as uint8_t,
    0x94 as core::ffi::c_int as uint8_t,
    0x9b as core::ffi::c_int as uint8_t,
    0x1e as core::ffi::c_int as uint8_t,
    0x87 as core::ffi::c_int as uint8_t,
    0xe9 as core::ffi::c_int as uint8_t,
    0xce as core::ffi::c_int as uint8_t,
    0x55 as core::ffi::c_int as uint8_t,
    0x28 as core::ffi::c_int as uint8_t,
    0xdf as core::ffi::c_int as uint8_t,
    0x8c as core::ffi::c_int as uint8_t,
    0xa1 as core::ffi::c_int as uint8_t,
    0x89 as core::ffi::c_int as uint8_t,
    0xd as core::ffi::c_int as uint8_t,
    0xbf as core::ffi::c_int as uint8_t,
    0xe6 as core::ffi::c_int as uint8_t,
    0x42 as core::ffi::c_int as uint8_t,
    0x68 as core::ffi::c_int as uint8_t,
    0x41 as core::ffi::c_int as uint8_t,
    0x99 as core::ffi::c_int as uint8_t,
    0x2d as core::ffi::c_int as uint8_t,
    0xf as core::ffi::c_int as uint8_t,
    0xb0 as core::ffi::c_int as uint8_t,
    0x54 as core::ffi::c_int as uint8_t,
    0xbb as core::ffi::c_int as uint8_t,
    0x16 as core::ffi::c_int as uint8_t,
];
static mut rsbox: [uint8_t; 256] = [
    0x52 as core::ffi::c_int as uint8_t,
    0x9 as core::ffi::c_int as uint8_t,
    0x6a as core::ffi::c_int as uint8_t,
    0xd5 as core::ffi::c_int as uint8_t,
    0x30 as core::ffi::c_int as uint8_t,
    0x36 as core::ffi::c_int as uint8_t,
    0xa5 as core::ffi::c_int as uint8_t,
    0x38 as core::ffi::c_int as uint8_t,
    0xbf as core::ffi::c_int as uint8_t,
    0x40 as core::ffi::c_int as uint8_t,
    0xa3 as core::ffi::c_int as uint8_t,
    0x9e as core::ffi::c_int as uint8_t,
    0x81 as core::ffi::c_int as uint8_t,
    0xf3 as core::ffi::c_int as uint8_t,
    0xd7 as core::ffi::c_int as uint8_t,
    0xfb as core::ffi::c_int as uint8_t,
    0x7c as core::ffi::c_int as uint8_t,
    0xe3 as core::ffi::c_int as uint8_t,
    0x39 as core::ffi::c_int as uint8_t,
    0x82 as core::ffi::c_int as uint8_t,
    0x9b as core::ffi::c_int as uint8_t,
    0x2f as core::ffi::c_int as uint8_t,
    0xff as core::ffi::c_int as uint8_t,
    0x87 as core::ffi::c_int as uint8_t,
    0x34 as core::ffi::c_int as uint8_t,
    0x8e as core::ffi::c_int as uint8_t,
    0x43 as core::ffi::c_int as uint8_t,
    0x44 as core::ffi::c_int as uint8_t,
    0xc4 as core::ffi::c_int as uint8_t,
    0xde as core::ffi::c_int as uint8_t,
    0xe9 as core::ffi::c_int as uint8_t,
    0xcb as core::ffi::c_int as uint8_t,
    0x54 as core::ffi::c_int as uint8_t,
    0x7b as core::ffi::c_int as uint8_t,
    0x94 as core::ffi::c_int as uint8_t,
    0x32 as core::ffi::c_int as uint8_t,
    0xa6 as core::ffi::c_int as uint8_t,
    0xc2 as core::ffi::c_int as uint8_t,
    0x23 as core::ffi::c_int as uint8_t,
    0x3d as core::ffi::c_int as uint8_t,
    0xee as core::ffi::c_int as uint8_t,
    0x4c as core::ffi::c_int as uint8_t,
    0x95 as core::ffi::c_int as uint8_t,
    0xb as core::ffi::c_int as uint8_t,
    0x42 as core::ffi::c_int as uint8_t,
    0xfa as core::ffi::c_int as uint8_t,
    0xc3 as core::ffi::c_int as uint8_t,
    0x4e as core::ffi::c_int as uint8_t,
    0x8 as core::ffi::c_int as uint8_t,
    0x2e as core::ffi::c_int as uint8_t,
    0xa1 as core::ffi::c_int as uint8_t,
    0x66 as core::ffi::c_int as uint8_t,
    0x28 as core::ffi::c_int as uint8_t,
    0xd9 as core::ffi::c_int as uint8_t,
    0x24 as core::ffi::c_int as uint8_t,
    0xb2 as core::ffi::c_int as uint8_t,
    0x76 as core::ffi::c_int as uint8_t,
    0x5b as core::ffi::c_int as uint8_t,
    0xa2 as core::ffi::c_int as uint8_t,
    0x49 as core::ffi::c_int as uint8_t,
    0x6d as core::ffi::c_int as uint8_t,
    0x8b as core::ffi::c_int as uint8_t,
    0xd1 as core::ffi::c_int as uint8_t,
    0x25 as core::ffi::c_int as uint8_t,
    0x72 as core::ffi::c_int as uint8_t,
    0xf8 as core::ffi::c_int as uint8_t,
    0xf6 as core::ffi::c_int as uint8_t,
    0x64 as core::ffi::c_int as uint8_t,
    0x86 as core::ffi::c_int as uint8_t,
    0x68 as core::ffi::c_int as uint8_t,
    0x98 as core::ffi::c_int as uint8_t,
    0x16 as core::ffi::c_int as uint8_t,
    0xd4 as core::ffi::c_int as uint8_t,
    0xa4 as core::ffi::c_int as uint8_t,
    0x5c as core::ffi::c_int as uint8_t,
    0xcc as core::ffi::c_int as uint8_t,
    0x5d as core::ffi::c_int as uint8_t,
    0x65 as core::ffi::c_int as uint8_t,
    0xb6 as core::ffi::c_int as uint8_t,
    0x92 as core::ffi::c_int as uint8_t,
    0x6c as core::ffi::c_int as uint8_t,
    0x70 as core::ffi::c_int as uint8_t,
    0x48 as core::ffi::c_int as uint8_t,
    0x50 as core::ffi::c_int as uint8_t,
    0xfd as core::ffi::c_int as uint8_t,
    0xed as core::ffi::c_int as uint8_t,
    0xb9 as core::ffi::c_int as uint8_t,
    0xda as core::ffi::c_int as uint8_t,
    0x5e as core::ffi::c_int as uint8_t,
    0x15 as core::ffi::c_int as uint8_t,
    0x46 as core::ffi::c_int as uint8_t,
    0x57 as core::ffi::c_int as uint8_t,
    0xa7 as core::ffi::c_int as uint8_t,
    0x8d as core::ffi::c_int as uint8_t,
    0x9d as core::ffi::c_int as uint8_t,
    0x84 as core::ffi::c_int as uint8_t,
    0x90 as core::ffi::c_int as uint8_t,
    0xd8 as core::ffi::c_int as uint8_t,
    0xab as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0x8c as core::ffi::c_int as uint8_t,
    0xbc as core::ffi::c_int as uint8_t,
    0xd3 as core::ffi::c_int as uint8_t,
    0xa as core::ffi::c_int as uint8_t,
    0xf7 as core::ffi::c_int as uint8_t,
    0xe4 as core::ffi::c_int as uint8_t,
    0x58 as core::ffi::c_int as uint8_t,
    0x5 as core::ffi::c_int as uint8_t,
    0xb8 as core::ffi::c_int as uint8_t,
    0xb3 as core::ffi::c_int as uint8_t,
    0x45 as core::ffi::c_int as uint8_t,
    0x6 as core::ffi::c_int as uint8_t,
    0xd0 as core::ffi::c_int as uint8_t,
    0x2c as core::ffi::c_int as uint8_t,
    0x1e as core::ffi::c_int as uint8_t,
    0x8f as core::ffi::c_int as uint8_t,
    0xca as core::ffi::c_int as uint8_t,
    0x3f as core::ffi::c_int as uint8_t,
    0xf as core::ffi::c_int as uint8_t,
    0x2 as core::ffi::c_int as uint8_t,
    0xc1 as core::ffi::c_int as uint8_t,
    0xaf as core::ffi::c_int as uint8_t,
    0xbd as core::ffi::c_int as uint8_t,
    0x3 as core::ffi::c_int as uint8_t,
    0x1 as core::ffi::c_int as uint8_t,
    0x13 as core::ffi::c_int as uint8_t,
    0x8a as core::ffi::c_int as uint8_t,
    0x6b as core::ffi::c_int as uint8_t,
    0x3a as core::ffi::c_int as uint8_t,
    0x91 as core::ffi::c_int as uint8_t,
    0x11 as core::ffi::c_int as uint8_t,
    0x41 as core::ffi::c_int as uint8_t,
    0x4f as core::ffi::c_int as uint8_t,
    0x67 as core::ffi::c_int as uint8_t,
    0xdc as core::ffi::c_int as uint8_t,
    0xea as core::ffi::c_int as uint8_t,
    0x97 as core::ffi::c_int as uint8_t,
    0xf2 as core::ffi::c_int as uint8_t,
    0xcf as core::ffi::c_int as uint8_t,
    0xce as core::ffi::c_int as uint8_t,
    0xf0 as core::ffi::c_int as uint8_t,
    0xb4 as core::ffi::c_int as uint8_t,
    0xe6 as core::ffi::c_int as uint8_t,
    0x73 as core::ffi::c_int as uint8_t,
    0x96 as core::ffi::c_int as uint8_t,
    0xac as core::ffi::c_int as uint8_t,
    0x74 as core::ffi::c_int as uint8_t,
    0x22 as core::ffi::c_int as uint8_t,
    0xe7 as core::ffi::c_int as uint8_t,
    0xad as core::ffi::c_int as uint8_t,
    0x35 as core::ffi::c_int as uint8_t,
    0x85 as core::ffi::c_int as uint8_t,
    0xe2 as core::ffi::c_int as uint8_t,
    0xf9 as core::ffi::c_int as uint8_t,
    0x37 as core::ffi::c_int as uint8_t,
    0xe8 as core::ffi::c_int as uint8_t,
    0x1c as core::ffi::c_int as uint8_t,
    0x75 as core::ffi::c_int as uint8_t,
    0xdf as core::ffi::c_int as uint8_t,
    0x6e as core::ffi::c_int as uint8_t,
    0x47 as core::ffi::c_int as uint8_t,
    0xf1 as core::ffi::c_int as uint8_t,
    0x1a as core::ffi::c_int as uint8_t,
    0x71 as core::ffi::c_int as uint8_t,
    0x1d as core::ffi::c_int as uint8_t,
    0x29 as core::ffi::c_int as uint8_t,
    0xc5 as core::ffi::c_int as uint8_t,
    0x89 as core::ffi::c_int as uint8_t,
    0x6f as core::ffi::c_int as uint8_t,
    0xb7 as core::ffi::c_int as uint8_t,
    0x62 as core::ffi::c_int as uint8_t,
    0xe as core::ffi::c_int as uint8_t,
    0xaa as core::ffi::c_int as uint8_t,
    0x18 as core::ffi::c_int as uint8_t,
    0xbe as core::ffi::c_int as uint8_t,
    0x1b as core::ffi::c_int as uint8_t,
    0xfc as core::ffi::c_int as uint8_t,
    0x56 as core::ffi::c_int as uint8_t,
    0x3e as core::ffi::c_int as uint8_t,
    0x4b as core::ffi::c_int as uint8_t,
    0xc6 as core::ffi::c_int as uint8_t,
    0xd2 as core::ffi::c_int as uint8_t,
    0x79 as core::ffi::c_int as uint8_t,
    0x20 as core::ffi::c_int as uint8_t,
    0x9a as core::ffi::c_int as uint8_t,
    0xdb as core::ffi::c_int as uint8_t,
    0xc0 as core::ffi::c_int as uint8_t,
    0xfe as core::ffi::c_int as uint8_t,
    0x78 as core::ffi::c_int as uint8_t,
    0xcd as core::ffi::c_int as uint8_t,
    0x5a as core::ffi::c_int as uint8_t,
    0xf4 as core::ffi::c_int as uint8_t,
    0x1f as core::ffi::c_int as uint8_t,
    0xdd as core::ffi::c_int as uint8_t,
    0xa8 as core::ffi::c_int as uint8_t,
    0x33 as core::ffi::c_int as uint8_t,
    0x88 as core::ffi::c_int as uint8_t,
    0x7 as core::ffi::c_int as uint8_t,
    0xc7 as core::ffi::c_int as uint8_t,
    0x31 as core::ffi::c_int as uint8_t,
    0xb1 as core::ffi::c_int as uint8_t,
    0x12 as core::ffi::c_int as uint8_t,
    0x10 as core::ffi::c_int as uint8_t,
    0x59 as core::ffi::c_int as uint8_t,
    0x27 as core::ffi::c_int as uint8_t,
    0x80 as core::ffi::c_int as uint8_t,
    0xec as core::ffi::c_int as uint8_t,
    0x5f as core::ffi::c_int as uint8_t,
    0x60 as core::ffi::c_int as uint8_t,
    0x51 as core::ffi::c_int as uint8_t,
    0x7f as core::ffi::c_int as uint8_t,
    0xa9 as core::ffi::c_int as uint8_t,
    0x19 as core::ffi::c_int as uint8_t,
    0xb5 as core::ffi::c_int as uint8_t,
    0x4a as core::ffi::c_int as uint8_t,
    0xd as core::ffi::c_int as uint8_t,
    0x2d as core::ffi::c_int as uint8_t,
    0xe5 as core::ffi::c_int as uint8_t,
    0x7a as core::ffi::c_int as uint8_t,
    0x9f as core::ffi::c_int as uint8_t,
    0x93 as core::ffi::c_int as uint8_t,
    0xc9 as core::ffi::c_int as uint8_t,
    0x9c as core::ffi::c_int as uint8_t,
    0xef as core::ffi::c_int as uint8_t,
    0xa0 as core::ffi::c_int as uint8_t,
    0xe0 as core::ffi::c_int as uint8_t,
    0x3b as core::ffi::c_int as uint8_t,
    0x4d as core::ffi::c_int as uint8_t,
    0xae as core::ffi::c_int as uint8_t,
    0x2a as core::ffi::c_int as uint8_t,
    0xf5 as core::ffi::c_int as uint8_t,
    0xb0 as core::ffi::c_int as uint8_t,
    0xc8 as core::ffi::c_int as uint8_t,
    0xeb as core::ffi::c_int as uint8_t,
    0xbb as core::ffi::c_int as uint8_t,
    0x3c as core::ffi::c_int as uint8_t,
    0x83 as core::ffi::c_int as uint8_t,
    0x53 as core::ffi::c_int as uint8_t,
    0x99 as core::ffi::c_int as uint8_t,
    0x61 as core::ffi::c_int as uint8_t,
    0x17 as core::ffi::c_int as uint8_t,
    0x2b as core::ffi::c_int as uint8_t,
    0x4 as core::ffi::c_int as uint8_t,
    0x7e as core::ffi::c_int as uint8_t,
    0xba as core::ffi::c_int as uint8_t,
    0x77 as core::ffi::c_int as uint8_t,
    0xd6 as core::ffi::c_int as uint8_t,
    0x26 as core::ffi::c_int as uint8_t,
    0xe1 as core::ffi::c_int as uint8_t,
    0x69 as core::ffi::c_int as uint8_t,
    0x14 as core::ffi::c_int as uint8_t,
    0x63 as core::ffi::c_int as uint8_t,
    0x55 as core::ffi::c_int as uint8_t,
    0x21 as core::ffi::c_int as uint8_t,
    0xc as core::ffi::c_int as uint8_t,
    0x7d as core::ffi::c_int as uint8_t,
];
static mut Rcon: [uint8_t; 11] = [
    0x8d as core::ffi::c_int as uint8_t,
    0x1 as core::ffi::c_int as uint8_t,
    0x2 as core::ffi::c_int as uint8_t,
    0x4 as core::ffi::c_int as uint8_t,
    0x8 as core::ffi::c_int as uint8_t,
    0x10 as core::ffi::c_int as uint8_t,
    0x20 as core::ffi::c_int as uint8_t,
    0x40 as core::ffi::c_int as uint8_t,
    0x80 as core::ffi::c_int as uint8_t,
    0x1b as core::ffi::c_int as uint8_t,
    0x36 as core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn KeyExpansion(mut RoundKey: *mut uint8_t, mut Key: *const uint8_t) {
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0;
    let mut k: core::ffi::c_uint = 0;
    let mut tempa: [uint8_t; 4] = [0; 4];
    i = 0 as core::ffi::c_uint;
    while i < Nk as core::ffi::c_uint {
        *RoundKey
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(0 as core::ffi::c_uint) as isize,
            ) = *Key
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(0 as core::ffi::c_uint) as isize,
            );
        *RoundKey
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as isize,
            ) = *Key
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint) as isize,
            );
        *RoundKey
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(2 as core::ffi::c_uint) as isize,
            ) = *Key
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(2 as core::ffi::c_uint) as isize,
            );
        *RoundKey
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(3 as core::ffi::c_uint) as isize,
            ) = *Key
            .offset(
                i
                    .wrapping_mul(4 as core::ffi::c_uint)
                    .wrapping_add(3 as core::ffi::c_uint) as isize,
            );
        i = i.wrapping_add(1);
    }
    i = Nk as core::ffi::c_uint;
    while i < (Nb * (Nr + 1 as core::ffi::c_int)) as core::ffi::c_uint {
        k = i.wrapping_sub(1 as core::ffi::c_uint).wrapping_mul(4 as core::ffi::c_uint);
        tempa[0 as core::ffi::c_int as usize] = *RoundKey
            .offset(k.wrapping_add(0 as core::ffi::c_uint) as isize);
        tempa[1 as core::ffi::c_int as usize] = *RoundKey
            .offset(k.wrapping_add(1 as core::ffi::c_uint) as isize);
        tempa[2 as core::ffi::c_int as usize] = *RoundKey
            .offset(k.wrapping_add(2 as core::ffi::c_uint) as isize);
        tempa[3 as core::ffi::c_int as usize] = *RoundKey
            .offset(k.wrapping_add(3 as core::ffi::c_uint) as isize);
        if i.wrapping_rem(Nk as core::ffi::c_uint) == 0 as core::ffi::c_uint {
            let u8tmp: uint8_t = tempa[0 as core::ffi::c_int as usize];
            tempa[0 as core::ffi::c_int as usize] = tempa[1 as core::ffi::c_int
                as usize];
            tempa[1 as core::ffi::c_int as usize] = tempa[2 as core::ffi::c_int
                as usize];
            tempa[2 as core::ffi::c_int as usize] = tempa[3 as core::ffi::c_int
                as usize];
            tempa[3 as core::ffi::c_int as usize] = u8tmp;
            tempa[0 as core::ffi::c_int as usize] = sbox[tempa[0 as core::ffi::c_int
                as usize] as usize];
            tempa[1 as core::ffi::c_int as usize] = sbox[tempa[1 as core::ffi::c_int
                as usize] as usize];
            tempa[2 as core::ffi::c_int as usize] = sbox[tempa[2 as core::ffi::c_int
                as usize] as usize];
            tempa[3 as core::ffi::c_int as usize] = sbox[tempa[3 as core::ffi::c_int
                as usize] as usize];
            tempa[0 as core::ffi::c_int as usize] = (tempa[0 as core::ffi::c_int
                as usize] as core::ffi::c_int
                ^ Rcon[i.wrapping_div(Nk as core::ffi::c_uint) as usize]
                    as core::ffi::c_int) as uint8_t;
        }
        j = i.wrapping_mul(4 as core::ffi::c_uint);
        k = i.wrapping_sub(Nk as core::ffi::c_uint).wrapping_mul(4 as core::ffi::c_uint);
        *RoundKey.offset(j.wrapping_add(0 as core::ffi::c_uint) as isize) = (*RoundKey
            .offset(k.wrapping_add(0 as core::ffi::c_uint) as isize) as core::ffi::c_int
            ^ tempa[0 as core::ffi::c_int as usize] as core::ffi::c_int) as uint8_t;
        *RoundKey.offset(j.wrapping_add(1 as core::ffi::c_uint) as isize) = (*RoundKey
            .offset(k.wrapping_add(1 as core::ffi::c_uint) as isize) as core::ffi::c_int
            ^ tempa[1 as core::ffi::c_int as usize] as core::ffi::c_int) as uint8_t;
        *RoundKey.offset(j.wrapping_add(2 as core::ffi::c_uint) as isize) = (*RoundKey
            .offset(k.wrapping_add(2 as core::ffi::c_uint) as isize) as core::ffi::c_int
            ^ tempa[2 as core::ffi::c_int as usize] as core::ffi::c_int) as uint8_t;
        *RoundKey.offset(j.wrapping_add(3 as core::ffi::c_uint) as isize) = (*RoundKey
            .offset(k.wrapping_add(3 as core::ffi::c_uint) as isize) as core::ffi::c_int
            ^ tempa[3 as core::ffi::c_int as usize] as core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AES_init_ctx(mut ctx: *mut AES_ctx, mut key: *const uint8_t) {
    KeyExpansion(((*ctx).RoundKey).as_mut_ptr(), key);
}
#[no_mangle]
pub unsafe extern "C" fn AES_init_ctx_iv(
    mut ctx: *mut AES_ctx,
    mut key: *const uint8_t,
    mut iv: *const uint8_t,
) {
    KeyExpansion(((*ctx).RoundKey).as_mut_ptr(), key);
    memcpy(
        ((*ctx).Iv).as_mut_ptr() as *mut core::ffi::c_void,
        iv as *const core::ffi::c_void,
        AES_BLOCKLEN as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn AES_ctx_set_iv(mut ctx: *mut AES_ctx, mut iv: *const uint8_t) {
    memcpy(
        ((*ctx).Iv).as_mut_ptr() as *mut core::ffi::c_void,
        iv as *const core::ffi::c_void,
        AES_BLOCKLEN as size_t,
    );
}
unsafe extern "C" fn AddRoundKey(
    mut round: uint8_t,
    mut state: *mut state_t,
    mut RoundKey: *const uint8_t,
) {
    let mut i: uint8_t = 0;
    let mut j: uint8_t = 0;
    i = 0 as uint8_t;
    while (i as core::ffi::c_int) < 4 as core::ffi::c_int {
        j = 0 as uint8_t;
        while (j as core::ffi::c_int) < 4 as core::ffi::c_int {
            (*state)[i as usize][j as usize] = ((*state)[i as usize][j as usize]
                as core::ffi::c_int
                ^ *RoundKey
                    .offset(
                        (round as core::ffi::c_int * Nb * 4 as core::ffi::c_int
                            + i as core::ffi::c_int * Nb + j as core::ffi::c_int)
                            as isize,
                    ) as core::ffi::c_int) as uint8_t;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn SubBytes(mut state: *mut state_t) {
    let mut i: uint8_t = 0;
    let mut j: uint8_t = 0;
    i = 0 as uint8_t;
    while (i as core::ffi::c_int) < 4 as core::ffi::c_int {
        j = 0 as uint8_t;
        while (j as core::ffi::c_int) < 4 as core::ffi::c_int {
            (*state)[j as usize][i as usize] = sbox[(*state)[j as usize][i as usize]
                as usize];
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ShiftRows(mut state: *mut state_t) {
    let mut temp: uint8_t = 0;
    temp = (*state)[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize];
    (*state)[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = (*state)[1
        as core::ffi::c_int as usize][1 as core::ffi::c_int as usize];
    (*state)[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = (*state)[2
        as core::ffi::c_int as usize][1 as core::ffi::c_int as usize];
    (*state)[2 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = (*state)[3
        as core::ffi::c_int as usize][1 as core::ffi::c_int as usize];
    (*state)[3 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = temp;
    temp = (*state)[0 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize];
    (*state)[0 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize] = (*state)[2
        as core::ffi::c_int as usize][2 as core::ffi::c_int as usize];
    (*state)[2 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize] = temp;
    temp = (*state)[1 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize];
    (*state)[1 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize] = (*state)[3
        as core::ffi::c_int as usize][2 as core::ffi::c_int as usize];
    (*state)[3 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize] = temp;
    temp = (*state)[0 as core::ffi::c_int as usize][3 as core::ffi::c_int as usize];
    (*state)[0 as core::ffi::c_int as usize][3 as core::ffi::c_int as usize] = (*state)[3
        as core::ffi::c_int as usize][3 as core::ffi::c_int as usize];
    (*state)[3 as core::ffi::c_int as usize][3 as core::ffi::c_int as usize] = (*state)[2
        as core::ffi::c_int as usize][3 as core::ffi::c_int as usize];
    (*state)[2 as core::ffi::c_int as usize][3 as core::ffi::c_int as usize] = (*state)[1
        as core::ffi::c_int as usize][3 as core::ffi::c_int as usize];
    (*state)[1 as core::ffi::c_int as usize][3 as core::ffi::c_int as usize] = temp;
}
unsafe extern "C" fn xtime(mut x: uint8_t) -> uint8_t {
    (((x as core::ffi::c_int) << 1 as core::ffi::c_int
        ^ ((x as core::ffi::c_int >> 7 as core::ffi::c_int 
        & 1 as core::ffi::c_int ) * (0x1b as core::ffi::c_int))) % 256) as uint8_t
}
unsafe extern "C" fn MixColumns(mut state: *mut state_t) {
    let mut i: uint8_t = 0;
    let mut Tmp: uint8_t = 0;
    let mut Tm: uint8_t = 0;
    let mut t: uint8_t = 0;
    i = 0 as uint8_t;
    while (i as core::ffi::c_int) < 4 as core::ffi::c_int {
        t = (*state)[i as usize][0 as core::ffi::c_int as usize];
        Tmp = ((*state)[i as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (*state)[i as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (*state)[i as usize][2 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (*state)[i as usize][3 as core::ffi::c_int as usize] as core::ffi::c_int)
            as uint8_t;
        Tm = ((*state)[i as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (*state)[i as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int)
            as uint8_t;
        Tm = xtime(Tm);
        (*state)[i as usize][0 as core::ffi::c_int as usize] = ((*state)[i
            as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (Tm as core::ffi::c_int ^ Tmp as core::ffi::c_int)) as uint8_t;
        Tm = ((*state)[i as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (*state)[i as usize][2 as core::ffi::c_int as usize] as core::ffi::c_int)
            as uint8_t;
        Tm = xtime(Tm);
        (*state)[i as usize][1 as core::ffi::c_int as usize] = ((*state)[i
            as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (Tm as core::ffi::c_int ^ Tmp as core::ffi::c_int)) as uint8_t;
        Tm = ((*state)[i as usize][2 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (*state)[i as usize][3 as core::ffi::c_int as usize] as core::ffi::c_int)
            as uint8_t;
        Tm = xtime(Tm);
        (*state)[i as usize][2 as core::ffi::c_int as usize] = ((*state)[i
            as usize][2 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (Tm as core::ffi::c_int ^ Tmp as core::ffi::c_int)) as uint8_t;
        Tm = ((*state)[i as usize][3 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ t as core::ffi::c_int) as uint8_t;
        Tm = xtime(Tm);
        (*state)[i as usize][3 as core::ffi::c_int as usize] = ((*state)[i
            as usize][3 as core::ffi::c_int as usize] as core::ffi::c_int
            ^ (Tm as core::ffi::c_int ^ Tmp as core::ffi::c_int)) as uint8_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn InvMixColumns(mut state: *mut state_t) {
    let mut i: core::ffi::c_int = 0;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        a = (*state)[i as usize][0 as core::ffi::c_int as usize];
        b = (*state)[i as usize][1 as core::ffi::c_int as usize];
        c = (*state)[i as usize][2 as core::ffi::c_int as usize];
        d = (*state)[i as usize][3 as core::ffi::c_int as usize];
        (*state)[i as usize][0 as core::ffi::c_int as usize] = ((0xe as core::ffi::c_int
            & 1 as core::ffi::c_int) * a as core::ffi::c_int
            ^ (0xe as core::ffi::c_int >> 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(a) as core::ffi::c_int
            ^ (0xe as core::ffi::c_int >> 2 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(a)) as core::ffi::c_int
            ^ (0xe as core::ffi::c_int >> 3 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(a))) as core::ffi::c_int
            ^ (0xe as core::ffi::c_int >> 4 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(xtime(a)))) as core::ffi::c_int
            ^ ((0xb as core::ffi::c_int & 1 as core::ffi::c_int) * b as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(b) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(b)) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(b))) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(b)))) as core::ffi::c_int)
            ^ ((0xd as core::ffi::c_int & 1 as core::ffi::c_int) * c as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(c) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(c)) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(c))) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(c)))) as core::ffi::c_int)
            ^ ((0x9 as core::ffi::c_int & 1 as core::ffi::c_int) * d as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(d) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(d)) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(d))) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(d)))) as core::ffi::c_int)) as uint8_t;
        (*state)[i as usize][1 as core::ffi::c_int as usize] = ((0x9 as core::ffi::c_int
            & 1 as core::ffi::c_int) * a as core::ffi::c_int
            ^ (0x9 as core::ffi::c_int >> 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(a) as core::ffi::c_int
            ^ (0x9 as core::ffi::c_int >> 2 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(a)) as core::ffi::c_int
            ^ (0x9 as core::ffi::c_int >> 3 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(a))) as core::ffi::c_int
            ^ (0x9 as core::ffi::c_int >> 4 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(xtime(a)))) as core::ffi::c_int
            ^ ((0xe as core::ffi::c_int & 1 as core::ffi::c_int) * b as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(b) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(b)) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(b))) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(b)))) as core::ffi::c_int)
            ^ ((0xb as core::ffi::c_int & 1 as core::ffi::c_int) * c as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(c) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(c)) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(c))) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(c)))) as core::ffi::c_int)
            ^ ((0xd as core::ffi::c_int & 1 as core::ffi::c_int) * d as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(d) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(d)) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(d))) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(d)))) as core::ffi::c_int)) as uint8_t;
        (*state)[i as usize][2 as core::ffi::c_int as usize] = ((0xd as core::ffi::c_int
            & 1 as core::ffi::c_int) * a as core::ffi::c_int
            ^ (0xd as core::ffi::c_int >> 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(a) as core::ffi::c_int
            ^ (0xd as core::ffi::c_int >> 2 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(a)) as core::ffi::c_int
            ^ (0xd as core::ffi::c_int >> 3 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(a))) as core::ffi::c_int
            ^ (0xd as core::ffi::c_int >> 4 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(xtime(a)))) as core::ffi::c_int
            ^ ((0x9 as core::ffi::c_int & 1 as core::ffi::c_int) * b as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(b) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(b)) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(b))) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(b)))) as core::ffi::c_int)
            ^ ((0xe as core::ffi::c_int & 1 as core::ffi::c_int) * c as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(c) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(c)) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(c))) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(c)))) as core::ffi::c_int)
            ^ ((0xb as core::ffi::c_int & 1 as core::ffi::c_int) * d as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(d) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(d)) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(d))) as core::ffi::c_int
                ^ (0xb as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(d)))) as core::ffi::c_int)) as uint8_t;
        (*state)[i as usize][3 as core::ffi::c_int as usize] = ((0xb as core::ffi::c_int
            & 1 as core::ffi::c_int) * a as core::ffi::c_int
            ^ (0xb as core::ffi::c_int >> 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(a) as core::ffi::c_int
            ^ (0xb as core::ffi::c_int >> 2 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(a)) as core::ffi::c_int
            ^ (0xb as core::ffi::c_int >> 3 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(a))) as core::ffi::c_int
            ^ (0xb as core::ffi::c_int >> 4 as core::ffi::c_int & 1 as core::ffi::c_int)
                * xtime(xtime(xtime(xtime(a)))) as core::ffi::c_int
            ^ ((0xd as core::ffi::c_int & 1 as core::ffi::c_int) * b as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(b) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(b)) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(b))) as core::ffi::c_int
                ^ (0xd as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(b)))) as core::ffi::c_int)
            ^ ((0x9 as core::ffi::c_int & 1 as core::ffi::c_int) * c as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(c) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(c)) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(c))) as core::ffi::c_int
                ^ (0x9 as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(c)))) as core::ffi::c_int)
            ^ ((0xe as core::ffi::c_int & 1 as core::ffi::c_int) * d as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 1 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(d) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 2 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(d)) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 3 as core::ffi::c_int
                    & 1 as core::ffi::c_int) * xtime(xtime(xtime(d))) as core::ffi::c_int
                ^ (0xe as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 1 as core::ffi::c_int)
                    * xtime(xtime(xtime(xtime(d)))) as core::ffi::c_int)) as uint8_t;
        i += 1;
    }
}
unsafe extern "C" fn InvSubBytes(mut state: *mut state_t) {
    let mut i: uint8_t = 0;
    let mut j: uint8_t = 0;
    i = 0 as uint8_t;
    while (i as core::ffi::c_int) < 4 as core::ffi::c_int {
        j = 0 as uint8_t;
        while (j as core::ffi::c_int) < 4 as core::ffi::c_int {
            (*state)[j as usize][i as usize] = rsbox[(*state)[j as usize][i as usize]
                as usize];
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn InvShiftRows(mut state: *mut state_t) {
    let state: &mut [u8; 16] = unsafe { &mut *(state as *mut [u8; 16]) };

    let temp = state[3];
    state[3] = state[2];
    state[2] = state[1];
    state[1] = state[0];
    state[0] = temp;

    let temp = state[7];
    state[7] = state[6];
    state[6] = temp;
    
    let temp = state[11];
    state[11] = state[10];
    state[10] = temp;

    let temp = state[15];
    state[15] = state[14];
    state[14] = state[13];
    state[13] = state[12];
    state[12] = temp;
}
unsafe extern "C" fn Cipher(mut state: *mut state_t, mut RoundKey: *const uint8_t) {
    let mut round: uint8_t = 0 as uint8_t;
    AddRoundKey(0 as uint8_t, state, RoundKey);
    round = 1 as uint8_t;
    loop {
        SubBytes(state);
        ShiftRows(state);
        if round as core::ffi::c_int == Nr {
            break;
        }
        MixColumns(state);
        AddRoundKey(round, state, RoundKey);
        round = round.wrapping_add(1);
    }
    AddRoundKey(Nr as uint8_t, state, RoundKey);
}
unsafe extern "C" fn InvCipher(mut state: *mut state_t, mut RoundKey: *const uint8_t) {
    let mut round: uint8_t = 0 as uint8_t;
    AddRoundKey(Nr as uint8_t, state, RoundKey);
    round = (Nr - 1 as core::ffi::c_int) as uint8_t;
    loop {
        InvShiftRows(state);
        InvSubBytes(state);
        AddRoundKey(round, state, RoundKey);
        if round as core::ffi::c_int == 0 as core::ffi::c_int {
            break;
        }
        InvMixColumns(state);
        round = round.wrapping_sub(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AES_ECB_encrypt(
    mut ctx: *const AES_ctx,
    mut buf: *mut uint8_t,
) {
    Cipher(buf as *mut state_t, ((*ctx).RoundKey).as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn AES_ECB_decrypt(
    mut ctx: *const AES_ctx,
    mut buf: *mut uint8_t,
) {

    let safe_ctx = unsafe { SafeAES_ctx::from_ptr(ctx) };
    InvCipher((buf as *mut [[u8; 4]; 4]), &safe_ctx.RoundKey[0]);
}
unsafe extern "C" fn XorWithIv(mut buf: *mut uint8_t, mut Iv: *const uint8_t) {
    let mut i: uint8_t = 0;
    i = 0 as uint8_t;
    while (i as core::ffi::c_int) < AES_BLOCKLEN {
        let ref mut fresh0 = *buf.offset(i as isize);
        *fresh0 = (*fresh0 as core::ffi::c_int
            ^ *Iv.offset(i as isize) as core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AES_CBC_encrypt_buffer(
    mut ctx: *mut AES_ctx,
    mut buf: *mut uint8_t,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    let mut Iv: *mut uint8_t = ((*ctx).Iv).as_mut_ptr();
    i = 0 as size_t;
    while i < length {
        XorWithIv(buf, Iv);
        Cipher(buf as *mut state_t, ((*ctx).RoundKey).as_mut_ptr());
        Iv = buf;
        buf = buf.offset(AES_BLOCKLEN as isize);
        i = i.wrapping_add(AES_BLOCKLEN as size_t);
    }
    memcpy(
        ((*ctx).Iv).as_mut_ptr() as *mut core::ffi::c_void,
        Iv as *const core::ffi::c_void,
        AES_BLOCKLEN as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn AES_CBC_decrypt_buffer(
    mut ctx: *mut AES_ctx,
    mut buf: *mut uint8_t,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    let mut storeNextIv: [uint8_t; 16] = [0; 16];
    i = 0 as size_t;
    while i < length {
        memcpy(
            storeNextIv.as_mut_ptr() as *mut core::ffi::c_void,
            buf as *const core::ffi::c_void,
            AES_BLOCKLEN as size_t,
        );
        InvCipher(buf as *mut state_t, ((*ctx).RoundKey).as_mut_ptr());
        XorWithIv(buf, ((*ctx).Iv).as_mut_ptr());
        memcpy(
            ((*ctx).Iv).as_mut_ptr() as *mut core::ffi::c_void,
            storeNextIv.as_mut_ptr() as *const core::ffi::c_void,
            AES_BLOCKLEN as size_t,
        );
        buf = buf.offset(AES_BLOCKLEN as isize);
        i = i.wrapping_add(AES_BLOCKLEN as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AES_CTR_xcrypt_buffer(
    mut ctx: *mut AES_ctx,
    mut buf: *mut uint8_t,
    mut length: size_t,
) {
    let mut buffer: [uint8_t; 16] = [0; 16];
    let mut i: size_t = 0;
    let mut bi: core::ffi::c_int = 0;
    i = 0 as size_t;
    bi = AES_BLOCKLEN;
    while i < length {
        if bi == AES_BLOCKLEN {
            memcpy(
                buffer.as_mut_ptr() as *mut core::ffi::c_void,
                ((*ctx).Iv).as_mut_ptr() as *const core::ffi::c_void,
                AES_BLOCKLEN as size_t,
            );
            Cipher(buffer.as_mut_ptr() as *mut state_t, ((*ctx).RoundKey).as_mut_ptr());
            bi = AES_BLOCKLEN - 1 as core::ffi::c_int;
            while bi >= 0 as core::ffi::c_int {
                if (*ctx).Iv[bi as usize] as core::ffi::c_int == 255 as core::ffi::c_int
                {
                    (*ctx).Iv[bi as usize] = 0 as uint8_t;
                    bi -= 1;
                } else {
                    (*ctx).Iv[bi as usize] = ((*ctx).Iv[bi as usize] as core::ffi::c_int
                        + 1 as core::ffi::c_int) as uint8_t;
                    break;
                }
            }
            bi = 0 as core::ffi::c_int;
        }
        *buf.offset(i as isize) = (*buf.offset(i as isize) as core::ffi::c_int
            ^ buffer[bi as usize] as core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
        bi += 1;
    }
}
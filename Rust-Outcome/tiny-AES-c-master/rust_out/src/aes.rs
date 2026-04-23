impl SafeAES_ctx  {
    pub unsafe fn from_ptr(ptr: *const AES_ctx) -> Option<Self>  {
        if ptr.is_null() {
            None
        } else {
            let ctx = &*ptr;
            Some(SafeAES_ctx {
                RoundKey: Vec::from(ctx.RoundKey),
                Iv: Vec::from(ctx.Iv),
            })
        }
    }
}





#[derive(Copy, Clone)]

 4]; 4];
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
pub fn key_expansion(round_key: &mut [u8; 240], key: &[u8; 32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut tempa = [0 as u8; 4];

    for i in 0..Nk {
        round_key[i * 4 + 0] = key[i * 4 + 0];
        round_key[i * 4 + 1] = key[i * 4 + 1];
        round_key[i * 4 + 2] = key[i * 4 + 2];
        round_key[i * 4 + 3] = key[i * 4 + 3];
    }
    
    for i in Nk..(Nb * (Nr + 1)) {
        k = (i - 1) * 4;
        
        tempa[0] = round_key[k as usize + 0];
        tempa[1] = round_key[k as usize + 1];
        tempa[2] = round_key[k as usize + 2];
        tempa[3] = round_key[k as usize + 3];
        
        if i % Nk == 0 {
            let u8tmp = tempa[0];
            
            for j in 0..3 {
                tempa[j] = tempa[j + 1];
            }
            
            tempa[3] = u8tmp;
            
            for j in 0..4 {
                tempa[j] = sbox[tempa[j] as usize];
            }
            
            tempa[0] ^= Rcon[(i / Nk) as usize];
        }
        
        #if defined(AES256) && (AES256 == 1)
        if i % Nk == 4 {
            for j in 0..4 {
                tempa[j] = sbox[tempa[j] as usize];
            }
        }
        #endif
        
        j = i * 4;
        k = (i - Nk) * 4;
        
        for j in 0..4 {
            round_key[j as usize + k as usize] ^= tempa[j as usize];
        }
    }
}
pub fn AES_init_ctx(mut ctx: Option<&mut SafeAES_ctx>, key: &[u8]) -> Result<(), ()>  {
    match SafeAES_ctx::from_ptr(ctx as *mut _) {
        Some(mut safe_ctx) => {
            KeyExpansion(&mut (safe_ctx.RoundKey)[..], key);
            Ok(())
        },
        None => Err(()),
    }
}
#[no_mangle]
pub unsafe extern "C" fn AES_init_ctx_iv(mut ctx: *mut AES_ctx, key: *const uint8_t, iv: *const uint8_t) {
    let safe_ctx = SafeAES_ctx::from_ptr(ctx).unwrap();
    KeyExpansion((*safe_ctx.RoundKey).as_mut_ptr(), key);
    memcpy(
        (*safe_ctx.Iv).as_mut_ptr() as *mut c_void, 
        iv as *const c_void, 
        AES_BLOCKLEN as size_t
    );
}
#[no_mangle]
pub unsafe extern "C" fn AES_ctx_set_iv(mut ctx_ptr: *mut AES_ctx, iv_ptr: *const uint8_t) {
    let safe_ctx = match SafeAES_ctx::from_ptr(ctx_ptr) {
        Some(safe_ctx) => safe_ctx,
        None => return,
    };
    
    let iv = std::slice::from_raw_parts(iv_ptr, AES_BLOCKLEN as usize);
    
    for i in 0..AES_BLOCKLEN {
        safe_ctx.Iv[i as usize] = unsafe { *iv.get_unchecked(i as usize) };
    }
}
pub fn add_round_key(mut round: u8, state: &mut [[u8; 4]; 4], round_key: &[u8]) {
    for i in 0..4 {
        for j in 0..4 {
            let idx = (round * NB * 4 + (i * NB) + j) as usize; // NB is assumed to be the size of a round key block
            state[i][j] ^= round_key.get(idx).unwrap_or(&0);
        }
    }
}
pub extern "C" fn SubBytes(state: *mut state_t) {
    if let Some(safe_state) = SafeState::from_ptr(state) {
        for i in 0..4 {
            for j in 0..4 {
                safe_state.data[j as usize][i as usize] = sbox[safe_state.data[j as usize][i as usize] as usize];
            }
        }
    }
}
pub fn shift_rows(mut state: *mut [[u8; 4]; 4]) {
    let mut temp = unsafe { (*state)[0][1] };
    (*state)[0][1] = (*state)[1][1];
    (*state)[1][1] = (*state)[2][1];
    (*state)[2][1] = (*state)[3][1];
    (*state)[3][1] = temp;
    
    let mut temp = unsafe { (*state)[0][2] };
    (*state)[0][2] = (*state)[2][2];
    (*state)[2][2] = temp;
    
    let mut temp = unsafe { (*state)[1][2] };
    (*state)[1][2] = (*state)[3][2];
    (*state)[3][2] = temp;
    
    let mut temp = unsafe { (*state)[0][3] };
    (*state)[0][3] = (*state)[3][3];
    (*state)[3][3] = (*state)[2][3];
    (*state)[2][3] = (*state)[1][3];
    (*state)[1][3] = temp;
}
fn xtime(x: u8) -> u8 {
    (x << 1) ^ ((x >> 7 & 1) * 0x1b)
}
pub extern "C" fn MixColumns(state: *mut state_t) {
    let mut state = unsafe { std::slice::from_raw_parts_mut((*state).as_mut(), 4) };

    for i in 0..4 {
        let t = state[i][0];
        let Tmp: u8 = (state[i][0] ^ state[i][1] ^ state[i][2] ^ state[i][3]) as u8;
        let mut Tm: u8 = (state[i][0] ^ state[i][1]) as u8;
        Tm = xtime(Tm);
        state[i][0] ^= Tm ^ Tmp;
        Tm = (state[i][1] ^ state[i][2]) as u8;
        Tm = xtime(Tm);
        state[i][1] ^= Tm ^ Tmp;
        Tm = (state[i][2] ^ state[i][3]) as u8;
        Tm = xtime(Tm);
        state[i][2] ^= Tm ^ Tmp;
        Tm = (state[i][3] ^ t) as u8;
        Tm = xtime(Tm);
        state[i][3] ^= Tm ^ Tmp;
    }
}
pub fn inv_mix_columns(mut state: &mut [u8; 4]) {
    let mut a = [0; 4];
    let mut b = [0; 4];
    for i in 0..4 {
        a[i] = state[i][0];
        b[i] = ((0xE * (a[i] as u32) << 1)) ^ ((a[i] as u32 >> 7) * 0x1B);
        state[i][0] = b[i] as u8;
    }

    for i in 0..4 {
        a[i] = state[i][1];
        b[i] = ((0xE * (a[i] as u32) << 1)) ^ ((a[i] as u32 >> 7) * 0x1B);
        state[i][1] = b[i] as u8;
    }

    for i in 0..4 {
        a[i] = state[i][2];
        b[i] = ((0xE * (a[i] as u32) << 1)) ^ ((a[i] as u32 >> 7) * 0x1B);
        state[i][2] = b[i] as u8;
    }

    for i in 0..4 {
        a[i] = state[i][3];
        b[i] = ((0xE * (a[i] as u32) << 1)) ^ ((a[i] as u32 >> 7) * 0x1B);
        state[i][3] = b[i] as u8;
    }
}
pub fn inv_sub_bytes(state: &mut SafeState) {
    for i in 0..4usize {
        for j in 0..4usize {
            state[j][i] = rsbox[state[j][i] as usize];
        }
    }
}
pub unsafe fn InvShiftRows(mut state: *mut [[MaybeUninit<u8>; 4]; 4]) {
    let mut temp = state[3][1].as_ptr() as *mut u8;
    
    swap(&mut *state[3][1], &mut *state[2][1]);
    swap(&mut *state[2][1], &mut *state[1][1]);
    swap(&mut *state[1][1], &mut *state[0][1]);
    
    let mut temp = state[0][2].as_ptr() as *mut u8;
    swap(&mut *state[0][2], &mut *state[2][2]);
    
    let mut temp = state[1][2].as_ptr() as *mut u8;
    swap(&mut *state[1][2], &mut *state[3][2]);
    
    let mut temp = state[0][3].as_ptr() as *mut u8;
    swap(&mut *state[0][3], &mut *state[1][3]);
    swap(&mut *state[1][3], &mut *state[2][3]);
    swap(&mut *state[2][3], &mut *state[3][3]);
}
pub fn process_user(u_ptr: *mut User) {
    // 1. ISOLATE UNSAFE BOUNDARY: Convert immediately using the bridge
    let safe_u = unsafe { SafeUser::from_ptr(u_ptr) };

    // 2. PURE SAFE LOGIC: No more raw pointers below this line!
    match safe_u {
        Some(safe_u) => {
            if safe_u.is_admin != 0 {
                println!("Admin state active.");
            }
        },
        None => {}
    }
}
impl SafeState {
    pub unsafe fn from_ptr(ptr: *const state_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            let state = &*ptr;
            Some(SafeState {
                data: Vec::from(*state),
            })
        }
    }
}

fn inv_cipher(mut state: &mut SafeState, mut round_key: &[u8]) {
    let nr = NR as u8;
    let mut round = 0;

    add_round_key(nr, &mut state.data, round_key);

    loop {
        round = if round == 0 { break };
        inv_shift_rows(&mut state.data);
        inv_sub_bytes(&mut state.data);
        add_round_key(round, &mut state.data, round_key);
        if round != 0 {
            inv_mix_columns(&mut state.data);
        }
        round -= 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn AES_ECB_encrypt(mut ctx: *const AES_ctx, mut buf: *mut uint8_t) {
    let safe_ctx = match SafeAES_ctx::from_ptr(ctx) {
        Some(safe_ctx) => safe_ctx,
        None => return,
    };
    
    Cipher(buf as *mut state_t, safe_ctx.RoundKey.as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn AES_ECB_decrypt(ctx: &mut SafeAES_ctx, buf: &mut [u8]) {
    let state = slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut _, 16);
    InvCipher(&mut state, ctx.RoundKey.as_slice());
}
pub fn xor_with_iv(buf: &mut [u8], iv: &[u8]) {
    for (i, v) in buf.iter_mut().zip(iv).enumerate() {
        *v ^= i as u8 % AES_BLOCKLEN;
    }
}
#[no_mangle]
pub unsafe extern "C" fn AES_CBC_encrypt_buffer(ctx: &mut SafeAES_ctx, buf: *mut u8, length: usize) {
    let mut iv = ctx.Iv.as_slice();
    
    for chunk in buf.to_bytes().chunks_exact_mut(AES_BLOCKLEN as usize) {
        XorWithIv(&mut *chunk, &iv);
        Cipher(chunk.cast::<state_t>(), ctx.RoundKey.as_slice());
        
        iv = chunk;
    }
    
    for (dst, src) in ctx.Iv.iter_mut().zip(iv) {
        *dst = *src;
    }
}
#[no_mangle]
pub fn AES_CBC_decrypt_buffer(ctx: &mut SafeAES_ctx, buf: &mut [u8], length: usize) {
    let mut storeNextIv = vec![0; 16];
    
    for i in (0..length).step_by(16) {
        storeNextIv.copy_from_slice(&buf[i..i+16]);
        
        InvCipher(&mut buf[i..i+16].chunks_exact_mut(4).collect::<Vec<&mut [u8; 4]>>(), &ctx.RoundKey);
        
        XorWithIv(&mut buf[i..i+16], &ctx.Iv);
        
        ctx.Iv.copy_from_slice(&storeNextIv);
    }
}
#[no_mangle]
pub fn AES_CTR_xcrypt_buffer(ctx: &mut SafeAES_ctx, buf: &mut Vec<u8>) -> std::result::Result<(), &'static str>  {
    let mut buffer = [0; 16];
    
    for (i, bi) in (0..buf.len()).zip((0..16).cycle()) {
        if bi == 0 {
            // Copying Iv to a local variable is safe because we have control over the size of SafeAES_ctx::Iv
            let iv = ctx.Iv.clone();
            
            // memcpy replaced with clone operation
            buffer = iv;
            
            // Assuming that Cipher function has been implemented and accepts a &mut state_t
            Cipher(&mut buffer, &ctx.RoundKey);

            // Updating the Iv (incrementing) without overflow is safe because we are controlling the value of i
            let increment = |x: u8| -> u8 { if x == 255 { 0 } else { x + 1 } };
            ctx.Iv = ctx.Iv.into_iter().map(increment).collect();
        }
        
        // XOR operation replaced with bitwise XOR operator ^
        buf[i] ^= buffer[bi];
    }
    
    Ok(())
}
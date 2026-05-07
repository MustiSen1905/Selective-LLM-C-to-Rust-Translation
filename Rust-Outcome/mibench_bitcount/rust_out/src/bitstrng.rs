#[no_mangle]
pub unsafe extern "C" fn bitstring(mut str: *mut core::ffi::c_char, mut byze: core::ffi::c_long, mut biz: core::ffi::c_int, mut strwid: core::ffi::c_int) {
    let mut i: core::ffi::c_int = 0;
    let zero = '0' as core::ffi::c_char as u8;
    let j: core::ffi::c_int = strwid - (biz + ((biz >> 2) as core::ffi::c_int)) 
        - if biz % 4 == 0 { 1 } else { 0 };

    while i < j {
        let fresh0 = str;
        *fresh0 = ' ' as u8 as core::ffi::c_char;
        str = str.offset(1);
        i += 1;
    }
    
    loop {
        biz -= 1;
        if !(biz >= 0) { break; }
        
        let fresh1 = str;
        *fresh1 = (((byze >> biz & 1) as u8 + zero) as core::ffi::c_char);
        str = str.offset(1);
        
        if biz % 4 == 0 && biz != 0 {
            let fresh2 = str;
            *fresh2 = ' ' as u8 as core::ffi::c_char;
            str = str.offset(1);
        }
    }
    
    *str = '\0' as u8 as core::ffi::c_char;
}
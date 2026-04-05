extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub username: [core::ffi::c_char; 16],
    pub isAdmin: core::ffi::c_int,
    pub session_token: *mut core::ffi::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn create_user(name: *const i8) -> *mut User {
    let mut u = Box::into_raw(Box::new(User{
        username: [0; 16],
        isAdmin: 0,
        session_token: std::ptr::null_mut(),
    })) as *mut User;
    
    strcpy((*u).username.as_mut_ptr(), name);
    
    (*u).isAdmin = 0;
    
    let init_session_token = b"INIT_TOKEN_ABC123\0".as_ptr() as *const i8;
    
    (*u).session_token = malloc(32) as  *mut i8;
    strcpy((*u).session_token, init_session_token);
    
    return u;
}
#[no_mangle]
pub unsafe extern "C" fn delete_user(mut u: *mut User) {
    if !u.is_null() {
        std::ptr::drop_in_place(Box::into_raw(Box::new(User {
            username: [0; 16],
            isAdmin: 0,
            session_token: core::ptr::null_mut(),
        })));
    }
}
use core::ffi::{CStr, c_char};
use std::os::raw::c_int;

#[no_mangle]
pub unsafe extern "C" fn elevate_privileges(mut u: *mut User, mut level: c_int) {
    (*u).isAdmin += level;
    printf(b"Privilegien auf %d gesetzt.\n\0".as_ptr() as *const c_char, (*u).isAdmin);
}

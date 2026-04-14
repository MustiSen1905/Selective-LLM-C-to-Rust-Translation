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
pub unsafe extern "C" fn create_user(mut name: *const core::ffi::c_char) -> *mut User {
    let mut u: *mut User = malloc(::core::mem::size_of::<User>() as size_t) as *mut User;
    strcpy(((*u).username).as_mut_ptr(), name);
    (*u).isAdmin = 0 as core::ffi::c_int;
    (*u).session_token = malloc(32 as size_t) as *mut core::ffi::c_char;
    strcpy(
        (*u).session_token,
        b"INIT_TOKEN_ABC123\0" as *const u8 as *const core::ffi::c_char,
    );
    return u;
}
#[no_mangle]
pub unsafe extern "C" fn delete_user(mut u: *mut User) {
    if !u.is_null() {
        free((*u).session_token as *mut std::os::raw::c_void);
        free(u as *mut std::os::raw::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn elevate_privileges(mut u: *mut User, level: i32) {
    (*u).isAdmin += level;
    println!("Privilegien auf {} gesetzt.", (*u).isAdmin);
}

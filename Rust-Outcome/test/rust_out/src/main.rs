extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn scanf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn create_user(name: *const core::ffi::c_char) -> *mut User;
    fn delete_user(u: *mut User);
    fn log_message(msg: *const core::ffi::c_char);
    fn read_log_unsafe(buffer: *mut core::ffi::c_char);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub username: [core::ffi::c_char; 16],
    pub isAdmin: core::ffi::c_int,
    pub session_token: *mut core::ffi::c_char,
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut input: [core::ffi::c_char; 128] = [0; 128];
    let mut log_buf: [core::ffi::c_char; 32] = [0; 32];
    printf(b"Neuen Usernamen eingeben: \0" as *const u8 as *const core::ffi::c_char);
    scanf(b"%s\0" as *const u8 as *const core::ffi::c_char, input.as_mut_ptr());
    let mut currentUser: *mut User = create_user(input.as_mut_ptr());
    log_message(input.as_mut_ptr());
    printf(
        b"Soll der User gel\xC3\xB6scht werden? (1=Ja): \0" as *const u8
            as *const core::ffi::c_char,
    );
    let mut choice: core::ffi::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const core::ffi::c_char,
        &mut choice as *mut core::ffi::c_int,
    );
    if choice == 1 as core::ffi::c_int {
        delete_user(currentUser);
    }
    printf(
        b"Session des Users %s ist noch aktiv.\n\0" as *const u8
            as *const core::ffi::c_char,
        ((*currentUser).username).as_mut_ptr(),
    );
    printf(b"Lese letzten Log-Eintrag...\n\0" as *const u8 as *const core::ffi::c_char);
    read_log_unsafe(log_buf.as_mut_ptr());
    printf(
        b"Log: %s\n\0" as *const u8 as *const core::ffi::c_char,
        log_buf.as_mut_ptr(),
    );
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

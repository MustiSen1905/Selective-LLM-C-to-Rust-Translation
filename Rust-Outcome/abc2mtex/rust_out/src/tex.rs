impl Safe_IO_FILE {
    #[allow(non_snake_case)]
    pub unsafe fn from_ptr(ptr: *const _IO_FILE) -> Self {
        if ptr.is_null() {
            return Self::default();
        }

        let f = &*ptr;

        Safe_IO_FILE {
            _flags: f._flags,
            _IO_read_ptr: std::ffi::CStr::from_ptr(f._IO_read_ptr).to_string_lossy().into_owned(),
            _IO_read_end: std::ffi::CStr::from_ptr(f._IO_read_end).to_string_lossy().into_owned(),
            _IO_read_base: std::ffi::CStr::from_ptr(f._IO_read_base).to_string_lossy().into_owned(),
            _IO_write_base: std::ffi::CStr::from_ptr(f._IO_write_base).to_string_lossy().into_owned(),
            _IO_write_ptr: std::ffi::CStr::from_ptr(f._IO_write_ptr).to_string_lossy().into_owned(),
            _IO_write_end: std::ffi::CStr::from_ptr(f._IO_write_end).to_string_lossy().into_owned(),
            _IO_buf_base: std::ffi::CStr::from_ptr(f._IO_buf_base).to_string_lossy().into_owned(),
            _IO_buf_end: std::ffi::CStr::from_ptr(f._IO_buf_end).to_string_lossy().into_owned(),
            _IO_save_base: std::ffi::CStr::from_ptr(f._IO_save_base).to_string_lossy().into_owned(),
            _IO_backup_base: std::ffi::CStr::from_ptr(f._IO_backup_base).to_string_lossy().into_owned(),
            _IO_save_end: std::ffi::CStr::from_ptr(f._IO_save_end).to_string_lossy().into_owned(),
            _markers: Box::new(Safe_IO_marker::from_unsafe(f._markers)),
            _chain: Box::new(Self::from_unsafe(&**f._chain)),
            _fileno: f._fileno,
            _flags2: f._flags2,
            _old_offset: f._old_offset,
            _cur_column: f._cur_column,
            _vtable_offset: f._vtable_offset,
            _shortbuf: std::ffi::CStr::from_ptr(f._shortbuf).to_string_lossy().into_owned(),
            // Lock should be treated as an opaque type here.
            _freeres_list: Box::new(Self::from_unsafe(&**f._freeres_list)),
            _mode: f._mode,
        }
    }
}
// Similar struct and impl blocks for the remaining C-style structs should follow...





#[derive(Copy, Clone)]



#[derive(Copy, Clone)]


#[derive(Copy, Clone)]

#[derive(Copy, Clone)]

#[derive(Copy, Clone)]

#[derive(Copy, Clone)]

#[derive(Copy, Clone)]

#[derive(Copy, Clone)]

#[derive(Copy, Clone)]

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub note: Note,
    pub bar: Barline,
    pub field: Field,
    pub misc: Misc,
}


pub const MISC: symbol_types = 4;
pub const FIELD: symbol_types = 3;
pub const NOTE: symbol_types = 2;
pub const BAR_LINE: symbol_types = 1;
pub const UNDETERMINED: symbol_types = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const GRACE: core::ffi::c_int = 1 as core::ffi::c_int;
#[no_mangle]
pub static mut Out: *mut FILE = 0 as *const FILE as *mut FILE;
static mut in_bar: core::ffi::c_int = 0 as core::ffi::c_int;
static mut in_notes: core::ffi::c_int = 0;
static mut hp: core::ffi::c_int = 0;
static mut new_tune: core::ffi::c_int = 0;
static mut musix: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn open_TeX(s: *const core::ffi::c_char, musix_out: core::ffi::c_int) {
    let safe_s = std::ffi::CStr::from_ptr(s);
    if safe_s.to_bytes()[0] == b'\0' {
        Out = fopen("music.tex", "w");
    } else {
        Out = fopen(safe_s, "w");
    }

    let musix = musix_out;

    if musix != 0 {
        fprintf(Out, r#"\def\abcmusix{Y}"#);
        if musix == 1 {
            fprintf(Out, r#"\def\abcopus{N}"#);
        }
        if musix == 2 {
            fprintf(Out, r#"\def\abcopus{Y}"#);
        }
    } else {
        fprintf(Out, r#"\def\abcmusix{N}"#);
    }
    
    if settings.mine != 0 {
        fprintf(Out, "\\input dscgrphy");
    }
    fprintf(Out, "\\input header\n%%");
    if musix == 1 {
        fprintf(Out, "\\startmuflex%%");
    }
}
#[no_mangle]
pub extern "C" fn draw_text(type_0: *mut core::ffi::c_char, string: *mut core::ffi::c_char) {
    let mut type_str = std::ffi::CStr::from_ptr(type_0).to_string_lossy();
    let mut string_str = std::ffi::CStr::from_ptr(string).to_string_lossy();
    
    if type_str.is_empty() {
        unsafe {
            fprintf(Out, format_args_ptr(format_args !("{}%\n", string_str)).as_c_void());
        }
    } else if type_str == "Z" {
        unsafe {
            fprintf(Out, format_args_ptr(format_args !("\\message{%}%\n", string_str)).as_c_void());
        }
    } else {
        for c in string_str.chars() {
            if c == '#' || c == '%' || c == '&' {
                let prev = unsafe { *string.offset(-1) };
                if prev as char != '\\' {
                    g_error(format_args_ptr(format_args!("unescaped special TeX character {} detected\nthis will cause TeX to choke", c)).as_c_void());
                }
            }
        }
        
        if !string_str.is_empty() {
            unsafe {
                fprintf(Out, format_args_ptr(format_args !("\\def\\{}{}true{{Y}}\\def\\{}{}string{{{}}}", type_str, type_str, type_str, string_str)).as_c_void());
            }
        } else {
            unsafe {
                fprintf(Out, format_args_ptr(format_args  !("\\def\\{}{}true{{N}}", type_str, type_str)).as_c_void());
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tune2tex(
    mut title: *mut [core::ffi::c_char; 99],
    mut titles: core::ffi::c_int,
    mut entry: *mut Record,
    mut n_symbols: core::ffi::c_int,
    mut symbols: *mut Symbol,
    mut key_field: *mut Field,
    mut meter_field: *mut Field,
    mut tempo_field: *mut Field,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    let mut other_titles: [core::ffi::c_char; 999] = [0; 999];
    let mut ttl: core::ffi::c_int = 0;
    new_tune = 1 as core::ffi::c_int;
    hp = 0 as core::ffi::c_int;
    draw_text(
        b"X\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[23 as core::ffi::c_int as usize],
    );
    draw_text(
        b"T\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*title.offset(0 as core::ffi::c_int as isize)).as_mut_ptr(),
    );
    draw_text(
        b"S\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[18 as core::ffi::c_int as usize],
    );
    draw_text(
        b"C\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[2 as core::ffi::c_int as usize],
    );
    draw_text(
        b"A\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[0 as core::ffi::c_int as usize],
    );
    draw_text(
        b"N\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[13 as core::ffi::c_int as usize],
    );
    other_titles[0 as core::ffi::c_int as usize] = '\0' as i32 as core::ffi::c_char;
    if titles > 1 as core::ffi::c_int {
        ttl = 1 as core::ffi::c_int;
        while ttl < titles {
            strcat(
                other_titles.as_mut_ptr(),
                (*title.offset(ttl as isize)).as_mut_ptr(),
            );
            if ttl < titles - 1 as core::ffi::c_int {
                strcat(
                    other_titles.as_mut_ptr(),
                    b"; \0" as *const u8 as *const core::ffi::c_char,
                );
            }
            ttl += 1;
        }
    }
    if titles < 6 as core::ffi::c_int {
        draw_text(
            b"Ta\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            other_titles.as_mut_ptr(),
        );
        draw_text(
            b"Tb\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        );
    } else {
        draw_text(
            b"Tb\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        );
        draw_text(
            b"Ta\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
            other_titles.as_mut_ptr(),
        );
    }
    draw_text(
        b"P\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
        (*entry).fields[15 as core::ffi::c_int as usize],
    );
    draw_header();
    key2tex(key_field);
    staves();
    draw_meter_new(meter_field);
    draw_size((*entry).fields[4 as core::ffi::c_int as usize]);
    if *((*entry).fields[16 as core::ffi::c_int as usize])
        .offset(0 as core::ffi::c_int as isize) != 0
    {
        draw_tempo(tempo_field);
    }
    in_bar = 1 as core::ffi::c_int;
    symbols = (*symbols).next as *mut Symbol;
    i = 1 as core::ffi::c_int;
    while i < n_symbols {
        match (*symbols).type_0 {
            2 => {
                n = (*symbols).u.note.n_notes;
                if (*symbols).u.note.pitch == 0 as core::ffi::c_int {
                    draw_usercmd(((*symbols).u.note.attributes).as_mut_ptr());
                } else if (*symbols).u.note.type_0 == GRACE {
                    if in_notes != 0 {
                        close_music();
                    }
                    beam2tex(n, symbols, -(1 as core::ffi::c_int));
                } else {
                    beam2tex(n, symbols, 0 as core::ffi::c_int);
                }
                j = 0 as core::ffi::c_int;
                while j < n {
                    symbols = (*symbols).next as *mut Symbol;
                    j += 1;
                }
                i += j;
            }
            1 => {
                bar2tex(symbols);
                i += 1 as core::ffi::c_int;
                symbols = (*symbols).next as *mut Symbol;
            }
            3 => {
                fields2tex(&mut (*symbols).u.field);
                i += 1 as core::ffi::c_int;
                symbols = (*symbols).next as *mut Symbol;
            }
            4 => {
                if (*symbols).u.misc.level == 2 as core::ffi::c_int {
                    close_open();
                } else if (*symbols).u.misc.level == 1 as core::ffi::c_int {
                    next_stave();
                } else {
                    g_error(b"\0" as *const u8 as *const core::ffi::c_char);
                }
                symbols = (*symbols).next as *mut Symbol;
                i += 1 as core::ffi::c_int;
            }
            _ => {
                g_error(b"\0" as *const u8 as *const core::ffi::c_char);
            }
        }
    }
    end_tune();
}
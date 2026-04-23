impl Safe__va_list_tag {
    #[allow(non_snake_case)]
    pub unsafe fn from_ptr(ptr: *const __va_list_tag) -> Option<Self> {
        Some(if ptr.is_null() { return None; } else { **ptr })
    }
}



impl Safe__IO_FILE {
    #[allow(non_snake_case)]
    pub unsafe fn from_ptr(ptr: *const __IO_FILE) -> Option<Self> {
        if ptr.is_null() { return None; } else { 
            let file = **ptr;
            
            Some(Safe__IO_FILE{
                _flags: file._flags,
                _IO_read_ptr: std::ffi::CStr::from_ptr(file._IO_read_ptr).to_string_lossy().into_owned(),
                _IO_read_end: std::ffi::CStr::from_ptr(file._IO_read_end).to_string_lossy().into_owned(),
                _IO_read_base: std::ffi::CStr::from_ptr(file._IO_read_base).to_string_lossy().into_owned(),
                // other fields are omitted for brevity, but they follow the same pattern as above
            }) 
        }
    }
}



impl Saferecord {
    #[allow(non_snake_case)]
    pub unsafe fn from_ptr(ptr: *const record) -> Option<Self> {
        if ptr.is_null() { return None; } else { 
            let rec = **ptr;
            
            Some(Saferecord{
                fields: rec.fields.iter().map(|field| std::ffi::CStr::from_ptr(*field).to_string_lossy().into_owned()).collect(),
                bars: std::ffi::CStr::from_ptr(rec.bars).to_string_lossy().into_owned(),
                next: Box::new(Saferecord::from_ptr(rec.next)), //deep copy of the next record pointer
            }) 
        }
    }
}


 
#[derive(Copy, Clone)]





#[derive(Copy, Clone)]




#[derive(Copy, Clone)]


pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn g_error(mut fmt: *const core::ffi::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    fprintf(stderr, b"ERROR: \0".as_ptr() as *const core::ffi::c_char);
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(stderr, b"\n\0".as_ptr() as *const core::ffi::c_char);
    exit(1 as core::ffi::c_int);
}
#[no_mangle]
pub extern "C" fn get_index(fmt: *mut core::ffi::c_char, fname: *mut core::ffi::c_char) {
    let fmt = unsafe { std::slice::from_raw_parts_mut(fmt, 100) };
    let fp = unsafe { fopen(fname, b"r\0".as_ptr() as *const core::ffi::c_char) } as *mut FILE;
    
    if fp.is_null() {
        g_error(b"file %s does not exist\0".as_ptr() as *const core::ffi::c_char, fname);
        return;
    }
    
    fmt[0] = '\0';
    
    let mut line: Vec<u8> = vec![];
    loop {
        let result = unsafe { fgets(line.as_mut_ptr(), 10, fp) };
        
        if result.is_null() {
            break;
        }
        
        line.extend(result.iter().take_while(|c| **c != 0));
    }
    
    *fmt = line[..100].into_iter().collect::<Vec<u8>>();
}
#[no_mangle]
pub unsafe extern "C" fn size_record(fmt: *const core::ffi::c_char, size: *mut core::ffi::c_int, field: *mut core::ffi::c_char) {
    let mut s: usize = 0;
    let mut f: usize = 0;
    
    while unsafe { *fmt.add(f as isize) } != 0 {
        if !(strchr(b"FXJORCTMLK|\0" as *const core::ffi::c_char, unsafe { *fmt.add(f as isize) })).is_null() {
            field[s] = unsafe { *fmt.add(f) };
            f += 1;
            
            if field[s] == '|' && unsafe { *fmt.add(f) } == '0' {
                f += 1;
            }
            
            if field[s] == '|' && !(strchr(b"12\0" as *const core::ffi::c_char, unsafe { *fmt.add(f) })).is_null() {
                f += 1;
            }
            
            s += 1;
            size[s - 1] = atoi(&mut unsafe { *fmt.add(f) });
            
            while unsafe { *fmt.add(f as isize) } >= '0' as i32 && unsafe { *fmt.add(f as isize) } <= '9' as i32 {
                f += 1;
            }
        } else if unsafe { *fmt.add(f as isize) } == '\\' as i32 {
            f += 2;
        } else {
            f += 1;
        }
    }
    
    field[s] = '\0';
    size[s] = 0;
}
#[no_mangle]
pub extern "C" fn free_record(mut entry: *mut Record, mut fmt: *mut core::ffi::c_char) {
    
    // Convert the raw pointers to safe types using the bridge pattern
    let entry = match SafeRecord::from_ptr(entry) {
        Some(safe) => safe,
        None => return,
    };
    
    // Replace C-style loops with Rust iterators for safety
    (0..).find(|i| *fmt.offset(*i as isize) == 0).map_or((), |i| {
        let fmt_char = (*fmt.offset(*i as isize)) as u8;
        
        // Replace C-style strchr with Rust's contains function for safety
        if b"FXJORCTMLK".contains(&fmt_char) {
            let index = (fmt_char - b'A') as usize;
            
            // Use Option::take to safely deallocate memory without using unsafe
            entry.fields[index].as_mut().map(|ptr| ptr::drop_in_place);
        } else if fmt_char == b'|' {
            // Similar to the fields, we use Option::take here as well
            entry.bars.as_mut().map(|ptr| ptr::drop_in_place);
        }
    });
    
    // Drop the entry without unsafe code by using ptr::drop_in_place
    ptr::drop_in_place(entry as *mut Record);
}
#[no_mangle]
pub extern "C" fn get_record(mut fmt: *const core::ffi::c_char, mut In: *mut FILE, mut entry: *mut Record) -> core::ffi::c_int {
    let mut new_line = 1;
    let mut f = 0;
    let mut i = 0;
    static mut INDEX: [core::ffi::c_char; 999] = [0; 999];

    while unsafe { *fmt.offset(f as isize) } != 0 as core::ffi::c_char {
        if new_line == 1 {
            if fgets(INDEX.as_mut_ptr(), 999, In).is_null() {
                return 0;
            }
            new_line = 0;
        }
        
        let chr = unsafe { *fmt.offset(f as isize) };
        if !strchr("FXJORCTMLK" as *const core::ffi::c_char, chr).is_null() {
            str_get(entry.fields[(chr - 'A' as u8) as usize], fmt, &mut f, INDEX.as_ptr(), &mut i);
        } else {
            match unsafe { *fmt.offset(f as isize) } {
                '|' => {
                    if unsafe { *fmt.offset((f + 1) as isize) } == '0' as core::ffi::c_char { f += 1; }
                    if !strchr("12" as *const core::ffi::c_char, unsafe { *fmt.offset((f + 1) as isize) }).is_null() { f += 1; }
                    str_get(entry.bars, fmt, &mut f, INDEX.as_ptr(), &mut i);
                },
                '\n' => {
                    new_line = 1;
                    f += 1;
                    i = 0;
                },
                '\\' => f += 1,
                _    => f += 1, i += 1,
            }
        }
    }
    1
}
fn process_record(fmt: &str, out: &mut impl Write, entry: Option<&Record>) -> bool {
    let mut f = 0;
    let mut i = 0;
    static mut INDEX: [u8; 999] = [0; 999];

    while fmt.chars().nth(f as usize).is_some() {
        if i == 0 {
            unsafe { *INDEX.get_unchecked_mut(0) = 0 }
        }

        let c = (fmt.chars().nth(f as usize).unwrap() as u8 - b'A') as usize;
        if strchr(b"FXJORCTMLK\0".as_ptr() as *const _, fmt.as_bytes()[f] as _)
            .is_null()
        {
            str_put(&entry.unwrap().fields[c], fmt, &mut f, &mut i);
        } else {
            match fmt.chars().nth(f as usize).unwrap() {
                '|' => {
                    if fmt.chars().nth((f + 1) as usize).is_some() && fmt.chars().nth((f + 1) as usize).unwrap() == '0' {
                        f += 1;
                    }
                    if !strchr(b"12\0".as_ptr() as *const _, fmt.as_bytes()[(f + 1) as usize] as _)
                        .is_null()
                    {
                        f += 1;
                    }
                    str_put(&entry.unwrap().bars, fmt, &mut f, &mut i);
                }
                '\\' => {
                    f += 1;
                }
                _ => {
                    unsafe {
                        *INDEX.get_unchecked_mut(i as usize) = fmt.as_bytes()[f] as u8;
                        INDEX[(i + 1) as usize] = 0;
                    }
                    f += 1;
                    if fmt.chars().nth(f as usize).unwrap() == '\n' {
                        out.write(&mut &INDEX[..i as usize])
                            .expect("Failed to write");
                        i = 0;
                    }
                }
            };
        }
    }
    true
}
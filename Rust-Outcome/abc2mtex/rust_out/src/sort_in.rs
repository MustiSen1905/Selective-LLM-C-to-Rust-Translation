impl Safe_IO_FILE {
    unsafe fn from_ptr(ptr: *const _IO_FILE) -> Self {
        if ptr.is_null() {
            panic!("Null pointer!");
        } else {
            let file = &*ptr;
            Safe_IO_FILE {
                _flags: file._flags,
                _IO_read_ptr: std::ffi::CStr::from_ptr(file._IO_read_ptr as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_read_end: std::ffi::CStr::from_ptr(file._IO_read_end as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_read_base: std::ffi::CStr::from_ptr(file._IO_read_base as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_write_base: std::ffi::CStr::from_ptr(file._IO_write_base as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_write_ptr: std::ffi::CStr::from_ptr(file._IO_write_ptr as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_write_end: std::ffi::CStr::from_ptr(file._IO_write_end as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_buf_base: std::ffi::CStr::from_ptr(file._IO_buf_base as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_buf_end: std::ffi::CStr::from_ptr(file._IO_buf_end as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_save_base: std::ffi::CStr::from_ptr(file._IO_save_base as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_backup_base: std::ffi::CStr::from_ptr(file._IO_backup_base as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _IO_save_end: std::ffi::CStr::from_ptr(file._IO_save_end as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _markers: std::ffi::CString::new(file._markers),
                _chain: Box::new(Safe_IO_FILE::from_ptr(file._chain)),
                _fileno: file._fileno,
                _flags2: file._flags2,
                _old_offset: file._old_offset as _,
                _cur_column: file._cur_column,
                _vtable_offset: file._vtable_offset,
                _shortbuf: std::ffi::CStr::from_ptr(file._shortbuf as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                _lock: std::ffi::CString::new(file._lock),
                _offset: file._offset,
                _codecvt: std::ffi::CString::new(file._codecvt),
                _wide_data: std::ffi::CString::new(file._wide_data),
                _freeres_list: Box::new(Safe_IO_FILE::from_ptr(file._freeres_list)),
                _freeres_buf: std::ffi::CString::new(file._freeres_buf),
                __pad5: file.__pad5,
                _mode: file._mode,
                _unused2: std::ffi::CStr::from_ptr(file._unused2 as *const i8)
                    .to_string_lossy()
                    .into_owned(),
            }
        }
    }
}


impl SafeRecord {
    unsafe fn from_ptr(ptr: *const record) -> Self {
        if ptr.is_null() {
            panic!("Null pointer!");
        } else {
            let rec = &*ptr;
            SafeRecord {
                fields: rec
                    .fields
                    .iter()
                    .map(|field| std::ffi::CStr::from_ptr(*field as *const i8)
                        .to_string_lossy()
                        .into_owned())
                    .collect(),
                bars: std::ffi::CStr::from_ptr(rec.bars as *const i8)
                    .to_string_lossy()
                    .into_owned(),
                next: if rec.next.is_null() { None } else { Some(Box::new(SafeRecord::from_ptr(rec.next))) },
            }
        }
    }
}





#[derive(Copy, Clone)]




#[derive(Copy, Clone)]



pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut priority: [core::ffi::c_char; 99] = [0; 99];
pub fn main_0(argc: i32, argv: *const *const u8) -> i32 {
    let mut format = [0u8; 999];
    let mut file = [0u8; 99];
    let mut fmt_file = [0u8; 99];
    let index: Box<Option<SafeIOFILE>> = unsafe { SafeIOFILE::from_ptr(std::ffi::c_void::null()) };
    let mut fields = [0u8; 12];
    let mut sizes = [0i32; 12];
    let mut entry: Box<Option<SafeRecord>> = unsafe { SafeRecord::from_ptr(std::ffi::c_void::null()) };
    let first_entry = entry.clone();
    let mut prev_entry = entry.clone();
    let mut next_entry: Box<Option<SafeRecord>> = unsafe { SafeRecord::from_ptr(std::ffi::c_void::null()) };
    let mut list = vec![0u8; 1];
    let mut i = 0i32;
    let mut n = 0i32;
    let mut arg = 1i32;
    
    strcpy(file.as_mut_ptr(), b"index\0".as_ptr() as *const u8);
    strcpy(priority.as_mut_ptr(), b"T\0".as_ptr() as *const u8);
    
    while arg < argc {
        if arg + 1 == argc {
            g_error(b"missing argument\0".as_ptr() as *const u8);
        }
        
        let argv_str = std::ffi::CStr::from_ptr(*argv.offset((arg - 1) as isize)).to_string_lossy().into_owned();
        if strcmp(argv_str.as_ptr() as *const u8, b"-f\0".as_ptr() as *const u8) == 0 {
            arg += 1;
            let next_arg = std::ffi::CStr::from_ptr(*argv.offset((arg - 1) as isize)).to_string_lossy().into_owned();
            strcpy(file.as_mut_ptr(), next_arg.as_ptr() as *const u8);
        } else if strcmp(argv_str.as_ptr() as *const u8, b"-p\0".as_ptr() as *const u8) == 0 {
            arg += 1;
            let next_arg = std::ffi::CStr::from_ptr(*argv.offset((arg - 1) as isize)).to_string_lossy().into_owned();
            strcpy(priority.as_mut_ptr(), next_arg.as_ptr() as *const u8);
        } else {
            g_error(b"unrecognised argument %s\0".as_ptr() as *const u8, argv_str.as_ptr() as *const u8);
        }
        
        arg += 1;
    }
    
    if strcmp(file.as_mut_ptr(), b"-\0".as_ptr() as *const u8) == 0 {
        index = unsafe { Box::new(Some(SafeIOFILE::from_ptr((index.unwrap().unwrap())._IO_read_base._file))) };
        strcpy(fmt_file.as_mut_ptr(), b"index\0".as_ptr() as *const u8);
    } else {
        let file_str = std::ffi::CStr::from_ptr(*argv).to_string_lossy().into_owned();
        index = unsafe { Box::new(Some(SafeIOFILE::from_ptr(fopen(file_str.as_ptr() as *const u8, b"r\0".as_ptr() as *const u8) as *mut _IO_FILE))) };
        
        if index.unwrap().is_none() {
            g_error(b"cannot open file %s\0".as_ptr() as *const u8, *argv);
        }
        
        strcpy(fmt_file.as_mut_ptr(), file.as_mut_ptr());
    }
    
    strcat(fmt_file.as_mut_ptr(), b".fmt\0".as_ptr() as *const u8);
    get_index(format.as_mut_ptr(), fmt_file.as_mut_ptr());
    size_record(format.as_mut_ptr(), sizes.as_mut_ptr(), fields.as_mut_ptr());
    
    entry = unsafe { Box::new(Some(SafeRecord::from_ptr(alloc_record(fields.as_mut_ptr(), sizes.as_mut_ptr())))) };
    let mut fresh0: *mut Record = std::slice::from_raw_parts_mut(&mut list[..1]).get(0).unwrap();
    **fresh0 = entry.unwrap().unwrap();
    
    while get_record(format.as_mut_ptr(), index.unwrap().unwrap()._IO_read_base._file, entry.unwrap().unwrap()) != 0 {
        prev_entry = entry;
        entry = unsafe { Box::new(Some(SafeRecord::from_ptr(alloc_record(fields.as_mut_ptr(), sizes.as_mut_ptr())))) };
        
        (**(*prev_entry).unwrap()).next = entry.clone();
        n += 1;
    }
    
    if strcmp(file.as_mut_ptr(), b"-\0".as_ptr() as *const u8) != 0 {
        fclose(index.unwrap().unwrap()._IO_read_base._file);
    }
    
    let mut fresh1: *mut Record = std::slice::from_raw_parts_mut(&mut list[..n as usize]).get(i as usize).unwrap();
    **fresh1 = (**(*first_entry).unwrap()).next.as_ref().unwrap();
    
    i = 0;
    
    while i < n {
        let mut fresh2: *mut Record = std::slice::from_raw_parts_mut(&mut list[..n as usize]).get(i as usize).unwrap();
        **fresh2 = (**(**(*fresh1).next.as_ref().unwrap())).unwrap();
        
        i += 1;
    }
    
    qsort(
        list.as_mut_ptr(),
        n as usize,
        core::mem::size_of::<*mut Record>(),
        |a, b| {
            if a == b || b.is_null() { 0 } else { compare(a, b) }
        },
    );
    
    if strcmp(file.as_mut_ptr(), b"-\0".as_ptr() as *const u8) == 0 {
        index = unsafe { Box::new(Some(SafeIOFILE::from_ptr(stdout))) };
    } else {
        let file_str = std::ffi::CStr::from_ptr(*argv).to_string_lossy().into_owned();
        index = unsafe { Box::new(Some(SafeIOFILE::from_ptr(fopen(file_str.as_ptr() as *const u8, b"w\0".as_ptr() as *const u8) as *mut _IO_FILE))) };
        
        if index.unwrap().is_none() {
            g_error(b"cannot open file %s\0".as_ptr() as *const u8, *argv);
        }
    }
    
    i = 0;
    
    while i < n {
        put_record(format.as_mut_ptr(), index.unwrap().unwrap()._IO_read_base._file, list[i as usize]);
        
        i += 1;
    }
    
    if strcmp(file.as_mut_ptr(), b"-\0".as_ptr() as *const u8) != 0 {
        fclose(index.unwrap().unwrap()._IO_read_base._file);
    }
    
    entry = first_entry;
    
    while let Some(_) = &*entry {
        next_entry = (**(*entry).unwrap()).next.as_ref();
        
        free_record(&mut *(**entry).unwrap(), fields.as_mut_ptr());
        entry = next_entry;
    }
    
    std::slice::from_raw_parts_mut(&mut list[..1]);
    0
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
    args.push(::core::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}
// --- auto-generated Safe stub types ---
pub type Safe__IO_codecvt = ();
pub type Safe__IO_marker = ();
pub type Safe__IO_wide_data = ();
// --- end stubs ---

#[derive(Debug, Clone)]
pub struct Safe_IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: String,
    pub _IO_read_end: String,
    pub _IO_read_base: String,
    pub _IO_write_base: String,
    pub _IO_write_ptr: String,
    pub _IO_write_end: String,
    pub _IO_buf_base: String,
    pub _IO_buf_end: String,
    pub _IO_save_base: String,
    pub _IO_backup_base: String,
    pub _IO_save_end: String,
    pub _markers: Box<Safe__IO_marker>,
    pub _chain: Option<Box<Safe_IO_FILE>>,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: isize,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: String,
    pub _lock: *mut core::ffi::c_void,
    pub _offset: i64,
    pub _codecvt: Box<Safe__IO_codecvt>,
    pub _wide_data: Box<Safe__IO_wide_data>,
    pub _freeres_list: Option<Box<Safe_IO_FILE>>,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: String,
}
impl Safe_IO_FILE { 
    pub unsafe fn from_ptr(ptr: *const _IO_FILE) -> Self { ... }
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn atol(__nptr: *const core::ffi::c_char) -> core::ffi::c_long;
    fn atoll(__nptr: *const core::ffi::c_char) -> core::ffi::c_longlong;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn strncpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
        __n: size_t,
    ) -> *mut core::ffi::c_char;
    fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strrchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strstr(
        __haystack: *const core::ffi::c_char,
        __needle: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strtok(
        __s: *mut core::ffi::c_char,
        __delim: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn strnlen(__string: *const core::ffi::c_char, __maxlen: size_t) -> size_t;
    fn __ctype_b_loc() -> *mut *const core::ffi::c_ushort;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn sprintf(
        __s: *mut core::ffi::c_char,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> core::ffi::c_int;
    fn fputc(__c: core::ffi::c_int, __stream: *mut FILE) -> core::ffi::c_int;
    fn fgets(
        __s: *mut core::ffi::c_char,
        __n: core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut core::ffi::c_char;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: core::ffi::c_long,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> core::ffi::c_long;
    fn rewind(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> core::ffi::c_int;
    fn safe_calloc(bytes: size_t) -> *mut core::ffi::c_void;
    fn new_creator(n_elements: *mut core::ffi::c_int) -> *mut pdf_creator_t;
    fn get_object_from_here(
        fp: *mut FILE,
        size: *mut size_t,
        is_stream: *mut core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn get_object(
        fp: *mut FILE,
        obj_id: core::ffi::c_int,
        xref: *const xref_t,
        size: *mut size_t,
        is_stream: *mut core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn get_type(
        fp: *mut FILE,
        obj_id: core::ffi::c_int,
        xref: *const xref_t,
    ) -> *const core::ffi::c_char;
    fn get_header(fp: *mut FILE) -> *mut core::ffi::c_char;
    fn decode_text_string(
        str: *const core::ffi::c_char,
        str_len: size_t,
    ) -> *mut core::ffi::c_char;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type pdf_flag_t = core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kv_t {
    pub key: [core::ffi::c_char; 32],
    pub value: [core::ffi::c_char; 128],
}
pub type kv_t = _kv_t;
pub type pdf_creator_t = kv_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xref_entry {
    pub obj_id: core::ffi::c_int,
    pub offset: core::ffi::c_long,
    pub gen_num: core::ffi::c_int,
    pub f_or_n: core::ffi::c_char,
}
pub type xref_entry_t = _xref_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xref_t {
    pub start: core::ffi::c_long,
    pub end: core::ffi::c_long,
    pub creator: *mut pdf_creator_t,
    pub n_creator_entries: core::ffi::c_int,
    pub n_entries: core::ffi::c_int,
    pub entries: *mut xref_entry_t,
    pub is_stream: core::ffi::c_int,
    pub is_linear: core::ffi::c_int,
    pub version: core::ffi::c_int,
}
pub type xref_t = _xref_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pdf_t {
    pub name: *mut core::ffi::c_char,
    pub pdf_major_version: core::ffi::c_short,
    pub pdf_minor_version: core::ffi::c_short,
    pub n_xrefs: core::ffi::c_int,
    pub xrefs: *mut xref_t,
    pub has_xref_streams: core::ffi::c_int,
}
pub type pdf_t = _pdf_t;
pub const EXIT_FAILURE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const EOF: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const SEEK_SET: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SEEK_CUR: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PDF_FLAG_QUIET: core::ffi::c_int = 1 as core::ffi::c_int;
pub const KV_MAX_VALUE_LENGTH: core::ffi::c_int = 128 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn pdf_delete(mut pdf: *mut pdf_t) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < (*pdf).n_xrefs {
        free((*((*pdf).xrefs).offset(i as isize)).creator as *mut core::ffi::c_void);
        free((*((*pdf).xrefs).offset(i as isize)).entries as *mut core::ffi::c_void);
        i += 1;
    }
    free((*pdf).name as *mut core::ffi::c_void);
    free((*pdf).xrefs as *mut core::ffi::c_void);
    free(pdf as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_is_pdf(mut fp: *mut FILE) -> core::ffi::c_int {
    let mut header = get_header(&fp);
    if header.is_null() {
        return 0 as core::ffi::c_int;
    }
    let c_str = b"%PDF-\0".cast();
    let mut c: *const core::ffi::c_char = strstr(header, c_str);
    let is_pdf: core::ffi::c_int = (!c.is_null() && (c as usize - header as usize + strlen(b"%PDF-M.m\0".cast())) < 1024) as core::ffi::c_int;
    free(header);
    return is_pdf;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_version(mut fp: *mut FILE, mut pdf: *mut pdf_t) {

    let header = get_header(fp);
    let c_str = std::ffi::CString::new(*header).unwrap();
    let c: Option<&str> = c_str.as_str().find("%PDF-");
    
    if let Some(c) = c {
        let mut s = c.chars().skip(6);
        let major = s.next().map_or('0', |x| x);
        let minor = s.nth(2).map_or('0', |x| x);
        
        if major.is_ascii_digit() && minor.is_ascii_digit() {
            (*pdf).pdf_major_version = major.to_string().parse::<i32>().unwrap();
            (*pdf).pdf_minor_version = minor.to_string().parse::<i32>().unwrap();
        }
    }
    
    drop(c_str);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_load_xrefs(
    mut fp: *mut FILE,
    mut pdf: *mut pdf_t,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut ver: core::ffi::c_int = 0;
    let mut is_linear: core::ffi::c_int = 0;
    let mut pos: core::ffi::c_long = 0;
    let mut pos_count: core::ffi::c_long = 0;
    let mut x: core::ffi::c_char = 0;
    let mut c: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut buf: [core::ffi::c_char; 256] = [0; 256];
    c = 0 as *mut core::ffi::c_char;
    (*pdf).n_xrefs = 0 as core::ffi::c_int;
    fseek(fp, 0 as core::ffi::c_long, SEEK_SET);
    while get_next_eof(fp) >= 0 as core::ffi::c_int {
        (*pdf).n_xrefs += 1;
    }
    if (*pdf).n_xrefs == 0 {
        return 0 as core::ffi::c_int;
    }
    fseek(fp, 0 as core::ffi::c_long, SEEK_SET);
    (*pdf).xrefs = safe_calloc(
        (::core::mem::size_of::<xref_t>() as size_t)
            .wrapping_mul((*pdf).n_xrefs as size_t),
    ) as *mut xref_t;
    ver = 1 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < (*pdf).n_xrefs {
        pos = get_next_eof(fp) as core::ffi::c_long;
        if pos < 0 as core::ffi::c_long {
            break;
        }
        let fresh0 = ver;
        ver = ver + 1;
        (*((*pdf).xrefs).offset(i as isize)).version = fresh0;
        pos_count = 0 as core::ffi::c_long;
        while ferror(fp) == 0 && feof(fp) == 0
            && {
                x = fgetc(fp) as core::ffi::c_char;
                x as core::ffi::c_int != 'f' as i32
            }
        {
            pos_count += 1;
            fseek(fp, pos - pos_count, SEEK_SET);
        }
        if pos_count as usize
            >= ::core::mem::size_of::<[core::ffi::c_char; 256]>() as usize
        {
            fprintf(
                stderr,
                b"[pdfresurrect] -- Error -- Failed to locate the startxref token. This might be a corrupt PDF.\n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            exit(EXIT_FAILURE);
        }
        memset(
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[core::ffi::c_char; 256]>() as size_t,
        );
        if fread(
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            1 as size_t,
            pos_count as size_t,
            fp,
        ) != pos_count as core::ffi::c_ulong
        {
            fprintf(
                stderr,
                b"[pdfresurrect] -- Error -- Failed to read startxref.\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            exit(EXIT_FAILURE);
        }
        c = buf.as_mut_ptr();
        while *c as core::ffi::c_int == ' ' as i32
            || *c as core::ffi::c_int == '\n' as i32
            || *c as core::ffi::c_int == '\r' as i32
        {
            c = c.offset(1);
        }
        (*((*pdf).xrefs).offset(i as isize)).start = atol(c);
        if (*((*pdf).xrefs).offset(i as isize)).start == 0 as core::ffi::c_long {
            get_xref_linear_skipped(fp, &mut *((*pdf).xrefs).offset(i as isize));
        } else {
            pos = ftell(fp);
            fseek(fp, (*((*pdf).xrefs).offset(i as isize)).start, SEEK_SET);
            (*((*pdf).xrefs).offset(i as isize)).end = get_next_eof(fp)
                as core::ffi::c_long;
            fseek(fp, pos, SEEK_SET);
        }
        if is_valid_xref(fp, pdf, &mut *((*pdf).xrefs).offset(i as isize)) == 0 {
            is_linear = (*((*pdf).xrefs).offset(i as isize)).is_linear;
            memset(
                &mut *((*pdf).xrefs).offset(i as isize) as *mut xref_t
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<xref_t>() as size_t,
            );
            (*((*pdf).xrefs).offset(i as isize)).is_linear = is_linear;
            rewind(fp);
            get_next_eof(fp);
        } else {
            load_xref_entries(fp, &mut *((*pdf).xrefs).offset(i as isize));
        }
        i += 1;
    }
    if (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize)).is_linear != 0 {
        resolve_linearized_pdf(pdf);
    }
    load_creator(fp, pdf);
    return (*pdf).n_xrefs;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_object_status(pdf: *const pdf_t, xref_idx: core::ffi::c_int, entry_idx: core::ffi::c_int) -> core::ffi::c_char {
    let curr = &*((*unsafe { &*pdf }).xrefs[xref_idx as usize].entries[entry_idx as usize]);
    let curr_ver = (*unsafe { &*pdf }).xrefs[xref_idx as usize].version;
    
    if curr_ver == 1 { return 'A' as core::ffi::c_char; }
    if curr.f_or_n == 'f' { return 'D' as core::ffi::c_char; }
    
    let mut prev_xref = std::ptr::null();
    for i in (0..xref_idx).rev() {
        if (*unsafe { &*pdf }).xrefs[i as usize].version < curr_ver {
            prev_xref = &(*unsafe { &*pdf }).xrefs[i as usize];
            break;
        }
    }
    
    if prev_xref.is_null() { return '?' as core::ffi::c_char; }
    
    let mut prev = std::ptr::null();
    for i in 0..prev_xref.n_entries {
        if (*unsafe { &*((*prev_xref).entries[i as usize]) }).obj_id == curr.obj_id {
            prev = &(*unsafe { &*((*prev_xref).entries[i as usize]) });
            break;
        }
    }
    
    if prev.is_null() || (prev.f_or_n == 'f' && curr.f_or_n == 'n') { return 'A' as core::ffi::c_char; }
    else if prev.offset != curr.offset { return 'M' as core::ffi::c_char; }
    
    '?' as core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn pdf_zero_object(
    mut fp: *mut FILE,
    mut pdf: *const pdf_t,
    mut xref_idx: core::ffi::c_int,
    mut entry_idx: core::ffi::c_int,
) {

    let mut i = 0;
    let entry = &(*pdf).xrefs[xref_idx].entries[entry_idx];
    let file = Safe_IO_FILE::from_ptr(fp as *mut _IO_FILE);
    fseek(&file, entry.offset as isize, SEEK_SET);
    
    // Get object and size 
    let mut obj = get_object(fp, entry.obj_id, &(*pdf).xrefs[xref_idx], None, None).unwrap();
    let mut obj_sz = 0;
    while strncmp(&obj[(i + 1)..], "endobj", 6) != 0 {
        i += 1;
        obj_sz = i;
    }
    
    if obj_sz > 0 {
        obj_sz += "endobj".len() + 1;
    }
    
    // Zero object 
    for _ in 0..obj_sz {
        fputc('0', fp);
    }
    
    eprintln!("Zeroed object {}", entry.obj_id);
    free(obj as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_summarize(
    mut fp: *mut FILE,
    mut pdf: *const pdf_t,
    mut name: *const core::ffi::c_char,
    mut flags: pdf_flag_t,
) {

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut page: usize = 0;
    let mut n_versions: isize = std::mem::transmute((*pdf).n_xrefs);
    let mut n_entries: usize = 0;
    let mut out: &mut Safe_IO_FILE = unsafe { Safe_IO_FILE::from_ptr(stdout) };
    
    if !(*name).is_null() {
        let dst_name = std::ffi::CStr::from_ptr(name).to_str().unwrap();
        let mut dst_path = format!("{}/{}.summary", name, name);
        let file: &mut Safe_IO_FILE = unsafe { Safe_IO_FILE::from_ptr(fopen(&dst_path[..], b"w\0")) };
        
        if file.is_null() {
            eprintln!("Could not open file '{}' for writing", &dst_path);
            return;
        }
        
        out = file;
    }
    
    n_versions -= if !(*pdf).has_xref_streams && (*pdf).n_xrefs > 0 { 1 } else { 0 };
    i = 1;
    
    while i < (*pdf).n_xrefs {
        let entries = unsafe { (*((*pdf).xrefs + (i as isize))).entries };
        
        if !((*((*pdf).xrefs + (i as isize))).is_linear) && 
            (*(entries + (j as isize))).obj_id == 0 {
                n_versions -= 1;
        }
            
        i += 1;
    }
    
    if (*pdf).n_xrefs == 0 || n_versions < 1 && 
         (*((*pdf).xrefs + (0 as isize))).is_linear {
            n_versions = 1;
        }
            
    i = 0;
        
    while !(*pdf).has_xref_streams && i < (*pdf).n_xrefs {
        let entries = unsafe { (*((*pdf).xrefs + (i as isize))).entries };
        let n_entries = (*((*pdf).xrefs + (i as isize))).n_entries;
            
        j = 0;
        
        while j < n_entries {
            n_entries += 1;
                
            fprintf::<usize>(out, 
                "{}: --{}-- Version {} -- Object {} ({})", 
                (*pdf).name, 
                pdf_get_object_status(pdf, i, j), 
                (*((*pdf).xrefs + (i as isize))).version, 
                (*(entries + (j as isize))).obj_id,
                get_type(fp, (*(entries + (j as isize))).obj_id, &mut *((*pdf).xrefs + (i as isize)))
            );
                
            if page != 0 {
                fprintf::<usize>(out, " Page({})", page);
            } else {
                fprintf::<()>(out, "\n");
            }
            
            j += 1;
        }
        
        i += 1;
    }
    
    if flags & PDF_FLAG_QUIET != 0 {
        if (*pdf).has_xref_streams || n_entries == 0 {
            fprintf::<()>(out, "{}: This PDF contains potential cross reference streams.\n{}: An object summary is not available.", (*pdf).name, (*pdf).name);
        }
        
        fprintf::<usize>(out, "---------- {} ----------\nVersions: {}", (*pdf).name, n_versions);
            
        if !(*pdf).has_xref_streams {
            i = 0;
                
            while i < (*pdf).n_xrefs {
                let xref = unsafe { *((*pdf).xrefs + (i as isize)) };
                    
                if !xref.is_linear && xref.n_entries > 0 {
                    n_entries = xref.n_entries;
                        
                    if (*((*pdf).xrefs + (0 as isize))).is_linear {
                        n_entries += (*((*pdf).xrefs + (0 as isize))).n_entries;
                    }
                        
                    if xref.version > 0 && n_entries > 0 {
                        fprintf::<usize, usize>(out, "Version {} -- {} objects", xref.version, n_entries);
                    }
                }
                    
                i += 1;
            }
        }
    } else {
        fprintf::<usize>(out, "{}: {}", (*pdf).name, n_versions);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_display_creator(
    mut pdf: *const pdf_t,
    mut xref_idx: core::ffi::c_int,
) -> core::ffi::c_int {

    let mut i = 0;
    if (*pdf).xrefs.get(xref_idx as usize).unwrap().creator.is_null() {
        return 0;
    }
    while i < (*pdf).xrefs.get(xref_idx as usize).unwrap().n_creator_entries {
        let key = std::ffi::CStr::from_ptr((*pdf)
            .xrefs
            .get(xref_idx as usize)
            .unwrap()
            .creator
            .offset(i as isize)
            .as_mut_ptr())
            .to_string_lossy();
        let value = std::ffi::CStr::from_ptr((*pdf)
            .xrefs
            .get(xref_idx as usize)
            .unwrap()
            .creator
            .offset(i as isize)
            .as_mut_ptr())
            .to_string_lossy();
        println!("{}: {}", key, value);
        i += 1;
    }
    return (i > 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_xref(
    mut fp: *mut FILE,
    mut pdf: *mut pdf_t,
    mut xref: *mut xref_t,
) -> core::ffi::c_int {

    let mut safe_fp = unsafe { Safe_IO_FILE::from_ptr(fp as _) };
    let buf: [i8; 16] = [0; 16];
    let start = fseek(&mut safe_fp, (*xref).start as isize, 0);
    if !fgets(buf.as_mut(), buf.len() as i32, &mut safe_fp).is_null() {
        exit(EXIT_FAILURE);
    }
    let mut is_valid = strncmp(buf.as_ptr() as *const u8, "xref\0".as_bytes().as_ptr(), 4) == 0;
    if !is_valid {
        fseek(&mut safe_fp, (*xref).start as isize, 0);
        let c = get_object_from_here(&mut safe_fp, None, &mut xref.is_stream);
        if c.is_some() && (*xref).is_stream != 0 {
            (*pdf).has_xref_streams = 1;
            is_valid = true;
        }
    }
    fseek(&mut safe_fp, start as _, 0);
    (is_valid as i32)
}
#[no_mangle]
pub unsafe extern "C" fn load_xref_entries(mut fp: *mut FILE, mut xref: *mut xref_t) {
    if (*xref).is_stream != 0 {
        load_xref_from_stream(fp, xref);
    } else {
        load_xref_from_plaintext(fp, xref);
    };
}
#[no_mangle]
pub unsafe extern "C" fn load_xref_from_plaintext(mut fp: *mut FILE, mut xref: *mut xref_t) {
    let mut ptr = fp;
 /* implementation */ }
#[no_mangle]
pub unsafe extern "C" fn load_xref_from_stream(mut fp: *mut FILE, mut xref: *mut xref_t) {
    let mut start: core::ffi::c_long = 0;
    let mut is_stream: core::ffi::c_int = 0;
    let mut stream: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut size: size_t = 0;
    start = ftell(fp);
    fseek(fp, (*xref).start, SEEK_SET);
    stream = 0 as *mut core::ffi::c_char;
    stream = get_object_from_here(fp, &mut size, &mut is_stream);
    fseek(fp, start, SEEK_SET);
    free(stream as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn get_xref_linear_skipped(mut fp: *mut FILE, mut xref: *mut xref_t) {
    let mut err: core::ffi::c_int = 0;
    let mut ch: core::ffi::c_char = 0;
    let mut buf: [core::ffi::c_char; 256] = [0; 256];
    if (*xref).start != 0 as core::ffi::c_long {
        return;
    }
    (*xref).is_linear = 1 as core::ffi::c_int;
    (*xref).end = get_next_eof(fp) as core::ffi::c_long;
    if (*xref).end < 0 as core::ffi::c_long {
        return;
    }
    err = 0 as core::ffi::c_int;
    loop {
        err = ferror(fp);
        if !(err == 0
            && fread(
                buf.as_mut_ptr() as *mut core::ffi::c_void,
                1 as size_t,
                8 as size_t,
                fp,
            ) != 0)
        {
            break;
        }
        if strncmp(
            buf.as_mut_ptr(),
            b"trailer\0" as *const u8 as *const core::ffi::c_char,
            strlen(b"trailer\0" as *const u8 as *const core::ffi::c_char),
        ) == 0 as core::ffi::c_int
        {
            break;
        }
        if (ftell(fp) - 9 as core::ffi::c_long) < 0 as core::ffi::c_long {
            return;
        }
        fseek(fp, -(9 as core::ffi::c_int) as core::ffi::c_long, SEEK_CUR);
    }
    if err != 0 {
        return;
    }
    ch = 0 as core::ffi::c_char;
    while ferror(fp) == 0 && feof(fp) == 0
        && {
            ch = fgetc(fp) as core::ffi::c_char;
            ch as core::ffi::c_int != 'x' as i32
        }
    {
        if fseek(fp, -(2 as core::ffi::c_int) as core::ffi::c_long, SEEK_CUR)
            == -(1 as core::ffi::c_int)
        {
            fprintf(
                stderr,
                b"[pdfresurrect] -- Error -- Failed to locate an xref.  This might be a corrupt PDF.\n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            exit(EXIT_FAILURE);
        }
    }
    if ch as core::ffi::c_int == 'x' as i32 {
        (*xref).start = ftell(fp) - 1 as core::ffi::c_long;
        fseek(fp, -(1 as core::ffi::c_int) as core::ffi::c_long, SEEK_CUR);
    }
    fseek(fp, (*xref).start, SEEK_SET);
}
#[no_mangle]
pub unsafe extern "C" fn resolve_linearized_pdf(mut pdf: *mut pdf_t) {

    let mut i: usize = 0;
    let mut buf = unsafe { (*(*pdf).xrefs[0]).clone() };
    
    if (*pdf).n_xrefs < 2 {
        return;
    }
    
    if !unsafe { *(**(*pdf).xrefs).get(0) }.is_linear {
        return;
    }
    
    buf = unsafe { **(*pdf).xrefs[0] };
    unsafe { 
        **(*pdf).xrefs.get_unchecked_mut(0) = *(*pdf).xrefs.get_unchecked(1);
        **(*pdf).xrefs.get_unchecked_mut(1) = buf;
     }
    
    unsafe { 
        (*(*(*pdf).xrefs).get(0)).is_linear = 1;
        (*(*(*pdf).xrefs).get(0)).version = 1;
        (*(*(*pdf).xrefs).get(1)).is_linear = 0;
        (*(*(*pdf).xrefs).get(1)).version = 1;
    }
    
    i = 2;
    while i < (*pdf).n_xrefs {
        unsafe { 
            (*(*(*pdf).xrefs).get(i)).version -= 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn load_creator(mut fp: *mut FILE, mut pdf: *mut pdf_t) {

    let mut start = ftell(fp);

    // For each PDF version
    for xref in (*pdf).xrefs.iter_mut() {
        if xref.version == 0 {
            continue;
        }
        
        // Find trailer
        fseek(fp, xref.start, SEEK_SET);
        while feof(fp) == 0 && fgetc(fp) != 't' as i32 {}

        let mut c = '\0';
        // Look for "<< ........ /Info ....."
        while feof(fp) == 0 && (c != '>' as u8 || c == '/' as u8) {
            if c == '/' as i32 && fgetc(fp) == 'I' as i32 && fgetc(fp) == 'n' as i32 {
                break;
            }
        }
        
        // Rest of the code...
    }
    
    fseek(fp, start, SEEK_SET);
}
#[no_mangle]
pub unsafe extern "C" fn load_creator_from_buf(
    mut fp: *mut FILE,
    mut xref: *mut xref_t,
    mut buf: *const core::ffi::c_char,
    mut buf_size: size_t,
) {
    let mut is_xml: core::ffi::c_int = 0;
    let mut c: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    if buf.is_null() {
        return;
    }
    c = strstr(buf, b"/Type\0" as *const u8 as *const core::ffi::c_char);
    if !c.is_null() {
        while *c as core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*c as core::ffi::c_int as isize)
                as core::ffi::c_int
                & _ISspace as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
                == 0
        {
            c = c.offset(1);
        }
    }
    is_xml = 0 as core::ffi::c_int;
    if !c.is_null() && *c as core::ffi::c_int == 'M' as i32 {
        is_xml = 1 as core::ffi::c_int;
    }
    if is_xml != 0 {
        load_creator_from_xml(xref, buf);
    } else {
        load_creator_from_old_format(fp, xref, buf, buf_size);
    };
}
#[no_mangle]
pub unsafe extern "C" fn load_creator_from_xml(
    mut xref: *mut xref_t,
    mut buf: *const core::ffi::c_char,
) {}
#[no_mangle]
pub unsafe extern "C" fn load_creator_from_old_format(
    mut fp: *mut FILE,
    mut xref: *mut xref_t,
    mut buf: *const core::ffi::c_char,
    mut buf_size: size_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut n_eles: core::ffi::c_int = 0;
    let mut length: core::ffi::c_int = 0;
    let mut is_escaped: core::ffi::c_int = 0;
    let mut obj_id: core::ffi::c_int = 0;
    let mut c: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut ascii: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut start: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut s: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut saved_buf_search: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut obj: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut obj_size: size_t = 0;
    let mut info: *mut pdf_creator_t = 0 as *mut pdf_creator_t;
    info = new_creator(&mut n_eles);
    if buf_size < 1 as size_t {
        return;
    }
    let mut buf_end: *const core::ffi::c_char = buf
        .offset(buf_size as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut end: *const core::ffi::c_char = buf_end;
    i = 0 as core::ffi::c_int;
    while i < n_eles {
        c = strstr(buf, ((*info.offset(i as isize)).key).as_mut_ptr());
        if !c.is_null() {
            c = c.offset(strlen(((*info.offset(i as isize)).key).as_mut_ptr()) as isize);
            while *(*__ctype_b_loc()).offset(*c as core::ffi::c_int as isize)
                as core::ffi::c_int
                & _ISspace as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
                != 0
            {
                c = c.offset(1);
            }
            if c >= buf_end as *mut core::ffi::c_char {
                fprintf(
                    stderr,
                    b"[pdfresurrect] -- Error -- Failed to locate space, likely a corrupt PDF.\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                exit(EXIT_FAILURE);
            }
            if !(*c as core::ffi::c_int == '/' as i32) {
                saved_buf_search = 0 as *mut core::ffi::c_char;
                obj = saved_buf_search;
                obj_size = 0 as size_t;
                end = buf_end;
                if *(*__ctype_b_loc()).offset(*c as core::ffi::c_int as isize)
                    as core::ffi::c_int
                    & _ISdigit as core::ffi::c_int as core::ffi::c_ushort
                        as core::ffi::c_int != 0
                {
                    obj_id = atoi(c);
                    saved_buf_search = c;
                    s = saved_buf_search;
                    obj = get_object(
                        fp,
                        obj_id,
                        xref,
                        &mut obj_size,
                        0 as *mut core::ffi::c_int,
                    );
                    end = obj.offset(obj_size as isize);
                    c = obj;
                    while !c.is_null() && *c as core::ffi::c_int != '(' as i32
                        && c < end as *mut core::ffi::c_char
                    {
                        c = c.offset(1);
                    }
                    if c >= end as *mut core::ffi::c_char {
                        fprintf(
                            stderr,
                            b"[pdfresurrect] -- Error -- Failed to locate a '(' character. This might be a corrupt PDF.\n\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                        exit(EXIT_FAILURE);
                    }
                    while !s.is_null() && *s as core::ffi::c_int == '/' as i32
                        && s < buf_end as *mut core::ffi::c_char
                    {
                        s = s.offset(1);
                    }
                    if s >= buf_end as *mut core::ffi::c_char {
                        fprintf(
                            stderr,
                            b"[pdfresurrect] -- Error -- Failed to locate a '/' character. This might be a corrupt PDF.\n\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                        exit(EXIT_FAILURE);
                    }
                    saved_buf_search = s;
                }
                start = c;
                is_escaped = 0 as core::ffi::c_int;
                length = is_escaped;
                while !c.is_null()
                    && (*c as core::ffi::c_int != '\r' as i32
                        && *c as core::ffi::c_int != '\n' as i32
                        && *c as core::ffi::c_int != '<' as i32)
                {
                    if is_escaped == 0 && *c as core::ffi::c_int == ')' as i32 {
                        break;
                    }
                    if *c as core::ffi::c_int == '\\' as i32 {
                        is_escaped = 1 as core::ffi::c_int;
                    } else {
                        is_escaped = 0 as core::ffi::c_int;
                    }
                    c = c.offset(1);
                    length += 1;
                    if c > end as *mut core::ffi::c_char {
                        fprintf(
                            stderr,
                            b"[pdfresurrect] -- Error -- Failed to locate the end of a value. This might be a corrupt PDF.\n\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                        exit(EXIT_FAILURE);
                    }
                }
                if !(length == 0 as core::ffi::c_int) {
                    if length != 0 {
                        length += 1 as core::ffi::c_int;
                    }
                    length = if length > KV_MAX_VALUE_LENGTH {
                        KV_MAX_VALUE_LENGTH
                    } else {
                        length
                    };
                    strncpy(
                        ((*info.offset(i as isize)).value).as_mut_ptr(),
                        start,
                        length as size_t,
                    );
                    (*info.offset(i as isize))
                        .value[(KV_MAX_VALUE_LENGTH - 1 as core::ffi::c_int) as usize] = '\0'
                        as i32 as core::ffi::c_char;
                    if !saved_buf_search.is_null() {
                        free(obj as *mut core::ffi::c_void);
                        c = saved_buf_search;
                    }
                }
            }
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < n_eles {
        let val_str_len: size_t = strnlen(
            ((*info.offset(i as isize)).value).as_mut_ptr(),
            KV_MAX_VALUE_LENGTH as size_t,
        ) as size_t;
        ascii = decode_text_string(
            ((*info.offset(i as isize)).value).as_mut_ptr(),
            val_str_len,
        );
        if !ascii.is_null() {
            strncpy(((*info.offset(i as isize)).value).as_mut_ptr(), ascii, val_str_len);
            free(ascii as *mut core::ffi::c_void);
        }
        i += 1;
    }
    (*xref).creator = info;
    (*xref).n_creator_entries = n_eles;
}
#[no_mangle]
pub unsafe extern "C" fn get_next_eof(mut fp: *mut FILE) -> core::ffi::c_int {
    let mut match_0: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let buf: [core::ffi::c_char; 6] = ::core::mem::transmute::<
        [u8; 6],
        [core::ffi::c_char; 6],
    >(*b"%%EOF\0");
    match_0 = 0 as core::ffi::c_int;
    loop {
        c = fgetc(fp);
        if !(c != EOF) {
            break;
        }
        if c == buf[match_0 as usize] as core::ffi::c_int {
            match_0 += 1;
        } else {
            match_0 = 0 as core::ffi::c_int;
        }
        if match_0 == 5 as core::ffi::c_int {
            return (ftell(fp) - 5 as core::ffi::c_long) as core::ffi::c_int;
        }
    }
    return -(1 as core::ffi::c_int);
}

// --- auto-generated manual Default impls (raw-pointer structs) ---
impl Default for Safe_IO_FILE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

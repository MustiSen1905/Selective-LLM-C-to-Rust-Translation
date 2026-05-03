// --- auto-generated Safe stub types ---
pub type SafeIoCodecvt = ();
pub type SafeIoMarker = ();
pub type SafeIoWideData = ();
// --- end stubs ---

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SafeIoFile {
    pub flags: i32,
    pub io_read_ptr: String,
    pub io_read_end: String,
    pub io_read_base: String,
    pub io_write_base: String,
    pub io_write_ptr: String,
    pub io_write_end: String,
    pub io_buf_base: String,
    pub io_buf_end: String,
    pub io_save_base: String,
    pub io_backup_base: String,
    pub io_save_end: String,
    pub markers: Vec<SafeIoMarker>,
    pub chain: Box<SafeIoFile>,
    pub fileno: i32,
    pub flags2: i32,
    pub old_offset: i64,
    pub cur_column: u16,
    pub vtable_offset: i8,
    pub shortbuf: String,
    pub lock: Box<()>,
    pub offset: i64,
    pub codecvt: Box<SafeIoCodecvt>,
    pub wide_data: Box<SafeIoWideData>,
    pub freeres_list: Box<SafeIoFile>,
    pub freeres_buf: Box<()>,
    pub pad5: usize,
    pub mode: i32,
    pub unused2: String,
}
impl SafeIoFile {
    pub unsafe fn from_ptr(ptr: *const _IO_FILE) -> Self {
        if ptr.is_null() { return Default::default(); }
        let orig = &*ptr;
        SafeIoFile {
            flags: orig.flags,
            io_read_ptr: std::ffi::CStr::from_ptr(orig._IO_read_ptr)
                .to_string_lossy().into_owned(),
            // ... similar for other fields
            markers: (0..orig.n_markers).map(|_| SafeIoMarker::from_ptr(null())).collect(),
            // and so on...
        }
    }
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
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
    fn snprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: core::ffi::c_long,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> core::ffi::c_long;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strrchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strstr(
        __haystack: *const core::ffi::c_char,
        __needle: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn closedir(__dirp: *mut DIR) -> core::ffi::c_int;
    fn opendir(__name: *const core::ffi::c_char) -> *mut DIR;
    fn mkdir(__path: *const core::ffi::c_char, __mode: __mode_t) -> core::ffi::c_int;
    fn safe_calloc(bytes: size_t) -> *mut core::ffi::c_void;
    fn pdf_delete(pdf: *mut pdf_t);
    fn pdf_is_pdf(fp: *mut FILE) -> core::ffi::c_int;
    fn pdf_summarize(
        fp: *mut FILE,
        pdf: *const pdf_t,
        name: *const core::ffi::c_char,
        flags: pdf_flag_t,
    );
    fn pdf_display_creator(
        pdf: *const pdf_t,
        xref_idx: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn init_pdf(fp: *mut FILE, name: *const core::ffi::c_char) -> *mut pdf_t;
}
pub type size_t = usize;
pub type __mode_t = core::ffi::c_uint;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
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
pub type DIR = __dirstream;
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
pub const SEEK_SET: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const __S_IREAD: core::ffi::c_int = 0o400 as core::ffi::c_int;
pub const __S_IWRITE: core::ffi::c_int = 0o200 as core::ffi::c_int;
pub const __S_IEXEC: core::ffi::c_int = 0o100 as core::ffi::c_int;
pub const S_IRWXU: core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const PDF_FLAG_QUIET: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PDF_FLAG_DISP_CREATOR: core::ffi::c_int = 2 as core::ffi::c_int;
#[no_mangle]
pub extern "C" fn usage() {
    let message = b"-- pdfresurrect v0.24b --\nUsage: ./pdfresurrect <file.pdf> [-i] [-w] [-q]\n\t  -i Display PDF creator information\n\t  -w Write the PDF versions and summary to disk\n\t  -q Display only the number of versions contained in the PDF\n\0".to_vec();
    let message = unsafe { std::ffi::CStr::from_bytes_with_nul(&message).to_str().unwrap() };
    
    printf(message.as_ptr() as *const core::ffi::c_char);
    exit(0 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn write_version(
    mut fp: *mut FILE,
    mut fname: *const core::ffi::c_char,
    mut dirname: *const core::ffi::c_char,
    mut xref: *mut xref_t,
) {
    let fname = unsafe { &*fname };
    let dirname = unsafe { &*dirname };

    let start = std::mem::transmute(ftell(&**(*fp).as_ptr()));
    let c: *mut core::ffi::c_char = 0 as _;
    let new_fname: *mut core::ffi::c_char = 0 as _;
    let data: core::ffi::c_char = 0;
    let new_fp: *mut FILE = 0 as _;
    
    if let Some(position) = fname.find(".pdf") {
        c = std::mem::transmute(&fname[position..]);
        **c = '\0' as i8;
    }

    new_fname = safe_calloc((dirname.len() + fname.len() + 32) as size_t);
    
    snprintf(new_fname, (dirname.len() + fname.len() + 32) as size_t, format!("{}/{}", dirname, fname).as_ptr(), (*xref).version);
    
    new_fp = std::mem::transmute(fopen(new_fname, ".w\0" as _));
    
    if new_fp.is_null() {
        fprintf(stderr, "[pdfresurrect] – Error – Could not create file '%s'\n", new_fname);
        std::mem::transmute(fseek(&**(*fp).as_ptr(), start, SEEK_SET));
        free(new_fname as _);
        return;
    }
    
    fseek(&**(*fp).as_ptr(), 0, SEEK_SET);
    
    while unsafe { fread(&mut data as _, 1, 1, &**(*fp).as_ptr()) != 0 } {
        unsafe { fwrite(&data as _, 1, 1, new_fp) };
    }
    
    fprintf(new_fp, "\r\nstartxref\r\n%ld\r\n%%%%EOF", (*xref).start);
    
    fclose(new_fp);
    
    free(new_fname as _);
    
    std::mem::transmute(fseek(&**(*fp).as_ptr(), start, SEEK_SET));
}
#[no_mangle]
pub unsafe extern "C" fn display_creator(mut fp: *mut FILE, mut pdf: *const pdf_t) {

    let mut i = 0;
    printf(b"PDF Version: %d.%d\n\0" as _, (*pdf).pdf_major_version as i32, (*pdf).pdf_minor_version as i32);
    while i < (*pdf).n_xrefs {
        let xref = (*pdf).xrefs.get(i as usize).unwrap();
        if xref.is_admin() {
            if unsafe { pdf_display_creator(pdf, i) } != 0 {
                printf(b"\n\0" as _);
            }
        }
        i += 1;
    }
}
unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut args = std::slice::from_raw_parts(argv, (argc as usize));

    if argc < 2 {
        usage();
        return -1;
    }

    let mut do_write = 0;
    let mut do_scrub = 0;
    let mut name: Option<&str> = None;
    let mut flags = 0 as u16;

    for arg in &args[1..] {
        match std::ffi::CStr::from_ptr(*arg).to_string_lossy().into_owned()as &str {
            "-w" => do_write = 1,
            "-i" => flags |= PDF_FLAG_DISP_CREATOR,
            "-q" => flags |= PDF_FLAG_QUIET,
            #[cfg(feature = "experimental")]
            "-s" => do_scrub = 1,
            x if !x.starts_with('-') => name = Some(x),
            _ => usage(),
        }
    }

    let name = name.unwrap_or_else(|| {
        usage();
        return -1;
    });

    let fp = std::fs::File::open(&name).ok().map_err(|e| {
        eprintln!("Could not open file '{}': {}", name, e);
        return -1 as i32;
    })?;

    if !pdf_is_pdf(&fp) {
        eprintln!("'{}' specified is not a valid PDF", name);
        return -1;
    }

    let pdf = init_pdf(fp, &name).ok().map_err(|e| {
        eprintln!("Failed to initialize pdf: {}", e);
        return -1 as i32;
    })?;

    let n_valid = pdf.xrefs.iter().filter(|xref| xref.version > 0).count() as i32;

    if n_valid < 2 {
        if flags & (PDF_FLAG_QUIET | PDF_FLAG_DISP_CREATOR) == 0 {
            println!("{}: There is only one version of this PDF", pdf.name);
        }
        if do_write != 0 {
            return 0;
        }
    }

    let dname = if do_write != 0 {
        Some(format![ "{}-versions", &pdf.name[..pdf.name.find('/').unwrap_or(pdf.name.len())]])
    } else {
        None
    };

    if let Some(dname) = dname {
        match std::fs::read_dir(&dname).ok().map_err(|e| {
            eprintln!("Failed to create directory '{}': {}", &dname, e);
            return -1 as i32;
        }) {
            Err(_) => (),
            Ok(_) => {
                eprintln!("This directory already exists, PDF version extraction will not occur.");
                return -1;
            }
        };

        for xref in &pdf.xrefs {
            if xref.version > 0 {
                write_version(&fp, &name, &dname, xref).ok().map_err(|e| {
                    eprintln!("Failed to write version: {}", e);
                    return -1 as i32;
                })?;
            }
        };
    }

    pdf_summarize(&fp, &pdf, dname.as_deref(), flags).ok().map_err(|e| {
        eprintln!("Failed to summarize pdf: {}", e);
        return -1 as i32;
    })?;

    if flags & PDF_FLAG_DISP_CREATOR != 0 {
        display_creator(&fp, &pdf).ok().map_err(|e| {
            eprintln!("Failed to display creator: {}", e);
            return -1 as i32;
        })?;
    }

    Ok(0)
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
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}
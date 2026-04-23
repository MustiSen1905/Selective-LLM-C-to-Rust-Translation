impl SafePdf {
    pub unsafe fn from_ptr(ptr: *const pdf_t) -> Self {
        if ptr.is_null() {
            return Default::default();
        }
        let xref = ptr.read().unwrap();

        let mut safe_xrefs = Vec::new();
        for i in 0..xref.n_entries as usize {
            unsafe {
                safe_xrefs.push(SafeXref::from_ptr((*xref.entries).offset(i as isize) as *const xref_entry_t));
            }
        }
        
        SafePdf{ 
            name: std::ffi::CStr::from_ptr(xref.name).to_string_lossy().into_owned(),
            pdf_major_version: xref.pdf_major_version,
            pdf_minor_version: xref.pdf_minor_version,
            n_xrefs: xref.n_xrefs,
            xrefs: safe_xrefs,
            has_xref_streams: xref.has_xref_streams, 
        }
    }
}



impl SafeXref {
    pub unsafe fn from_ptr(ptr: *const xref_entry_t) -> Self {
        if ptr.is_null() {
            return Default::default();
        }
        let entry = ptr.read().unwrap();
        
        SafeXref{ 
            obj_id: entry.obj_id, 
            offset: entry.offset, 
            gen_num: entry.gen_num, 
            f_or_n: entry.f_or_n 
        }
    }
}






#[derive(Copy, Clone)]





#[derive(Copy, Clone)]



#[derive(Copy, Clone)]


#[derive(Copy, Clone)]


#[derive(Copy, Clone)]


pub const SEEK_SET: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const __S_IREAD: core::ffi::c_int = 0o400 as core::ffi::c_int;
pub const __S_IWRITE: core::ffi::c_int = 0o200 as core::ffi::c_int;
pub const __S_IEXEC: core::ffi::c_int = 0o100 as core::ffi::c_int;
pub const S_IRWXU: core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const PDF_FLAG_QUIET: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PDF_FLAG_DISP_CREATOR: core::ffi::c_int = 2 as core::ffi::c_int;
unsafe extern "C" fn usage() {
    printf(
        b"-- pdfresurrect v0.24b --\nUsage: ./pdfresurrect <file.pdf> [-i] [-w] [-q]\n\t -i Display PDF creator information\n\t -w Write the PDF versions and summary to disk\n\t -q Display only the number of versions contained in the PDF\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    exit(0 as core::ffi::c_int);
}
unsafe extern "C" fn write_version(
    mut fp: *mut FILE,
    mut fname: *const core::ffi::c_char,
    mut dirname: *const core::ffi::c_char,
    mut xref: *mut xref_t,
) {
    let mut start: core::ffi::c_long = 0;
    let mut c: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut new_fname: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut data: core::ffi::c_char = 0;
    let mut new_fp: *mut FILE = 0 as *mut FILE;
    start = ftell(fp);
    c = strstr(fname, b".pdf\0" as *const u8 as *const core::ffi::c_char);
    if !c.is_null() {
        *c = '\0' as i32 as core::ffi::c_char;
    }
    new_fname = safe_calloc(
        (strlen(fname)).wrapping_add(strlen(dirname)).wrapping_add(32 as size_t),
    ) as *mut core::ffi::c_char;
    snprintf(
        new_fname,
        (strlen(fname)).wrapping_add(strlen(dirname)).wrapping_add(32 as size_t),
        b"%s/%s-version-%d.pdf\0" as *const u8 as *const core::ffi::c_char,
        dirname,
        fname,
        (*xref).version,
    );
    new_fp = fopen(new_fname, b"w\0" as *const u8 as *const core::ffi::c_char)
        as *mut FILE;
    if new_fp.is_null() {
        fprintf(
            stderr,
            b"[pdfresurrect] -- Error -- Could not create file '%s'\n\0" as *const u8
                as *const core::ffi::c_char,
            new_fname,
        );
        fseek(fp, start, SEEK_SET);
        free(new_fname as *mut core::ffi::c_void);
        return;
    }
    fseek(fp, 0 as core::ffi::c_long, SEEK_SET);
    while fread(
        &mut data as *mut core::ffi::c_char as *mut core::ffi::c_void,
        1 as size_t,
        1 as size_t,
        fp,
    ) != 0
    {
        fwrite(
            &mut data as *mut core::ffi::c_char as *const core::ffi::c_void,
            1 as size_t,
            1 as size_t,
            new_fp,
        );
    }
    fprintf(
        new_fp,
        b"\r\nstartxref\r\n%ld\r\n%%%%EOF\0" as *const u8 as *const core::ffi::c_char,
        (*xref).start,
    );
    fclose(new_fp);
    free(new_fname as *mut core::ffi::c_void);
    fseek(fp, start, SEEK_SET);
}
unsafe extern "C" fn display_creator(mut fp: *mut FILE, mut pdf: *const pdf_t) {
    let mut i: core::ffi::c_int = 0;
    printf(
        b"PDF Version: %d.%d\n\0" as *const u8 as *const core::ffi::c_char,
        (*pdf).pdf_major_version as core::ffi::c_int,
        (*pdf).pdf_minor_version as core::ffi::c_int,
    );
    i = 0 as core::ffi::c_int;
    while i < (*pdf).n_xrefs {
        if !((*((*pdf).xrefs).offset(i as isize)).version == 0) {
            if pdf_display_creator(pdf, i) != 0 {
                printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
            }
        }
        i += 1;
    }
}
unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut n_valid: core::ffi::c_int = 0;
    let mut do_write: core::ffi::c_int = 0;
    let mut do_scrub: core::ffi::c_int = 0;
    let mut c: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut dname: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut name: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut pdf: *mut pdf_t = 0 as *mut pdf_t;
    let mut flags: pdf_flag_t = 0;
    if argc < 2 as core::ffi::c_int {
        usage();
    }
    flags = 0 as pdf_flag_t;
    do_scrub = flags as core::ffi::c_int;
    do_write = do_scrub;
    name = 0 as *mut core::ffi::c_char;
    i = 1 as core::ffi::c_int;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize),
            b"-w\0" as *const u8 as *const core::ffi::c_char,
            2 as size_t,
        ) == 0 as core::ffi::c_int
        {
            do_write = 1 as core::ffi::c_int;
        } else if strncmp(
            *argv.offset(i as isize),
            b"-i\0" as *const u8 as *const core::ffi::c_char,
            2 as size_t,
        ) == 0 as core::ffi::c_int
        {
            flags = (flags as core::ffi::c_int | PDF_FLAG_DISP_CREATOR) as pdf_flag_t;
        } else if strncmp(
            *argv.offset(i as isize),
            b"-q\0" as *const u8 as *const core::ffi::c_char,
            2 as size_t,
        ) == 0 as core::ffi::c_int
        {
            flags = (flags as core::ffi::c_int | PDF_FLAG_QUIET) as pdf_flag_t;
        } else if *(*argv.offset(i as isize)).offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int != '-' as i32
        {
            name = *argv.offset(i as isize);
        } else if *(*argv.offset(i as isize)).offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int == '-' as i32
        {
            usage();
        }
        i += 1;
    }
    if name.is_null() {
        usage();
    }
    fp = fopen(name, b"r\0" as *const u8 as *const core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"[pdfresurrect] -- Error -- Could not open file '%s'\n\0" as *const u8
                as *const core::ffi::c_char,
            *argv.offset(1 as core::ffi::c_int as isize),
        );
        return -(1 as core::ffi::c_int);
    } else if pdf_is_pdf(fp) == 0 {
        fprintf(
            stderr,
            b"[pdfresurrect] -- Error -- '%s' specified is not a valid PDF\n\0"
                as *const u8 as *const core::ffi::c_char,
            name,
        );
        fclose(fp);
        return -(1 as core::ffi::c_int);
    }
    pdf = init_pdf(fp, name);
    if pdf.is_null() {
        fclose(fp);
        return -(1 as core::ffi::c_int);
    }
    i = 0 as core::ffi::c_int;
    n_valid = 0 as core::ffi::c_int;
    while i < (*pdf).n_xrefs {
        if (*((*pdf).xrefs).offset(i as isize)).version != 0 {
            n_valid += 1;
        }
        i += 1;
    }
    if n_valid < 2 as core::ffi::c_int {
        if flags as core::ffi::c_int & (PDF_FLAG_QUIET | PDF_FLAG_DISP_CREATOR) == 0 {
            printf(
                b"%s: There is only one version of this PDF\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*pdf).name,
            );
        }
        if do_write != 0 {
            fclose(fp);
            pdf_delete(pdf);
            return 0 as core::ffi::c_int;
        }
    }
    dname = 0 as *mut core::ffi::c_char;
    if do_write != 0 {
        c = strrchr(name, '/' as i32);
        if !c.is_null() {
            name = c.offset(1 as core::ffi::c_int as isize);
        }
        c = strrchr(name, '.' as i32);
        if !c.is_null() {
            *c = '\0' as i32 as core::ffi::c_char;
        }
        dname = safe_calloc((strlen(name)).wrapping_add(16 as size_t))
            as *mut core::ffi::c_char;
        sprintf(dname, b"%s-versions\0" as *const u8 as *const core::ffi::c_char, name);
        dir = opendir(dname);
        if dir.is_null() {
            mkdir(dname, S_IRWXU as __mode_t);
        } else {
            fprintf(
                stderr,
                b"[pdfresurrect] -- Error -- This directory already exists, PDF version extraction will not occur.\n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            fclose(fp);
            closedir(dir);
            free(dname as *mut core::ffi::c_void);
            pdf_delete(pdf);
            return -(1 as core::ffi::c_int);
        }
        i = 0 as core::ffi::c_int;
        while i < (*pdf).n_xrefs {
            if (*((*pdf).xrefs).offset(i as isize)).version != 0 {
                write_version(fp, name, dname, &mut *((*pdf).xrefs).offset(i as isize));
            }
            i += 1;
        }
    }
    pdf_summarize(fp, pdf, dname, flags);
    if flags as core::ffi::c_int & PDF_FLAG_DISP_CREATOR != 0 {
        display_creator(fp, pdf);
    }
    fclose(fp);
    free(dname as *mut core::ffi::c_void);
    pdf_delete(pdf);
    return 0 as core::ffi::c_int;
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
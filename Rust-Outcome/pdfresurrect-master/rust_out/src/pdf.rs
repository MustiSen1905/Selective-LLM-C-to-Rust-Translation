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
    let mut header: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    header = get_header(fp);
    if header.is_null() {
        return 0 as core::ffi::c_int;
    }
    let mut c: *const core::ffi::c_char = strstr(
        header,
        b"%PDF-\0" as *const u8 as *const core::ffi::c_char,
    );
    let is_pdf: core::ffi::c_int = (!c.is_null()
        && (c.offset_from(header) as core::ffi::c_long as size_t)
            .wrapping_add(strlen(b"%PDF-M.m\0" as *const u8 as *const core::ffi::c_char))
            < 1024 as size_t) as core::ffi::c_int;
    free(header as *mut core::ffi::c_void);
    return is_pdf;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_version(mut fp: *mut FILE, mut pdf: *mut pdf_t) {
    let mut header: *mut core::ffi::c_char = get_header(fp);
    let mut c: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    c = strstr(header, b"%PDF-\0" as *const u8 as *const core::ffi::c_char);
    if !c.is_null()
        && *c
            .offset(6 as core::ffi::c_int as isize)
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int == '.' as i32
        && *(*__ctype_b_loc())
            .offset(
                *c
                    .offset(5 as core::ffi::c_int as isize)
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int as isize,
            ) as core::ffi::c_int
            & _ISdigit as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
            != 0
        && *(*__ctype_b_loc())
            .offset(
                *c
                    .offset(7 as core::ffi::c_int as isize)
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int as isize,
            ) as core::ffi::c_int
            & _ISdigit as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
            != 0
    {
        (*pdf).pdf_major_version = atoi(
            c
                .offset(
                    strlen(b"%PDF-\0" as *const u8 as *const core::ffi::c_char) as isize,
                ),
        ) as core::ffi::c_short;
        (*pdf).pdf_minor_version = atoi(
            c
                .offset(
                    strlen(b"%PDF-M.\0" as *const u8 as *const core::ffi::c_char)
                        as isize,
                ),
        ) as core::ffi::c_short;
    }
    free(header as *mut core::ffi::c_void);
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
pub unsafe extern "C" fn pdf_get_object_status(
    mut pdf: *const pdf_t,
    mut xref_idx: core::ffi::c_int,
    mut entry_idx: core::ffi::c_int,
) -> core::ffi::c_char {
    let mut i: core::ffi::c_int = 0;
    let mut curr_ver: core::ffi::c_int = 0;
    let mut prev_xref: *const xref_t = 0 as *const xref_t;
    let mut prev: *const xref_entry_t = 0 as *const xref_entry_t;
    let mut curr: *const xref_entry_t = 0 as *const xref_entry_t;
    curr = &mut *((*((*pdf).xrefs).offset(xref_idx as isize)).entries)
        .offset(entry_idx as isize) as *mut xref_entry_t;
    curr_ver = (*((*pdf).xrefs).offset(xref_idx as isize)).version;
    if curr_ver == 1 as core::ffi::c_int {
        return 'A' as i32 as core::ffi::c_char;
    }
    if (*curr).f_or_n as core::ffi::c_int == 'f' as i32 {
        return 'D' as i32 as core::ffi::c_char;
    }
    prev_xref = 0 as *const xref_t;
    i = xref_idx;
    while i > -(1 as core::ffi::c_int) {
        if (*((*pdf).xrefs).offset(i as isize)).version < curr_ver {
            prev_xref = &mut *((*pdf).xrefs).offset(i as isize) as *mut xref_t;
            break;
        } else {
            i -= 1;
        }
    }
    if prev_xref.is_null() {
        return '?' as i32 as core::ffi::c_char;
    }
    prev = 0 as *const xref_entry_t;
    i = 0 as core::ffi::c_int;
    while i < (*prev_xref).n_entries {
        if (*((*prev_xref).entries).offset(i as isize)).obj_id == (*curr).obj_id {
            prev = &mut *((*prev_xref).entries).offset(i as isize) as *mut xref_entry_t;
            break;
        } else {
            i += 1;
        }
    }
    if prev.is_null()
        || (*prev).f_or_n as core::ffi::c_int == 'f' as i32
            && (*curr).f_or_n as core::ffi::c_int == 'n' as i32
    {
        return 'A' as i32 as core::ffi::c_char
    } else if (*prev).offset != (*curr).offset {
        return 'M' as i32 as core::ffi::c_char
    }
    return '?' as i32 as core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_zero_object(
    mut fp: *mut FILE,
    mut pdf: *const pdf_t,
    mut xref_idx: core::ffi::c_int,
    mut entry_idx: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut obj: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut obj_sz: size_t = 0;
    let mut entry: *mut xref_entry_t = 0 as *mut xref_entry_t;
    entry = &mut *((*((*pdf).xrefs).offset(xref_idx as isize)).entries)
        .offset(entry_idx as isize) as *mut xref_entry_t;
    fseek(fp, (*entry).offset, SEEK_SET);
    obj = get_object(
        fp,
        (*entry).obj_id,
        &mut *((*pdf).xrefs).offset(xref_idx as isize),
        0 as *mut size_t,
        0 as *mut core::ffi::c_int,
    );
    obj_sz = 0 as size_t;
    i = obj_sz as core::ffi::c_int;
    loop {
        i += 1;
        if !(strncmp(
            obj.offset(i as isize),
            b"endobj\0" as *const u8 as *const core::ffi::c_char,
            6 as size_t,
        ) != 0)
        {
            break;
        }
        obj_sz = obj_sz.wrapping_add(1);
    }
    if obj_sz != 0 {
        obj_sz = obj_sz
            .wrapping_add(
                (strlen(b"endobj\0" as *const u8 as *const core::ffi::c_char))
                    .wrapping_add(1 as size_t),
            );
    }
    i = 0 as core::ffi::c_int;
    while (i as size_t) < obj_sz {
        fputc('0' as i32, fp);
        i += 1;
    }
    printf(
        b"Zeroed object %d\n\0" as *const u8 as *const core::ffi::c_char,
        (*entry).obj_id,
    );
    free(obj as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_summarize(
    mut fp: *mut FILE,
    mut pdf: *const pdf_t,
    mut name: *const core::ffi::c_char,
    mut flags: pdf_flag_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut page: core::ffi::c_int = 0;
    let mut n_versions: core::ffi::c_int = 0;
    let mut n_entries: core::ffi::c_int = 0;
    let mut dst: *mut FILE = 0 as *mut FILE;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut dst_name: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut c: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    dst = 0 as *mut FILE;
    dst_name = 0 as *mut core::ffi::c_char;
    if !name.is_null() {
        dst_name = safe_calloc(
            (strlen(name)).wrapping_mul(2 as size_t).wrapping_add(16 as size_t),
        ) as *mut core::ffi::c_char;
        sprintf(
            dst_name,
            b"%s/%s\0" as *const u8 as *const core::ffi::c_char,
            name,
            name,
        );
        c = strrchr(dst_name, '.' as i32);
        if !c.is_null()
            && strncmp(
                c,
                b".pdf\0" as *const u8 as *const core::ffi::c_char,
                4 as size_t,
            ) == 0 as core::ffi::c_int
        {
            *c = '\0' as i32 as core::ffi::c_char;
        }
        strcat(dst_name, b".summary\0" as *const u8 as *const core::ffi::c_char);
        dst = fopen(dst_name, b"w\0" as *const u8 as *const core::ffi::c_char)
            as *mut FILE;
        if dst.is_null() {
            fprintf(
                stderr,
                b"[pdfresurrect] -- Error -- Could not open file '%s' for writing\n\0"
                    as *const u8 as *const core::ffi::c_char,
                dst_name,
            );
            return;
        }
    }
    out = if !dst.is_null() { dst } else { stdout };
    n_versions = (*pdf).n_xrefs;
    if n_versions != 0
        && (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize)).is_linear != 0
    {
        n_versions -= 1;
    }
    i = 1 as core::ffi::c_int;
    while i < (*pdf).n_xrefs {
        if (*((*pdf).xrefs).offset(i as isize)).end == 0 as core::ffi::c_long {
            n_versions -= 1;
        }
        i += 1;
    }
    if (*pdf).n_xrefs == 0
        || n_versions == 0
            && (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize)).is_linear != 0
    {
        n_versions = 1 as core::ffi::c_int;
    }
    n_entries = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while (*pdf).has_xref_streams == 0 && i < (*pdf).n_xrefs {
        if !(flags as core::ffi::c_int & PDF_FLAG_QUIET != 0) {
            j = 0 as core::ffi::c_int;
            while j < (*((*pdf).xrefs).offset(i as isize)).n_entries {
                n_entries += 1;
                fprintf(
                    out,
                    b"%s: --%c-- Version %d -- Object %d (%s)\0" as *const u8
                        as *const core::ffi::c_char,
                    (*pdf).name,
                    pdf_get_object_status(pdf, i, j) as core::ffi::c_int,
                    (*((*pdf).xrefs).offset(i as isize)).version,
                    (*((*((*pdf).xrefs).offset(i as isize)).entries).offset(j as isize))
                        .obj_id,
                    get_type(
                        fp,
                        (*((*((*pdf).xrefs).offset(i as isize)).entries)
                            .offset(j as isize))
                            .obj_id,
                        &mut *((*pdf).xrefs).offset(i as isize),
                    ),
                );
                fprintf(out, b"\n\0" as *const u8 as *const core::ffi::c_char);
                j += 1;
            }
        }
        i += 1;
    }
    if flags as core::ffi::c_int & PDF_FLAG_QUIET == 0 {
        if (*pdf).has_xref_streams != 0 || n_entries == 0 {
            fprintf(
                out,
                b"%s: This PDF contains potential cross reference streams.\n%s: An object summary is not available.\n\0"
                    as *const u8 as *const core::ffi::c_char,
                (*pdf).name,
                (*pdf).name,
            );
        }
        fprintf(
            out,
            b"---------- %s ----------\nVersions: %d\n\0" as *const u8
                as *const core::ffi::c_char,
            (*pdf).name,
            n_versions,
        );
        if (*pdf).has_xref_streams == 0 {
            i = 0 as core::ffi::c_int;
            while i < (*pdf).n_xrefs {
                if !((*((*pdf).xrefs).offset(i as isize)).is_linear != 0) {
                    n_entries = (*((*pdf).xrefs).offset(i as isize)).n_entries;
                    if (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize)).is_linear
                        != 0
                    {
                        n_entries
                            += (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize))
                                .n_entries;
                    }
                    if (*((*pdf).xrefs).offset(i as isize)).version != 0
                        && n_entries != 0
                    {
                        fprintf(
                            out,
                            b"Version %d -- %d objects\n\0" as *const u8
                                as *const core::ffi::c_char,
                            (*((*pdf).xrefs).offset(i as isize)).version,
                            n_entries,
                        );
                    }
                }
                i += 1;
            }
        }
    } else {
        fprintf(
            out,
            b"%s: %d\n\0" as *const u8 as *const core::ffi::c_char,
            (*pdf).name,
            n_versions,
        );
    }
    if !dst.is_null() {
        fclose(dst);
        free(dst_name as *mut core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_display_creator(
    mut pdf: *const pdf_t,
    mut xref_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    if ((*((*pdf).xrefs).offset(xref_idx as isize)).creator).is_null() {
        return 0 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < (*((*pdf).xrefs).offset(xref_idx as isize)).n_creator_entries {
        printf(
            b"%s: %s\n\0" as *const u8 as *const core::ffi::c_char,
            ((*((*((*pdf).xrefs).offset(xref_idx as isize)).creator).offset(i as isize))
                .key)
                .as_mut_ptr(),
            ((*((*((*pdf).xrefs).offset(xref_idx as isize)).creator).offset(i as isize))
                .value)
                .as_mut_ptr(),
        );
        i += 1;
    }
    return (i > 0 as core::ffi::c_int) as core::ffi::c_int;
}
unsafe extern "C" fn is_valid_xref(
    mut fp: *mut FILE,
    mut pdf: *mut pdf_t,
    mut xref: *mut xref_t,
) -> core::ffi::c_int {
    let mut is_valid: core::ffi::c_int = 0;
    let mut start: core::ffi::c_long = 0;
    let mut c: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut buf: [core::ffi::c_char; 16] = [0; 16];
    memset(
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_char; 16]>() as size_t,
    );
    is_valid = 0 as core::ffi::c_int;
    start = ftell(fp);
    fseek(fp, (*xref).start, SEEK_SET);
    if (fgets(buf.as_mut_ptr(), 16 as core::ffi::c_int, fp)).is_null() {
        fprintf(
            stderr,
            b"[pdfresurrect] -- Error -- Failed to load xref string.\0" as *const u8
                as *const core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    if strncmp(
        buf.as_mut_ptr(),
        b"xref\0" as *const u8 as *const core::ffi::c_char,
        strlen(b"xref\0" as *const u8 as *const core::ffi::c_char),
    ) == 0 as core::ffi::c_int
    {
        is_valid = 1 as core::ffi::c_int;
    } else {
        fseek(fp, (*xref).start, SEEK_SET);
        c = get_object_from_here(fp, 0 as *mut size_t, &mut (*xref).is_stream);
        if !c.is_null() && (*xref).is_stream != 0 {
            (*pdf).has_xref_streams = 1 as core::ffi::c_int;
            is_valid = 1 as core::ffi::c_int;
        }
        free(c as *mut core::ffi::c_void);
    }
    fseek(fp, start, SEEK_SET);
    return is_valid;
}
unsafe extern "C" fn load_xref_entries(mut fp: *mut FILE, mut xref: *mut xref_t) {
    if (*xref).is_stream != 0 {
        load_xref_from_stream(fp, xref);
    } else {
        load_xref_from_plaintext(fp, xref);
    };
}
unsafe extern "C" fn load_xref_from_plaintext(mut fp: *mut FILE, mut xref: *mut xref_t) {
    let mut i: core::ffi::c_int = 0;
    let mut obj_id: core::ffi::c_int = 0;
    let mut added_entries: core::ffi::c_int = 0;
    let mut c: core::ffi::c_char = 0;
    let mut buf: [core::ffi::c_char; 32] = [
        0 as core::ffi::c_int as core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut start: core::ffi::c_long = 0;
    let mut pos: core::ffi::c_long = 0;
    let mut buf_idx: size_t = 0;
    start = ftell(fp);
    pos = (*xref).end;
    fseek(fp, pos, SEEK_SET);
    while ftell(fp) != 0 as core::ffi::c_long {
        if ferror(fp) == 0 && feof(fp) == 0
            && (fgetc(fp) == '/' as i32 && fgetc(fp) == 'S' as i32)
        {
            break;
        }
        pos -= 1;
        if fseek(fp, pos, 0 as core::ffi::c_int) != 0 as core::ffi::c_int {
            fprintf(
                stderr,
                b"[pdfresurrect] -- Error -- Failed seek to xref /Size.\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            exit(EXIT_FAILURE);
        }
    }
    if fread(buf.as_mut_ptr() as *mut core::ffi::c_void, 1 as size_t, 21 as size_t, fp)
        != 21 as core::ffi::c_ulong
    {
        fprintf(
            stderr,
            b"[pdfresurrect] -- Error -- Failed to load entry Size string.\n\0"
                as *const u8 as *const core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    (*xref).n_entries = atoi(
        buf
            .as_mut_ptr()
            .offset(strlen(b"ize \0" as *const u8 as *const core::ffi::c_char) as isize),
    );
    (*xref).entries = safe_calloc(
        ((*xref).n_entries as size_t)
            .wrapping_mul(::core::mem::size_of::<_xref_entry>() as size_t),
    ) as *mut xref_entry_t;
    obj_id = 0 as core::ffi::c_int;
    fseek(
        fp,
        ((*xref).start as size_t)
            .wrapping_add(strlen(b"xref\0" as *const u8 as *const core::ffi::c_char))
            as core::ffi::c_long,
        SEEK_SET,
    );
    added_entries = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < (*xref).n_entries {
        c = fgetc(fp) as core::ffi::c_char;
        while c as core::ffi::c_int == '\n' as i32
            || c as core::ffi::c_int == '\r' as i32
        {
            c = fgetc(fp) as core::ffi::c_char;
        }
        if ferror(fp) != 0 || feof(fp) != 0 {
            break;
        }
        buf_idx = 0 as size_t;
        while c as core::ffi::c_int != '\n' as i32
            && c as core::ffi::c_int != '\r' as i32 && feof(fp) == 0 && ferror(fp) == 0
            && buf_idx < ::core::mem::size_of::<[core::ffi::c_char; 32]>() as usize
        {
            let fresh4 = buf_idx;
            buf_idx = buf_idx.wrapping_add(1);
            buf[fresh4 as usize] = c;
            c = fgetc(fp) as core::ffi::c_char;
        }
        if buf_idx >= ::core::mem::size_of::<[core::ffi::c_char; 32]>() as usize {
            fprintf(
                stderr,
                b"[pdfresurrect] -- Error -- Failed to locate newline character. This might be a corrupt PDF.\n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            exit(EXIT_FAILURE);
        }
        buf[buf_idx as usize] = '\0' as i32 as core::ffi::c_char;
        if !(strchr(buf.as_mut_ptr(), 't' as i32)).is_null() {
            break;
        }
        if strlen(buf.as_mut_ptr()) > 17 as size_t {
            let mut token: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
            let fresh5 = obj_id;
            obj_id = obj_id + 1;
            (*((*xref).entries).offset(i as isize)).obj_id = fresh5;
            token = strtok(
                buf.as_mut_ptr(),
                b" \0" as *const u8 as *const core::ffi::c_char,
            );
            if token.is_null() {
                fprintf(
                    stderr,
                    b"[pdfresurrect] -- Error -- Failed to parse xref entry. This might be a corrupt PDF.\n\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                exit(EXIT_FAILURE);
            }
            (*((*xref).entries).offset(i as isize)).offset = atol(token);
            token = strtok(
                0 as *mut core::ffi::c_char,
                b" \0" as *const u8 as *const core::ffi::c_char,
            );
            if token.is_null() {
                fprintf(
                    stderr,
                    b"[pdfresurrect] -- Error -- Failed to parse xref entry. This might be a corrupt PDF.\n\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                exit(EXIT_FAILURE);
            }
            (*((*xref).entries).offset(i as isize)).gen_num = atoi(token);
            (*((*xref).entries).offset(i as isize)).f_or_n = buf[17 as core::ffi::c_int
                as usize];
            added_entries += 1;
        } else {
            obj_id = atoi(buf.as_mut_ptr());
            i -= 1;
        }
        i += 1;
    }
    (*xref).n_entries = added_entries;
    fseek(fp, start, SEEK_SET);
}
unsafe extern "C" fn load_xref_from_stream(mut fp: *mut FILE, mut xref: *mut xref_t) {
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
unsafe extern "C" fn get_xref_linear_skipped(mut fp: *mut FILE, mut xref: *mut xref_t) {
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
unsafe extern "C" fn resolve_linearized_pdf(mut pdf: *mut pdf_t) {
    let mut i: core::ffi::c_int = 0;
    let mut buf: xref_t = _xref_t {
        start: 0,
        end: 0,
        creator: 0 as *mut pdf_creator_t,
        n_creator_entries: 0,
        n_entries: 0,
        entries: 0 as *mut xref_entry_t,
        is_stream: 0,
        is_linear: 0,
        version: 0,
    };
    if (*pdf).n_xrefs < 2 as core::ffi::c_int {
        return;
    }
    if (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize)).is_linear == 0 {
        return;
    }
    buf = *((*pdf).xrefs).offset(0 as core::ffi::c_int as isize);
    *((*pdf).xrefs).offset(0 as core::ffi::c_int as isize) = *((*pdf).xrefs)
        .offset(1 as core::ffi::c_int as isize);
    *((*pdf).xrefs).offset(1 as core::ffi::c_int as isize) = buf;
    (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize)).is_linear = 1
        as core::ffi::c_int;
    (*((*pdf).xrefs).offset(0 as core::ffi::c_int as isize)).version = 1
        as core::ffi::c_int;
    (*((*pdf).xrefs).offset(1 as core::ffi::c_int as isize)).is_linear = 0
        as core::ffi::c_int;
    (*((*pdf).xrefs).offset(1 as core::ffi::c_int as isize)).version = 1
        as core::ffi::c_int;
    i = 2 as core::ffi::c_int;
    while i < (*pdf).n_xrefs {
        let ref mut fresh3 = (*((*pdf).xrefs).offset(i as isize)).version;
        *fresh3 -= 1;
        i += 1;
    }
}
unsafe extern "C" fn load_creator(mut fp: *mut FILE, mut pdf: *mut pdf_t) {
    let mut i: core::ffi::c_int = 0;
    let mut buf_idx: core::ffi::c_int = 0;
    let mut c: core::ffi::c_char = 0;
    let mut buf: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut obj_id_buf: [core::ffi::c_char; 32] = [
        0 as core::ffi::c_int as core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut start: core::ffi::c_long = 0;
    let mut sz: size_t = 0;
    start = ftell(fp);
    i = 0 as core::ffi::c_int;
    while i < (*pdf).n_xrefs {
        if !((*((*pdf).xrefs).offset(i as isize)).version == 0) {
            fseek(fp, (*((*pdf).xrefs).offset(i as isize)).start, SEEK_SET);
            while ferror(fp) == 0 && feof(fp) == 0 && fgetc(fp) != 't' as i32 {}
            c = '\0' as i32 as core::ffi::c_char;
            while ferror(fp) == 0 && feof(fp) == 0
                && {
                    c = fgetc(fp) as core::ffi::c_char;
                    c as core::ffi::c_int != '>' as i32
                }
            {
                if ferror(fp) == 0 && feof(fp) == 0
                    && (c as core::ffi::c_int == '/' as i32 && fgetc(fp) == 'I' as i32
                        && fgetc(fp) == 'n' as i32)
                {
                    break;
                }
            }
            if c as core::ffi::c_int == '>' as i32 {
                fseek(fp, start, SEEK_SET);
            } else {
                while ferror(fp) == 0 && feof(fp) == 0
                    && {
                        c = fgetc(fp) as core::ffi::c_char;
                        *(*__ctype_b_loc()).offset(c as core::ffi::c_int as isize)
                            as core::ffi::c_int
                            & _ISspace as core::ffi::c_int as core::ffi::c_ushort
                                as core::ffi::c_int == 0
                            && c as core::ffi::c_int != '>' as i32
                    }
                {}
                if c as core::ffi::c_int == '>' as i32 {
                    fseek(fp, start, SEEK_SET);
                } else {
                    while ferror(fp) == 0 && feof(fp) == 0
                        && {
                            c = fgetc(fp) as core::ffi::c_char;
                            *(*__ctype_b_loc()).offset(c as core::ffi::c_int as isize)
                                as core::ffi::c_int
                                & _ISspace as core::ffi::c_int as core::ffi::c_ushort
                                    as core::ffi::c_int != 0
                                && c as core::ffi::c_int != '>' as i32
                        }
                    {}
                    if c as core::ffi::c_int == '>' as i32 {
                        fseek(fp, start, SEEK_SET);
                    } else {
                        buf_idx = 0 as core::ffi::c_int;
                        let fresh1 = buf_idx;
                        buf_idx = buf_idx + 1;
                        obj_id_buf[fresh1 as usize] = c;
                        while (buf_idx as usize)
                            < (::core::mem::size_of::<[core::ffi::c_char; 32]>()
                                as usize)
                                .wrapping_sub(1 as usize)
                            && (ferror(fp) == 0 && feof(fp) == 0
                                && {
                                    c = fgetc(fp) as core::ffi::c_char;
                                    *(*__ctype_b_loc()).offset(c as core::ffi::c_int as isize)
                                        as core::ffi::c_int
                                        & _ISspace as core::ffi::c_int as core::ffi::c_ushort
                                            as core::ffi::c_int == 0
                                        && c as core::ffi::c_int != '>' as i32
                                })
                        {
                            let fresh2 = buf_idx;
                            buf_idx = buf_idx + 1;
                            obj_id_buf[fresh2 as usize] = c;
                        }
                        if c as core::ffi::c_int == '>' as i32 {
                            fseek(fp, start, SEEK_SET);
                        } else {
                            buf = get_object(
                                fp,
                                atoll(obj_id_buf.as_mut_ptr()) as core::ffi::c_int,
                                &mut *((*pdf).xrefs).offset(i as isize),
                                &mut sz,
                                0 as *mut core::ffi::c_int,
                            );
                            if buf.is_null()
                                && (*((*pdf).xrefs).offset(i as isize)).is_linear != 0
                                && (i + 1 as core::ffi::c_int) < (*pdf).n_xrefs
                            {
                                buf = get_object(
                                    fp,
                                    atoll(obj_id_buf.as_mut_ptr()) as core::ffi::c_int,
                                    &mut *((*pdf).xrefs)
                                        .offset((i + 1 as core::ffi::c_int) as isize),
                                    &mut sz,
                                    0 as *mut core::ffi::c_int,
                                );
                            }
                            load_creator_from_buf(
                                fp,
                                &mut *((*pdf).xrefs).offset(i as isize),
                                buf,
                                sz,
                            );
                            free(buf as *mut core::ffi::c_void);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    fseek(fp, start, SEEK_SET);
}
unsafe extern "C" fn load_creator_from_buf(
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
unsafe extern "C" fn load_creator_from_xml(
    mut xref: *mut xref_t,
    mut buf: *const core::ffi::c_char,
) {}
unsafe extern "C" fn load_creator_from_old_format(
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
unsafe extern "C" fn get_next_eof(mut fp: *mut FILE) -> core::ffi::c_int {
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

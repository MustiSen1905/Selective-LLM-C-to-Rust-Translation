extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn _TIFFmemset(p: *mut core::ffi::c_void, v: core::ffi::c_int, c: tmsize_t);
    fn _TIFFfree(p: *mut core::ffi::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type uint8 = core::ffi::c_int;
pub type uint16 = core::ffi::c_int;
pub type uint32 = core::ffi::c_int;
pub type uint64 = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIFFHeaderCommon {
    pub tiff_magic: uint16,
    pub tiff_version: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIFFHeaderClassic {
    pub tiff_magic: uint16,
    pub tiff_version: uint16,
    pub tiff_diroff: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIFFHeaderBig {
    pub tiff_magic: uint16,
    pub tiff_version: uint16,
    pub tiff_offsetsize: uint16,
    pub tiff_unused: uint16,
    pub tiff_diroff: uint64,
}
pub type TIFFDataType = core::ffi::c_uint;
pub const TIFF_IFD8: TIFFDataType = 18;
pub const TIFF_SLONG8: TIFFDataType = 17;
pub const TIFF_LONG8: TIFFDataType = 16;
pub const TIFF_IFD: TIFFDataType = 13;
pub const TIFF_DOUBLE: TIFFDataType = 12;
pub const TIFF_FLOAT: TIFFDataType = 11;
pub const TIFF_SRATIONAL: TIFFDataType = 10;
pub const TIFF_SLONG: TIFFDataType = 9;
pub const TIFF_SSHORT: TIFFDataType = 8;
pub const TIFF_UNDEFINED: TIFFDataType = 7;
pub const TIFF_SBYTE: TIFFDataType = 6;
pub const TIFF_RATIONAL: TIFFDataType = 5;
pub const TIFF_LONG: TIFFDataType = 4;
pub const TIFF_SHORT: TIFFDataType = 3;
pub const TIFF_ASCII: TIFFDataType = 2;
pub const TIFF_BYTE: TIFFDataType = 1;
pub const TIFF_NOTYPE: TIFFDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tiff {
    pub tif_name: *mut core::ffi::c_char,
    pub tif_fd: core::ffi::c_int,
    pub tif_mode: core::ffi::c_int,
    pub tif_flags: uint32,
    pub tif_diroff: uint64,
    pub tif_nextdiroff: uint64,
    pub tif_dirlist: *mut uint64,
    pub tif_dirlistsize: uint16,
    pub tif_dirnumber: uint16,
    pub tif_dir: TIFFDirectory,
    pub tif_customdir: TIFFDirectory,
    pub tif_header: C2RustUnnamed,
    pub tif_header_size: uint16,
    pub tif_row: uint32,
    pub tif_curdir: uint16,
    pub tif_curstrip: uint32,
    pub tif_curoff: uint64,
    pub tif_dataoff: uint64,
    pub tif_nsubifd: uint16,
    pub tif_subifdoff: uint64,
    pub tif_col: uint32,
    pub tif_curtile: uint32,
    pub tif_tilesize: tmsize_t,
    pub tif_decodestatus: core::ffi::c_int,
    pub tif_fixuptags: TIFFBoolMethod,
    pub tif_setupdecode: TIFFBoolMethod,
    pub tif_predecode: TIFFPreMethod,
    pub tif_setupencode: TIFFBoolMethod,
    pub tif_encodestatus: core::ffi::c_int,
    pub tif_preencode: TIFFPreMethod,
    pub tif_postencode: TIFFBoolMethod,
    pub tif_decoderow: TIFFCodeMethod,
    pub tif_encoderow: TIFFCodeMethod,
    pub tif_decodestrip: TIFFCodeMethod,
    pub tif_encodestrip: TIFFCodeMethod,
    pub tif_decodetile: TIFFCodeMethod,
    pub tif_encodetile: TIFFCodeMethod,
    pub tif_close: TIFFVoidMethod,
    pub tif_seek: TIFFSeekMethod,
    pub tif_cleanup: TIFFVoidMethod,
    pub tif_defstripsize: TIFFStripMethod,
    pub tif_deftilesize: TIFFTileMethod,
    pub tif_data: *mut uint8,
    pub tif_scanlinesize: tmsize_t,
    pub tif_scanlineskew: tmsize_t,
    pub tif_rawdata: *mut uint8,
    pub tif_rawdatasize: tmsize_t,
    pub tif_rawdataoff: tmsize_t,
    pub tif_rawdataloaded: tmsize_t,
    pub tif_rawcp: *mut uint8,
    pub tif_rawcc: tmsize_t,
    pub tif_base: *mut uint8,
    pub tif_size: tmsize_t,
    pub tif_mapproc: TIFFMapFileProc,
    pub tif_unmapproc: TIFFUnmapFileProc,
    pub tif_clientdata: thandle_t,
    pub tif_readproc: TIFFReadWriteProc,
    pub tif_writeproc: TIFFReadWriteProc,
    pub tif_seekproc: TIFFSeekProc,
    pub tif_closeproc: TIFFCloseProc,
    pub tif_sizeproc: TIFFSizeProc,
    pub tif_postdecode: TIFFPostMethod,
    pub tif_fields: *mut *mut TIFFField,
    pub tif_nfields: size_t,
    pub tif_foundfield: *const TIFFField,
    pub tif_tagmethods: TIFFTagMethods,
    pub tif_clientinfo: *mut TIFFClientInfoLink,
    pub tif_fieldscompat: *mut TIFFFieldArray,
    pub tif_nfieldscompat: size_t,
}
pub type size_t = usize;
pub type TIFFFieldArray = _TIFFFieldArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TIFFFieldArray {
    pub type_0: TIFFFieldArrayType,
    pub allocated_size: uint32,
    pub count: uint32,
    pub fields: *mut TIFFField,
}
pub type TIFFField = _TIFFField;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TIFFField {
    pub field_tag: uint32,
    pub field_readcount: core::ffi::c_short,
    pub field_writecount: core::ffi::c_short,
    pub field_type: TIFFDataType,
    pub reserved: uint32,
    pub set_field_type: TIFFSetGetFieldType,
    pub get_field_type: TIFFSetGetFieldType,
    pub field_bit: core::ffi::c_ushort,
    pub field_oktochange: core::ffi::c_uchar,
    pub field_passcount: core::ffi::c_uchar,
    pub field_name: *mut core::ffi::c_char,
    pub field_subfields: *mut TIFFFieldArray,
}
pub type TIFFSetGetFieldType = core::ffi::c_uint;
pub const TIFF_SETGET_OTHER: TIFFSetGetFieldType = 51;
pub const TIFF_SETGET_C32_IFD8: TIFFSetGetFieldType = 50;
pub const TIFF_SETGET_C32_DOUBLE: TIFFSetGetFieldType = 49;
pub const TIFF_SETGET_C32_FLOAT: TIFFSetGetFieldType = 48;
pub const TIFF_SETGET_C32_SINT64: TIFFSetGetFieldType = 47;
pub const TIFF_SETGET_C32_UINT64: TIFFSetGetFieldType = 46;
pub const TIFF_SETGET_C32_SINT32: TIFFSetGetFieldType = 45;
pub const TIFF_SETGET_C32_UINT32: TIFFSetGetFieldType = 44;
pub const TIFF_SETGET_C32_SINT16: TIFFSetGetFieldType = 43;
pub const TIFF_SETGET_C32_UINT16: TIFFSetGetFieldType = 42;
pub const TIFF_SETGET_C32_SINT8: TIFFSetGetFieldType = 41;
pub const TIFF_SETGET_C32_UINT8: TIFFSetGetFieldType = 40;
pub const TIFF_SETGET_C32_ASCII: TIFFSetGetFieldType = 39;
pub const TIFF_SETGET_C16_IFD8: TIFFSetGetFieldType = 38;
pub const TIFF_SETGET_C16_DOUBLE: TIFFSetGetFieldType = 37;
pub const TIFF_SETGET_C16_FLOAT: TIFFSetGetFieldType = 36;
pub const TIFF_SETGET_C16_SINT64: TIFFSetGetFieldType = 35;
pub const TIFF_SETGET_C16_UINT64: TIFFSetGetFieldType = 34;
pub const TIFF_SETGET_C16_SINT32: TIFFSetGetFieldType = 33;
pub const TIFF_SETGET_C16_UINT32: TIFFSetGetFieldType = 32;
pub const TIFF_SETGET_C16_SINT16: TIFFSetGetFieldType = 31;
pub const TIFF_SETGET_C16_UINT16: TIFFSetGetFieldType = 30;
pub const TIFF_SETGET_C16_SINT8: TIFFSetGetFieldType = 29;
pub const TIFF_SETGET_C16_UINT8: TIFFSetGetFieldType = 28;
pub const TIFF_SETGET_C16_ASCII: TIFFSetGetFieldType = 27;
pub const TIFF_SETGET_C0_IFD8: TIFFSetGetFieldType = 26;
pub const TIFF_SETGET_C0_DOUBLE: TIFFSetGetFieldType = 25;
pub const TIFF_SETGET_C0_FLOAT: TIFFSetGetFieldType = 24;
pub const TIFF_SETGET_C0_SINT64: TIFFSetGetFieldType = 23;
pub const TIFF_SETGET_C0_UINT64: TIFFSetGetFieldType = 22;
pub const TIFF_SETGET_C0_SINT32: TIFFSetGetFieldType = 21;
pub const TIFF_SETGET_C0_UINT32: TIFFSetGetFieldType = 20;
pub const TIFF_SETGET_C0_SINT16: TIFFSetGetFieldType = 19;
pub const TIFF_SETGET_C0_UINT16: TIFFSetGetFieldType = 18;
pub const TIFF_SETGET_C0_SINT8: TIFFSetGetFieldType = 17;
pub const TIFF_SETGET_C0_UINT8: TIFFSetGetFieldType = 16;
pub const TIFF_SETGET_C0_ASCII: TIFFSetGetFieldType = 15;
pub const TIFF_SETGET_UINT16_PAIR: TIFFSetGetFieldType = 14;
pub const TIFF_SETGET_INT: TIFFSetGetFieldType = 13;
pub const TIFF_SETGET_IFD8: TIFFSetGetFieldType = 12;
pub const TIFF_SETGET_DOUBLE: TIFFSetGetFieldType = 11;
pub const TIFF_SETGET_FLOAT: TIFFSetGetFieldType = 10;
pub const TIFF_SETGET_SINT64: TIFFSetGetFieldType = 9;
pub const TIFF_SETGET_UINT64: TIFFSetGetFieldType = 8;
pub const TIFF_SETGET_SINT32: TIFFSetGetFieldType = 7;
pub const TIFF_SETGET_UINT32: TIFFSetGetFieldType = 6;
pub const TIFF_SETGET_SINT16: TIFFSetGetFieldType = 5;
pub const TIFF_SETGET_UINT16: TIFFSetGetFieldType = 4;
pub const TIFF_SETGET_SINT8: TIFFSetGetFieldType = 3;
pub const TIFF_SETGET_UINT8: TIFFSetGetFieldType = 2;
pub const TIFF_SETGET_ASCII: TIFFSetGetFieldType = 1;
pub const TIFF_SETGET_UNDEFINED: TIFFSetGetFieldType = 0;
pub type TIFFFieldArrayType = core::ffi::c_uint;
pub const tfiatOther: TIFFFieldArrayType = 2;
pub const tfiatExif: TIFFFieldArrayType = 1;
pub const tfiatImage: TIFFFieldArrayType = 0;
pub type TIFFClientInfoLink = client_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_info {
    pub next: *mut client_info,
    pub data: *mut core::ffi::c_void,
    pub name: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIFFTagMethods {
    pub vsetfield: TIFFVSetMethod,
    pub vgetfield: TIFFVGetMethod,
    pub printdir: TIFFPrintMethod,
}
pub type TIFFPrintMethod = Option<
    unsafe extern "C" fn(*mut TIFF, *mut FILE, core::ffi::c_long) -> (),
>;
pub type FILE = _IO_FILE;
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
pub type __off64_t = core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = core::ffi::c_long;
pub type TIFF = tiff;
pub type TIFFVGetMethod = Option<
    unsafe extern "C" fn(*mut TIFF, uint32, *mut __va_list_tag) -> core::ffi::c_int,
>;
pub type TIFFVSetMethod = Option<
    unsafe extern "C" fn(*mut TIFF, uint32, *mut __va_list_tag) -> core::ffi::c_int,
>;
pub type TIFFPostMethod = Option<
    unsafe extern "C" fn(*mut TIFF, *mut uint8, tmsize_t) -> (),
>;
pub type tmsize_t = core::ffi::c_int;
pub type TIFFSizeProc = Option<unsafe extern "C" fn(thandle_t) -> toff_t>;
pub type thandle_t = *mut core::ffi::c_void;
pub type toff_t = uint64;
pub type TIFFCloseProc = Option<unsafe extern "C" fn(thandle_t) -> core::ffi::c_int>;
pub type TIFFSeekProc = Option<
    unsafe extern "C" fn(thandle_t, toff_t, core::ffi::c_int) -> toff_t,
>;
pub type TIFFReadWriteProc = Option<
    unsafe extern "C" fn(thandle_t, *mut core::ffi::c_void, tmsize_t) -> tmsize_t,
>;
pub type TIFFUnmapFileProc = Option<
    unsafe extern "C" fn(thandle_t, *mut core::ffi::c_void, toff_t) -> (),
>;
pub type TIFFMapFileProc = Option<
    unsafe extern "C" fn(
        thandle_t,
        *mut *mut core::ffi::c_void,
        *mut toff_t,
    ) -> core::ffi::c_int,
>;
pub type TIFFTileMethod = Option<
    unsafe extern "C" fn(*mut TIFF, *mut uint32, *mut uint32) -> (),
>;
pub type TIFFStripMethod = Option<unsafe extern "C" fn(*mut TIFF, uint32) -> uint32>;
pub type TIFFVoidMethod = Option<unsafe extern "C" fn(*mut TIFF) -> ()>;
pub type TIFFSeekMethod = Option<
    unsafe extern "C" fn(*mut TIFF, uint32) -> core::ffi::c_int,
>;
pub type TIFFCodeMethod = Option<
    unsafe extern "C" fn(*mut TIFF, *mut uint8, tmsize_t, uint16) -> core::ffi::c_int,
>;
pub type TIFFBoolMethod = Option<unsafe extern "C" fn(*mut TIFF) -> core::ffi::c_int>;
pub type TIFFPreMethod = Option<
    unsafe extern "C" fn(*mut TIFF, uint16) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub common: TIFFHeaderCommon,
    pub classic: TIFFHeaderClassic,
    pub big: TIFFHeaderBig,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIFFDirectory {
    pub td_fieldsset: [core::ffi::c_ulong; 4],
    pub td_imagewidth: uint32,
    pub td_imagelength: uint32,
    pub td_imagedepth: uint32,
    pub td_tilewidth: uint32,
    pub td_tilelength: uint32,
    pub td_tiledepth: uint32,
    pub td_subfiletype: uint32,
    pub td_bitspersample: uint16,
    pub td_sampleformat: uint16,
    pub td_compression: uint16,
    pub td_photometric: uint16,
    pub td_threshholding: uint16,
    pub td_fillorder: uint16,
    pub td_orientation: uint16,
    pub td_samplesperpixel: uint16,
    pub td_rowsperstrip: uint32,
    pub td_minsamplevalue: uint16,
    pub td_maxsamplevalue: uint16,
    pub td_sminsamplevalue: *mut core::ffi::c_double,
    pub td_smaxsamplevalue: *mut core::ffi::c_double,
    pub td_xresolution: core::ffi::c_float,
    pub td_yresolution: core::ffi::c_float,
    pub td_resolutionunit: uint16,
    pub td_planarconfig: uint16,
    pub td_xposition: core::ffi::c_float,
    pub td_yposition: core::ffi::c_float,
    pub td_pagenumber: [uint16; 2],
    pub td_colormap: [*mut uint16; 3],
    pub td_halftonehints: [uint16; 2],
    pub td_extrasamples: uint16,
    pub td_sampleinfo: *mut uint16,
    pub td_stripsperimage: uint32,
    pub td_nstrips: uint32,
    pub td_stripoffset: *mut uint64,
    pub td_stripbytecount: *mut uint64,
    pub td_stripbytecountsorted: core::ffi::c_int,
    pub td_nsubifd: uint16,
    pub td_subifd: *mut uint64,
    pub td_ycbcrsubsampling: [uint16; 2],
    pub td_ycbcrpositioning: uint16,
    pub td_transferfunction: [*mut uint16; 3],
    pub td_refblackwhite: *mut core::ffi::c_float,
    pub td_inknameslen: core::ffi::c_int,
    pub td_inknames: *mut core::ffi::c_char,
    pub td_customValueCount: core::ffi::c_int,
    pub td_customValues: *mut TIFFTagValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIFFTagValue {
    pub info: *const TIFFField,
    pub count: core::ffi::c_int,
    pub value: *mut core::ffi::c_void,
}
pub const FIELD_SETLONGS: core::ffi::c_int = 4 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn TIFFFreeDirectory(mut tif: *mut TIFF) {
    let mut td: *mut TIFFDirectory = &mut (*tif).tif_dir;
    let mut i: core::ffi::c_int = 0;
    _TIFFmemset(
        ((*td).td_fieldsset).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        FIELD_SETLONGS,
    );
    (*tif)
        .tif_dir
        .td_fieldsset[(39 as core::ffi::c_int / 32 as core::ffi::c_int) as usize]
        &= !((1 as core::ffi::c_long as core::ffi::c_ulong)
            << (39 as core::ffi::c_int & 0x1f as core::ffi::c_int));
    (*tif)
        .tif_dir
        .td_fieldsset[(40 as core::ffi::c_int / 32 as core::ffi::c_int) as usize]
        &= !((1 as core::ffi::c_long as core::ffi::c_ulong)
            << (40 as core::ffi::c_int & 0x1f as core::ffi::c_int));
    i = 0 as core::ffi::c_int;
    while i < (*td).td_customValueCount {
        if !((*((*td).td_customValues).offset(i as isize)).value).is_null() {
            _TIFFfree((*((*td).td_customValues).offset(i as isize)).value);
        }
        i += 1;
    }
    (*td).td_customValueCount = 0 as core::ffi::c_int;
}

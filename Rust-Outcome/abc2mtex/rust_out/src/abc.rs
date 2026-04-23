impl SafeFrac {
    pub unsafe fn from_ptr(ptr: *const frac) -> Self {
        if ptr.is_null() {
            return Self { n: "".to_string(), d: "".to_string() };
        }
        
        let c_str = std::ffi::CStr::from_ptr(ptr);
        let str_slice = c_str.to_bytes();
        
        // Convert C-strings to Rust Strings
        let n = String::from_utf8_lossy(&str_slice[0..4]).into_owned();
        let d = String::from_utf8_lossy(&str_slice[4..8]).into_owned();
        
        Self { n, d }
    }
}



impl SafeNote {
    pub unsafe fn from_ptr(ptr: *const Note) -> Self {
        if ptr.is_null() {
            return Self { length: 0, type_0: 0, pitch: 0, attributes: vec![], gchord: "".to_string(), chord: 0, tuplet: 0, start: vec![], end: vec![], n_notes: 0, iaccidental: 0, broken: SafeFrac { n: "".to_string(), d: "".to_string() } };
        }
        
        let c_str = std::ffi::CStr::from_ptr(ptr);
        let str_slice = c_str.to_bytes();
        
        // Convert C-strings to Rust Strings
        let length = ptr.length;
        let type_0 = ptr.type_0;
        let pitch = ptr.pitch;
        let attributes = str_slice[8..(9*26)].chunks(4).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).collect();
        let gchord = String::from_utf8_lossy(&str_slice[(9*26)..((9*26)+4)]).into_owned();
        let chord = ptr.chord;
        let tuplet = ptr.tuplet;
        let start = str_slice[((9*26)+4)..((9*26)+8+(9*3))].chunks(4).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).collect();
        let end = str_slice[((9*26)+8+(9*3))..((9*26)+16+(9*3))].chunks(4).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).collect();
        let n_notes = ptr.n_notes;
        let iaccidental = ptr.iaccidental;
        let broken = SafeFrac { n: String::from_utf8_lossy(&str_slice[((9*26)+16+(9*3))..((9*26)+24+(9*3))]).into_owned(), d: String::from_utf8_lossy(&str_slice[((9*26)+24+(9*3))..((9*26)+32+(9*3))]).into_owned() };
        
        Self { length, type_0, pitch, attributes, gchord, chord, tuplet, start, end, n_notes, iaccidental, broken }
    }
}


 
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

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub note: Note,
    pub bar: Barline,
    pub field: Field,
    pub misc: Misc,
}

pub const FIELD: symbol_types = 3;
pub const NOTE: symbol_types = 2;
pub const TWO_BARS_PLUS: two_bar_types = 4;
pub const TWO_BARS: two_bar_types = 2;
pub const ONE_BAR_PLUS: two_bar_types = 3;
pub const TEX_OUTPUT: output_types = 1;
#[derive(Copy, Clone)]

pub const NATURAL: accidental_types = 3;
pub const FLAT: accidental_types = 2;
pub const DBL_FLAT: accidental_types = 1;
pub const SHARP: accidental_types = 4;
pub const DBL_SHARP: accidental_types = 5;
pub const LDBL_BAR: bar_types = 3;
pub const REPEAT: bar_types = 5;
pub const RREPEAT: bar_types = 7;
pub const RREPEAT2: bar_types = 8;
pub const BAR: bar_types = 0;
pub const BAR1: bar_types = 1;
pub const LREPEAT: bar_types = 6;
pub const RDBL_BAR: bar_types = 4;
pub const DBL_BAR: bar_types = 2;
pub const INDEX_OUTPUT: output_types = 2;
pub const NEWLINE_TKN: token_types = 16;
pub const BAR_LINE: symbol_types = 1;

pub const HASH_OUTPUT: output_types = 3;
pub const NO_OUTPUT: output_types = 0;

pub const ONE_BAR: two_bar_types = 1;
pub const NO_BARS: two_bar_types = 0;

pub const MISC: symbol_types = 4;
pub const UNDETERMINED: symbol_types = 0;


pub const NONE: accidental_types = 0;

pub const MAX_TKNS: token_types = 25;
pub const TRAILING_TKN: token_types = 24;
pub const AMPERSAND_TKN: token_types = 23;
pub const CLOSE_SLUR_TKN: token_types = 22;
pub const OPEN_SLUR_TKN: token_types = 21;
pub const CLOSE_CHORD_TKN: token_types = 20;
pub const OPEN_CHORD_TKN: token_types = 19;
pub const CLOSE_GRACE_TKN: token_types = 18;
pub const OPEN_GRACE_TKN: token_types = 17;
pub const JUSTIFY_TKN: token_types = 15;
pub const CONTINUE_TKN: token_types = 14;
pub const MACRO_TKN: token_types = 13;
pub const TUPLET_TKN: token_types = 12;
pub const TIE_TKN: token_types = 11;
pub const SPACE_TKN: token_types = 10;
pub const REPEAT_TKN: token_types = 9;
pub const BAR_TKN: token_types = 8;
pub const BROKEN_TKN: token_types = 7;
pub const DIVISOR_TKN: token_types = 6;
pub const LENGTH_TKN: token_types = 5;
pub const OCTAVER_TKN: token_types = 4;
pub const NOTE_TKN: token_types = 3;
pub const ACCIDENTAL_TKN: token_types = 2;
pub const ACCENT_TKN: token_types = 1;
pub const GCHORD_TKN: token_types = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const CROTCHET: core::ffi::c_int = 8 as core::ffi::c_int;
pub const QUAVER: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SEMIQUAVER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const DEMISEMIQUAVER: core::ffi::c_int = 1 as core::ffi::c_int;
#[no_mangle]
pub static mut Trans: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut settings: Setting = {
    let mut init = Setting {
        gchords_above: 0 as core::ffi::c_int,
        autobeam: 0 as core::ffi::c_int,
        old_slurs: 0 as core::ffi::c_int,
        old_chords: 0 as core::ffi::c_int,
        old_repeats: 0 as core::ffi::c_int,
        justification: 0 as core::ffi::c_int,
        mine: 0 as core::ffi::c_int,
    };
    init
};
#[no_mangle]
pub static mut bass: core::ffi::c_int = 0;
#[no_mangle]
pub static mut treble: core::ffi::c_int = 0;
#[no_mangle]
pub static mut dnl: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub static mut transpose: core::ffi::c_int = 0;
#[no_mangle]
pub static mut offset: core::ffi::c_int = 0;
#[no_mangle]
pub static mut nblanks: core::ffi::c_int = 0;
static mut abc_file: [core::ffi::c_char; 99] = [0; 99];
static mut input_line: core::ffi::c_int = 0;
static mut just_comment: core::ffi::c_int = 0;
static mut bar_start: *mut Barline = 0 as *const Barline as *mut Barline;
static mut note: *mut Note = 0 as *const Note as *mut Note;
static mut current: *mut Symbol = 0 as *const Symbol as *mut Symbol;
static mut token: core::ffi::c_int = 0;
static mut token_length: core::ffi::c_int = 0 as core::ffi::c_int;
static mut token_ptr: *mut core::ffi::c_char = 0 as *const core::ffi::c_char
    as *mut core::ffi::c_char;
static mut n_blocks: core::ffi::c_int = 0;
static mut block: [*mut Symbol; 10] = [0 as *const Symbol as *mut Symbol; 10];
static mut n_symbols: core::ffi::c_int = 0 as core::ffi::c_int;
static mut fbar_total: frac = frac { n: 0, d: 0 };
static mut bar_no: core::ffi::c_int = 0;
static mut bar: [[core::ffi::c_char; 99]; 3] = [[0; 99]; 3];
static mut current_bar: [core::ffi::c_char; 99] = [0; 99];
static mut stave: core::ffi::c_int = 0;
static mut output: Output = Output {
    type_0: 0,
    transpose: 0,
    nbars: 0,
    warnings: 0,
};
static mut beam_length: core::ffi::c_int = 0 as core::ffi::c_int;
static mut continuation: core::ffi::c_int = 0 as core::ffi::c_int;
static mut ignore: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn abc_warning(mut fmt: *const core::ffi::c_char) {
    if output.warnings == std::os::raw::c_int(0) {
        return;
    }
    let mut args = std::ffi::VaListImpl{};
    
    // Get the CStr from the pointer
    let cstr_fmt = match std::ffi::CStr::from_ptr(fmt) {
        Err(_) => return,
        Ok(cstr) => cstr.to_bytes(),
    };
    let fmt_string: String = String::from_utf8(cstr_fmt).unwrap();
    
    println!("WARNING: line no. {} - {}", input_line as usize, fmt_string);
}
#[no_mangle]
pub extern "C" fn abc_error(fmt: *const std::os::raw::c_char) {
    let fmt = unsafe { std::ffi::CStr::from_ptr(fmt).to_str().unwrap() };

    eprintln!("error in input file {}", abc_file);
    println!("{}: line no. {} - {}", abc_file, input_line, fmt);
    std::process::exit(1);
}
#[no_mangle]
pub extern "C" fn read_settings() {
    let fp = std::fs::File::open("settings").ok()?;
    let mut reader = std::io::BufReader::new(fp);
    let mut line = String::with_capacity(99);
    
    for result in reader.lines() {
        match result {
            Err(_) => break,
            Ok(l) => line = l
        };
        
        if &line[..8] == "justify\0" {
            settings.justification = 1;
        } else if &line[..12] == "gchords above\0" {
            settings.gchords_above = 1;
        } else if &line[..9] == "autobeam\0" {
            settings.autobeam = 1;
        } else if &line[..11] == "oldrepeats\0" {
            settings.old_repeats = 1;
        } else if &line[..10] == "oldchords\0" {
            settings.old_chords = 1;
        } else if &line[..9] == "oldslurs\0" {
            settings.old_slurs = 1;
        } else if &line[..5] == "mine\0" {
            settings.mine = 1;
        } else {
            g_error("in settings – unrecognised line: %s", line);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_field(line_ptr: *mut core::ffi::c_char) -> i32 {
    let safe_line = SafeLine::from_ptr(line_ptr);
    
    if safe_line.get_char(1) == ':' && ('A'..='Z').contains(&safe_line.get_char(0)) {
        1
    } else {
        0
    }
}

pub struct SafeLine(*const core::ffi::c_char);
impl SafeLine {
    pub unsafe fn from_ptr(ptr: *mut core::ffi::c_char) -> Self {
        if ptr.is_null() {
            return Self(std::ptr::null());
        } else {
            let c_str = std::ffi::CStr::from_ptr(*ptr);
            let str_slice = c_str.to_bytes();
            
            // Convert C-strings to Rust Strings
            Self(&str_slice[0] as *const core::ffi::c_char)
        }
    }
    
    pub unsafe fn get_char(&self, index: usize) -> char {
        let c = unsafe { *(self.0.offset(index as isize)) };
        c as char
    }
}
#[no_mangle]
pub unsafe extern "C" fn strip(str_ptr: *mut core::ffi::c_char, comment_ptr: *mut core::ffi::c_char) {
    let mut str = std::ffi::CStr::from_ptr(str_ptr);
    let mut comment = std::ffi::CStr::from_ptr(comment_ptr);
    
    // Convert C-strings to Rust Strings
    let str: String = str.to_bytes().iter().map(|&c| c as char).collect();
    let mut comment: Vec<char> = comment.to_bytes().iter().map(|&c| c as char).collect();
    
    // Remove trailing spaces and comments
    if let Some(index) = str.rfind(|c| !matches_whitespace(c)) {
        comment.clear();
        
        for c in &str[..=index] {
            match *c {
                '%' | '\n' => break,
                ' ' | '\t' if comment.is_empty() => {},
                _ => comment.push(*c),
            }
        }
        
        // Update the C-strings with the new values
        let str = str[..=index].trim_end_matches(|c| matches_whitespace(c)).to_string();
        let comment: String = comment.into_iter().collect();
        
        std::ffi::CString::new(str).unwrap().into_raw(*str_ptr);
        std::ffi::CString::new(comment).unwrap().into_raw(*comment_ptr);
    } else {
        str.clear();
        comment.clear();
        
        std::ffi::CString::new("").unwrap().into_raw(*str_ptr);
        std::ffi::CString::new("").unwrap().into_raw(*comment_ptr);
    }
}
pub unsafe extern "C" fn stripcpy(out_str: *mut core::ffi::c_char, in_str: *mut core::ffi::c_char) {
    let safe_in_str = std::ffi::CStr::from_ptr(in_str);
    let mut stripped = String::new();
    for ch in safe_in_str.chars() {
        if !ch.is_whitespace() {
            stripped.push(ch);
        }
    }
    let c_stripped = stripped.as_ptr();
    strcpy(out_str, c_stripped as *mut core::ffi::c_char);
}

fn strcpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char) {
    let mut p1 = dest;
    let p2 = src;
    loop {
        if unsafe { *p2 } == '\0' as i8 {
            return;
        } else {
            unsafe {
                *p1 = *p2;
                p1 += 1;
                p2 += 1;
            }
        }
    }
}
pub fn output_transline(s: *mut core::ffi::c_char) -> i32 {
    let nblanks = 0 as isize;
    
    match unsafe { CString::from_raw(s) } {
        Err(_) => return 1,
        Ok(s) => {
            if fputs(s.as_ptr(), Trans) == std::ptr::null() {
                return 1;
            }
        },
    };
    0
}
#[no_mangle]
pub unsafe extern "C" fn get_dnl(entry: &mut Record) {
    let mut meter = ["0".repeat(98).as_bytes(), b'\0'];
    let mut meter_field: Field = Field::new();
    *meter_field.string() = meter.as_ptr().cast();
    unsafe {
        strcpy(*meter_field.string().cast_mut(), b"M:\0".as_ptr());
        strcat(*meter_field.string().cast_mut(), entry.fields[12]);
    }
    set_base(&mut meter_field);
    sprintf(entry.fields[11], b"1/%d\0", 32 / dnl);
}
#[no_mangle]
pub unsafe extern "C" fn set_dnl(mut dnl_str: *mut core::ffi::c_char) {
    while *dnl_str as i32 == ' ' as i32 {
        dnl_str = dnl_str.offset(1);
    }
    
    let sdl_str = std::ffi::CStr::from_ptr(dnl_str).to_bytes();
    if sdl_str[0..4] == b"1/4 " {
        *const_cast::<core::ffi::c_char>(dnl_str) = 6; // dnl = CROTCHET
    } else if sdl_str[0..4] == b"1/8 " {
        *const_cast::<core::ffi::c_char>(dnl_str) = 5; // dnl = QUAVER
    } else if sdl_str[0..5] == b"1/16 " {
        *const_cast::<core::ffi::c_char>(dnl_str) = 4; // dnl = SEMIQUAVER
    } else if sdl_str[0..5] == b"1/32 " {
        *const_cast::<core::ffi::c_char>(dnl_str) = 3; // dnl = DEMISEMIQUAVER
    } else {
        printf(b"default note length not recognised\n\0".as_ptr() as _, );
        *const_cast::<core::ffi::c_char>(dnl_str) = 5; // dnl = QUAVER
    };
}
#[no_mangle]
pub unsafe extern "C" fn range(
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut yfirst: *mut core::ffi::c_int,
    mut ylast: *mut core::ffi::c_int,
    mut input: *mut *const u8,
) -> core::ffi::c_int {
    let format = b"%d\0".as_ptr() as *const i8;

    *ylast = 4999;
    
    if **input == '\0' as u8 {
        *first = 4999;
        *last = 4999;
        *yfirst = 4999;
        return 1;
    }

    let input_str = std::ffi::CStr::from_ptr(unsafe { *input });
    
    if input_str.as_bytes()[0] == b'-'[0] as u8 {
        *first = 1;
        *yfirst = 1;
    } else if input_str.as_bytes()[0].is_numeric() {
        unsafe { sscanf(input, format, first); }
        
        while input_str.as_bytes()[0].is_numeric() {
            *input = (*input).offset(1);
            let input_str = std::ffi::CStr::from_ptr(unsafe { *input });
        }

        if input_str.as_bytes()[0] == b'.'[0] as u8 {
            *input = (*input).offset(1);
            
            if input_str.as_bytes()[0].is_numeric() {
                *yfirst = (input_str.as_bytes()[0] - b'0'[0]) as i32;
            } else if input_str.as_bytes()[0] >= b'a'[0] as u8 && input_str.as_bytes()[0] <= b'z'[0] as u8 {
                *yfirst = (input_str.as_bytes()[0] - b'a'[0] + 10) as i32;
            } else {
                return 0;
            }
            
            *input = (*input).offset(1);
        } else {
            *yfirst = 0;
        }
    } else {
        return 0;
    }

    let input_str = std::ffi::CStr::from_ptr(unsafe { *input });
    
    if input_str.as_bytes()[0] == b'-'[0] as u8 {
        *input = (*input).offset(1);
        
        if input_str.as_bytes()[0] == '\0' as u8 {
            *last = 4999;
        } else if input_str.as_bytes()[0].is_numeric() {
            unsafe { sscanf(input, format, last); }
            
            while input_str.as_bytes()[0].is_numeric() {
                *input = (*input).offset(1);
                let input_str = std::ffi::CStr::from_ptr(unsafe { *input });
            }

            if input_str.as_bytes()[0] == b'.'[0] as u8 {
                *input = (*input).offset(1);
                
                if input_str.as_bytes()[0].is_numeric() {
                    *ylast = (input_str.as_bytes()[0] - b'0'[0]) as i32;
                } else if input_str.as_bytes()[0] >= b'a'[0] as u8 && input_str.as_bytes()[0] <= b'z'[0] as u8 {
                    *ylast = (input_str.as_bytes()[0] - b'a'[0] + 10) as i32;
                } else {
                    return 0;
                }
                
                *input = (*input).offset(1);
            }
            
            if input_str.as_bytes()[0] == b','[0] as u8 {
                *input = (*input).offset(1);
            }
        } else {
            return 0;
        }
    } else if input_str.as_bytes()[0] == b','[0] as u8 {
        *last = *first;
        
        if *yfirst != 0 {
            *ylast = *yfirst;
        }
        
        *input = (*input).offset(1);
    } else if input_str.as_bytes()[0] == '\0' as u8 {
        *last = *first;
        
        if *yfirst != 0 {
            *ylast = *yfirst;
        }
    } else {
        return 0;
    }
    
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn process_abc(
    mut title: *mut [core::ffi::c_char; 99],
    mut titles: core::ffi::c_int,
    mut entry: *mut Record,
    mut key_comment: *mut core::ffi::c_char,
    mut bars: *mut core::ffi::c_char,
    mut tune: *mut core::ffi::c_char,
    mut input: *mut core::ffi::c_char,
    mut output_type: core::ffi::c_int,
    mut nbars: core::ffi::c_int,
    mut force: core::ffi::c_int,
    mut hash_array: *mut core::ffi::c_int,
) {
    let mut c: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut line: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut str: [core::ffi::c_char; 99] = [0; 99];
    let mut key: [core::ffi::c_char; 99] = [0; 99];
    let mut key_field: Field = Field {
        string: 0 as *mut core::ffi::c_char,
        info1: 0,
        info2: 0,
    };
    let mut meter: [core::ffi::c_char; 99] = [0; 99];
    let mut meter_field: Field = Field {
        string: 0 as *mut core::ffi::c_char,
        info1: 0,
        info2: 0,
    };
    let mut tempo: [core::ffi::c_char; 99] = [0; 99];
    let mut tempo_field: Field = Field {
        string: 0 as *mut core::ffi::c_char,
        info1: 0,
        info2: 0,
    };
    let mut s: *mut Symbol = 0 as *mut Symbol;
    output.type_0 = output_type;
    if offset != 0 || transpose != 0 {
        output.transpose = 1 as core::ffi::c_int;
    } else {
        output.transpose = 0 as core::ffi::c_int;
    }
    output.nbars = nbars;
    if output_type == TEX_OUTPUT as core::ffi::c_int
        || output_type == INDEX_OUTPUT as core::ffi::c_int
    {
        output.warnings = 1 as core::ffi::c_int;
    } else {
        output.warnings = 0 as core::ffi::c_int;
    }
    ignore = 0 as core::ffi::c_int;
    n_symbols = -(1 as core::ffi::c_int);
    n_blocks = -(1 as core::ffi::c_int);
    current = 0 as *mut Symbol;
    note = 0 as *mut Note;
    beam_length = 0 as core::ffi::c_int;
    new_symbol(BAR_LINE as core::ffi::c_int);
    if input.is_null() {
        line = calloc(
            999 as core::ffi::c_int as core::ffi::c_uint as size_t,
            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
        ) as *mut core::ffi::c_char;
        if line.is_null() {
            g_error(
                b"alloc failure in char_array\0" as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    bar_start = &mut (*current).u.bar;
    token = NEWLINE_TKN as core::ffi::c_int;
    treble = 0 as core::ffi::c_int;
    bass = treble;
    stave = bass;
    bar[2 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = '\0' as i32
        as core::ffi::c_char;
    bar[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = bar[2
        as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
    bar[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = bar[1
        as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
    current_bar[0 as core::ffi::c_int as usize] = bar[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize];
    key_field.string = &mut *key.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut core::ffi::c_char;
    key_field.info2 = 0 as core::ffi::c_int;
    key_field.info1 = key_field.info2;
    strcpy(key.as_mut_ptr(), b"K:\0" as *const u8 as *const core::ffi::c_char);
    strcat(key.as_mut_ptr(), (*entry).fields[10 as core::ffi::c_int as usize]);
    sharps_flats(&mut key_field);
    if offset != 0 || transpose != 0 {
        fprintf(Trans, b"%s\0" as *const u8 as *const core::ffi::c_char, key_comment);
    }
    meter_field.string = &mut *meter.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut core::ffi::c_char;
    meter_field.info2 = 0 as core::ffi::c_int;
    meter_field.info1 = meter_field.info2;
    strcpy(meter.as_mut_ptr(), b"M:\0" as *const u8 as *const core::ffi::c_char);
    strcat(meter.as_mut_ptr(), (*entry).fields[12 as core::ffi::c_int as usize]);
    set_base(&mut meter_field);
    if *((*entry).fields[11 as core::ffi::c_int as usize])
        .offset(0 as core::ffi::c_int as isize) != 0
    {
        set_dnl((*entry).fields[11 as core::ffi::c_int as usize]);
    }
    tempo_field.string = &mut *tempo.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut core::ffi::c_char;
    tempo_field.info2 = 0 as core::ffi::c_int;
    tempo_field.info1 = tempo_field.info2;
    if !((*entry).fields[16 as core::ffi::c_int as usize]).is_null()
        && *((*entry).fields[16 as core::ffi::c_int as usize])
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
    {
        strcpy(tempo.as_mut_ptr(), b"Q:\0" as *const u8 as *const core::ffi::c_char);
        strcat(tempo.as_mut_ptr(), (*entry).fields[16 as core::ffi::c_int as usize]);
        tempo_field.info1 = dnl;
    }
    while !input.is_null() || !(getsIn(line)).is_null() {
        fbar_total.n = 0 as core::ffi::c_int;
        fbar_total.d = 1 as core::ffi::c_int;
        bar_no = 0 as core::ffi::c_int;
        if !input.is_null() {
            line = input;
        }
        if *tune.offset(0 as core::ffi::c_int as isize) != 0 {
            strcat(tune, line);
        }
        i = 0 as core::ffi::c_int;
        while *line.offset(i as isize) as core::ffi::c_int == ' ' as i32
            || *line.offset(i as isize) as core::ffi::c_int == '\t' as i32
        {
            i += 1;
        }
        if *line.offset(i as isize) as core::ffi::c_int == '\n' as i32 {
            token_ptr = line;
            token_length = i + 1 as core::ffi::c_int;
            output_transpose();
            nblanks = 1 as core::ffi::c_int;
            ignore = 0 as core::ffi::c_int;
            break;
        } else {
            if output.type_0 == INDEX_OUTPUT as core::ffi::c_int
                && output.nbars == 0 as core::ffi::c_int
            {
                continue;
            }
            if *line.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == '\\' as i32 || is_field(line) != 0
            {
                token_ptr = line;
                token_length = strlen(line) as core::ffi::c_int;
                process_field(line);
                if output.nbars != 0 {
                    if *line.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        == 'L' as i32
                    {
                        strcpy(
                            (*entry).fields[11 as core::ffi::c_int as usize],
                            &mut *line.offset(2 as core::ffi::c_int as isize),
                        );
                    } else if *line.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int == 'M' as i32
                    {
                        strcpy(
                            (*entry).fields[12 as core::ffi::c_int as usize],
                            &mut *line.offset(2 as core::ffi::c_int as isize),
                        );
                    }
                }
            } else {
                continuation = 0 as core::ffi::c_int;
                c = 0 as core::ffi::c_int;
                while *line.offset(c as isize) != 0 {
                    if output.nbars == 0 as core::ffi::c_int && nbars != 0 {
                        break;
                    }
                    token_ptr = &mut *line.offset(c as isize) as *mut core::ffi::c_char;
                    token_length = 1 as core::ffi::c_int;
                    let mut current_block_188: u64;
                    match *line.offset(c as isize) as core::ffi::c_int {
                        67 | 68 | 69 | 70 | 71 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'C' as i32
                                    + 16 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        65 | 66 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'A' as i32
                                    + 21 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        99 | 100 | 101 | 102 | 103 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'c' as i32
                                    + 23 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        97 | 98 => {
                            process_note(
                                *line.offset(c as isize) as core::ffi::c_int - 'a' as i32
                                    + 28 as core::ffi::c_int + offset,
                            );
                            current_block_188 = 13193481930143188038;
                        }
                        122 => {
                            process_note(-(1 as core::ffi::c_int));
                            current_block_188 = 13193481930143188038;
                        }
                        32 | 9 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == ' ' as i32
                                || *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int == '\t' as i32
                            {
                                token_length += 1;
                            }
                            if *line.offset((c + token_length) as isize)
                                as core::ffi::c_int != '%' as i32
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int != '\n' as i32
                            {
                                process_space();
                                current_block_188 = 13193481930143188038;
                            } else {
                                current_block_188 = 11473691171471082492;
                            }
                        }
                        37 => {
                            current_block_188 = 11473691171471082492;
                        }
                        124 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '|' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(DBL_BAR as core::ffi::c_int);
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == ']' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(RDBL_BAR as core::ffi::c_int);
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == ':' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(LREPEAT as core::ffi::c_int);
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '1' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(BAR1 as core::ffi::c_int);
                            } else {
                                process_bar(BAR as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        58 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '|' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                if *line.offset((c + 2 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int == '2' as i32
                                {
                                    token_length += 1 as core::ffi::c_int;
                                    process_bar(RREPEAT2 as core::ffi::c_int);
                                } else {
                                    process_bar(RREPEAT as core::ffi::c_int);
                                }
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == ':' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(REPEAT as core::ffi::c_int);
                            } else {
                                abc_error(
                                    b"\0" as *const u8 as *const core::ffi::c_char
                                        as *mut core::ffi::c_char,
                                );
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        91 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '1' as i32
                                || *line.offset((c + 1 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int == '2' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_repeat(
                                    atoi(
                                        &mut *line.offset((c + 1 as core::ffi::c_int) as isize),
                                    ),
                                );
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '|' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_bar(LDBL_BAR as core::ffi::c_int);
                            } else {
                                process_open_chord();
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        93 => {
                            process_close_chord();
                            current_block_188 = 13193481930143188038;
                        }
                        49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 => {
                            while !(strchr(
                                b"0123456789\0" as *const u8 as *const core::ffi::c_char,
                                *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_length(atoi(&mut *line.offset(c as isize)));
                            current_block_188 = 13193481930143188038;
                        }
                        47 => {
                            if !(strchr(
                                b"0123456789\0" as *const u8 as *const core::ffi::c_char,
                                *line.offset((c + 1 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                while !(strchr(
                                    b"0123456789\0" as *const u8 as *const core::ffi::c_char,
                                    *line.offset((c + token_length) as isize)
                                        as core::ffi::c_int,
                                ))
                                    .is_null()
                                {
                                    token_length += 1 as core::ffi::c_int;
                                }
                                process_divisor(
                                    atoi(
                                        &mut *line.offset((c + 1 as core::ffi::c_int) as isize),
                                    ),
                                );
                            } else if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '/' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_divisor(4 as core::ffi::c_int);
                            } else {
                                process_divisor(2 as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        62 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == '>' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_broken(token_length);
                            current_block_188 = 13193481930143188038;
                        }
                        60 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == '<' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_broken(-(1 as core::ffi::c_int) * token_length);
                            current_block_188 = 13193481930143188038;
                        }
                        43 => {
                            process_open_close_chord();
                            current_block_188 = 13193481930143188038;
                        }
                        115 => {
                            process_open_close_slur();
                            current_block_188 = 13193481930143188038;
                        }
                        42 => {
                            process_justify();
                            current_block_188 = 13193481930143188038;
                        }
                        123 => {
                            process_open_grace();
                            current_block_188 = 13193481930143188038;
                        }
                        125 => {
                            process_close_grace();
                            current_block_188 = 13193481930143188038;
                        }
                        40 => {
                            if !(strchr(
                                b"123456789\0" as *const u8 as *const core::ffi::c_char,
                                *line.offset((c + 1 as core::ffi::c_int) as isize)
                                    as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                while !(strchr(
                                    b":0123456789\0" as *const u8 as *const core::ffi::c_char,
                                    *line.offset((c + token_length) as isize)
                                        as core::ffi::c_int,
                                ))
                                    .is_null()
                                {
                                    token_length += 1 as core::ffi::c_int;
                                }
                                process_tuplet(
                                    &mut *line.offset((c + 1 as core::ffi::c_int) as isize),
                                );
                            } else {
                                process_open_slur();
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        41 => {
                            process_close_slur();
                            current_block_188 = 13193481930143188038;
                        }
                        45 => {
                            process_tie();
                            current_block_188 = 13193481930143188038;
                        }
                        126 | 46 | 117 | 118 => {
                            process_accent(*line.offset(c as isize));
                            current_block_188 = 13193481930143188038;
                        }
                        94 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '^' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_accidental(DBL_SHARP as core::ffi::c_int);
                            } else {
                                process_accidental(SHARP as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        95 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '_' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                                process_accidental(DBL_FLAT as core::ffi::c_int);
                            } else {
                                process_accidental(FLAT as core::ffi::c_int);
                            }
                            current_block_188 = 13193481930143188038;
                        }
                        61 => {
                            process_accidental(NATURAL as core::ffi::c_int);
                            current_block_188 = 13193481930143188038;
                        }
                        39 => {
                            process_octaver(7 as core::ffi::c_int);
                            current_block_188 = 13193481930143188038;
                        }
                        44 => {
                            process_octaver(-(7 as core::ffi::c_int));
                            current_block_188 = 13193481930143188038;
                        }
                        34 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int != 0
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int != '"' as i32
                            {
                                str[(token_length - 1 as core::ffi::c_int) as usize] = *line
                                    .offset((c + token_length) as isize);
                                token_length += 1;
                            }
                            if *line.offset((c + token_length) as isize)
                                as core::ffi::c_int == '\0' as i32
                            {
                                abc_error(
                                    b"line ended mid gchord\0" as *const u8
                                        as *const core::ffi::c_char as *mut core::ffi::c_char,
                                );
                            }
                            token_length += 1 as core::ffi::c_int;
                            str[(token_length - 2 as core::ffi::c_int) as usize] = '\0'
                                as i32 as core::ffi::c_char;
                            process_gchord(str.as_mut_ptr());
                            current_block_188 = 13193481930143188038;
                        }
                        92 => {
                            process_continuation();
                            current_block_188 = 13193481930143188038;
                        }
                        10 => {
                            process_newline();
                            current_block_188 = 13193481930143188038;
                        }
                        38 => {
                            if *line.offset((c + 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_int == '&' as i32
                            {
                                token_length += 1 as core::ffi::c_int;
                            }
                            process_ampersand(token_length);
                            current_block_188 = 13193481930143188038;
                        }
                        _ => {
                            if !(strchr(
                                b"HIJKLMNOPQRSTUVWXYZ\0" as *const u8
                                    as *const core::ffi::c_char,
                                *line.offset(c as isize) as core::ffi::c_int,
                            ))
                                .is_null()
                            {
                                process_macro(*line.offset(c as isize));
                            } else {
                                abc_error(
                                    b"unrecognised symbol\0" as *const u8
                                        as *const core::ffi::c_char as *mut core::ffi::c_char,
                                );
                            }
                            current_block_188 = 13193481930143188038;
                        }
                    }
                    match current_block_188 {
                        11473691171471082492 => {
                            while *line.offset((c + token_length) as isize)
                                as core::ffi::c_int != 0
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int != '\n' as i32
                            {
                                token_length += 1;
                            }
                            if just_comment != 0
                                && *line.offset((c + token_length) as isize)
                                    as core::ffi::c_int == '\n' as i32
                            {
                                token_length += 1;
                            }
                            process_trailing();
                        }
                        _ => {}
                    }
                    c += token_length;
                }
                if just_comment == 0 as core::ffi::c_int {
                    if output.nbars != 0
                        && current_bar[0 as core::ffi::c_int as usize]
                            as core::ffi::c_int != 0 && bar_no < 3 as core::ffi::c_int
                    {
                        strcpy(
                            (bar[bar_no as usize]).as_mut_ptr(),
                            current_bar.as_mut_ptr(),
                        );
                    }
                    output.nbars = 0 as core::ffi::c_int;
                }
                if !input.is_null() {
                    break;
                }
            }
        }
    }
    end_beam();
    (*current).newline = 2 as core::ffi::c_int;
    if settings.justification != 0 {
        (*current).justify = 1 as core::ffi::c_int;
    }
    n_symbols += 1 as core::ffi::c_int;
    process_beams(
        n_symbols,
        &mut *(*block.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
            .offset(0 as core::ffi::c_int as isize),
    );
    if output.type_0 == TEX_OUTPUT as core::ffi::c_int {
        tune2tex(
            title,
            titles,
            entry,
            n_symbols,
            &mut *(*block.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                .offset(0 as core::ffi::c_int as isize),
            &mut key_field,
            &mut meter_field,
            &mut tempo_field,
        );
    } else if !hash_array.is_null() {
        tune2hash(&mut key_field, hash_array, force);
    } else if nbars != 0 {
        *bars.offset(0 as core::ffi::c_int as isize) = '\0' as i32 as core::ffi::c_char;
        if nbars == ONE_BAR_PLUS as core::ffi::c_int
            || nbars == TWO_BARS_PLUS as core::ffi::c_int
        {
            strcpy(bars, (bar[0 as core::ffi::c_int as usize]).as_mut_ptr());
        }
        strcat(bars, (bar[1 as core::ffi::c_int as usize]).as_mut_ptr());
        if nbars == TWO_BARS as core::ffi::c_int
            || nbars == TWO_BARS_PLUS as core::ffi::c_int
        {
            strcat(bars, (bar[2 as core::ffi::c_int as usize]).as_mut_ptr());
        }
    }
    s = &mut *(*block.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
        .offset(0 as core::ffi::c_int as isize) as *mut Symbol;
    while !s.is_null() {
        if (*s).type_0 == NOTE as core::ffi::c_int && !((*s).u.note.gchord).is_null() {
            free((*s).u.note.gchord as *mut core::ffi::c_void);
        } else if (*s).type_0 == FIELD as core::ffi::c_int {
            free((*s).u.field.string as *mut core::ffi::c_void);
        }
        s = (*s).next as *mut Symbol;
    }
    i = 0 as core::ffi::c_int;
    while i <= n_blocks {
        free(block[i as usize] as *mut core::ffi::c_void);
        i += 1;
    }
    if input.is_null() {
        free(line as *mut core::ffi::c_void);
    }
}
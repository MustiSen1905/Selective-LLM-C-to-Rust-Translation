fn main() {
        cc::Build::new()
            .file("safe_crc_32.c")
            .include(".")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
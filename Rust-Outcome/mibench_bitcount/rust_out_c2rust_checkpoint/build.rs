fn main() {
        cc::Build::new()
            .file("safe_bitcnt_1.c").file("safe_bitcnt_2.c").file("safe_bitcnt_3.c").file("safe_bitcnt_4.c").file("safe_bitfiles.c")
            .include(".")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
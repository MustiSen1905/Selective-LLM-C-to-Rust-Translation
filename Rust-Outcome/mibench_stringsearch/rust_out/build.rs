fn main() {
        cc::Build::new()
            .file("safe_bmhasrch.c").file("safe_bmhisrch.c").file("safe_bmhsrch.c").file("safe_pbmsrch.c")
            .include(".")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
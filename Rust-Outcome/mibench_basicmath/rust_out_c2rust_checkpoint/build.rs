fn main() {
        cc::Build::new()
            .file("safe_isqrt.c").file("safe_rad2deg.c")
            .include(".")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
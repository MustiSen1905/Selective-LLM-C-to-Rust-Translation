fn main() {
        cc::Build::new()
            .file("safe_main.c").file("core/safe_calc.c")
            .include("core").include("util")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
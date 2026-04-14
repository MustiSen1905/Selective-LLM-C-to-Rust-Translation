fn main() {
        cc::Build::new()
            .file("safe_main.c").file("safe_pdf.c")
            .include(".")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
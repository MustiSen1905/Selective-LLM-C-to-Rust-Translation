fn main() {
        cc::Build::new()
            .file("safe_abc.c").file("safe_fields.c").file("safe_index.c").file("safe_search.c").file("safe_sort_in.c").file("safe_tex.c")
            .include(".")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
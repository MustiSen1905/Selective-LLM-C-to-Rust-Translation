fn main() {
    cc::Build::new().files(["abc_c_safe.c", "fields_c_safe.c", "index_c_safe.c", "search_c_safe.c", "sort_in_c_safe.c", "tex_c_safe.c"]).include(".").flag("-std=gnu89").flag("-fcommon").warnings(false).compile("c_parts");
}
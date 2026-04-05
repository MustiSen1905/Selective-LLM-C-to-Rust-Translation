fn main() {
    cc::Build::new().files(["safe_abc.c", "safe_fields.c", "safe_index.c", "safe_search.c", "safe_sort_in.c", "safe_tex.c"]).include(".").flag("-std=c99").flag("-fcommon").warnings(false).compile("c_parts");
}
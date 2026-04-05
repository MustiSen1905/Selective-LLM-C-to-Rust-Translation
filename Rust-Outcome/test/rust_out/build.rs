fn main() {
    cc::Build::new().files(["safe_utils.c"]).include(".").flag("-std=c99").flag("-fcommon").warnings(false).compile("c_parts");
}
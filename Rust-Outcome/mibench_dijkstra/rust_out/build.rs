fn main() {
        cc::Build::new()
            .file("safe_dijkstra.c")
            .include(".")
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }
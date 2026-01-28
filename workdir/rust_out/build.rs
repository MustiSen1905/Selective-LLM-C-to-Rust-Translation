fn main() {
    cc::Build::new()
        .file("safe.c")
        .include(".")
        .compile("safe");
}

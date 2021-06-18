fn main() {
    cc::Build::new()
        .file("src/extlib.c")
        .compile("extlib");
    println!("cargo:rerun-if-changed=src/extlib.c")
}
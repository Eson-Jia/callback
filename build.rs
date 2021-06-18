fn main() {
    println!("cargo:rustc-link-lib=extlib");
    // println!("cargo:rustc-link-search=.");
}

// fn main(){
// cc::Build::new()
//     .file("src/extlib.c")
//     .compile("extlib");
//     println!("cargo:rerun-if-changed=src/extlib.c");
// }
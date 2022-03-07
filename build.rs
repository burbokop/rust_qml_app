// build.rs


fn main() {    
    println!("cargo:rustc-flags=-l inkview -l hwconfig");
    println!("cargo:rerun-if-changed=build.rs");
}

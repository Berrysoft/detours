//! Build detours.
fn build_detours() {
    cc::Build::new()
        .include("deps/detours-src/src/")
        .define("WIN32_LEAN_AND_MEAN", "1")
        .define("_WIN32_WINNT", "0x501")
        .file("deps/detours-src/src/detours.cpp")
        .file("deps/detours-src/src/modules.cpp")
        .file("deps/detours-src/src/disasm.cpp")
        .file("deps/detours-src/src/image.cpp")
        .file("deps/detours-src/src/creatwth.cpp")
        .cpp(true)
        .compile("detours");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_detours();
}

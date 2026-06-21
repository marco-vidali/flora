fn main() {
    // recompile if boot.s or entry.s change
    println!("cargo:rerun-if-changed=src/board/boot.s");
    println!("cargo:rerun-if-changed=src/board/entry.s");

    // initialize and configure the assembler
    let mut build = cc::Build::new();

    build
        .file("src/board/boot.s")
        .file("src/board/entry.s")
        .target("aarch64-unknown-none")
        .compiler("clang");

    build.flag("-target");
    build.flag("aarch64-unknown-none");

    // do not generate warnings for unused flags
    build.flag("-Qunused-arguments");

    // do not generate static archive
    build.cargo_metadata(false);

    // generate only object files
    let output = build.compile_intermediates();

    // compile generated object files
    for path in output {
        println!("cargo:rustc-link-arg={}", path.to_str().unwrap());
    }
}

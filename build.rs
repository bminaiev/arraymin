fn main() {
    //
    cc::Build::new()
        .file("src/c_impl.c")
        // .flag("-march=native")
        .compile("c_impl");

    cargo_emit::rerun_if_changed!("src/c_impl.c");
}

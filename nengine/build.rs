fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = std::path::PathBuf::from(out_dir);

    if target_os == "linux" {
        bindgen::builder()
            .header("src/ffi/xcb.h")
            .prepend_enum_name(false)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Failed to generate bindings for XCB")
            .write_to_file(out_path.join("xcb_bindings.rs"))
            .expect("Failed to write bindings to a file");
    } else {
        panic!("Only Linux is supported for now.")
    }
}

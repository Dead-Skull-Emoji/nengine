fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    if target_os == "linux" {
        let outpath = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        
        bindgen::builder()
            .header("src/backend/raw.h")
            .prepend_enum_name(false)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Failed to generate FFI bindings")
            .write_to_file(outpath.join("raw.rs"))
            .expect("Failed to write bindings to a file");
    } else if target_os == "windows" {
        panic!("Windows is not supported yet.");
    }
}
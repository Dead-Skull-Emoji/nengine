fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    if target_os == "linux" {
        let outpath = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        
        println!("cargo:rustc-link-lib=vulkan");
        
        bindgen::builder()
            .header("src/backend/raw.h")
            .prepend_enum_name(false)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Failed to generate FFI bindings")
            .write_to_file(outpath.join("raw.rs"))
            .expect("Failed to write bindings to a file");
    } else if target_os == "windows" {
        let outpath = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let vulkan_location = std::env::var("VULKAN_SDK").expect("Unable to find the Vulkan SDK!");
        
        println!("cargo:rustc-link-search={}", vulkan_location.clone() + "\\Lib");
        println!("cargo:rustc-link-lib=vulkan-1");
        
        bindgen::builder()
            .clang_arg(format!("-I{}", vulkan_location + "\\Include"))
            .header("src/backend/raw.h")
            .prepend_enum_name(false)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Failed to generate FFI bindings")
            .write_to_file(outpath.join("raw.rs"))
            .expect("Failed to write bindings to a file");
    } else {
        panic!("Unsupported operating system!");
    }
}
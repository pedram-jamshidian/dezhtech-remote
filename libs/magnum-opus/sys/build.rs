use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let is_windows = target.contains("windows");
    
    // ======================================
    // مرحله ۱: پیدا کردن کتابخانه opus
    // ======================================
    let mut include_paths: Vec<PathBuf> = Vec::new();
    
    #[cfg(feature = "build-opus")]
    {
        // ساخت opus از سورس با cmake
        let mut config = cmake::Config::new("opus");
        config.define("BUILD_SHARED_LIBS", "OFF");
        config.define("OPUS_BUILD_PROGRAMS", "OFF");
        config.define("OPUS_BUILD_TESTING", "OFF");
        let dst = config.build();
        
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-lib=static=opus");
        include_paths.push(dst.join("include"));
    }
    
    #[cfg(not(feature = "build-opus"))]
    {
        if is_windows {
            // ویندوز: استفاده از vcpkg
            find_opus_windows(&mut include_paths);
        } else {
            // لینوکس/مک: استفاده از pkg-config
            find_opus_unix(&mut include_paths);
        }
    }
    
    // ======================================
    // مرحله ۲: تولید bindings با bindgen
    // ======================================
    generate_bindings(&out_path, &include_paths);
}

/// پیدا کردن opus در ویندوز با vcpkg
#[cfg(not(feature = "build-opus"))]
fn find_opus_windows(include_paths: &mut Vec<PathBuf>) {
    println!("cargo:info=Searching for opus via vcpkg on Windows...");
    
    // تلاش اول: vcpkg crate
    if let Ok(lib) = vcpkg::find_package("opus") {
        println!("cargo:info=Found opus via vcpkg crate");
        for path in &lib.include_paths {
            println!("cargo:info=Include path: {}", path.display());
            include_paths.push(path.clone());
        }
        return;
    }
    
    // تلاش دوم: مسیرهای دستی vcpkg
    let vcpkg_paths = [
        // GitHub Actions با VCPKG_ROOT
        env::var("VCPKG_ROOT").ok().map(|r| PathBuf::from(r).join("installed/x64-windows-static")),
        // مسیر vcpkg_installed در پروژه
        Some(PathBuf::from("vcpkg_installed/x64-windows-static")),
        Some(PathBuf::from("vcpkg_installed/x64-windows")),
        // مسیرهای معمول
        Some(PathBuf::from("C:/vcpkg/installed/x64-windows-static")),
        Some(PathBuf::from("C:/vcpkg/installed/x64-windows")),
    ];
    
    for path_opt in vcpkg_paths.iter() {
        if let Some(base_path) = path_opt {
            let include_dir = base_path.join("include");
            let lib_dir = base_path.join("lib");
            
            if include_dir.join("opus/opus.h").exists() || include_dir.join("opus.h").exists() {
                println!("cargo:info=Found opus at: {}", base_path.display());
                println!("cargo:rustc-link-search=native={}", lib_dir.display());
                println!("cargo:rustc-link-lib=static=opus");
                include_paths.push(include_dir);
                return;
            }
        }
    }
    
    // تلاش سوم: متغیر OPUS_INCLUDE_DIR
    if let Ok(opus_include) = env::var("OPUS_INCLUDE_DIR") {
        let include_dir = PathBuf::from(&opus_include);
        if include_dir.exists() {
            println!("cargo:info=Using OPUS_INCLUDE_DIR: {}", opus_include);
            include_paths.push(include_dir);
            
            if let Ok(opus_lib) = env::var("OPUS_LIB_DIR") {
                println!("cargo:rustc-link-search=native={}", opus_lib);
            }
            println!("cargo:rustc-link-lib=static=opus");
            return;
        }
    }
    
    panic!(
        "Could not find opus library!\n\
        Please ensure vcpkg is properly configured:\n\
        1. Set VCPKG_ROOT environment variable\n\
        2. Run: vcpkg install opus:x64-windows-static\n\
        3. Or set OPUS_INCLUDE_DIR and OPUS_LIB_DIR"
    );
}

/// پیدا کردن opus در لینوکس/مک با pkg-config
#[cfg(not(feature = "build-opus"))]
fn find_opus_unix(include_paths: &mut Vec<PathBuf>) {
    println!("cargo:info=Searching for opus via pkg-config...");
    
    match pkg_config::Config::new().atleast_version("1.1").probe("opus") {
        Ok(lib) => {
            println!("cargo:info=Found opus via pkg-config");
            for path in lib.include_paths {
                include_paths.push(path);
            }
        }
        Err(e) => {
            panic!(
                "Could not find opus via pkg-config: {}\n\
                Please install libopus-dev:\n\
                  Ubuntu/Debian: sudo apt install libopus-dev\n\
                  Fedora: sudo dnf install opus-devel\n\
                  macOS: brew install opus",
                e
            );
        }
    }
}

/// تولید bindings با bindgen
fn generate_bindings(out_path: &PathBuf, include_paths: &[PathBuf]) {
    println!("cargo:rerun-if-changed=wrapper.h");
    
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("opus_.*")
        .allowlist_type("Opus.*")
        .allowlist_var("OPUS_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    
    // اضافه کردن مسیرهای include
    for path in include_paths {
        let clang_arg = format!("-I{}", path.display());
        println!("cargo:info=Adding clang arg: {}", clang_arg);
        builder = builder.clang_arg(&clang_arg);
    }
    
    // مسیرهای اضافی برای ویندوز
    if env::var("TARGET").unwrap().contains("windows") {
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            let extra_include = format!("-I{}/installed/x64-windows-static/include", vcpkg_root);
            builder = builder.clang_arg(&extra_include);
        }
    }
    
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    println!("cargo:info=Successfully generated bindings.rs");
}

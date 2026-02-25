use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let is_windows = target.contains("windows");

    let mut include_paths: Vec<PathBuf> = Vec::new();

    if is_windows {
        find_opus_windows(&mut include_paths);
    } else {
        find_opus_unix(&mut include_paths);
    }

    generate_bindings(&out_path, &include_paths);
}

fn find_opus_windows(include_paths: &mut Vec<PathBuf>) {
    println!("cargo:info=Searching for opus on Windows...");

    // تلاش ۱: متغیر OPUS_INCLUDE_DIR (بهترین روش برای CI)
    if let (Ok(inc), Ok(lib)) = (env::var("OPUS_INCLUDE_DIR"), env::var("OPUS_LIB_DIR")) {
        let inc_path = PathBuf::from(&inc);
        let lib_path = PathBuf::from(&lib);
        if inc_path.exists() && lib_path.exists() {
            println!("cargo:info=Using OPUS_INCLUDE_DIR={}", inc);
            println!("cargo:info=Using OPUS_LIB_DIR={}", lib);
            println!("cargo:rustc-link-search=native={}", lib);
            println!("cargo:rustc-link-lib=static=opus");
            include_paths.push(inc_path);
            return;
        }
    }

    // تلاش ۲: vcpkg crate
    let triplets = ["x64-windows-static-md", "x64-windows-static", "x64-windows"];
    for triplet in &triplets {
        std::env::set_var("VCPKGRS_TRIPLET", triplet);
        if let Ok(lib) = vcpkg::Config::new()
            .emit_includes(true)
            .find_package("opus")
        {
            println!("cargo:info=Found opus via vcpkg with triplet {}", triplet);
            for path in &lib.include_paths {
                include_paths.push(path.clone());
            }
            return;
        }
    }

    // تلاش ۳: مسیرهای دستی vcpkg
    let vcpkg_root = env::var("VCPKG_ROOT").unwrap_or_else(|_| "C:\\vcpkg".to_string());
    let search_paths = vec![
        format!("{}/installed/x64-windows-static-md", vcpkg_root),
        format!("{}/installed/x64-windows-static", vcpkg_root),
        format!("{}/installed/x64-windows", vcpkg_root),
        "vcpkg_installed/x64-windows-static-md".to_string(),
        "vcpkg_installed/x64-windows-static".to_string(),
        "vcpkg_installed/x64-windows".to_string(),
    ];

    for base in &search_paths {
        let inc = PathBuf::from(base).join("include");
        let lib = PathBuf::from(base).join("lib");
        let opus_h = inc.join("opus").join("opus.h");

        if opus_h.exists() {
            println!("cargo:info=Found opus at: {}", base);
            println!("cargo:rustc-link-search=native={}", lib.display());
            println!("cargo:rustc-link-lib=static=opus");
            include_paths.push(inc);
            return;
        }
    }

    panic!(
        "Could not find opus library on Windows!\n\
        Set OPUS_INCLUDE_DIR and OPUS_LIB_DIR environment variables,\n\
        or install via vcpkg: vcpkg install opus:x64-windows-static-md"
    );
}

fn find_opus_unix(include_paths: &mut Vec<PathBuf>) {
    match pkg_config::Config::new()
        .atleast_version("1.1")
        .probe("opus")
    {
        Ok(lib) => {
            for path in lib.include_paths {
                include_paths.push(path);
            }
        }
        Err(e) => {
            panic!("Could not find opus via pkg-config: {}", e);
        }
    }
}

fn generate_bindings(out_path: &PathBuf, include_paths: &[PathBuf]) {
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("opus_.*")
        .allowlist_type("Opus.*")
        .allowlist_var("OPUS_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    for path in include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    // مسیر اضافی برای ویندوز
    if let Ok(opus_inc) = env::var("OPUS_INCLUDE_DIR") {
        builder = builder.clang_arg(format!("-I{}", opus_inc));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:info=Successfully generated bindings.rs");
}

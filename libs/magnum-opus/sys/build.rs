use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    #[cfg(feature = "build-opus")]
    {
        let mut config = cmake::Config::new("opus");
        config.define("BUILD_SHARED_LIBS", "OFF");
        config.define("OPUS_BUILD_PROGRAMS", "OFF");
        config.define("OPUS_BUILD_TESTING", "OFF");
        let dst = config.build();
        
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-lib=static=opus");
    }
    
    #[cfg(not(feature = "build-opus"))]
    {
        pkg_config::Config::new()
            .atleast_version("1.1")
            .probe("opus")
            .unwrap();
    }
    
    println!("cargo:rerun-if-changed=wrapper.h");
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("opus_.*")
        .allowlist_type("Opus.*")
        .allowlist_var("OPUS_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(OpusCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
struct OpusCallbacks;

impl bindgen::callbacks::ParseCallbacks for OpusCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        patch_missing_constants(original_item_name)
    }
}

fn patch_missing_constants(name: &str) -> Option<String> {
    // Constants that may be missing in some versions
    let constants = [
        ("OPUS_OK", 0),
        ("OPUS_BAD_ARG", -1),
        ("OPUS_BUFFER_TOO_SMALL", -2),
        ("OPUS_INTERNAL_ERROR", -3),
        ("OPUS_INVALID_PACKET", -4),
        ("OPUS_UNIMPLEMENTED", -5),
        ("OPUS_INVALID_STATE", -6),
        ("OPUS_ALLOC_FAIL", -7),
        
        ("OPUS_APPLICATION_VOIP", 2048),
        ("OPUS_APPLICATION_AUDIO", 2049),
        ("OPUS_APPLICATION_RESTRICTED_LOWDELAY", 2051),
        
        ("OPUS_SIGNAL_VOICE", 3001),
        ("OPUS_SIGNAL_MUSIC", 3002),
        
        ("OPUS_BANDWIDTH_NARROWBAND", 1101),
        ("OPUS_BANDWIDTH_MEDIUMBAND", 1102),
        ("OPUS_BANDWIDTH_WIDEBAND", 1103),
        ("OPUS_BANDWIDTH_SUPERWIDEBAND", 1104),
        ("OPUS_BANDWIDTH_FULLBAND", 1105),
        
        ("OPUS_FRAMESIZE_ARG", 5000),
        ("OPUS_FRAMESIZE_2_5_MS", 5001),
        ("OPUS_FRAMESIZE_5_MS", 5002),
        ("OPUS_FRAMESIZE_10_MS", 5003),
        ("OPUS_FRAMESIZE_20_MS", 5004),
        ("OPUS_FRAMESIZE_40_MS", 5005),
        ("OPUS_FRAMESIZE_60_MS", 5006),
        ("OPUS_FRAMESIZE_80_MS", 5007),
        ("OPUS_FRAMESIZE_100_MS", 5008),
        ("OPUS_FRAMESIZE_120_MS", 5009),
        
        // Control codes
        ("OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST", 4046),
        ("OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST", 4047),
        ("OPUS_GET_IN_DTX_REQUEST", 4049),
    ];
    
    // Check if this constant needs patching
    for (const_name, _value) in &constants {
        if name == *const_name {
            // Don't rename, bindgen will handle it
            return None;
        }
    }
    
    None
}

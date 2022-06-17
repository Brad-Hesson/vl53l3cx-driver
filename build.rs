use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src", "core", "src", "vl53lx_api.c"].iter().collect();
    let search_paths: Vec<PathBuf> = vec![
        [env!("CARGO_MANIFEST_DIR"), "src", "wrapper"].iter().collect(),
        [env!("CARGO_MANIFEST_DIR"), "src", "core", "src"].iter().collect(),
        [env!("CARGO_MANIFEST_DIR"), "src", "core", "inc"].iter().collect(),
    ];
    run_bindgen(file.clone(), search_paths.clone());
    compile_c_code(file, search_paths);
}

fn compile_c_code(file: PathBuf, search_paths: Vec<PathBuf>) {
    let mut build = cc::Build::new();
    for dir in search_paths.clone() {
        for file in fs::read_dir(dir).unwrap() {
            let path = file.unwrap().path();
            if path.extension().unwrap() == "c" {
                println!("Compiling: {}", path.display());
                build.file(path);
            }
        }
    }
    for sp in search_paths {
        build.include(sp);
    }
    build.flag("-Wno-builtin-declaration-mismatch");
    build.flag("-Wno-implicit-function-declaration");
    build.flag("-Wno-format");
    build.compile(file.file_stem().unwrap().to_str().unwrap());
}

fn run_bindgen(file: PathBuf, search_paths: Vec<PathBuf>) {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", file.display());

    let mut builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("cty")
        .use_core();
    builder = builder.header(file.to_str().unwrap());
    for sp in search_paths {
        builder = builder.clang_arg(format!("-I{}", sp.display()));
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

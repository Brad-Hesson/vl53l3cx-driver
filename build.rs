#![feature(iter_intersperse)]
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let file: PathBuf = ["src", "core", "src", "vl53lx_api.c"].iter().collect();
    let search_paths: Vec<PathBuf> = vec![
        ["src", "wrapper"].iter().collect(),
        ["src", "core", "src"].iter().collect(),
        ["src", "core", "inc"].iter().collect(),
    ];
    run_bindgen(file.clone(), search_paths.clone());
    compile_c_code(file, search_paths);
}

fn compile_c_code(file: PathBuf, search_paths: Vec<PathBuf>) {
    let mut build = cc::Build::new();
    build.compiler("clang");
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
    if let Some(target) = get_riscv_target_fixed() {
        build.target(target.as_str());
    }
    build.flag_if_supported("-Wno-builtin-declaration-mismatch");
    build.flag_if_supported("-Wno-missing-declarations");
    build.flag("-Wno-#pragma-messages");
    build.flag("-Wno-implicit-function-declaration");
    build.flag_if_supported("-Wno-format");
    build.compile(file.file_stem().unwrap().to_str().unwrap());
}

fn run_bindgen(file: PathBuf, search_paths: Vec<PathBuf>) {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", file.display());

    let mut builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("cty")
        .header(file.to_str().unwrap())
        .use_core();
    if let Some(target) = get_riscv_target_fixed() {
        builder = builder.clang_arg(format!("--target={}", target));
    }
    for sp in search_paths {
        builder = builder.clang_arg(format!("-I{}", sp.display()));
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_riscv_target_fixed() -> Option<String> {
    let target = env::var("TARGET").expect("Target environment variable should be set");
    let mut triple = target.split("-").collect::<Vec<_>>();
    if triple[0].starts_with("riscv") {
        triple[0] = &triple[0][..7];
        return Some(triple.into_iter().intersperse("-").collect());
    }
    None
}

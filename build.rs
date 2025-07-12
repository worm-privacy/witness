use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rustc-link-search=./rapidsnark-linux-x86_64-v0.0.7/lib");
    let bindings = bindgen::Builder::default()
        .header("rapidsnark-linux-x86_64-v0.0.7/include/prover.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rustc-link-lib=static=rapidsnark-fr-fq");
    println!("cargo:rustc-link-lib=static=rapidsnark");

    println!("cargo::rerun-if-changed=proof_of_burn/*");
    println!("cargo::rerun-if-changed=spend/*");

    make("proof_of_burn");
    make("spend");

    println!("cargo:rustc-link-search=./proof_of_burn");
    println!("cargo:rustc-link-lib=static=proof_of_burn");
    println!("cargo:rustc-link-search=./spend");
    println!("cargo:rustc-link-lib=static=spend");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

fn make(dir: &str) {
    let status = Command::new("make")
        .current_dir(dir)
        .status()
        .expect("Failed to make");

    if !status.success() {
        panic!("Failed to make {}", dir);
    }
}

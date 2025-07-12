use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=proof_of_burn/*");
    println!("cargo::rerun-if-changed=spend/*");
    Command::new("make")
        .current_dir("proof_of_burn")
        .output()
        .expect("Failed to make");
    Command::new("make")
        .current_dir("spend")
        .output()
        .expect("Failed to make");
    println!("cargo:rustc-link-search=./proof_of_burn");
    println!("cargo:rustc-link-lib=static=proof_of_burn");
    println!("cargo:rustc-link-search=./spend");
    println!("cargo:rustc-link-lib=static=spend");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

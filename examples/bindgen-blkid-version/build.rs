use std::path::PathBuf;

use cc_args::{bindgen, pkg_config, MergeCcArgs};

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=wrapper.h");
    bindgen::Builder::default()
        .merge_cc_args(&pkg_config::probe_library("libzstd").unwrap())
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}

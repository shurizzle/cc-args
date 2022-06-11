# cc-args
[![crate.io](https://img.shields.io/crates/v/cc-args.svg)](https://crates.io/crates/cc-args)
[![rustdocs](https://docs.rs/cc-args/badge.svg)](https://docs.rs/cc-args)

### tl;dr
```rust
bindgen::Builder::merge_cc_args(&self, &pkg_config::Library)
```

This crate exports `CcArgs` and `MergeCcArgs`.

###### CcArgs

It provides link_paths, include_paths, framework_paths, frameworks, libs, ld_args and defines.

###### MergeCcArgs

It provides the method `merge_cc_args` that accept a `CcArgs` implementation.

###### Features

- **pkg_config** implements `CcArgs` for `pkg_config::Library`
- **cc** implements `MergeCcArgs` for `cc::Build`
- **bindgen** implements `MergeCcArgs` for `bindgen::Builder`

### Example

`build.rs`
```rust
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
```

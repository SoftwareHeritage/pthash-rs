// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use std::path::Path;

use thiserror::Error;

const BRIDGE_MODULES: [&str; 5] = [
    "src/partitioned_phf.rs",
    "src/single_phf.rs",
    "src/hashing.rs",
    "src/build.rs",
    "src/utils.rs",
];

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("autocxx engine error: {0}")]
    AutoCxxBuilder(#[from] autocxx_engine::BuilderError),
}

fn main() {
    if let Err(e) = main_() {
        eprintln!("Failed to generate PTHash FFI: {}", e);
        std::process::exit(1);
    }
}

fn main_() -> Result<(), BuildError> {
    let manifest_dir =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").expect("Missing CARGO_MANIFEST_DIR"))
            .to_owned();
    let pthash_src_dir = Path::new(&manifest_dir).join("pthash");
    let pthash_src_dir = pthash_src_dir.as_path();

    let mut b = autocxx_build::Builder::new(
        "src/structs.rs",
        &[
            &manifest_dir.join("src"),
            pthash_src_dir,
            &pthash_src_dir.join("include/"),
            &pthash_src_dir.join("external/essentials/include/"),
        ],
    )
    .extra_clang_args(&["-std=c++17"])
    .build()?;
    b.flag("-std=c++17").compile("pthash-ffi");

    cxx_build::bridges(BRIDGE_MODULES)
        .include("src")
        .include(pthash_src_dir)
        .include(&pthash_src_dir.join("include/"))
        .include(&pthash_src_dir.join("external/essentials/include/"))
        .compile("pthash");

    for module in BRIDGE_MODULES {
        println!("cargo:rerun-if-changed={}", module);
    }
    println!("cargo:rerun-if-changed=src/structs.rs");
    println!("cargo:rerun-if-changed=src/cpp-utils.hpp");
    println!("cargo:rerun-if-changed=src/concrete.hpp");
    println!("cargo:rerun-if-changed=src/workarounds.hpp");

    Ok(())
}

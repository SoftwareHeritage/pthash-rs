// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use std::io::Write;
use std::path::{Path, PathBuf};

use thiserror::Error;

const BRIDGE_MODULES: [&str; 3] = ["src/hashing.rs", "src/build.rs", "src/utils.rs"];

const BACKENDS_BRIDGE_PRELUDE: &str = r#"
#[cfg_attr(not(all(feature = "hash64", feature = "hash128")), allow(dead_code))]
#[cxx::bridge]
mod ffi {
    #[namespace = "pthash"]
    unsafe extern "C++" {
        include!("pthash.hpp");

        type build_configuration = crate::build::ffi::build_configuration;
        type hash64 = crate::structs::hash64;
        type hash128 = crate::structs::hash128;
    }

    #[namespace = "pthash_rs::concrete"]
    unsafe extern "C++" {
        include!("concrete.hpp");

        type internal_memory_builder_single_phf_64 =
            crate::build::ffi::internal_memory_builder_single_phf_64;
        type internal_memory_builder_single_phf_128 =
            crate::build::ffi::internal_memory_builder_single_phf_128;
        type internal_memory_builder_partitioned_phf_64 =
            crate::build::ffi::internal_memory_builder_partitioned_phf_64;
        type internal_memory_builder_partitioned_phf_128 =
            crate::build::ffi::internal_memory_builder_partitioned_phf_128;
    }
"#;

const BACKENDS_BRIDGE_TEMPLATE: &str = r#"
    #[namespace = "pthash_rs::concrete"]
    unsafe extern "C++" {
        include!("concrete.hpp");

        type $$STRUCT_NAME$$;
    }

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("pthash.hpp");
        include!("cpp-utils.hpp");

        #[cxx_name = "construct"]
        fn $$STRUCT_NAME$$_new() -> UniquePtr<$$STRUCT_NAME$$>;

        fn build(
            self: Pin<&mut $$STRUCT_NAME$$>,
            builder: &$$BUILDER_NAME$$,
            config: &build_configuration,
        ) -> Result<f64>;

        fn position(self: &$$STRUCT_NAME$$, hash: $$HASH_TYPE$$) -> u64;
        fn num_bits(self: &$$STRUCT_NAME$$) -> usize;
        fn num_keys(self: &$$STRUCT_NAME$$) -> u64;
        fn table_size(self: &$$STRUCT_NAME$$) -> u64;
        fn seed(self: &$$STRUCT_NAME$$) -> u64;
    }

    #[namespace = "essentials"]
    unsafe extern "C++" {
        include!("pthash.hpp");

        #[cxx_name = "save"]
        unsafe fn $$STRUCT_NAME$$_save(
            data_structure: Pin<&mut $$STRUCT_NAME$$>,
            filename: *const c_char,
        ) -> Result<usize>;

        #[cxx_name = "load"]
        unsafe fn $$STRUCT_NAME$$_load(
            data_structure: Pin<&mut $$STRUCT_NAME$$>,
            filename: *const c_char,
        ) -> Result<usize>;
    }
"#;

const BACKENDS_BRIDGE_POSTLUDE: &str = r#"
}

#[cfg(feature = "hash64")]
pub(crate) use ffi::{
    internal_memory_builder_partitioned_phf_64, internal_memory_builder_single_phf_64,
};

#[cfg(feature = "hash128")]
pub(crate) use ffi::{
    internal_memory_builder_partitioned_phf_128, internal_memory_builder_single_phf_128,
};
"#;

const BACKENDS_IMPL_TEMPLATE: &str = r#"
pub(crate) use ffi::$$STRUCT_NAME$$;

impl BackendPhf for $$STRUCT_NAME$$ {
    type Hash = ffi::$$HASH_TYPE$$;
    type Encoder = $$ENCODER_NAME$$;
    type Builder = $$BUILDER_NAME$$;

    fn new() -> UniquePtr<Self> {
        ffi::$$STRUCT_NAME$$_new()
    }
    fn position(&self, hash: Self::Hash) -> u64 {
        <$$STRUCT_NAME$$>::position(self, hash)
    }
    fn num_bits(&self) -> usize {
        <$$STRUCT_NAME$$>::num_bits(self)
    }
    fn num_keys(&self) -> u64 {
        <$$STRUCT_NAME$$>::num_keys(self)
    }
    fn table_size(&self) -> u64 {
        <$$STRUCT_NAME$$>::table_size(self)
    }
    fn seed(&self) -> u64 {
        <$$STRUCT_NAME$$>::seed(self)
    }
    fn build(
        self: Pin<&mut Self>,
        builder: &Self::Builder,
        config: &ffi::build_configuration,
    ) -> Result<f64> {
        <$$STRUCT_NAME$$>::build(self, builder, config)
    }

    unsafe fn save(self: Pin<&mut Self>, filename: *const i8) -> Result<usize> {
        ffi::$$STRUCT_NAME$$_save(self, filename)
    }
    unsafe fn load(self: Pin<&mut Self>, filename: *const i8) -> Result<usize> {
        ffi::$$STRUCT_NAME$$_load(self, filename)
    }
}
"#;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("autocxx engine error: {0}")]
    AutoCxxBuilder(#[from] autocxx_engine::BuilderError),
    #[error("could not create {0}: {1}")]
    CreateFile(PathBuf, std::io::Error),
    #[error("could not write to {0}: {1}")]
    WriteFile(PathBuf, std::io::Error),
    #[error("at least one of the encoder features must be enabled")]
    NoEncoder,
    #[error("at least one of the hash size features must be enabled")]
    NoHashSize,
    #[error("at least one of 'minimal' and 'nonminimal' features must be enabled")]
    NoMinimality,
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
    let out_dir = Path::new(&std::env::var("OUT_DIR").expect("Missing OUT_DIR")).to_owned();

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

    let backends_path = out_dir.join("backends_codegen.rs.inc");

    let mut fd = std::fs::File::create(&backends_path)
        .map_err(|e| BuildError::CreateFile(backends_path.clone(), e))?;

    // Write bridge
    fd.write_all(BACKENDS_BRIDGE_PRELUDE.as_bytes())
        .map_err(|e| BuildError::WriteFile(backends_path.clone(), e))?;
    for concrete_struct in concrete_structs()? {
        fd.write_all(&subst(concrete_struct, BACKENDS_BRIDGE_TEMPLATE))
            .map_err(|e| BuildError::WriteFile(backends_path.clone(), e))?;
    }
    fd.write_all(BACKENDS_BRIDGE_POSTLUDE.as_bytes())
        .map_err(|e| BuildError::WriteFile(backends_path.clone(), e))?;

    // Write implementations
    for concrete_struct in concrete_structs()? {
        fd.write_all(&subst(concrete_struct, BACKENDS_IMPL_TEMPLATE))
            .map_err(|e| BuildError::WriteFile(backends_path.clone(), e))?;
    }

    drop(fd);

    let mut bridge_modules: Vec<_> = BRIDGE_MODULES.iter().map(ToString::to_string).collect();
    bridge_modules.push(backends_path.display().to_string());

    cxx_build::bridges(bridge_modules)
        .flag("-std=c++17")
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

    Ok(())
}

fn subst(concrete_struct: ConcreteStruct, template: &str) -> Vec<u8> {
    template
        .replace("$$STRUCT_NAME$$", &concrete_struct.struct_name)
        .replace("$$ENCODER_NAME$$", &concrete_struct.encoder_name)
        .replace("$$HASH_TYPE$$", &concrete_struct.hash_type)
        .replace("$$BUILDER_NAME$$", &concrete_struct.builder_name)
        .into_bytes()
}

struct ConcreteStruct {
    struct_name: String,
    encoder_name: String,
    hash_type: String,
    builder_name: String,
}

fn has_feature(feature: &str) -> bool {
    std::env::var(&format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

fn concrete_structs() -> Result<Vec<ConcreteStruct>, BuildError> {
    let encoders: Vec<_> = [
        ("dictionary_dictionary", "DictionaryDictionary"),
        ("partitioned_compact", "PartitionedCompact"),
        ("elias_fano", "EliasFano"),
    ]
    .into_iter()
    .filter(|(snakecase, _camelcase)| has_feature(snakecase))
    .collect();

    if encoders.is_empty() {
        return Err(BuildError::NoEncoder);
    }

    let hash_sizes: Vec<_> = ["64", "128"]
        .into_iter()
        .filter(|hash_size| has_feature(&format!("hash{}", hash_size)))
        .collect();

    if hash_sizes.is_empty() {
        return Err(BuildError::NoHashSize);
    }

    let minimalities: Vec<_> = ["nonminimal", "minimal"]
        .into_iter()
        .filter(|minimality| has_feature(minimality))
        .collect();

    if minimalities.is_empty() {
        return Err(BuildError::NoMinimality);
    }

    let mut concrete_structs = Vec::new();
    for (encoder_snakecase, encoder_camelcase) in encoders {
        for hash_size in &hash_sizes {
            for phf_type in ["single", "partitioned"] {
                for minimality in &minimalities {
                    concrete_structs.push(ConcreteStruct {
                        struct_name: format!(
                            "{}phf_{}_{}_{}",
                            phf_type, hash_size, encoder_snakecase, minimality
                        ),
                        encoder_name: encoder_camelcase.to_string(),
                        hash_type: format!("hash{}", hash_size),
                        builder_name: format!(
                            "internal_memory_builder_{}_phf_{}",
                            phf_type, hash_size
                        ),
                    })
                }
            }
        }
    }

    Ok(concrete_structs)
}

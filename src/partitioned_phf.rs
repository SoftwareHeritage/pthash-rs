// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#![allow(non_camel_case_types)]

use std::marker::PhantomData;
use std::path::Path;

//use autocxx::prelude::*;
use cxx::{Exception, UniquePtr};
use rand::Rng;

use crate::build::{BuildConfiguration, BuildTimings};
use crate::hashing::{Hashable, Hasher};
use crate::Phf;

#[cxx::bridge]
mod ffi {
    #[namespace = "pthash"]
    unsafe extern "C++" {
        include!("pthash.hpp");

        type build_timings = crate::structs::build_timings;
        type build_configuration = crate::build::ffi::build_configuration;
        type hash64 = crate::structs::hash64;
    }

    #[namespace = "pthash_rs::concrete"]
    unsafe extern "C++" {
        include!("concrete.hpp");

        type partitionedphf_dictionary_minimal;
        type internal_memory_builder_partitioned_phf;
    }

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("pthash.hpp");
        include!("cpp-utils.hpp");

        #[cxx_name = "construct"]
        fn internal_memory_builder_partitioned_phf_new(
        ) -> UniquePtr<internal_memory_builder_partitioned_phf>;

        unsafe fn build_from_hashes(
            self: Pin<&mut internal_memory_builder_partitioned_phf>,
            hashes: *const hash64,
            num_keys: u64,
            config: &build_configuration,
        ) -> Result<build_timings>;

        #[cxx_name = "construct"]
        fn partitionedphf_dictionary_minimal_new() -> UniquePtr<partitionedphf_dictionary_minimal>;

        fn build(
            self: Pin<&mut partitionedphf_dictionary_minimal>,
            builder: &internal_memory_builder_partitioned_phf,
            config: &build_configuration,
        ) -> f64;

        fn position(self: &partitionedphf_dictionary_minimal, hash: hash64) -> u64;
        fn num_bits(self: &partitionedphf_dictionary_minimal) -> usize;
        fn num_keys(self: &partitionedphf_dictionary_minimal) -> u64;
        fn table_size(self: &partitionedphf_dictionary_minimal) -> u64;
    }

    #[namespace = "essentials"]
    unsafe extern "C++" {
        include!("pthash.hpp");

        #[cxx_name = "save"]
        unsafe fn partitionedphf_dictionary_minimal_save(
            data_structure: Pin<&mut partitionedphf_dictionary_minimal>,
            filename: *const c_char,
        ) -> Result<usize>;

        #[cxx_name = "load"]
        unsafe fn partitionedphf_dictionary_minimal_load(
            data_structure: Pin<&mut partitionedphf_dictionary_minimal>,
            filename: *const c_char,
        ) -> Result<usize>;
    }

    #[namespace = "pthash_rs::workarounds"]
    unsafe extern "C++" {
        include!("workarounds.hpp");

        #[cxx_name = "set_seed"]
        fn internal_memory_builder_partitioned_phf_set_seed(
            function: Pin<&mut internal_memory_builder_partitioned_phf>,
            seed: u64,
        ) -> Result<()>;

        #[cxx_name = "get_seed"]
        fn partitionedphf_dictionary_minimal_get_seed(
            function: Pin<&mut partitionedphf_dictionary_minimal>,
        ) -> Result<u64>;
    }
}

pub struct PartitionedPhf_Dictionary_Minimal<H: Hasher> {
    inner: UniquePtr<ffi::partitionedphf_dictionary_minimal>,
    seed: u64,
    marker: PhantomData<H>,
}

impl<H: Hasher> PartitionedPhf_Dictionary_Minimal<H> {
    pub fn new() -> Self {
        PartitionedPhf_Dictionary_Minimal {
            inner: ffi::partitionedphf_dictionary_minimal_new(),
            seed: 0,
            marker: PhantomData,
        }
    }
}

impl<H: Hasher> Phf for PartitionedPhf_Dictionary_Minimal<H> {
    const MINIMAL: bool = true;

    fn build_in_internal_memory_from_bytes<Keys: IntoIterator>(
        &mut self,
        keys: Keys,
        config: &BuildConfiguration,
    ) -> Result<BuildTimings, Exception>
    where
        <Keys as IntoIterator>::IntoIter: ExactSizeIterator + Clone,
        <<Keys as IntoIterator>::IntoIter as Iterator>::Item: Hashable,
    {
        // This is a Rust rewrite of internal_memory_builder_partitioned_phf::build_from_keys
        // so we can use generics

        let mut config = (*config).clone();
        if !crate::utils::valid_seed(config.seed) {
            let mut rng = rand::thread_rng();
            config.seed = rng.gen();
        }
        self.seed = config.seed;

        let keys = keys.into_iter();

        let hashes: Vec<_> = keys.clone().map(|key| H::hash(key, config.seed)).collect();

        let mut builder = ffi::internal_memory_builder_partitioned_phf_new();

        // internal_memory_builder_partitioned_phf::build_from_hashes ignores config.seed
        // and expects to be called by internal_memory_builder_partitioned_phf::build_from_keys
        // which sets it
        ffi::internal_memory_builder_partitioned_phf_set_seed(builder.pin_mut(), config.seed)?;

        let config = config.to_ffi();
        let mut timings = unsafe {
            builder
                .pin_mut()
                .build_from_hashes(hashes.as_ptr(), hashes.len() as u64, &config)
        }?;

        timings.encoding_seconds = self.inner.pin_mut().build(&builder, &config);
        Ok(BuildTimings::from_ffi(&timings))
    }

    fn hash(&self, key: impl Hashable) -> u64 {
        self.inner.position(H::hash(key, self.seed))
    }

    fn num_bits(&self) -> usize {
        self.inner.num_bits()
    }

    fn num_keys(&self) -> u64 {
        self.inner.num_keys()
    }

    fn table_size(&self) -> u64 {
        self.inner.table_size()
    }

    fn save(&mut self, path: impl AsRef<Path>) -> Result<usize, Exception> {
        let mut path = path.as_ref().as_os_str().to_owned().into_encoded_bytes();
        path.push(0); // null terminator
        let path = path.as_ptr() as *const i8;

        unsafe { ffi::partitionedphf_dictionary_minimal_save(self.inner.pin_mut(), path) }
    }
    fn load(path: impl AsRef<Path>) -> Result<Self, Exception> {
        let mut f = Self::new();

        let mut path = path.as_ref().as_os_str().to_owned().into_encoded_bytes();
        path.push(0); // null terminator
        let path = path.as_ptr() as *const i8;

        unsafe { ffi::partitionedphf_dictionary_minimal_load(f.inner.pin_mut(), path) }?;

        f.seed = ffi::partitionedphf_dictionary_minimal_get_seed(f.inner.pin_mut())?;

        Ok(f)
    }
}

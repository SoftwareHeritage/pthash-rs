// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use std::pin::Pin;

use cxx::{Exception, UniquePtr};

use crate::hashing::Hash;
use crate::structs::{hash64, hash128, build_timings};

type Result<T> = std::result::Result<T, Exception>;

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

        type singlephf_dictionary_minimal;
        type internal_memory_builder_single_phf;

        type partitionedphf_dictionary_minimal;
        type internal_memory_builder_partitioned_phf;
    }

    /**********************************************************************************
     * builders
     **********************************************************************************/

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("pthash.hpp");
        include!("cpp-utils.hpp");
        #[cxx_name = "construct"]
        fn internal_memory_builder_single_phf_new() -> UniquePtr<internal_memory_builder_single_phf>;

        #[rust_name = "build_from_hashes1"] // why?
        unsafe fn build_from_hashes(
            self: Pin<&mut internal_memory_builder_single_phf>,
            hashes: *const hash64,
            num_keys: u64,
            config: &build_configuration,
        ) -> Result<build_timings>;
    }

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("pthash.hpp");
        include!("cpp-utils.hpp");
        #[cxx_name = "construct"]
        fn internal_memory_builder_partitioned_phf_new(
        ) -> UniquePtr<internal_memory_builder_partitioned_phf>;

        #[rust_name = "build_from_hashes2"] // why?
        unsafe fn build_from_hashes(
            self: Pin<&mut internal_memory_builder_partitioned_phf>,
            hashes: *const hash64,
            num_keys: u64,
            config: &build_configuration,
        ) -> Result<build_timings>;
    }

    /**********************************************************************************
     * singlephf_dictionary_minimal
     **********************************************************************************/

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("pthash.hpp");
        include!("cpp-utils.hpp");

        #[cxx_name = "construct"]
        fn singlephf_dictionary_minimal_new() -> UniquePtr<singlephf_dictionary_minimal>;

        fn build(
            self: Pin<&mut singlephf_dictionary_minimal>,
            builder: &internal_memory_builder_single_phf,
            config: &build_configuration,
        ) -> f64;

        fn position(self: &singlephf_dictionary_minimal, hash: hash64) -> u64;
        fn num_bits(self: &singlephf_dictionary_minimal) -> usize;
        fn num_keys(self: &singlephf_dictionary_minimal) -> u64;
        fn table_size(self: &singlephf_dictionary_minimal) -> u64;
    }

    #[namespace = "essentials"]
    unsafe extern "C++" {
        include!("pthash.hpp");

        #[cxx_name = "save"]
        unsafe fn singlephf_dictionary_minimal_save(
            data_structure: Pin<&mut singlephf_dictionary_minimal>,
            filename: *const c_char,
        ) -> Result<usize>;

        #[cxx_name = "load"]
        unsafe fn singlephf_dictionary_minimal_load(
            data_structure: Pin<&mut singlephf_dictionary_minimal>,
            filename: *const c_char,
        ) -> Result<usize>;
    }

    #[namespace = "pthash_rs::workarounds"]
    unsafe extern "C++" {
        include!("workarounds.hpp");

        #[cxx_name = "set_seed"]
        fn internal_memory_builder_single_phf_set_seed(
            function: Pin<&mut internal_memory_builder_single_phf>,
            seed: u64,
        ) -> Result<()>;

        #[cxx_name = "get_seed"]
        fn singlephf_dictionary_minimal_get_seed(
            function: Pin<&mut singlephf_dictionary_minimal>,
        ) -> Result<u64>;
    }

    /**********************************************************************************
     * partitionedphf_dictionary_minimal
     **********************************************************************************/

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("pthash.hpp");
        include!("cpp-utils.hpp");

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

/**********************************************************************************
 * Glue to make the concrete types above usable as instances of a generic type
 **********************************************************************************/

pub(crate) use ffi::{
    internal_memory_builder_partitioned_phf, internal_memory_builder_single_phf,
    partitionedphf_dictionary_minimal, singlephf_dictionary_minimal,
};

pub(crate) trait Builder: Sized + cxx::memory::UniquePtrTarget {
    type Hash: Hash;

    fn new() -> UniquePtr<Self>;
    fn set_seed(self: Pin<&mut Self>, seed: u64) -> Result<()>;

    unsafe fn build_from_hashes(
        self: Pin<&mut Self>,
        hashes: *const Self::Hash,
        num_keys: u64,
        config: &ffi::build_configuration,
    ) -> Result<build_timings>;
}

impl Builder for internal_memory_builder_single_phf {
    type Hash = hash64;

    fn new() -> UniquePtr<Self> {
        ffi::internal_memory_builder_single_phf_new()
    }
    fn set_seed(self: Pin<&mut Self>, seed: u64) -> Result<()> {
        ffi::internal_memory_builder_single_phf_set_seed(self, seed)
    }
    unsafe fn build_from_hashes(
        self: Pin<&mut Self>,
        hashes: *const Self::Hash,
        num_keys: u64,
        config: &ffi::build_configuration,
    ) -> Result<build_timings> {
        internal_memory_builder_single_phf::build_from_hashes1(self, hashes, num_keys, config)
    }
}

impl Builder for internal_memory_builder_partitioned_phf {
    type Hash = hash64;

    fn new() -> UniquePtr<Self> {
        ffi::internal_memory_builder_partitioned_phf_new()
    }
    fn set_seed(self: Pin<&mut Self>, seed: u64) -> Result<()> {
        ffi::internal_memory_builder_partitioned_phf_set_seed(self, seed)
    }
    unsafe fn build_from_hashes(
        self: Pin<&mut Self>,
        hashes: *const Self::Hash,
        num_keys: u64,
        config: &ffi::build_configuration,
    ) -> Result<build_timings> {
        internal_memory_builder_partitioned_phf::build_from_hashes2(self, hashes, num_keys, config)
    }
}

pub(crate) trait BackendPhf: Sized + cxx::memory::UniquePtrTarget {
    type Hash: Hash;
    type Builder: Builder<Hash = Self::Hash>;

    fn new() -> UniquePtr<Self>;
    fn position(&self, hash: Self::Hash) -> u64;
    fn num_bits(&self) -> usize;
    fn num_keys(&self) -> u64;
    fn table_size(&self) -> u64;
    fn get_seed(self: Pin<&mut Self>) -> Result<u64>;
    fn build(self: Pin<&mut Self>, builder: &Self::Builder, config: &ffi::build_configuration) -> f64;

    unsafe fn save(self: Pin<&mut Self>, filename: *const i8) -> Result<usize>;
    unsafe fn load(self: Pin<&mut Self>, filename: *const i8) -> Result<usize>;

}

macro_rules! impl_backend_methods {
    ($type:ty, $get_seed:path, $save:path, $load:path,) => {
        fn position(&self, hash: Self::Hash) -> u64 {
            <$type>::position(self, hash)
        }
        fn num_bits(&self) -> usize {
            <$type>::num_bits(self)
        }
        fn num_keys(&self) -> u64 {
            <$type>::num_keys(self)
        }
        fn table_size(&self) -> u64 {
            <$type>::table_size(self)
        }
        fn get_seed(self: Pin<&mut Self>) -> Result<u64> {
            $get_seed(self)
        }
        fn build(self: Pin<&mut Self>, builder: &Self::Builder, config: &ffi::build_configuration) -> f64 {
            <$type>::build(self, builder, config)
        }

        unsafe fn save(self: Pin<&mut Self>, filename: *const i8) -> Result<usize> {
            $save(self, filename)
        }
        unsafe fn load(self: Pin<&mut Self>, filename: *const i8) -> Result<usize> {
            $load(self, filename)
        }
    };
}

impl BackendPhf for singlephf_dictionary_minimal {
    type Hash = ffi::hash64;
    type Builder = internal_memory_builder_single_phf;

    fn new() -> UniquePtr<Self> {
        ffi::singlephf_dictionary_minimal_new()
    }
    impl_backend_methods!(
        singlephf_dictionary_minimal,
        ffi::singlephf_dictionary_minimal_get_seed,
        ffi::singlephf_dictionary_minimal_save,
        ffi::singlephf_dictionary_minimal_load,
    );
}

impl BackendPhf for partitionedphf_dictionary_minimal {
    type Hash = ffi::hash64;
    type Builder = internal_memory_builder_partitioned_phf;

    fn new() -> UniquePtr<Self> {
        ffi::partitionedphf_dictionary_minimal_new()
    }
    impl_backend_methods!(
        partitionedphf_dictionary_minimal,
        ffi::partitionedphf_dictionary_minimal_get_seed,
        ffi::partitionedphf_dictionary_minimal_save,
        ffi::partitionedphf_dictionary_minimal_load,
    );
}

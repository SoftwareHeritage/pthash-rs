// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use std::path::PathBuf;
use std::time::Duration;

use cxx::{let_cxx_string, UniquePtr};

use crate::structs::build_timings;

#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "pthash"]
    unsafe extern "C++" {
        include!("pthash.hpp");

        type build_configuration;
    }

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("cpp-utils.hpp");
        #[cxx_name = "construct"]
        fn build_configuration_new() -> UniquePtr<build_configuration>;
    }

    #[namespace = "pthash_rs::accessors"]
    unsafe extern "C++" {
        include!("cpp-utils.hpp");

        #[rust_name = "build_configuration_get_c"]
        fn get_c(conf: &build_configuration) -> f64;
        #[rust_name = "build_configuration_set_c"]
        fn set_c(conf: &mut UniquePtr<build_configuration>, value: f64);

        #[rust_name = "build_configuration_get_alpha"]
        fn get_alpha(conf: &build_configuration) -> f64;
        #[rust_name = "build_configuration_set_alpha"]
        fn set_alpha(conf: &mut UniquePtr<build_configuration>, value: f64);

        #[rust_name = "build_configuration_get_num_partitions"]
        fn get_num_partitions(conf: &build_configuration) -> u64;
        #[rust_name = "build_configuration_set_num_partitions"]
        fn set_num_partitions(conf: &mut UniquePtr<build_configuration>, value: u64);

        #[rust_name = "build_configuration_get_num_buckets"]
        fn get_num_buckets(conf: &build_configuration) -> u64;
        #[rust_name = "build_configuration_set_num_buckets"]
        fn set_num_buckets(conf: &mut UniquePtr<build_configuration>, value: u64);

        #[rust_name = "build_configuration_get_num_threads"]
        fn get_num_threads(conf: &build_configuration) -> u64;
        #[rust_name = "build_configuration_set_num_threads"]
        fn set_num_threads(conf: &mut UniquePtr<build_configuration>, value: u64);

        #[rust_name = "build_configuration_get_seed"]
        fn get_seed(conf: &build_configuration) -> u64;
        #[rust_name = "build_configuration_set_seed"]
        fn set_seed(conf: &mut UniquePtr<build_configuration>, value: u64);

        #[rust_name = "build_configuration_get_ram"]
        fn get_ram(conf: &build_configuration) -> u64;
        #[rust_name = "build_configuration_set_ram"]
        fn set_ram(conf: &mut UniquePtr<build_configuration>, value: u64);

        #[rust_name = "build_configuration_set_tmp_dir"]
        fn set_tmp_dir(conf: &mut UniquePtr<build_configuration>, value: Pin<&mut CxxString>);

        #[rust_name = "build_configuration_get_minimal_output"]
        fn get_minimal_output(conf: &build_configuration) -> bool;
        #[rust_name = "build_configuration_set_minimal_output"]
        fn set_minimal_output(conf: &mut UniquePtr<build_configuration>, value: bool);

        #[rust_name = "build_configuration_get_verbose_output"]
        fn get_verbose_output(conf: &build_configuration) -> bool;
        #[rust_name = "build_configuration_set_verbose_output"]
        fn set_verbose_output(conf: &mut UniquePtr<build_configuration>, value: bool);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuildConfiguration {
    pub c: f64,
    pub alpha: f64,
    pub num_partitions: u64,
    pub num_buckets: u64,
    pub num_threads: u64,
    pub seed: u64,
    pub ram: u64,
    pub tmp_dir: PathBuf,
    pub minimal_output: bool,
    pub verbose_output: bool,
}

impl BuildConfiguration {
    pub fn new(tmp_dir: PathBuf) -> BuildConfiguration {
        let defaults = ffi::build_configuration_new();
        BuildConfiguration {
            c: ffi::build_configuration_get_c(&defaults),
            alpha: ffi::build_configuration_get_alpha(&defaults),
            num_partitions: ffi::build_configuration_get_num_partitions(&defaults),
            num_buckets: ffi::build_configuration_get_num_buckets(&defaults),
            num_threads: ffi::build_configuration_get_num_threads(&defaults),
            seed: ffi::build_configuration_get_seed(&defaults),
            ram: ffi::build_configuration_get_ram(&defaults),
            tmp_dir,
            minimal_output: ffi::build_configuration_get_minimal_output(&defaults),
            verbose_output: ffi::build_configuration_get_verbose_output(&defaults),
        }
    }

    /// Returns pthash's native [`build_configuration`]
    pub(crate) fn to_ffi(&self) -> UniquePtr<ffi::build_configuration> {
        let mut conf = ffi::build_configuration_new();
        ffi::build_configuration_set_c(&mut conf, self.c);
        ffi::build_configuration_set_alpha(&mut conf, self.alpha);
        ffi::build_configuration_set_num_partitions(&mut conf, self.num_partitions);
        ffi::build_configuration_set_num_buckets(&mut conf, self.num_buckets);
        ffi::build_configuration_set_num_threads(&mut conf, self.num_threads);
        ffi::build_configuration_set_seed(&mut conf, self.seed);
        ffi::build_configuration_set_ram(&mut conf, self.ram);
        let_cxx_string!(tmp_dir = self.tmp_dir.as_os_str().as_encoded_bytes());
        ffi::build_configuration_set_tmp_dir(&mut conf, tmp_dir);
        ffi::build_configuration_set_minimal_output(&mut conf, self.minimal_output);
        ffi::build_configuration_set_verbose_output(&mut conf, self.verbose_output);
        conf
    }
}

pub struct BuildTimings {
    pub partitioning_seconds: Duration,
    pub mapping_ordering_seconds: Duration,
    pub searching_seconds: Duration,
    pub encoding_seconds: Duration,
}

impl BuildTimings {
    pub(crate) fn from_ffi(timings: &build_timings) -> Self {
        BuildTimings {
            partitioning_seconds: Duration::from_secs_f64(timings.partitioning_seconds),
            mapping_ordering_seconds: Duration::from_secs_f64(timings.mapping_ordering_seconds),
            searching_seconds: Duration::from_secs_f64(timings.searching_seconds),
            encoding_seconds: Duration::from_secs_f64(timings.encoding_seconds),
        }
    }
}

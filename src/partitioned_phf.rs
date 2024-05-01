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

use crate::backends::BackendPhf;
use crate::build::{BuildConfiguration, BuildTimings, Builder};
use crate::hashing::{Hash, Hashable, Hasher};
use crate::{Encoder, Phf};

/// Partitioned minimal perfect hash function
///
/// This is a binding for `pthash::partitioned_phf<H, dictionary_dictionary, true>`
pub struct PartitionedPhf_Minimal<H: Hasher, E: Encoder> {
    inner: UniquePtr<<H::Hash as Hash>::PartitionedPhfBackend<E>>,
    seed: u64,
    marker: PhantomData<H>,
}

unsafe impl<H: Hasher, E: Encoder> Send for PartitionedPhf_Minimal<H, E> {}
unsafe impl<H: Hasher, E: Encoder> Sync for PartitionedPhf_Minimal<H, E> {}

impl<H: Hasher, E: Encoder> PartitionedPhf_Minimal<H, E> {
    pub fn new() -> Self {
        PartitionedPhf_Minimal {
            inner: BackendPhf::new(),
            seed: 0,
            marker: PhantomData,
        }
    }
}

impl<H: Hasher, E: Encoder> Phf for PartitionedPhf_Minimal<H, E> {
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

        let mut builder =
            <<H::Hash as Hash>::PartitionedPhfBackend<E> as BackendPhf>::Builder::new();

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

        unsafe { self.inner.pin_mut().save(path) }
    }
    fn load(path: impl AsRef<Path>) -> Result<Self, Exception> {
        let mut f = Self::new();

        let mut path = path.as_ref().as_os_str().to_owned().into_encoded_bytes();
        path.push(0); // null terminator
        let path = path.as_ptr() as *const i8;

        unsafe { f.inner.pin_mut().load(path) }?;

        f.seed = f.inner.seed();

        Ok(f)
    }
}

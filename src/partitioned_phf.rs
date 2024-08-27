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
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::backends::BackendPhf;
use crate::build::{BuildConfiguration, BuildTimings, Builder};
use crate::hashing::{Hashable, Hasher};
use crate::{Encoder, Minimality, Phf, SealedMinimality};

/// Partitioned minimal perfect hash function
///
/// This is a binding for `pthash::partitioned_phf<H, dictionary_dictionary, true>`
pub struct PartitionedPhf<M: Minimality, H: Hasher, E: Encoder> {
    inner: UniquePtr<<M as SealedMinimality>::PartitionedPhfBackend<H::Hash, E>>,
    seed: u64,
    marker: PhantomData<M>,
}

unsafe impl<M: Minimality, H: Hasher, E: Encoder> Send for PartitionedPhf<M, H, E> {}
unsafe impl<M: Minimality, H: Hasher, E: Encoder> Sync for PartitionedPhf<M, H, E> {}

impl<M: Minimality, H: Hasher, E: Encoder> PartitionedPhf<M, H, E> {
    pub fn new() -> Self {
        PartitionedPhf {
            inner: BackendPhf::new(),
            seed: 0,
            marker: PhantomData,
        }
    }
}

macro_rules! build_in_internal_memory_from_bytes {
    ($self:expr, $keys:expr, $config:expr, $into_iter:ident) => {{
        let keys = $keys;
        let config = $config;

        // This is a Rust rewrite of internal_memory_builder_partitioned_phf::build_from_keys
        // so we can use generics

        let mut config = (*config).clone();
        if !crate::utils::valid_seed(config.seed) {
            let mut rng = rand::thread_rng();
            config.seed = rng.gen();
        }
        $self.seed = config.seed;

        let hashes: Vec<_> = keys.$into_iter().map(|key| H::hash(key, config.seed)).collect();

        let mut builder =
            <<M as SealedMinimality>::PartitionedPhfBackend<H::Hash, E> as BackendPhf>::Builder::new();

        let config = config.to_ffi(M::AS_BOOL);
        let mut timings = unsafe {
            builder
                .pin_mut()
                .build_from_hashes(hashes.as_ptr(), hashes.len() as u64, &config)
        }?;

        timings.encoding_seconds = $self.inner.pin_mut().build(&builder, &config)?;
        Ok(BuildTimings::from_ffi(&timings))
    }}
}

impl<M: Minimality, H: Hasher, E: Encoder> Phf for PartitionedPhf<M, H, E>
{
    const MINIMAL: bool = M::AS_BOOL;

    fn build_in_internal_memory_from_bytes<Keys: IntoIterator>(
        &mut self,
        keys: Keys,
        config: &BuildConfiguration,
    ) -> Result<BuildTimings, Exception>
    where
        <<Keys as IntoIterator>::IntoIter as Iterator>::Item: Hashable,
    {
        build_in_internal_memory_from_bytes!(self, keys, config, into_iter)
    }

    #[cfg(feature = "rayon")]
    fn par_build_in_internal_memory_from_bytes<Keys: IntoParallelIterator>(
        &mut self,
        keys: Keys,
        config: &BuildConfiguration,
    ) -> Result<BuildTimings, Exception>
    where
        <<Keys as IntoParallelIterator>::Iter as ParallelIterator>::Item: Hashable,
    {
        build_in_internal_memory_from_bytes!(self, keys, config, into_par_iter)
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

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
use crate::encoders::Encoder;
use crate::hashing::{Hashable, Hasher};
use crate::{Minimality, Phf, SealedMinimality};

/// Non-partitioned minimal perfect-hash function
///
/// This is a binding for `pthash::single_phf<H, dictionary_dictionary, true>`
pub struct SinglePhf<M: Minimality, H: Hasher, E: Encoder> {
    inner: UniquePtr<<M as SealedMinimality>::SinglePhfBackend<H::Hash, E>>,
    seed: u64,
    marker: PhantomData<H>,
}

unsafe impl<M: Minimality, H: Hasher, E: Encoder> Send for SinglePhf<M, H, E> {}
unsafe impl<M: Minimality, H: Hasher, E: Encoder> Sync for SinglePhf<M, H, E> {}

impl<M: Minimality, H: Hasher, E: Encoder> SinglePhf<M, H, E> {
    pub fn new() -> Self {
        SinglePhf {
            inner: BackendPhf::new(),
            seed: 0,
            marker: PhantomData,
        }
    }
}

impl<M: Minimality, H: Hasher, E: Encoder> Phf for SinglePhf<M, H, E> {
    const MINIMAL: bool = M::AS_BOOL;

    fn build_in_internal_memory_from_bytes<Keys: IntoIterator>(
        &mut self,
        keys: Keys,
        config: &BuildConfiguration,
    ) -> Result<BuildTimings, Exception>
    where
        <Keys as IntoIterator>::IntoIter: ExactSizeIterator + Clone,
        <<Keys as IntoIterator>::IntoIter as Iterator>::Item: Hashable,
    {
        // This is a Rust rewrite of internal_memory_builder_single_phf::build_from_keys
        // so we can use generics

        let seeds = if crate::utils::valid_seed(config.seed) {
            vec![config.seed]
        } else {
            let mut rng = rand::thread_rng();
            (0..10).map(|_| rng.gen()).collect()
        };

        let keys = keys.into_iter();

        let mut last_error = None;
        for (i, seed) in seeds.into_iter().enumerate() {
            let hashes: Vec<_> = keys.clone().map(|key| H::hash(key, seed)).collect();
            self.seed = seed;

            let mut builder =
                <<M as SealedMinimality>::SinglePhfBackend<H::Hash, E> as BackendPhf>::Builder::new(
                );

            let mut config = (*config).clone();
            config.seed = seed;

            let config = config.to_ffi();
            let res = unsafe {
                builder
                    .pin_mut()
                    .build_from_hashes(hashes.as_ptr(), hashes.len() as u64, &config)
            };
            match res {
                Ok(mut timings) => {
                    timings.encoding_seconds = self.inner.pin_mut().build(&builder, &config)?;
                    return Ok(BuildTimings::from_ffi(&timings));
                }
                Err(e) => {
                    log::info!("Attempt {} failed", i + 1);
                    last_error = Some(e);
                    // Try again with the next seed, if any
                }
            }
        }

        // All seeds failed
        Err(last_error.unwrap())
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

// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#![cfg_attr(all(feature = "minimal", feature = "nonminimal", feature = "dictionary_dictionary", feature = "hash64"), doc = include_str!("../README.md"))]

use std::path::Path;

use cxx::Exception;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub mod build;
pub use build::*;

mod backends;

pub mod encoders;
pub use encoders::*;

pub mod hashing;
pub use hashing::*;

pub mod minimality;
pub use minimality::*;

mod partitioned_phf;
pub use partitioned_phf::*;

mod structs;

mod single_phf;
pub use single_phf::*;

mod utils;
#[allow(unused_imports)] // check() is feature-gated
pub use utils::*;

/// A [perfect-hash function](https://en.wikipedia.org/wiki/Perfect_hash_function)
/// implemented with the [PTHash algorithm](https://dl.acm.org/doi/10.1145/3404835.3462849)
pub trait Phf: Sized + Send + Sync {
    /// Whether instances of this function have their values in the range `[0; num_keys)`.
    const MINIMAL: bool;

    /// Builds the function from a set of keys
    ///
    /// In plain English, this function's trait bound on keys is that they should be
    /// a collection that can provide cloneable iterators of hashable values.
    fn build_in_internal_memory_from_bytes<Keys: IntoIterator>(
        &mut self,
        keys: impl FnMut() -> Keys,
        config: &BuildConfiguration,
    ) -> Result<BuildTimings, Exception>
    where
        <<Keys as IntoIterator>::IntoIter as Iterator>::Item: Hashable;

    #[cfg(feature = "rayon")]
    /// Same as [`Self::build_in_internal_memory_from_bytes`], but hashes in parallel
    fn par_build_in_internal_memory_from_bytes<Keys: IntoParallelIterator>(
        &mut self,
        keys: impl FnMut() -> Keys,
        config: &BuildConfiguration,
    ) -> Result<BuildTimings, Exception>
    where
        <<Keys as IntoParallelIterator>::Iter as ParallelIterator>::Item: Hashable;

    /// Returns the hash of the given key
    ///
    /// If the `key` was not one of the keys passed to
    /// [`build_in_internal_memory_from_bytes`](Self::build_in_internal_memory_from_bytes)
    /// when building the function, the hash will collide with another key's
    fn hash(&self, key: impl Hashable) -> u64;

    /// Returns the number of bits needed to represent this perfect-hash function
    fn num_bits(&self) -> usize;
    /// Returns the number of keys used to build this perfect-hash function
    fn num_keys(&self) -> u64;
    /// Largest value returned by [`Self::hash`] plus 1
    fn table_size(&self) -> u64;

    /// Dump this function to disk
    fn save(&mut self, path: impl AsRef<Path>) -> Result<usize, Exception>;
    /// Load this function from disk
    fn load(path: impl AsRef<Path>) -> Result<Self, Exception>;
}

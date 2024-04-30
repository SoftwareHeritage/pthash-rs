// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use std::path::Path;

use cxx::Exception;

mod backends;

mod hashing;
pub use hashing::*;

mod build;
pub use build::*;

mod partitioned_phf;
pub use partitioned_phf::*;

mod structs;

mod single_phf;
pub use single_phf::*;

mod utils;
pub use utils::*;

pub trait Phf: Sized {
    /// Whether instances of this function have their values in the range `[0; num_keys)`.
    const MINIMAL: bool = true;

    fn build_in_internal_memory_from_bytes<Keys: IntoIterator>(
        &mut self,
        keys: Keys,
        config: &BuildConfiguration,
    ) -> Result<BuildTimings, Exception>
    where
        <Keys as IntoIterator>::IntoIter: ExactSizeIterator + Clone,
        <<Keys as IntoIterator>::IntoIter as Iterator>::Item: Hashable;

    fn hash(&self, key: impl Hashable) -> u64;

    fn num_bits(&self) -> usize;
    fn num_keys(&self) -> u64;
    /// Largest value returned by [`Self::hash`] plus 1
    fn table_size(&self) -> u64;

    fn save(&mut self, path: impl AsRef<Path>) -> Result<usize, Exception>;
    fn load(path: impl AsRef<Path>) -> Result<Self, Exception>;
}

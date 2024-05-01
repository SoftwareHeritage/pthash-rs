// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use crate::hashing::Hash;
#[cfg(feature = "hash64")]
use crate::structs::hash64;
#[cfg(feature = "hash128")]
use crate::structs::hash128;


#[cfg(all(feature = "hash64", feature = "hash128"))]
#[allow(private_bounds)] // Users shouldn't be able to impl the Encoder trait
pub trait Encoder: BackendForEncoderByHash<hash64> + BackendForEncoderByHash<hash128> {}
#[cfg(all(feature = "hash64", not(feature = "hash128")))]
#[allow(private_bounds)]
pub trait Encoder: BackendForEncoderByHash<hash64> {}
#[cfg(all(not(feature = "hash64"), feature = "hash128"))]
#[allow(private_bounds)]
pub trait Encoder: BackendForEncoderByHash<hash128> {}
// build.rs rejects both hash64 and hash128 being disabled

/// Type trickery to make [`Hash`] implementable
pub(crate) trait BackendForEncoderByHash<H: Hash> {
    type SinglePhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
    type PartitionedPhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
}

#[cfg(feature = "dictionary_dictionary")]
mod dictionary_dictionary {
    use super::*;

    /// Encoder known as "D-D" in the PTHash papers
    pub struct DictionaryDictionary;
    impl Encoder for DictionaryDictionary {}

    #[cfg(feature = "hash64")]
    impl BackendForEncoderByHash<hash64> for DictionaryDictionary {
        type SinglePhfBackend = crate::backends::singlephf_64_dictionary_dictionary_minimal;
        type PartitionedPhfBackend =
            crate::backends::partitionedphf_64_dictionary_dictionary_minimal;
    }

    #[cfg(feature = "hash128")]
    impl BackendForEncoderByHash<hash128> for DictionaryDictionary {
        type SinglePhfBackend = crate::backends::singlephf_128_dictionary_dictionary_minimal;
        type PartitionedPhfBackend =
            crate::backends::partitionedphf_128_dictionary_dictionary_minimal;
    }
}

#[cfg(feature = "dictionary_dictionary")]
pub use dictionary_dictionary::*;

#[cfg(feature = "partitioned_compact")]
mod partitioned_compact {
    use super::*;

    /// Encoder known as "DC" in the PTHash papers
    pub struct PartitionedCompact;
    impl Encoder for PartitionedCompact {}

    #[cfg(feature = "hash64")]
    impl BackendForEncoderByHash<hash64> for PartitionedCompact {
        type SinglePhfBackend = crate::backends::singlephf_64_partitioned_compact_minimal;
        type PartitionedPhfBackend = crate::backends::partitionedphf_64_partitioned_compact_minimal;
    }

    #[cfg(feature = "hash128")]
    impl BackendForEncoderByHash<hash128> for PartitionedCompact {
        type SinglePhfBackend = crate::backends::singlephf_128_partitioned_compact_minimal;
        type PartitionedPhfBackend =
            crate::backends::partitionedphf_128_partitioned_compact_minimal;
    }
}

#[cfg(feature = "partitioned_compact")]
pub use partitioned_compact::*;

#[cfg(feature = "elias_fano")]
mod elias_fano {
    use super::*;

    /// Encoder known as "EF" in the PTHash papers
    pub struct EliasFano;
    impl Encoder for EliasFano {}

    #[cfg(feature = "hash64")]
    impl BackendForEncoderByHash<hash64> for EliasFano {
        type SinglePhfBackend = crate::backends::singlephf_64_elias_fano_minimal;
        type PartitionedPhfBackend = crate::backends::partitionedphf_64_elias_fano_minimal;
    }

    #[cfg(feature = "hash128")]
    impl BackendForEncoderByHash<hash128> for EliasFano {
        type SinglePhfBackend = crate::backends::singlephf_128_elias_fano_minimal;
        type PartitionedPhfBackend = crate::backends::partitionedphf_128_elias_fano_minimal;
    }
}

#[cfg(feature = "elias_fano")]
pub use elias_fano::*;

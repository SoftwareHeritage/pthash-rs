// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

//! Implementations of the last type parameter of [`SinglePhf`](crate::SinglePhf) and
//! [`PartitionedPhf`](crate::PartitionedPhf) ([`DictionaryDictionary`],
//! [`PartitionedCompact`], and [`EliasFano`])

use crate::hashing::Hash;
#[cfg(feature = "hash128")]
use crate::structs::hash128;
#[cfg(feature = "hash64")]
use crate::structs::hash64;

#[cfg(all(feature = "hash64", feature = "hash128"))]
#[allow(private_bounds)] // Users shouldn't be able to impl the Encoder trait
pub trait Encoder: BackendForEncoderByHash<hash64> + BackendForEncoderByHash<hash128> {
    /// Same value as the one passed as PTHash's CLI's -e argument
    const NAME: &'static str;
}
#[cfg(all(feature = "hash64", not(feature = "hash128")))]
#[allow(private_bounds)]
pub trait Encoder: BackendForEncoderByHash<hash64> {
    /// Same value as the one passed as PTHash's CLI's -e argument
    const NAME: &'static str;
}
#[cfg(all(not(feature = "hash64"), feature = "hash128"))]
#[allow(private_bounds)]
pub trait Encoder: BackendForEncoderByHash<hash128> {
    /// Same value as the one passed as PTHash's CLI's -e argument
    const NAME: &'static str;
}
// build.rs rejects both hash64 and hash128 being disabled

/// Type trickery to make [`Hash`] implementable
pub(crate) trait BackendForEncoderByHash<H: Hash> {
    #[cfg(feature = "minimal")]
    type MinimalSinglePhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
    #[cfg(feature = "nonminimal")]
    type NonminimalSinglePhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
    #[cfg(feature = "minimal")]
    type MinimalPartitionedPhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
    #[cfg(feature = "nonminimal")]
    type NonminimalPartitionedPhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
}

#[cfg(feature = "dictionary_dictionary")]
mod dictionary_dictionary {
    use super::*;

    /// Encoder known as "D-D" in the PTHash papers
    pub struct DictionaryDictionary;
    impl Encoder for DictionaryDictionary {
        const NAME: &'static str = "dictionary_dictionary";
    }

    #[cfg(feature = "hash64")]
    impl BackendForEncoderByHash<hash64> for DictionaryDictionary {
        #[cfg(feature = "minimal")]
        type MinimalSinglePhfBackend = crate::backends::singlephf_64_dictionary_dictionary_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalSinglePhfBackend =
            crate::backends::singlephf_64_dictionary_dictionary_nonminimal;
        #[cfg(feature = "minimal")]
        type MinimalPartitionedPhfBackend =
            crate::backends::partitionedphf_64_dictionary_dictionary_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalPartitionedPhfBackend =
            crate::backends::partitionedphf_64_dictionary_dictionary_nonminimal;
    }

    #[cfg(feature = "hash128")]
    impl BackendForEncoderByHash<hash128> for DictionaryDictionary {
        #[cfg(feature = "minimal")]
        type MinimalSinglePhfBackend = crate::backends::singlephf_128_dictionary_dictionary_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalSinglePhfBackend =
            crate::backends::singlephf_128_dictionary_dictionary_nonminimal;
        #[cfg(feature = "minimal")]
        type MinimalPartitionedPhfBackend =
            crate::backends::partitionedphf_128_dictionary_dictionary_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalPartitionedPhfBackend =
            crate::backends::partitionedphf_128_dictionary_dictionary_nonminimal;
    }
}

#[cfg(feature = "dictionary_dictionary")]
pub use dictionary_dictionary::*;

#[cfg(feature = "partitioned_compact")]
mod partitioned_compact {
    use super::*;

    /// Encoder known as "DC" in the PTHash papers
    pub struct PartitionedCompact;
    impl Encoder for PartitionedCompact {
        const NAME: &'static str = "partitioned_compact";
    }

    #[cfg(feature = "hash64")]
    impl BackendForEncoderByHash<hash64> for PartitionedCompact {
        #[cfg(feature = "minimal")]
        type MinimalSinglePhfBackend = crate::backends::singlephf_64_partitioned_compact_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalSinglePhfBackend =
            crate::backends::singlephf_64_partitioned_compact_nonminimal;
        #[cfg(feature = "minimal")]
        type MinimalPartitionedPhfBackend =
            crate::backends::partitionedphf_64_partitioned_compact_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalPartitionedPhfBackend =
            crate::backends::partitionedphf_64_partitioned_compact_nonminimal;
    }

    #[cfg(feature = "hash128")]
    impl BackendForEncoderByHash<hash128> for PartitionedCompact {
        #[cfg(feature = "minimal")]
        type MinimalSinglePhfBackend = crate::backends::singlephf_128_partitioned_compact_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalSinglePhfBackend =
            crate::backends::singlephf_128_partitioned_compact_nonminimal;
        #[cfg(feature = "minimal")]
        type MinimalPartitionedPhfBackend =
            crate::backends::partitionedphf_128_partitioned_compact_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalPartitionedPhfBackend =
            crate::backends::partitionedphf_128_partitioned_compact_nonminimal;
    }
}

#[cfg(feature = "partitioned_compact")]
pub use partitioned_compact::*;

#[cfg(feature = "elias_fano")]
mod elias_fano {
    use super::*;

    /// Encoder known as "EF" in the PTHash papers
    pub struct EliasFano;
    impl Encoder for EliasFano {
        const NAME: &'static str = "elias_fano";
    }

    #[cfg(feature = "hash64")]
    impl BackendForEncoderByHash<hash64> for EliasFano {
        #[cfg(feature = "minimal")]
        type MinimalSinglePhfBackend = crate::backends::singlephf_64_elias_fano_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalSinglePhfBackend = crate::backends::singlephf_64_elias_fano_nonminimal;
        #[cfg(feature = "minimal")]
        type MinimalPartitionedPhfBackend = crate::backends::partitionedphf_64_elias_fano_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalPartitionedPhfBackend =
            crate::backends::partitionedphf_64_elias_fano_nonminimal;
    }

    #[cfg(feature = "hash128")]
    impl BackendForEncoderByHash<hash128> for EliasFano {
        #[cfg(feature = "minimal")]
        type MinimalSinglePhfBackend = crate::backends::singlephf_128_elias_fano_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalSinglePhfBackend = crate::backends::singlephf_128_elias_fano_nonminimal;
        #[cfg(feature = "minimal")]
        type MinimalPartitionedPhfBackend = crate::backends::partitionedphf_128_elias_fano_minimal;
        #[cfg(feature = "nonminimal")]
        type NonminimalPartitionedPhfBackend =
            crate::backends::partitionedphf_128_elias_fano_nonminimal;
    }
}

#[cfg(feature = "elias_fano")]
pub use elias_fano::*;

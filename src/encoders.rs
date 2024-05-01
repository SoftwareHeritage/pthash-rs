// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use crate::hashing::Hash;
use crate::structs::{hash128, hash64};

pub trait Encoder {}

pub struct DictionaryDictionary;
impl Encoder for DictionaryDictionary {}

/// Type trickery to make [`Hash`] implementable
pub(crate) trait BackendForEncoderByHash<H: Hash>: Encoder {
    type SinglePhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
    type PartitionedPhfBackend: crate::backends::BackendPhf<Hash = H, Encoder = Self>;
}

impl BackendForEncoderByHash<hash64> for DictionaryDictionary {
    type SinglePhfBackend = crate::backends::singlephf_64_dictionary_minimal;
    type PartitionedPhfBackend = crate::backends::partitionedphf_64_dictionary_minimal;
}

impl BackendForEncoderByHash<hash128> for DictionaryDictionary {
    type SinglePhfBackend = crate::backends::singlephf_128_dictionary_minimal;
    type PartitionedPhfBackend = crate::backends::partitionedphf_128_dictionary_minimal;
}

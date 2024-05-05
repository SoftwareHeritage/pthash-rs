// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

//! Tests building and calling a [`SinglePhf`] with a custom [`Hasher`] defined in Rust

use std::hash::Hasher;

use anyhow::{Context, Result};

use pthash::*;

struct CustomHasher64;

impl pthash::Hasher for CustomHasher64 {
    type Hash = hashing::hash64;

    fn hash(val: impl Hashable, seed: u64) -> Self::Hash {
        // Reuse Rust's hashing algorithm
        let mut hasher = std::hash::DefaultHasher::new();
        hasher.write_u64(seed);
        hasher.write(val.as_bytes().as_ref());
        hasher.finish().into()
    }
}

struct CustomHasher128;

impl pthash::Hasher for CustomHasher128 {
    type Hash = hashing::hash128;

    fn hash(val: impl Hashable, seed: u64) -> Self::Hash {
        let mut high_hasher = std::hash::DefaultHasher::new();
        high_hasher.write_u64(seed);
        high_hasher.write(val.as_bytes().as_ref());

        let mut low_hasher = std::hash::DefaultHasher::new();
        low_hasher.write_u64(!seed);
        low_hasher.write(val.as_bytes().as_ref());

        (high_hasher.finish(), low_hasher.finish()).into()
    }
}

fn test_single<M: Minimality, H: pthash::Hasher, E: Encoder>() -> Result<()> {
    let temp_dir = tempfile::tempdir().context("Could not create temp dir")?;
    let mut config = BuildConfiguration::new(temp_dir.path().to_owned());
    config.verbose_output = false;

    let keys: Vec<&[u8]> = vec!["abc".as_bytes(), "def".as_bytes(), "ghikl".as_bytes()];

    let mut f = SinglePhf::<M, MurmurHash2_64, DictionaryDictionary>::new();
    f.build_in_internal_memory_from_bytes(&keys, &config)
        .context("Failed to build")?;

    // Hashes are unique
    let mut hashes: Vec<u64> = keys.iter().map(|key| f.hash(key)).collect();
    hashes.sort();
    assert_eq!(hashes, vec![0, 1, 2]);

    // Hashing an object that wasn't provided when building the function collides
    assert!(f.hash(b"not_a_key".as_bytes()) < 3);

    Ok(())
}

#[cfg(all(
    feature = "minimal",
    feature = "hash64",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_custom_hasher64_dictionary_dictionary() -> Result<()> {
    test_single::<Minimal, CustomHasher64, DictionaryDictionary>()
}

#[cfg(all(
    feature = "minimal",
    feature = "hash64",
    feature = "partitioned_compact"
))]
#[test]
fn test_custom_hasher64_partitioned_compact() -> Result<()> {
    test_single::<Minimal, CustomHasher64, PartitionedCompact>()
}

#[cfg(all(feature = "minimal", feature = "hash64", feature = "elias_fano"))]
#[test]
fn test_custom_hasher64_elias_fano() -> Result<()> {
    test_single::<Minimal, CustomHasher64, EliasFano>()
}

#[cfg(all(
    feature = "minimal",
    feature = "hash128",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_custom_hasher128_dictionary_dictionary() -> Result<()> {
    test_single::<Minimal, CustomHasher128, DictionaryDictionary>()
}

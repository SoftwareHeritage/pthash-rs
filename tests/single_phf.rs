// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

//! Tests building a [`SinglePhf`] with every possible type parameter, then
//! hashing a few keys.

use anyhow::{Context, Result};

use pthash::*;

fn test_single<M: Minimality, H: Hasher, E: Encoder>() -> Result<()> {
    let temp_dir = tempfile::tempdir().context("Could not create temp dir")?;
    let mut config = BuildConfiguration::new(temp_dir.path().to_owned());
    config.minimal_output = M::AS_BOOL;
    config.verbose_output = false;

    let keys: Vec<&[u8]> = vec!["abc".as_bytes(), "def".as_bytes(), "ghikl".as_bytes()];

    let mut f = SinglePhf::<Minimal, MurmurHash2_64, DictionaryDictionary>::new();
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
fn test_single_minimal_hash64_dictionary_dictionary() -> Result<()> {
    test_single::<Minimal, MurmurHash2_64, DictionaryDictionary>()
}

#[cfg(all(
    feature = "minimal",
    feature = "hash64",
    feature = "partitioned_compact"
))]
#[test]
fn test_single_minimal_hash64_partitioned_compact() -> Result<()> {
    test_single::<Minimal, MurmurHash2_64, PartitionedCompact>()
}

#[cfg(all(feature = "minimal", feature = "hash64", feature = "elias_fano"))]
#[test]
fn test_single_minimal_hash64_elias_fano() -> Result<()> {
    test_single::<Minimal, MurmurHash2_64, EliasFano>()
}

#[cfg(all(
    feature = "minimal",
    feature = "hash128",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_single_minimal_hash128_dictionary_dictionary() -> Result<()> {
    test_single::<Minimal, MurmurHash2_128, DictionaryDictionary>()
}

#[cfg(all(
    feature = "minimal",
    feature = "hash128",
    feature = "partitioned_compact"
))]
#[test]
fn test_single_minimal_hash128_partitioned_compact() -> Result<()> {
    test_single::<Minimal, MurmurHash2_128, PartitionedCompact>()
}

#[cfg(all(feature = "minimal", feature = "hash128", feature = "elias_fano"))]
#[test]
fn test_single_minimal_hash128_elias_fano() -> Result<()> {
    test_single::<Minimal, MurmurHash2_128, EliasFano>()
}

#[cfg(all(
    feature = "nonminimal",
    feature = "hash64",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_single_nonminimal_hash64_dictionary_dictionary() -> Result<()> {
    test_single::<Nonminimal, MurmurHash2_64, DictionaryDictionary>()
}

#[cfg(all(
    feature = "nonminimal",
    feature = "hash64",
    feature = "partitioned_compact"
))]
#[test]
fn test_single_nonminimal_hash64_partitioned_compact() -> Result<()> {
    test_single::<Nonminimal, MurmurHash2_64, PartitionedCompact>()
}

#[cfg(all(feature = "nonminimal", feature = "hash64", feature = "elias_fano"))]
#[test]
fn test_single_nonminimal_hash64_elias_fano() -> Result<()> {
    test_single::<Nonminimal, MurmurHash2_64, EliasFano>()
}

#[cfg(all(
    feature = "nonminimal",
    feature = "hash128",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_single_nonminimal_hash128_dictionary_dictionary() -> Result<()> {
    test_single::<Nonminimal, MurmurHash2_128, DictionaryDictionary>()
}

#[cfg(all(
    feature = "nonminimal",
    feature = "hash128",
    feature = "partitioned_compact"
))]
#[test]
fn test_single_nonminimal_hash128_partitioned_compact() -> Result<()> {
    test_single::<Nonminimal, MurmurHash2_128, PartitionedCompact>()
}

#[cfg(all(feature = "nonminimal", feature = "hash128", feature = "elias_fano"))]
#[test]
fn test_single_nonminimal_hash128_elias_fano() -> Result<()> {
    test_single::<Nonminimal, MurmurHash2_128, EliasFano>()
}

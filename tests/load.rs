// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

//! Tests functions generated purely with the C++ implementation can be read from
//! Rust and compute the same values.
//!
//! This assumes PTHash's executable is available as `pthash/build/build`
//! (built with `mkdir pthash/build; cd pthash/build; cmake ..; make`)

use std::collections::HashSet;
use std::env;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::Once;

use anyhow::{bail, Context, Result};
use rand::distr::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use pthash::*;

static INIT: Once = Once::new();

pub fn setup_test() {
    INIT.call_once(|| {
        // Build pthash executable named 'build' required for tests
        let mut out_dir = env::current_dir().unwrap();
        out_dir.push("pthash");

        cmake::Config::new("pthash")
            .out_dir(out_dir.as_path())
            .target(target_triple::TARGET)
            .host(target_triple::HOST)
            .profile("Debug")
            .build_target("build")
            .build();
    });
}

macro_rules! impl_test {
    ($test_name:ident, $struct_name:ident) => {
        fn $test_name<M: Minimality, H: Hasher, E: Encoder>(
            mut num_keys: u64,
            num_partitions: u64,
        ) -> Result<()> {
            setup_test();
            let temp_dir = tempfile::tempdir().context("Could not create temp dir")?;
            let phf_path = temp_dir.path().join("phf.bin");

            let mut rng = StdRng::seed_from_u64(42);
            let mut keys = Vec::new();
            let mut key_set = HashSet::new();
            for _ in (0..num_keys) {
                let len: u8 = rng.random();
                let key: Vec<u8> = (0..len).map(|_| rng.sample(&Alphanumeric)).collect();
                if !key_set.contains(&key) {
                    // Cannot have duplicates in the set of keys.
                    // We push to a vec in order for the test to be deterministic
                    key_set.insert(key.clone());
                    keys.push(key);
                }
            }
            drop(key_set);
            num_keys = keys.len() as u64;

            let log_path = temp_dir.path().join("logs.txt");
            let log_file = std::fs::File::create(&log_path).context("Could not create log file")?;

            let mut cmd = Command::new("./pthash/build/build");
            cmd.arg("-d")
                .arg(temp_dir.path())
                .arg("--check")
                .arg("-i")
                .arg("-") // Read from stdin
                .arg("-n")
                .arg(num_keys.to_string())
                .arg("-c")
                .arg("3.0")
                .arg("-e")
                .arg(E::NAME)
                .arg("-a")
                .arg("0.5")
                .arg("-o")
                .arg(&phf_path)
                .arg("-p")
                .arg(num_partitions.to_string())
                .stdout(log_file.try_clone().context("Could not dup file")?)
                .stderr(log_file.try_clone().context("Could not dup file")?)
                .stdin(Stdio::piped());
            drop(log_file);

            if M::AS_BOOL {
                cmd.arg("--minimal");
            }

            let mut proc = cmd
                .spawn()
                .context("Could not spawn ./pthash/build/build")?;
            let stdin = proc.stdin.as_mut().unwrap();
            for key in &keys {
                stdin
                    .write(key)
                    .context("Could not write to pthash/build/build's stdin")?;
                stdin
                    .write(b"\n")
                    .context("Could not write to pthash/build/build's stdin")?;
            }
            if !proc
                .wait()
                .context("Could not wait ./pthash/build/build")?
                .success()
            {
                let mut logs = String::new();
                let mut log_file =
                    std::fs::File::open(&log_path).context("Could not open log file")?;
                log_file
                    .read_to_string(&mut logs)
                    .context("Could not read log file")?;
                bail!("{:?} failed with:\n{}", cmd, logs,);
            }

            let f = $struct_name::<M, H, E>::load(&phf_path).context("Failed to load PHF")?;

            if M::AS_BOOL {
                // Hashes are unique and in the [0; num_keys) segment
                let mut hashes: Vec<u64> = keys.iter().map(|key| f.hash(&**key)).collect();
                hashes.sort();
                assert_eq!(hashes, Vec::from_iter(0..num_keys));

                // Hashing an object that wasn't provided when building the function collides
                assert!(f.hash(b"not_a_key".as_bytes()) < num_keys);
            } else {
                // Hashes are unique
                let mut hashes: Vec<u64> = keys.iter().map(|key| f.hash(&**key)).collect();
                hashes.sort();

                // But not in the [0; num_keys) segment (not minimal)
                assert_ne!(hashes, Vec::from_iter(0..num_keys));

                // Hashing an object that wasn't provided when building the function may collide
                // or exceed the maximum
                assert!(f.hash(b"not_a_key".as_bytes()) < *hashes.last().unwrap() * 1000);
            }

            Ok(())
        }
    };
}

impl_test!(test_single, SinglePhf);
impl_test!(test_partitioned, PartitionedPhf);

#[cfg(all(
    feature = "minimal",
    feature = "hash64",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_single_minimal_hash64_dictionary_dictionary() -> Result<()> {
    test_single::<Minimal, MurmurHash2_64, DictionaryDictionary>(100, 1)
}

#[cfg(all(
    feature = "minimal",
    feature = "hash64",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_single_minimal_hash64_dictionary_dictionary_many_keys() -> Result<()> {
    test_single::<Minimal, MurmurHash2_64, DictionaryDictionary>(200000, 1)
}

#[cfg(all(
    feature = "nonminimal",
    feature = "hash64",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_single_nonminimal_hash64_dictionary_dictionary() -> Result<()> {
    test_single::<Nonminimal, MurmurHash2_64, DictionaryDictionary>(100, 1)
}

#[cfg(all(
    feature = "minimal",
    feature = "hash64",
    feature = "partitioned_compact"
))]
#[test]
fn test_single_minimal_hash64_partitioned_compact() -> Result<()> {
    test_single::<Minimal, MurmurHash2_64, PartitionedCompact>(100, 1)
}

#[cfg(all(feature = "minimal", feature = "hash64", feature = "elias_fano"))]
#[test]
fn test_single_minimal_hash64_elias_fano() -> Result<()> {
    test_single::<Minimal, MurmurHash2_64, EliasFano>(100, 1)
}

#[cfg(all(
    feature = "minimal",
    feature = "hash64",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_2partitions_minimal_hash64_dictionary_dictionary() -> Result<()> {
    test_partitioned::<Minimal, MurmurHash2_64, DictionaryDictionary>(200000, 2)
}

#[cfg(all(
    feature = "nonminimal",
    feature = "hash64",
    feature = "dictionary_dictionary"
))]
#[test]
fn test_2partitions_nonminimal_hash64_dictionary_dictionary() -> Result<()> {
    test_partitioned::<Nonminimal, MurmurHash2_64, DictionaryDictionary>(200000, 2)
}

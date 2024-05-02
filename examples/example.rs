// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

/*
 * This example is translated from https://archive.softwareheritage.org/swh:1:cnt:3b8644b100f586474f43335e5f057d4c82dcb225;origin=https://github.com/jermp/pthash;visit=swh:1:snp:86eb28469f3ee79401a63b464948e17c210bdb37;anchor=swh:1:rev:28aedcb03af096c0e9988b1ad240da7b4cf010d7;path=/src/example.cpp
 * which is distributed under the following license:
 *
 * Copyright 2020-2024 Giulio Ermanno Pibiri and Roberto Trani
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use std::collections::HashSet;
use std::time::Instant;

use rand::prelude::*;
use thiserror::Error;

use pthash::{
    BuildConfiguration, DictionaryDictionary, Hashable, Minimal, MurmurHash2_64, Phf, SinglePhf,
};

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("I/O error: {0}")]
    CxxIO(cxx::Exception),
    #[error("Could not build MPH: {0}")]
    Building(cxx::Exception),
    #[error("Function violates invariant: {0}")]
    ViolatedInvariant(#[from] ViolatedInvariant),
}

fn main() {
    if let Err(e) = main_() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
fn main_() -> Result<(), Error> {
    stderrlog::new()
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .expect("Could not initialize stderrlog");

    /* Generate 10M random 64-bit keys as input data. */
    let num_keys: usize = 10000000;
    let seed: u64 = 1234567890;
    log::info!("generating input data...");
    let mut keys = HashSet::<u64>::with_capacity(num_keys);
    let mut rng = StdRng::seed_from_u64(seed);
    while keys.len() < num_keys {
        keys.insert(rng.gen());
    }
    let mut keys: Vec<_> = keys.into_iter().collect();
    // iterating on a HashSet does not have a guaranteed order, and we want determinism
    keys.sort();
    keys.shuffle(&mut rng);

    /* Set up a build configuration. */
    let temp_dir = tempfile::tempdir()?;
    let mut config = BuildConfiguration::new(temp_dir.path().to_owned());
    config.c = 6.0;
    config.alpha = 0.94;
    config.minimal_output = true; // mphf
    config.verbose_output = true;

    /* Declare the PTHash function. */
    let mut f = SinglePhf::<Minimal, MurmurHash2_64, DictionaryDictionary>::new();

    // config.num_partitions = 50;
    // config.num_threads = 4;
    // let f = PartitionedPhf_Minimal::<MurmurHash2_64, DictionaryDictionary>::new()

    /* Build the function in internal memory. */
    log::info!("building the function...");
    let start = Instant::now();
    let timings = f
        .build_in_internal_memory_from_bytes(&keys, &config)
        .map_err(Error::Building)?;
    // let timings = f.build_in_external_memory(keys, config);
    log::info!("function built in {} seconds", start.elapsed().as_secs());
    let total_seconds = timings.partitioning_seconds
        + timings.mapping_ordering_seconds
        + timings.searching_seconds
        + timings.encoding_seconds;
    log::info!("computed: {} seconds", total_seconds.as_secs_f64());

    /* Compute and print the number of bits spent per key. */
    let bits_per_key = (f.num_bits() as f64) / (f.num_keys() as f64);
    log::info!("function uses {} [bits/key]", bits_per_key);

    /* Sanity check! */
    check(&keys, &f)?;
    log::info!("EVERYTHING OK!");

    /* Now evaluate f on some keys. */
    for i in 0..10 {
        log::info!("f({}) = {}", keys[i], f.hash(keys[i]));
    }

    /* Serialize the data structure to a file. */
    log::info!("serializing the function to disk...");
    let output_path = temp_dir.path().join("pthash.bin");
    f.save(&output_path).map_err(Error::CxxIO)?;

    log::info!("reading the function from disk...");
    {
        /* Now reload from disk and query. */
        let other = SinglePhf::<Minimal, MurmurHash2_64, encoders::DictionaryDictionary>::load(
            &output_path,
        )
        .map_err(Error::CxxIO)?;
        for i in 0..10 {
            log::info!("f({}) = {}", keys[i], other.hash(keys[i]));
            assert_eq!(f.hash(keys[i]), other.hash(keys[i]));
        }
    }

    Ok(())
}

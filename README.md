# pthash-rs

Rust bindings for [PTHash](https://github.com/jermp/pthash), a C++ library implementing
perfect-hash functions using:

* Giulio Ermanno Pibiri and Roberto Trani. [*"PTHash: Revisiting FCH Minimal Perfect Hashing"*](https://dl.acm.org/doi/10.1145/3404835.3462849). In Proceedings of the 44th International
Conference on Research and Development in Information Retrieval (SIGIR). 2021.
* Giulio Ermanno Pibiri and Roberto Trani. [*"Parallel and External-Memory Construction of Minimal Perfect Hash Functions with PTHash"*](https://ieeexplore.ieee.org/document/10210677). Transactions on Knowledge and Data Engineering (TKDE). 2023.

## Building

```ignore
apt install build-essential libclang-dev
git clone https://gitlab.softwareheritage.org/swh/devel/pthash-rs.git
cd pthash-rs
git submodule update --init --recursive
cargo build
```

If you get a panic at build time with message `Non floating-type complex? Type(_Complex _Float16, kind: Complex`,
this is due to [a bug in autocxx-bindgen](https://github.com/google/autocxx/issues/1341).
To fix it, make sure you have `autocxx` 0.27 or greater in your `Cargo.lock`.
IF you can't (eg. to support Rust < 1.80), a workaround is to change your `Cargo.toml` to [override the autocxx-bindgen
dependency](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html)
to a fork with a cherry-picked patch:

```ignore
[patch.crates-io]
autocxx-bindgen = { git = "https://gitlab.softwareheritage.org/vlorentz/rust-bindgen.git", rev = "fe69d3e4f51b9d586f56f930ac8c7a17ad35dc62" }
```


## Internal code structure

Due to C++ templates being closer to macros than to Rust generics, every possible instantiation
of type parameters of the Rust struct needs to be mapped to a concrete C++ class.

This is invisible when using the crate, but means only hash algorithms and encoders
explicitly allowed by this crate can be used.
Additionally, the allow list can be adjusted through features to cut down on
the combinatorial explosion of template instantiations and linking with Rust types;
see `Cargo.toml` for details.

## Examples

## Minimal PHF

```
use pthash::{
    BuildConfiguration, DictionaryDictionary, Hashable, Minimal, MurmurHash2_64, Phf, SinglePhf
};

let temp_dir = tempfile::tempdir().unwrap();
let mut config = BuildConfiguration::new(temp_dir.path().to_owned());
// config.minimal_output = true; // unlike the C++ API, this is implicit from f's type

let keys: Vec<&[u8]> = vec!["abc".as_bytes(), "def".as_bytes(), "ghikl".as_bytes()];

let mut f = SinglePhf::<Minimal, MurmurHash2_64, DictionaryDictionary>::new();
f.build_in_internal_memory_from_bytes(&keys, &config).expect("Failed to build");

// Hashes are unique and in the [0; 3) segment
let mut hashes: Vec<u64> = keys.iter().map(|key| f.hash(key)).collect();
hashes.sort();
assert_eq!(hashes, vec![0, 1, 2]);

// Hashing an object that wasn't provided when building the function collides
assert!(f.hash(b"not_a_key".as_bytes()) < 3);
```

## Non-minimal PHF

```
use pthash::{
    BuildConfiguration, DictionaryDictionary, Hashable, Nonminimal, MurmurHash2_64, Phf, SinglePhf
};

let temp_dir = tempfile::tempdir().unwrap();
let mut config = BuildConfiguration::new(temp_dir.path().to_owned());

let keys: Vec<&[u8]> = vec!["abc".as_bytes(), "def".as_bytes(), "ghikl".as_bytes()];

let mut f = SinglePhf::<Nonminimal, MurmurHash2_64, DictionaryDictionary>::new();
f.build_in_internal_memory_from_bytes(&keys, &config).expect("Failed to build");

// Hashes are unique
let mut hashes: Vec<u64> = keys.iter().map(|key| f.hash(key)).collect();
hashes.sort();

// But not necessarily in the [0; 3) segment (not minimal)
// assert_eq!(hashes, vec![0, 1, 2]);

// Hashing an object that wasn't provided when building the function may collide
// assert!(!hashes.contains(f.hash(b"not_a_key".as_bytes())));
```

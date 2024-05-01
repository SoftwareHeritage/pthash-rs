# pthash-rs

Rust bindings for [PTHash](https://github.com/jermp/pthash), a C++ library implementing
perfect-hash functions using:

* Giulio Ermanno Pibiri and Roberto Trani. [*"PTHash: Revisiting FCH Minimal Perfect Hashing"*](https://dl.acm.org/doi/10.1145/3404835.3462849). In Proceedings of the 44th International
Conference on Research and Development in Information Retrieval (SIGIR). 2021.
* Giulio Ermanno Pibiri and Roberto Trani. [*"Parallel and External-Memory Construction of Minimal Perfect Hash Functions with PTHash"*](https://ieeexplore.ieee.org/document/10210677). Transactions on Knowledge and Data Engineering (TKDE). 2023.

# Example

```
use pthash::*;

let temp_dir = tempfile::tempdir().unwrap();
let mut config = BuildConfiguration::new(temp_dir.path().to_owned());

let keys: Vec<&[u8]> = vec!["abc".as_bytes(), "def".as_bytes(), "ghikl".as_bytes()];

let mut f = SinglePhf_Minimal::<MurmurHash2_64, DictionaryDictionary>::new();
f.build_in_internal_memory_from_bytes(&keys, &config).expect("Failed to build");

// Hashes are unique
let mut hashes: Vec<u64> = keys.iter().map(|key| f.hash(key)).collect();
hashes.sort();
assert_eq!(hashes, vec![0, 1, 2]);

// Hashing an object that wasn't provided when building the function collides
assert!(f.hash(b"not_a_key".as_bytes()) < 3);
```


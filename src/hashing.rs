// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use autocxx::prelude::*;

use crate::structs::{hash128, hash64};

pub(crate) trait Hash: Sized {
    type SinglePhfBackend: crate::backends::BackendPhf<Hash = Self>;
    type PartitionedPhfBackend: crate::backends::BackendPhf<Hash = Self>;
}

impl Hash for hash64 {
    type SinglePhfBackend = crate::backends::singlephf_dictionary_minimal;
    type PartitionedPhfBackend = crate::backends::partitionedphf_dictionary_minimal;
}

pub trait Hashable {
    type Bytes<'a>: AsRef<[u8]>
    where
        Self: 'a;

    fn as_bytes(&self) -> Self::Bytes<'_>;
}

impl Hashable for [u8] {
    type Bytes<'a> = &'a [u8];

    fn as_bytes(&self) -> Self::Bytes<'_> {
        self
    }
}

impl<'a, T: Hashable> Hashable for &'a T {
    type Bytes<'b> = T::Bytes<'b> where Self: 'b;

    fn as_bytes(&self) -> Self::Bytes<'_> {
        T::as_bytes(self)
    }
}

impl Hashable for u64 {
    type Bytes<'a> = [u8; 8] where Self: 'a;

    fn as_bytes(&self) -> Self::Bytes<'_> {
        // quirk-compatibility with the C++ implementation
        #[cfg(target_endian = "little")]
        let bytes = self.to_le_bytes();
        #[cfg(target_endian = "big")]
        let bytes = self.to_be_bytes();
        bytes
    }
}

pub trait Hasher {
    #[allow(private_bounds)] // Users shouldn't be able to impl the Hash trait
    type Hash: Hash;

    fn hash(val: impl Hashable, seed: u64) -> Self::Hash;
}

#[cxx::bridge]
mod ffi {
    struct byte_range {
        begin: *const u8,
        end: *const u8,
    }

    #[namespace = "pthash_rs::utils"]
    unsafe extern "C++" {
        include!("cpp-utils.hpp");

        type c_void; // https://github.com/dtolnay/cxx/issues/1049#issuecomment-1312854737
    }

    #[namespace = "pthash"]
    unsafe extern "C++" {
        include!("pthash.hpp");

        unsafe fn MurmurHash2_64(key: *const c_void, len: usize, seed: u64) -> u64;
    }
}

pub struct MurmurHash2_64;

impl Hasher for MurmurHash2_64 {
    type Hash = hash64;

    fn hash(val: impl Hashable, seed: u64) -> Self::Hash {
        let val = val.as_bytes();
        let val = val.as_ref();
        moveit! {
            let h = unsafe { hash64::new1(
                ffi::MurmurHash2_64(
                val.as_ptr() as *const ffi::c_void,
                val.len(),
                seed,
                )
            ) };
        };
        autocxx::moveit::MoveRef::into_inner(std::pin::Pin::into_inner(h))
    }
}

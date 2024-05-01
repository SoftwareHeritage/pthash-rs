// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use autocxx::prelude::*;

use crate::encoders::{BackendForEncoderByHash, DictionaryDictionary, Encoder};
#[cfg(feature = "hash64")]
use crate::structs::hash64;
#[cfg(feature = "hash128")]
use crate::structs::hash128;

pub(crate) trait Hash: Sized {
    type SinglePhfBackend<E: Encoder>: crate::backends::BackendPhf<Hash = Self>;
    type PartitionedPhfBackend<E: Encoder>: crate::backends::BackendPhf<Hash = Self>;
}

#[cfg(feature = "hash64")]
impl Hash for hash64 {
    type SinglePhfBackend<E: Encoder> =
        <DictionaryDictionary as BackendForEncoderByHash<Self>>::SinglePhfBackend;
    type PartitionedPhfBackend<E: Encoder> =
        <DictionaryDictionary as BackendForEncoderByHash<Self>>::PartitionedPhfBackend;
}

#[cfg(feature = "hash128")]
impl Hash for hash128 {
    type SinglePhfBackend<E: Encoder> =
        <DictionaryDictionary as BackendForEncoderByHash<Self>>::SinglePhfBackend;
    type PartitionedPhfBackend<E: Encoder> =
        <DictionaryDictionary as BackendForEncoderByHash<Self>>::PartitionedPhfBackend;
}

/// Trait of types which can be hashed with PTHash perfect hash functions.
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

impl<'a, T: Hashable + ?Sized> Hashable for &'a T {
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

/// Trait of generic non-cryptographic hash function, which can be used to back
/// a PTHash perfect hash function.
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

#[cfg(feature = "hash64")]
/// Implementation of the Murmur2 64-bits hash
///
/// This is a reimplementation of `pthash::murmurhash2_64` on top of `pthash::MurmurHash2_64`
/// (not a binding for `pthash::MurmurHash2_64` or `pthash::murmurhash2_64`).
pub struct MurmurHash2_64;

#[cfg(feature = "hash64")]
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

#[cfg(feature = "hash128")]
/// Implementation of a Murmur2 128-bits hash
///
/// This hash is obtained by computing [`MurmurHash2_64`] for both the seed and
/// the bitwise negation of the seed and concatenating them.
///
/// This is a reimplementation of `pthash::murmurhash2_128` on top of `pthash::MurmurHash2_64`
/// (not a binding for `pthash::MurmurHash2_128`).
pub struct MurmurHash2_128;

#[cfg(feature = "hash128")]
impl Hasher for MurmurHash2_128 {
    type Hash = hash128;

    fn hash(val: impl Hashable, seed: u64) -> Self::Hash {
        let val = val.as_bytes();
        let val = val.as_ref();
        moveit! {
            let h = unsafe { hash128::new1(
                ffi::MurmurHash2_64(
                    val.as_ptr() as *const ffi::c_void,
                    val.len(),
                    seed,
                ),
                ffi::MurmurHash2_64(
                    val.as_ptr() as *const ffi::c_void,
                    val.len(),
                    !seed,
                ),
            ) };
        };
        autocxx::moveit::MoveRef::into_inner(std::pin::Pin::into_inner(h))
    }
}

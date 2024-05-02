// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

//! [`Minimal`] and [`Nonminimal`] unit structs used as type parameter to indicate whether
//! a PHF is minimal (aka. a MPHF).
//!
//! Ideally this would be a `bool` const generic, but we need to use a trait instead
//! due to the underlying C++ binding.

use crate::encoders::Encoder;
use crate::hashing::Hash;

pub(crate) trait SealedMinimality {
    type SinglePhfBackend<H: Hash, E: Encoder>: crate::backends::BackendPhf<Hash = H>;
    type PartitionedPhfBackend<H: Hash, E: Encoder>: crate::backends::BackendPhf<Hash = H>;
}

#[allow(private_bounds)]
pub trait Minimality: SealedMinimality {
    const AS_BOOL: bool;
}

#[cfg(feature = "minimal")]
/// Type parameter of PHFs whose values are contiguous in the `[0; num_keys)` segment
pub struct Minimal;
#[cfg(feature = "minimal")]
impl Minimality for Minimal {
    const AS_BOOL: bool = true;
}
#[cfg(feature = "minimal")]
impl SealedMinimality for Minimal {
    type SinglePhfBackend<H: Hash, E: Encoder> = <H as Hash>::MinimalSinglePhfBackend<E>;
    type PartitionedPhfBackend<H: Hash, E: Encoder> = <H as Hash>::MinimalPartitionedPhfBackend<E>;
}

#[cfg(feature = "nonminimal")]
/// Opposite of [`Nonminimal`]
pub struct Nonminimal;
#[cfg(feature = "nonminimal")]
impl Minimality for Nonminimal {
    const AS_BOOL: bool = false;
}
#[cfg(feature = "nonminimal")]
impl SealedMinimality for Nonminimal {
    type SinglePhfBackend<H: Hash, E: Encoder> = <H as Hash>::NonminimalSinglePhfBackend<E>;
    type PartitionedPhfBackend<H: Hash, E: Encoder> =
        <H as Hash>::NonminimalPartitionedPhfBackend<E>;
}

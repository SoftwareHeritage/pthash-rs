// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#![allow(clippy::missing_safety_doc)]

use autocxx::prelude::*;

include_cpp! {
    #include "pthash.hpp"

    generate_pod!("pthash::build_timings")
    generate_pod!("pthash::hash64")
    generate_pod!("pthash::hash128")
}

pub(crate) use ffi::pthash::build_timings;
pub use ffi::pthash::{hash128, hash64};

impl From<u64> for hash64 {
    fn from(value: u64) -> Self {
        moveit! {
            let h = unsafe { hash64::new1(value) };
        };
        autocxx::moveit::MoveRef::into_inner(std::pin::Pin::into_inner(h))
    }
}

impl From<u128> for hash128 {
    fn from(value: u128) -> Self {
        let high = (value >> 64) as u64;
        let low = (value & 0xFFFFFFFF) as u64;
        (high, low).into()
    }
}

/// Builds a hash128 from a pair of `(high_bits, low_bits)`
impl From<(u64, u64)> for hash128 {
    fn from(value: (u64, u64)) -> Self {
        let (high, low) = value;
        moveit! {
            let h = unsafe { hash128::new1(high, low) };
        };
        autocxx::moveit::MoveRef::into_inner(std::pin::Pin::into_inner(h))
    }
}

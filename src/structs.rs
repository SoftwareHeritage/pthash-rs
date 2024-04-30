// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use autocxx::prelude::*;

include_cpp! {
    #include "pthash.hpp"

    //generate_pod!("pthash::util::byte_range")
    generate_pod!("pthash::build_timings")
    generate_pod!("pthash::hash64")
    generate_pod!("pthash::hash128")
}

//fn test(val: ffi::byte_range) {}

pub(crate) use ffi::pthash::{build_timings, hash128, hash64};

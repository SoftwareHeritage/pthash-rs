// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#pragma once

#include <pthash.hpp>

#define concrete(hash_size, encoder) \
    typedef pthash::single_phf<mock_hasher ## hash_size, pthash::encoder, true> \
        singlephf_## hash_size ## _ ## encoder ## _minimal; \
    typedef pthash::partitioned_phf<mock_hasher ## hash_size, pthash::encoder, true> \
        partitionedphf_## hash_size ## _ ## encoder ## _minimal;

namespace pthash_rs {

    namespace concrete {
        // Dummies, used as template values but it is actually dead code
        struct mock_hasher64 {
            typedef pthash::hash64 hash_type;
        };
        struct mock_hasher128 {
            typedef pthash::hash128 hash_type;
        };

        typedef pthash::internal_memory_builder_single_phf<mock_hasher64>
            internal_memory_builder_single_phf_64;

        typedef pthash::internal_memory_builder_single_phf<mock_hasher128>
            internal_memory_builder_single_phf_128;

        typedef pthash::internal_memory_builder_partitioned_phf<mock_hasher64>
            internal_memory_builder_partitioned_phf_64;

        typedef pthash::internal_memory_builder_partitioned_phf<mock_hasher128>
            internal_memory_builder_partitioned_phf_128;

        concrete(64, dictionary_dictionary);
        concrete(128, dictionary_dictionary);
        concrete(64, partitioned_compact);
        concrete(128, partitioned_compact);
        concrete(64, elias_fano);
        concrete(128, elias_fano);
    }

}

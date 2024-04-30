// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#pragma once

#include <pthash.hpp>

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

        typedef pthash::single_phf<
                mock_hasher64,
                pthash::dictionary_dictionary,
                true
            >
            singlephf_64_dictionary_minimal;

        typedef pthash::single_phf<
                mock_hasher128,
                pthash::dictionary_dictionary,
                true
            >
            singlephf_128_dictionary_minimal;

        typedef pthash::internal_memory_builder_partitioned_phf<mock_hasher64>
            internal_memory_builder_partitioned_phf_64;

        typedef pthash::internal_memory_builder_partitioned_phf<mock_hasher128>
            internal_memory_builder_partitioned_phf_128;

        typedef pthash::partitioned_phf<
                mock_hasher64,
                pthash::dictionary_dictionary,
                true
            >
            partitionedphf_64_dictionary_minimal;

        typedef pthash::partitioned_phf<
                mock_hasher128,
                pthash::dictionary_dictionary,
                true
            >
            partitionedphf_128_dictionary_minimal;
    }

}

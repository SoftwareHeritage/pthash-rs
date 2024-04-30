// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#pragma once

#include <pthash.hpp>

namespace pthash_rs {

    namespace concrete {
        // Dummy, used as a template value but it is actually dead code
        struct mock_hasher {
            typedef pthash::hash64 hash_type;
        };

        typedef pthash::internal_memory_builder_single_phf<mock_hasher>
            internal_memory_builder_single_phf;

        typedef pthash::single_phf<
                mock_hasher,
                pthash::dictionary_dictionary,
                true
            >
            singlephf_dictionary_minimal;
    }

}

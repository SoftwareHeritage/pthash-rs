// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information


namespace pthash_rs {
    namespace workarounds {
        // A visitor for single_phf that gets the seed (a private attribute) and makes
        // it available to the Rust code
        struct get_seed_visitor {
            // Whether the seed was set
            bool got_seed = false;

            // Value of the seed obtained from the function
            uint64_t seed;

            void visit(uint64_t &value) {
                // The seed is the very first field to be serialized.
                if (!got_seed) {
                    got_seed = true;
                    seed = value;
                }
            }

            template <typename T>
            void visit(T &value) {
                (void) value;
                if (!got_seed) {
                    throw std::runtime_error("seed was not the first visited field");
                }
            }
        };

        template <typename Function>
        uint64_t get_seed(Function &f) {
            get_seed_visitor visitor;
            f.visit(visitor);

            if (!visitor.got_seed) {
                throw std::runtime_error("Could not get seed from function");
            }
            return visitor.seed;
        }

        // A visitor for single_phf that allows Rust code to set the seed
        // (a private attribute) and makes
        struct set_seed_visitor {
            // Whether the seed was set
            bool set_seed = false;

            // Value of the seed obtained from the function
            uint64_t seed;

            void visit(uint64_t &value) {
                // The seed is the very first field to be serialized.
                if (!set_seed) {
                    value = seed;
                    set_seed = true;
                }
            }

            template <typename T>
            void visit(T &value) {
                (void) value;
                if (!set_seed) {
                    throw std::runtime_error("seed was not the first visited field");
                }
            }
        };

        template <typename Builder>
        void set_seed(Builder &builder, uint64_t seed) {
            set_seed_visitor visitor;
            visitor.seed = seed;

            builder.visit(visitor);

            if (!visitor.set_seed) {
                throw std::runtime_error("Could not set function's seed");
            }
        }
    }
}

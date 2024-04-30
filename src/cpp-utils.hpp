// Copyright (C) 2023-2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#pragma once

#include <memory>

#include <pthash.hpp>

#define getter(name) \
    template<typename T, typename Ret> \
    Ret \
    get_## name(T &obj) \
    { \
      return obj.name; \
    }

#define setter(name) \
    template<typename T, typename Val> \
    void \
    set_## name(std::unique_ptr<T> &obj, Val value) \
    { \
      obj->name = value; \
    }

#define gettersetter(name) \
    getter(name) \
    setter(name)

namespace pthash_rs {
    namespace utils {
        using c_void = void; // https://github.com/dtolnay/cxx/issues/1049#issuecomment-1312854737

        // Constructs a C++ object using this trick:
        // https://github.com/dtolnay/cxx/issues/280#issuecomment-1344153115
        template<typename T, typename... Args>
        std::unique_ptr<T>
        construct(Args... args)
        {
          return std::make_unique<T>(args...);
        }
        template<typename T, typename... Args>
        T
        construct_noalloc(Args... args)
        {
          return T(args...);
        }

        template<typename T>
        std::unique_ptr<T>
        construct_copy(std::unique_ptr<T> const &obj)
        {
          return std::make_unique<T>(T(*obj));
        }

        template<typename T, typename Ret>
        Ret
        try_into(T obj) {
          return dynamic_cast<Ret>(obj);
        }

        template<typename T, typename Ret>
        std::unique_ptr<Ret>
    ptr_try_into(std::unique_ptr<T> obj) {
      std::unique_ptr<Ret> p(dynamic_cast<Ret*>(obj.get()));
          return p;
        }

        template<typename T, typename Ret>
        Ret
        into(T obj)
        {
          return obj;
        }

        template<typename T>
        std::unique_ptr<std::string> toString(T &obj) {
            return std::make_unique<std::string>(obj.toString());
        }

        template<typename T> // Force C++ compiler to inline it
        bool valid_seed(T seed) {
            return seed != ::pthash::constants::invalid_seed;
        }
    }


    namespace accessors {
        gettersetter(c)
        gettersetter(alpha)
        gettersetter(num_partitions)
        gettersetter(num_threads)
        gettersetter(seed)
        gettersetter(ram)
        gettersetter(tmp_dir)
        gettersetter(num_buckets)
        gettersetter(minimal_output)
        gettersetter(verbose_output)
    }
}



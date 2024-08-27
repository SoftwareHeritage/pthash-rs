# v0.4.0

*2024-08-27*

Breaking change:

* `build_in_internal_memory_from_bytes`: Expect closure yielding iterables, instead of needing clonable iterables

Improvements:

* Remove unnecessary `ExactSizeIterator` bound on `build_in_internal_memory_from_bytes`'s argument
* Add `par_build_in_internal_memory_from_bytes` to hash in parallel (using [Rayon](https://crates.io/crates/rayon))

Documentation:

* README: Don't highlight console block as Rust code
* Fix keywords and categories

# v0.3.3

*2024-08-22*

* Fix homepage/repository URL

# v0.3.2

*2024-08-13*

* Lower MSRV to 1.77

# v0.3.1

*2024-08-01*

* Add support for autocxx 0.27 to fix build in some environments (Clang >= 16?)
* Bump MSRV to 1.80

# v0.3.0

*2024-08-01*

Accidentally published without the expected changes, yanked

# v0.2.0

*2024-06-18*

* Add support for defining custom hashers
* Fix build on GCC <= 10

# v0.1.0

*2024-05-03*

Initial release.


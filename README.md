<!-- Generated by cargo-onedoc. DO NOT EDIT. -->

# itermore

[![Crates.io Version](https://badgers.space/crates/version/itermore)](https://crates.io/crates/itermore)
[![Docs.rs Latest](https://badgers.space/badge/docs.rs/latest/blue)](https://docs.rs/itermore)
[![Build Status](https://badgers.space/github/checks/rossmacarthur/itermore?label=build)](https://github.com/rossmacarthur/itermore/actions/workflows/build.yaml)

🤸‍♀️ More iterator adaptors.

This crate provides some useful iterator adaptors and functions. Unlike
[`itertools`](https://docs.rs/itertools) this crate provides a separate
extension trait for each adaptor. Additionally, each type of adaptor is
feature flagged so you only have to compile the features you need.

## Getting started

Add the crate to Cargo manifest.

```sh
cargo add itermore --features full
```

And bring the extension traits into scope.

```rust
use itermore::prelude::*;
```

Now you can use extension methods like [`array_windows`] on any iterator.

```rust
for [a, b, c] in iter.array_windows() {
    println!("{} {} {}", a, b, c)
}
// Outputs
//    1 2 3
//    2 3 4
//    3 4 5
```

It is recommended to only enable the features that you need. For example if
you only want the [`array_combinations`] adaptor you would add the following
to your Cargo manifest.

```toml
[dependencies]
itermore = { version = "*", features = ["array_combinations"]}
```

## Provided functionality

### Methods

- [`collect_array`]: Collects an iterator into an array.
- [`min_max`] and friends: Returns the minimum and maximum element of an
  iterator.
- [`next_chunk`]: Returns the next `N` elements of the iterator as an array.
- [`sorted`] and friends: Returns a new iterator with all elements sorted.

### Adaptors

- [`array_chunks`] returns an iterator over `N` elements of the iterator at
  a time.
- [`array_windows`] returns an iterator over all contiguous windows of
  length `N`.
- [`array_combinations`] returns an iterator over `K` length combinations of
  all the elements in the underlying iterator.
- [`array_combinations_with_reps`] returns an iterator over `K` length
  combinations with repetitions/replacements of all the elements in the
  underlying iterator.
- [`cartesian_product`] returns an iterator over the cartesian product of
  the element sets of two iterators.
- [`circular_array_windows`] returns an iterator over all contiguous windows
  of length `N` that wraps around at the end.
- [`combinations`] returns an iterator over `k` length combinations of all
  the elements in the underlying iterator.
- [`combinations_with_reps`] returns an iterator over `k` length
  combinations with repetitions/replacements of all the elements in the
  underlying iterator.

[`array_windows`]: IterArrayWindows::array_windows
[`array_combinations`]: IterArrayCombinations::array_combinations
[`collect_array`]: IterCollectArray::collect_array
[`min_max`]: IterMinMax::min_max
[`next_chunk`]: IterNextChunk::next_chunk
[`sorted`]: IterSorted::sorted
[`array_chunks`]: IterArrayChunks::array_chunks
[`array_combinations_with_reps`]: IterArrayCombinationsWithReps::array_combinations_with_reps
[`cartesian_product`]: IterCartesianProduct::cartesian_product
[`circular_array_windows`]: IterCircularArrayWindows::circular_array_windows
[`combinations`]: IterCombinations::combinations
[`combinations_with_reps`]: IterCombinationsWithReps::combinations_with_reps

## License

This project is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

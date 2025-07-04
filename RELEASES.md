# 📝 Release notes

## 0.8.0

*July 4th, 2025*

- [Force exhaustion in `ArrayChunks::into_remainder`][6ab487a9]. This follows
  the [upstream change](https://github.com/rust-lang/rust/pull/123406) in the
  standard library.

- [Fix typo in `sorted_unstable_by` method name][1e157a7c]

[6ab487a9]: https://github.com/rossmacarthur/itermore/commit/6ab487a95e979e0364c664f684ed24281fb13a59
[1e157a7c]: https://github.com/rossmacarthur/itermore/commit/1e157a7c01290e66c9a108faacd7c27e5ac2762a

## 0.7.1

*December 15th, 2023*

- [Add extension trait to provide the `collect_array` method][42134c73]. The
  allows you to collect an iterator into an array panicking if the iterator
  contains too little or too many elements to fit in the array.

- [Correct MSRV to Rust 1.60][2aab2598]. Previously it was set to 1.56 in
  Cargo.toml and 1.65 was tested in CI.

[42134c73]: https://github.com/rossmacarthur/itermore/commit/42134c733014f3e5bae6aaf5e8c095b5cd73487a
[2aab2598]: https://github.com/rossmacarthur/itermore/commit/2aab2598c85a2ceea4ce3b44262a989a351529f6

## 0.7.0

*December 15th, 2023*

- [Fix catastrophic bug in min max implementation][7623f760]

- [Add remainder to next chunk and array chunks adaptor][e0e5df9a]. This adds a
  `into_remainder` method to the `ArrayChunks` adaptor which will return an
  iterator over the remaining elements of the original iterator that do not fit
  into the chunk size.

- Implement the following missing traits for various adaptors:
  - `ArrayChunks`: [`ExactSizeIterator`][f02a80e5], [`FusedIterator`][efe54e95]
  - `ArrayWindows`: [`ExactSizeIterator`][8189b80c], [`FusedIterator`][efe54e95]
  - `CartesianProduct`: [`FusedIterator`][efe54e95]
  - `CircularArrayWindows`: [`Debug`, `Clone`][0e9855a0]
  - `ArrayCombinations`: [`Debug`][0e9855a0]
  - `ArrayCombinationsWithReps`: [`Debug`, `Clone`][0e9855a0]
  - `Combinations`: [`Debug`, `Clone`][0e9855a0]
  - `CombinationsWithReps`: [`Debug`, `Clone`][0e9855a0]

- [Move combinations with reps to own adaptor][bc3fb83f]

- [Move `next_chunk` to its own trait][1493a169]

- [Do not enable any features by default, add "full" feature][dab76ed5]

- [Move array combinations with reps to own adaptor][5fa95b4f]

- [Add documentation feature labels][72a0c8c1]. This means that the
  documentation will now show which traits and structs are gated behind which
  feature.

[7623f760]: https://github.com/rossmacarthur/itermore/commit/7623f760147843b295c0f1c7c88edf6a528923a3
[e0e5df9a]: https://github.com/rossmacarthur/itermore/commit/e0e5df9a7d958cc293507fc264154dd0f019814c
[8189b80c]: https://github.com/rossmacarthur/itermore/commit/8189b80c5c6a21a3d5a22aaf6f7c91d676950ea4
[efe54e95]: https://github.com/rossmacarthur/itermore/commit/efe54e951217fbc270a5a2ca5a1570e6156bc4d9
[0e9855a0]: https://github.com/rossmacarthur/itermore/commit/0e9855a08d3c628d4dc46bef2a0cbb645015e8c2
[f02a80e5]: https://github.com/rossmacarthur/itermore/commit/f02a80e54c4f50b8bff7efd1d424c01a86120eb3
[bc3fb83f]: https://github.com/rossmacarthur/itermore/commit/bc3fb83feaaea191588d9c1145d49b2b8522d851
[1493a169]: https://github.com/rossmacarthur/itermore/commit/1493a169be4454ad32f1e66050c1258df18eb4f7
[dab76ed5]: https://github.com/rossmacarthur/itermore/commit/dab76ed509659a0d151adde2bc8fefaedf7db7ec
[5fa95b4f]: https://github.com/rossmacarthur/itermore/commit/5fa95b4fe3bee1efc37b3c692010cd8baaa14761
[72a0c8c1]: https://github.com/rossmacarthur/itermore/commit/72a0c8c13a95ea743c269f4c9f5929963c5de6e0

## 0.6.0

*November 28th, 2023*

- [Add circular array windows adaptor][fed53912]

- [Move array windows implementation into `itermore`][7831710d]

- [Move array chunks implementation into `itermore`][e74a8c3f]

- [Add combinations adaptor][3ebec68b]

- [Implement the following missing traits for various adaptors][36c9080c]:
  - `ArrayChunks`: `Debug`, `Clone`
  - `CartesianProduct`: `Debug`, `Clone`

- [Add `cartesian_product!` macro][eb2e8295]

- [Add `cartesian_product` method][9739c78d]

- [Add `IterMinMax` extension trait][0fc1fc48]

[fed53912]: https://github.com/rossmacarthur/itermore/commit/fed53912d6960ded022ed41ae4b6cb0dd93c4d3a
[7831710d]: https://github.com/rossmacarthur/itermore/commit/7831710d6f3366411d8bfceb2ae50cc7c0c87491
[e74a8c3f]: https://github.com/rossmacarthur/itermore/commit/e74a8c3f8149de1499a11e43239161775765f8c2
[3ebec68b]: https://github.com/rossmacarthur/itermore/commit/3ebec68b0906b62b84e8e23a0e59ef702b21d605
[36c9080c]: https://github.com/rossmacarthur/itermore/commit/36c9080c455c1ee66dbdbac7562a0c94252bb5b5
[eb2e8295]: https://github.com/rossmacarthur/itermore/commit/eb2e8295b55a882694e1cefcb556ba6ecca5a897
[9739c78d]: https://github.com/rossmacarthur/itermore/commit/9739c78dadb57d733196641f3bae61d84397f686
[886ff849]: https://github.com/rossmacarthur/itermore/commit/886ff849a2516b460338314125d893daadc0a3ee
[35b700b6]: https://github.com/rossmacarthur/itermore/commit/35b700b6688c40502de9c8004c1e66afe9c93f35
[0fc1fc48]: https://github.com/rossmacarthur/itermore/commit/0fc1fc48b7f32e7f260a5b90a90175de6445b13a

## 0.5.0

*December 4th, 2022*

- [Add array combinations with replacements adaptor][4808ff9c]

- [Rename the following adaptors and features][c41a0826]:
  - `IterChunks` -> `IterArrayChunks`
  - `IterWindows` -> `IterArrayWindows`
  - `IterCombinations` -> `IterArrayCombinations`

- [Add `array_combinations` method][4cb224f3]

[4808ff9c]: https://github.com/rossmacarthur/itermore/commit/4808ff9c61b97170743737cc993e9418d753774f
[c41a0826]: https://github.com/rossmacarthur/itermore/commit/c41a0826d0a14372ce75dea3df7c0b54e1364bc6
[4cb224f3]: https://github.com/rossmacarthur/itermore/commit/4cb224f37fdd57fed9bbaad80b1e77f062d25ed3

## 0.4.0

*December 1st, 2022*

- [Rename iterator chunks and windows methods][246ed5e5]:
  - `chunks` -> `array_chunks`
  - `windows` -> `array_windows`

[246ed5e5]: https://github.com/rossmacarthur/itermore/commit/246ed5e525005468a254f79b6995c2d1faa6ac20

## 0.3.1

*November 29th, 2022*

- [chunks: Add `next_array` and `chunked` aliases][568d6e3e]. These are
  identical to `next_chunk` and `chunks` but don't collide with the standard
  library implementation.

- [Add `IterSorted` extension trait][8e2c63d8]

[568d6e3e]: https://github.com/rossmacarthur/itermore/commit/568d6e3eb2b5dfba8b1db71dadf7cdc337f8b274
[8e2c63d8]: https://github.com/rossmacarthur/itermore/commit/8e2c63d87250fd704eb419c44ee4c4868dd81ed1

## 0.3.0

*November 28th, 2022*

- [Re-export traits from crates, add features][9d012e9f]

- [Move `iterchunks` and `iterwindows` crates here][fd97f755]

- [Refactor `.array_windows()` implementation][998eec2d].
  - Panic if `N = 0`
  - Implement `ExactSizeIterator` for `ArrayWindows`

- [Refactor `.array_chunks()` implementation][c2a5852b].
  - Fuse inner iterator
  - Fix `DoubleEndedIterator` implementation
  - Panic if `N = 0`

[9d012e9f]: https://github.com/rossmacarthur/itermore/commit/9d012e9ffe2680c28056228897220123597a54a7
[fd97f755]: https://github.com/rossmacarthur/itermore/commit/fd97f755065777fcebf36655a0307fa4ce9413aa
[998eec2d]: https://github.com/rossmacarthur/itermore/commit/998eec2d36ca15771c67b01cfe2f817d1d736381
[c2a5852b]: https://github.com/rossmacarthur/itermore/commit/c2a5852b02fe11d0daa335f51a5fedd3dc45d9fd

## 0.2.0

*December 19th, 2021*

- [Implement `DoubleEndedIterator` and `FusedIterator` for adaptors][f8447d52]

- [Add `.next_array()` method][ee9bd1a7]

- [Rename to not conflict with `itertools` crate][7481fc75]:
  - `windows` -> `array_windows`
  - `chunks` -> `array_chunks`

[f8447d52]: https://github.com/rossmacarthur/itermore/commit/f8447d52b5aba8c06ee3a2847e17867a1d86a846
[ee9bd1a7]: https://github.com/rossmacarthur/itermore/commit/ee9bd1a745b147e2148428829c953f0f26a59e74
[7481fc75]: https://github.com/rossmacarthur/itermore/commit/7481fc75b0a3b96420f5d428de39c65c172cf3eb

## 0.1.0

*October 6th, 2021*

Initial release

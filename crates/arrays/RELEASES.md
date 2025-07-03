# 📝 Release notes

## 0.2.1

*Unreleased*

- [Correct MSRV to 1.60][92874a44]

  The code relied on standard library features that were only stabilized
  in Rust 1.60.

[92874a44]: https://github.com/rossmacarthur/itermore/commit/92874a445919b884501c601540c63132d1e9dc78

## 0.2.0

*December 15th, 2023*

- [Rename `next_chunk*` to `from_iter*`][d88bc8f4]
- [Rename `collect*` to `next_chunk*`][5a90772d]
- [Return remainder from `collect*` methods][330ec6a8]
- [Add `IntoIter` iterator][a8fda9e3]

[d88bc8f4]: https://github.com/rossmacarthur/itermore/commit/d88bc8f4913d1bb2b95f14200f443706cfbbe7ed
[5a90772d]: https://github.com/rossmacarthur/itermore/commit/5a90772d07f8e55e406fa33b388fc77fb8e445e5
[330ec6a8]: https://github.com/rossmacarthur/itermore/commit/330ec6a8dcf82a1d3daa2ab50634b865ffec4414
[a8fda9e3]: https://github.com/rossmacarthur/itermore/commit/a8fda9e3f9d80e04ccd699edbe069092cc002493

## 0.1.0

*November 29th, 2023*

Initial release

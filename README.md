# itermore

[![Crates.io Version](https://img.shields.io/crates/v/itermore.svg)](https://crates.io/crates/itermore)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/itermore)

More iterator adaptors.

## 🚀 Getting started

Add the following to your Cargo manifest.

```toml
[dependencies]
itermore = "0.1"
```

And bring the [`IterMore`] trait into scope

```rust
use itermore::IterMore;
```

## 🤸 Usage

The following adaptors are provided.

### [`chunks`](https://docs.rs/itermore/0.1/itermore/trait.IterMore.html#method.chunks)

*Similar to `slice::chunks_exact` but for any iterator.*

Returns an iterator over `N` elements of the iterator at a time.

```rust
let data = [1, 1, 2, -2, 6, 0, 3, 1];
//          ^-----^  ^------^
for [x, y, z] in data.iter().chunks() {
    let sum = x + y + z;
    assert_eq!(sum, 4);
}
```

### [`windows`](https://docs.rs/itermore/0.1/itermore/trait.IterMore.html#method.chunks)

*Similar to `slice::windows` but for any iterator.*

Returns an iterator over all contiguous windows of length `N`. The windows
overlap.

```rust
let data = [10, 8, 6, 4];
//          ^---^
//              ^--^
//                 ^--^
for [x, y] in data.iter().windows() {
    assert_eq!(x - y, 2);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

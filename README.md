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

The following additional iterator methods are provided.

### [`next_array`](https://docs.rs/itermore/0.2/itermore/trait.IterMore.html#method.next_array)

```rust
let mut data = 1..5;
let [x, y] = data.next_array().unwrap();
assert_eq!(x, 1);
assert_eq!(y, 2);
```

The following adaptors are provided.

### [`array_chunks`](https://docs.rs/itermore/0.2/itermore/trait.IterMore.html#method.array_chunks)

*Similar to `slice::array_chunks` but for any iterator.*

Returns an iterator over `N` elements of the iterator at a time.

```rust
let data = [1, 1, 2, -2, 6, 0, 3, 1];
//          ^-----^  ^------^
for [x, y, z] in data.iter().array_chunks() {
    let sum = x + y + z;
    assert_eq!(sum, 4);
}
```

### [`array_windows`](https://docs.rs/itermore/0.2/itermore/trait.IterMore.html#method.array_windows)

*Similar to `slice::array_windows` but for any iterator.*

Returns an iterator over all contiguous windows of length `N`. The windows
overlap.

```rust
let data = [10, 8, 6, 4];
//          ^---^
//              ^--^
//                 ^--^
for [x, y] in data.iter().array_windows() {
    assert_eq!(x - y, 2);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

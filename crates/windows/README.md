<!-- Generated by cargo-onedoc. DO NOT EDIT. -->

# iterwindows

[![Crates.io Version](https://img.shields.io/crates/v/iterwindows.svg)](https://crates.io/crates/iterwindows)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/iterwindows)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/itermore/build/trunk)](https://github.com/rossmacarthur/itermore/actions?query=workflow%3Abuild)

This crate provides an iterator adapter to iterate over all contiguous
windows of length `N`.

## Getting started

Add the crate to your Cargo manifest.

```sh
cargo add iterwindows
```

And bring the `IterWindows` trait into scope.

```rust
use iterwindows::IterWindows;
```

Now you can use the `windows` method on any
iterator.

```rust
for [a, b, c] in iter.windows() {
    println!("{} {} {}", a, b, c)
}
```

Generally the size of `N` can be inferred by the compiler but you can also
specify it manually.

```rust
let w = iter.windows::<3>();
```

## License

This project is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
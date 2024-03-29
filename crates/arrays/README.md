<!-- Generated by cargo-onedoc. DO NOT EDIT. -->

# arrays

[![Crates.io Version](https://badgers.space/crates/version/arrays)](https://crates.io/crates/arrays)
[![Docs.rs Latest](https://badgers.space/badge/docs.rs/latest/blue)](https://docs.rs/arrays)
[![Build Status](https://badgers.space/github/checks/rossmacarthur/itermore?label=build)](https://github.com/rossmacarthur/itermore/actions/workflows/build.yaml)

Construct an array from an iterator and other helpers.

## Getting started

Add the `arrays` crate to your Cargo manifest.

```sh
cargo add arrays
```

Now get the next N items from an iterator.

```rust
let arr: [_; 3] = arrays::from_iter(iter).unwrap();
```

## License

This project is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

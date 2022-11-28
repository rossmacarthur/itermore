//! 🤸‍♀️ More iterator adaptors.
//!
//! This crate provides some useful iterator adaptors like
//! [`chunks`][IterChunks::chunks] and [`windows`][IterWindows::windows]. Unlike
//! [`itertools`](https://docs.rs/itertools) this crate provides a separate
//! extension trait for each adaptor. Additionally, each type of adaptor is
//! feature flagged so you only have to compile the features you need.
//!
//! # Getting started
//!
//! Add the crate to Cargo manifest.
//! ```sh
//! cargo add itermore
//! ```
//!
//! And bring the extension traits into scope.
//!
//! ```
//! use itermore::prelude::*;
//! ```
//!
//! Now you can use extension methods like [`windows`][IterWindows::windows] on
//! any iterator.
//! ```
//! # use itermore::prelude::*;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.windows() {
//!     println!("{} {} {}", a, b, c)
//! }
//! // Outputs
//! //    1 2 3
//! //    2 3 4
//! //    3 4 5
//! ```

#![warn(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "chunks")]
pub use iterchunks::{Chunks, IterChunks};

#[cfg(feature = "windows")]
pub use iterwindows::{IterWindows, Windows};

/// Re-exports all iterator extension traits.
pub mod prelude {
    #[cfg(feature = "chunks")]
    pub use iterchunks::IterChunks;

    #[cfg(feature = "windows")]
    pub use iterwindows::IterWindows;
}

//! 🤸‍♀️ More iterator adaptors.
//!
//! This crate provides some useful iterator adaptors like [`array_chunks`] and
//! [`array_windows`]. Unlike [`itertools`](https://docs.rs/itertools) this
//! crate provides a separate extension trait for each adaptor. Additionally,
//! each type of adaptor is feature flagged so you only have to compile the
//! features you need.
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
//! Now you can use extension methods like [`array_windows`] on any iterator.
//! ```
//! # use itermore::prelude::*;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.array_windows() {
//!     println!("{} {} {}", a, b, c)
//! }
//! // Outputs
//! //    1 2 3
//! //    2 3 4
//! //    3 4 5
//! ```
//!
//! [`array_chunks`]: IterChunks::array_chunks
//! [`array_windows`]: IterWindows::array_windows

#![warn(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(feature = "alloc"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod adaptors;
mod xtraits;

#[cfg(feature = "array_chunks")]
pub use iterchunks::{ArrayChunks, IterArrayChunks};

#[cfg(feature = "array_combinations")]
pub use crate::adaptors::array_combinations::{ArrayCombinations, IterArrayCombinations};

#[cfg(feature = "array_windows")]
pub use iterwindows::{ArrayWindows, IterArrayWindows};

#[cfg(feature = "sorted")]
pub use crate::xtraits::sorted::IterSorted;

/// Re-exports all iterator extension traits.
pub mod prelude {
    #[cfg(feature = "array_chunks")]
    pub use super::IterArrayChunks;

    #[cfg(feature = "array_combinations")]
    pub use super::IterArrayCombinations;

    #[cfg(feature = "array_windows")]
    pub use super::IterArrayWindows;

    #[cfg(feature = "sorted")]
    pub use super::IterSorted;
}

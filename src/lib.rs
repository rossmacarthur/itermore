//! 🤸‍♀️ More iterator adaptors.
//!
//! This crate provides some useful iterator adaptors and functions. Unlike
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
//! It is recommended to only enable the features that you need, you can do this
//! by disabling all features and turning on the ones you want. For example if
//! you only want the [`array_combinations`] adaptor you would add the following
//! to your Cargo manifest.
//! ```toml
//! [dependencies]
//! itermore = { version = "*",  default-features = false, features = ["array_combinations"]}
//! ```
//!
//! # Provided functionality
//!
//! ## Methods
//!
//! - [`min_max`] and friends: Returns the minimum and maximum element of an
//!   iterator.
//! - [`next_chunk`]: Returns the next `N` elements of the iterator as an array.
//! - [`sorted`] and friends: Returns a new iterator with all elements sorted.
//!
//! ## Adaptors
//!
//! - [`array_chunks`] returns an iterator over `N` elements of the iterator at
//!   a time.
//! - [`array_windows`] returns an iterator over all contiguous windows of
//!   length `N`.
//! - [`array_combinations`] returns an iterator over `K` length combinations of
//!   all the elements in the underlying iterator.
//! - [`array_combinations_with_reps`] returns an iterator over `K` length
//!   combinations with repetitions/replacements of all the elements in the
//!   underlying iterator.
//! - [`cartesian_product`] returns an iterator over the cartesian product of
//!   the element sets of two iterators.
//! - [`circular_array_windows`] returns an iterator over all contiguous windows
//!   of length `N` that wraps around at the end.
//! - [`combinations`] returns an iterator over `k` length combinations of all
//!   the elements in the underlying iterator.
//! - [`combinations_with_reps`] returns an iterator over `k` length
//!   combinations with repetitions/replacements of all the elements in the
//!   underlying iterator.
//!
//! [`next_chunk`]: IterArrayChunks::next_chunk
//! [`array_chunks`]: IterArrayChunks::array_chunks
//! [`array_combinations`]: IterArrayCombinations::array_combinations
//! [`array_combinations_with_reps`]: IterArrayCombinationsWithReps::array_combinations_with_reps
//! [`array_windows`]: IterArrayWindows::array_windows
//! [`cartesian_product`]: IterCartesianProduct::cartesian_product
//! [`circular_array_windows`]: IterCircularArrayWindows::circular_array_windows
//! [`combinations`]: IterCombinations::combinations
//! [`combinations_with_reps`]: IterCombinations::combinations_with_reps
//! [`min_max`]: IterMinMax::min_max
//! [`sorted`]: IterSorted::sorted

#![warn(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(feature = "alloc"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod adaptors;
#[cfg(feature = "cartesian_product")]
mod flatten_tuple;
mod xtraits;

// For macros
#[doc(hidden)]
#[cfg(feature = "cartesian_product")]
pub use crate::flatten_tuple::flatten_tuple;
#[doc(hidden)]
pub use core;

#[cfg(feature = "array_chunks")]
pub use crate::adaptors::array_chunks::{ArrayChunks, IterArrayChunks};

#[cfg(feature = "array_combinations")]
pub use crate::adaptors::array_combinations::{ArrayCombinations, IterArrayCombinations};

#[cfg(feature = "array_combinations_with_reps")]
pub use crate::adaptors::array_combinations_with_reps::{
    ArrayCombinationsWithReps, IterArrayCombinationsWithReps,
};

#[cfg(feature = "array_windows")]
pub use crate::adaptors::array_windows::{ArrayWindows, IterArrayWindows};

#[cfg(feature = "cartesian_product")]
pub use crate::adaptors::cartesian_product::{CartesianProduct, IterCartesianProduct};

#[cfg(feature = "circular_array_windows")]
pub use crate::adaptors::circular_array_windows::{CircularArrayWindows, IterCircularArrayWindows};

#[cfg(feature = "combinations")]
pub use crate::adaptors::combinations::{Combinations, CombinationsWithReps, IterCombinations};

#[cfg(feature = "min_max")]
pub use crate::xtraits::min_max::IterMinMax;

#[cfg(feature = "sorted")]
pub use crate::xtraits::sorted::IterSorted;

/// Re-exports all iterator extension traits.
///
/// The intention is that this module is used as a `*` import.
/// ```
/// use itermore::prelude::*;
/// ```
/// If you want to refer to a trait directly rather import it from the crate
/// root.
pub mod prelude {
    #[cfg(feature = "array_chunks")]
    pub use super::IterArrayChunks;

    #[cfg(feature = "array_combinations")]
    pub use super::IterArrayCombinations;

    #[cfg(feature = "array_combinations_with_reps")]
    pub use super::IterArrayCombinationsWithReps;

    #[cfg(feature = "array_windows")]
    pub use super::IterArrayWindows;

    #[cfg(feature = "cartesian_product")]
    pub use super::IterCartesianProduct;

    #[cfg(feature = "circular_array_windows")]
    pub use super::IterCircularArrayWindows;

    #[cfg(feature = "combinations")]
    pub use super::IterCombinations;

    #[cfg(feature = "min_max")]
    pub use super::IterMinMax;

    #[cfg(feature = "sorted")]
    pub use super::IterSorted;
}

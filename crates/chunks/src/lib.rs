//! This crate provides an iterator adapter that yields N elements of the
//! iterator at a time.
//!
//! ## Deprecated
//!
//! This crate is deprecated in favour of the [`itermore`] crate and it
//! currently just re-exports types from there. The following dependency
//! definition is the equivalent of using this crate.
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! itermore = { version = "...", default-features = false, features = ["array_chunks"] }
//! ```
//!
//! # Getting started
//!
//! Add the crate to your Cargo manifest.
//! ```sh
//! cargo add iterchunks
//! ```
//!
//! And bring the [`IterArrayChunks`] trait into scope.
//!
//! ```
//! use iterchunks::IterArrayChunks;
//! ```
//!
//! Now you can use the [`array_chunks`] method on any iterator.
//!
//! ```
//! # use iterchunks::IterArrayChunks;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.array_chunks() {
//!     println!("{} {} {}", a, b, c)
//! }
//! ```
//!
//! Generally the size of `N` can be inferred by the compiler but you can also
//! specify it manually.
//! ```
//! # use iterchunks::IterArrayChunks;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! let c = iter.array_chunks::<3>();
//! ```
//!
//! [`array_chunks`]: IterArrayChunks::array_chunks

#![no_std]
#![deprecated]

pub use itermore::{ArrayChunks, IterArrayChunks};

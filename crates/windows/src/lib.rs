//! This crate provides an iterator adaptor to iterate over all contiguous
//! windows of length `N`.
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
//! itermore = { version = "...", default-features = false, features = ["array_windows"] }
//! ```
//!
//! [`itermore`]: https://crates.io/crates/itermore
//!
//! # Getting started
//!
//! Add the crate to your Cargo manifest.
//! ```sh
//! cargo add iterwindows
//! ```
//!
//! And bring the [`IterArrayWindows`] trait into scope.
//!
//! ```
//! use iterwindows::IterArrayWindows;
//! ```
//!
//! Now you can use the [`array_windows`] method on any iterator.
//!
//! ```
//! # use iterwindows::IterArrayWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.array_windows() {
//!     println!("{} {} {}", a, b, c)
//! }
//! ```
//!
//! Generally the size of `N` can be inferred by the compiler but you can also
//! specify it manually.
//! ```
//! # use iterwindows::IterArrayWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! let w = iter.array_windows::<3>();
//! ```
//!
//! [`array_windows`]: IterArrayWindows::array_windows

#![no_std]
#![deprecated]

pub use itermore::{ArrayWindows, IterArrayWindows};

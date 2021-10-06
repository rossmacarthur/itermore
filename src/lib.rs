//! More iterator adaptors,
//!
//! To extend [`Iterator`] with methods in this crate, import the [`IterMore`]
//! trait
//!
//! ```
//! use itermore::IterMore;
//! ```
//!
//! Now new methods like [`chunks`][IterMore::chunks] are available.
//!
//! ```
//! # use itermore::IterMore;
//! for [x, y, z] in (1..100).chunks() {
//!     // ...
//! }
//! ```

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
extern crate alloc;

mod array;
mod chunks;

pub use crate::chunks::Chunks;

/// Provides extra adaptors to anything implementing [`Iterator`].
pub trait IterMore: Iterator {
    /// Returns an iterator over `N` elements of the iterator at a time,
    ///
    /// The chunks are arrays and do not overlap. If `N` does not divide the
    /// length of the iterator, then the last up to `N-1` elements will be
    /// omitted.
    ///
    /// **Note:** if you have something that dereferences to a slice you should
    /// consider [`slice::chunks_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use itermore::IterMore;
    /// let data = [1, 1, 2, -2, 6, 0, 3, 1];
    /// //          ^-----^  ^------^
    /// for [x, y, z] in data.iter().chunks() {
    ///     let sum = x + y + z;
    ///     assert_eq!(sum, 4);
    /// }
    /// ```
    ///
    /// ```
    /// # use itermore::IterMore;
    /// let mut iter = ['l', 'o', 'r', 'e', 'm'].iter().copied().chunks();
    /// assert_eq!(iter.next().unwrap(), ['l', 'o']);
    /// assert_eq!(iter.next().unwrap(), ['r', 'e']);
    /// assert!(iter.next().is_none());
    /// ```
    #[inline]
    fn chunks<const N: usize>(self) -> Chunks<Self, Self::Item, N>
    where
        Self: Sized,
    {
        Chunks::new(self)
    }
}

impl<I: ?Sized> IterMore for I where I: Iterator {}

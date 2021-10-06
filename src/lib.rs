//! More iterator adaptors,
//!
//! To extend [`Iterator`] with methods in this crate, import the [`IterMore`]
//! trait
//!
//! ```
//! use itermore::IterMore;
//! ```
//!
//! Now the new methods [`chunks`][IterMore::chunks] and
//! [`windows`][IterMore::windows] are available.
//!
//! ```
//! # use itermore::IterMore;
//! for [x, y, z] in (1..100).chunks() {
//!     // ...
//! }
//!
//! for [a, b] in (1..33).windows() {
//!     // ...
//! }
//! ```

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
extern crate alloc;

mod array;
mod chunks;
mod windows;

pub use crate::chunks::Chunks;
pub use crate::windows::Windows;

/// Provides extra adaptors to anything implementing [`Iterator`].
pub trait IterMore: Iterator {
    /// Returns an iterator over `N` elements of the iterator at a time,
    ///
    /// The chunks are arrays and do not overlap. If `N` does not divide the
    /// length of the iterator, then the last up to `N-1` elements will be
    /// omitted.
    ///
    /// **Note:** if you have something that dereferences to a slice you should
    /// consider [`slice::chunks_exact`] or [`slice::array_chunks`].
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

    /// Returns an iterator over all contiguous windows of length `N`. The
    /// windows overlap. If the iterator is shorter than `N`, the iterator
    /// returns no values.
    ///
    /// `windows` clones the iterator elements so that they can be part of
    /// successive windows, this makes this it most suited for iterators of
    /// references and other values that are cheap to copy.
    ///
    /// **Note:** if you have something that dereferences to a slice you should
    /// consider [`slice::windows`] or [`slice::array_windows`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use itermore::IterMore;
    /// let data = [10, 8, 6, 4];
    /// //          ^---^
    /// //              ^--^
    /// //                 ^--^
    /// for [x, y] in data.iter().windows() {
    ///     assert_eq!(x - y, 2);
    /// }
    /// ```
    ///
    /// ```
    /// # use itermore::IterMore;
    /// let mut iter = ['r', 'u', 's', 't'].iter().copied().windows();
    /// assert_eq!(iter.next().unwrap(), ['r', 'u']);
    /// assert_eq!(iter.next().unwrap(), ['u', 's']);
    /// assert_eq!(iter.next().unwrap(), ['s', 't']);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// If the iterator is shorter than `N`
    /// ```
    /// # use itermore::IterMore;
    /// let mut iter = ['f', 'o', 'o'].iter().copied().windows::<4>();
    /// assert!(iter.next().is_none());
    /// ```
    #[inline]
    fn windows<const N: usize>(self) -> Windows<Self, Self::Item, N>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Windows::new(self)
    }
}

impl<I: ?Sized> IterMore for I where I: Iterator {}

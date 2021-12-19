//! More iterator adaptors,
//!
//! To extend [`Iterator`] with methods in this crate, import the [`Itermore`]
//! trait
//!
//! ```
//! use itermore::Itermore;
//! ```
//!
//! Now the new methods [`array_chunks`][Itermore::array_chunks] and
//! [`array_windows`][Itermore::array_windows] are available.
//!
//! ```
//! # use itermore::Itermore;
//! for [x, y, z] in (1..100).array_chunks() {
//!     // ...
//! }
//!
//! for [a, b] in (1..33).array_windows() {
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

pub use crate::chunks::ArrayChunks;
pub use crate::windows::ArrayWindows;

/// Provides extra adaptors to anything implementing [`Iterator`].
pub trait Itermore: Iterator {
    /// Advances the iterator `N` times and returns the elements as an array.
    ///
    /// If there are not enough elements the fill the array then `None` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use itermore::Itermore;
    /// let mut data = 1..5;
    /// let [x, y] = data.next_array().unwrap();
    /// assert_eq!(x, 1);
    /// assert_eq!(y, 2);
    /// ```
    #[inline]
    fn next_array<const N: usize>(&mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        array::collect(self)
    }

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
    /// # use itermore::Itermore;
    /// let data = [1, 1, 2, -2, 6, 0, 3, 1];
    /// //          ^-----^  ^------^
    /// for [x, y, z] in data.iter().array_chunks() {
    ///     let sum = x + y + z;
    ///     assert_eq!(sum, 4);
    /// }
    /// ```
    ///
    /// ```
    /// # use itermore::Itermore;
    /// let mut iter = ['l', 'o', 'r', 'e', 'm'].iter().copied().array_chunks();
    /// assert_eq!(iter.next().unwrap(), ['l', 'o']);
    /// assert_eq!(iter.next().unwrap(), ['r', 'e']);
    /// assert!(iter.next().is_none());
    /// ```
    #[inline]
    fn array_chunks<const N: usize>(self) -> ArrayChunks<Self, Self::Item, N>
    where
        Self: Sized,
    {
        ArrayChunks::new(self)
    }

    /// Returns an iterator over all contiguous windows of length `N`. The
    /// windows overlap. If the iterator is shorter than `N`, the iterator
    /// returns no values.
    ///
    /// `array_windows` clones the iterator elements so that they can be part of
    /// successive windows, this makes this it most suited for iterators of
    /// references and other values that are cheap to copy.
    ///
    /// **Note:** if you have something that dereferences to a slice you should
    /// consider [`slice::windows`] or [`slice::array_windows`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use itermore::Itermore;
    /// let data = [10, 8, 6, 4];
    /// //          ^---^
    /// //              ^--^
    /// //                 ^--^
    /// for [x, y] in data.iter().array_windows() {
    ///     assert_eq!(x - y, 2);
    /// }
    /// ```
    ///
    /// ```
    /// # use itermore::Itermore;
    /// let mut iter = ['r', 'u', 's', 't'].iter().copied().array_windows();
    /// assert_eq!(iter.next().unwrap(), ['r', 'u']);
    /// assert_eq!(iter.next().unwrap(), ['u', 's']);
    /// assert_eq!(iter.next().unwrap(), ['s', 't']);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// If the iterator is shorter than `N`
    /// ```
    /// # use itermore::Itermore;
    /// let mut iter = ['f', 'o', 'o'].iter().copied().array_windows::<4>();
    /// assert!(iter.next().is_none());
    /// ```
    #[inline]
    fn array_windows<const N: usize>(self) -> ArrayWindows<Self, Self::Item, N>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        ArrayWindows::new(self)
    }
}

impl<I: ?Sized> Itermore for I where I: Iterator {}

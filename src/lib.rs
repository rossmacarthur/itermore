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

    /// Returns an iterator over `N` elements of the iterator at a time.
    ///
    /// The chunks do not overlap. If `N` does not divide the length of the
    /// iterator, then the last up to `N-1` elements will be omitted.
    ///
    /// # Panics
    ///
    /// If called with `N = 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use itermore::Itermore;
    ///
    /// let mut iter = "lorem".chars().array_chunks();
    /// assert_eq!(iter.next(), Some(['l', 'o']));
    /// assert_eq!(iter.next(), Some(['r', 'e']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use itermore::Itermore;
    ///
    /// let data = [1, 1, 2, -2, 6, 0, 3, 1];
    /// //          ^-----^  ^------^
    /// for [x, y, z] in data.iter().array_chunks() {
    ///     assert_eq!(x + y + z, 4);
    /// }
    /// ```
    #[inline]
    fn array_chunks<const N: usize>(self) -> ArrayChunks<Self, N>
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
    /// references and other values that are cheap to clone.
    ///
    /// # Panics
    ///
    /// If called with `N = 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use itermore::Itermore;
    ///
    /// let mut iter = "rust".chars().array_windows();
    /// assert_eq!(iter.next(), Some(['r', 'u']));
    /// assert_eq!(iter.next(), Some(['u', 's']));
    /// assert_eq!(iter.next(), Some(['s', 't']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use itermore::Itermore;
    ///
    /// let seq: &[i32] = &[0, 1, 1, 2, 3, 5, 8, 13];
    /// for [x, y, z] in seq.iter().copied().array_windows() {
    ///     assert_eq!(x + y, z);
    /// }
    /// ```
    #[inline]
    fn array_windows<const N: usize>(self) -> ArrayWindows<Self, N>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        ArrayWindows::new(self)
    }
}

impl<I: ?Sized> Itermore for I where I: Iterator {}

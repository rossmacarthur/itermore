//! This crate provides an iterator adapter that yields N elements of the
//! iterator at a time.
//!
//! This methods provided here have the corresponding nightly APIs:
//! - [Iterator::next_chunk](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.next_chunk)
//! - [Iterator::array_chunks](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.array_chunks)
//!
//! The nightly APIs handle remainders better and will likely have better
//! performance, so they should be preferred if possible.
//!
//! # Getting started
//!
//! Add the crate to your Cargo manifest.
//! ```sh
//! cargo add iterchunks
//! ```
//!
//! And bring the [`IterChunks`] trait into scope.
//!
//! ```
//! use iterchunks::IterChunks;
//! ```
//!
//! Now you can use the [`array_chunks`] method on any iterator.
//!
//! ```
//! # use iterchunks::IterChunks;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.array_chunks() {
//!     println!("{} {} {}", a, b, c)
//! }
//! ```
//!
//! Generally the size of `N` can be inferred by the compiler but you can also
//! specify it manually.
//! ```
//! # use iterchunks::IterChunks;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! let c = iter.array_chunks::<3>();
//! ```
//!
//! [`array_chunks`]: IterChunks::array_chunks

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

/// An extension trait that provides the [`array_chunks`] method for iterators.
///
/// [`array_chunks`]: IterChunks::array_chunks
pub trait IterChunks: Iterator {
    /// Advances the iterator and returns an array containing the next `N`
    /// values.
    ///
    /// If there are not enough elements to fill the array then `None` is
    /// returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterchunks::IterChunks;
    ///
    /// let mut iter = "lorem".chars();
    ///
    /// assert_eq!(iter.next_chunk().unwrap(), ['l', 'o']);              // N is inferred as 2
    /// assert_eq!(iter.next_chunk().unwrap(), ['r', 'e', 'm']);         // N is inferred as 3
    /// assert!(iter.next_chunk::<4>().is_none()); // N is explicitly 4
    /// ```
    ///
    /// Split a string and get the first three items.
    ///
    /// ```
    /// use iterchunks::IterChunks;
    ///
    /// let quote = "not all those who wander are lost";
    /// let [first, second, third] = quote.split_whitespace().next_chunk().unwrap();
    /// assert_eq!(first, "not");
    /// assert_eq!(second, "all");
    /// assert_eq!(third, "those");
    /// ```
    #[inline]
    fn next_chunk<const N: usize>(&mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        arrays::collect(self.by_ref())
    }

    /// Identical to [`next_chunk`][IterChunks::next_chunk] but doesn't collide
    /// with the standard library name.
    #[inline]
    fn next_array<const N: usize>(&mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        arrays::collect(self.by_ref())
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
    /// use iterchunks::IterChunks;
    ///
    /// let mut iter = "lorem".chars().array_chunks();
    /// assert_eq!(iter.next(), Some(['l', 'o']));
    /// assert_eq!(iter.next(), Some(['r', 'e']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use iterchunks::IterChunks;
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

    /// Identical to [`array_chunks`][IterChunks::array_chunks] but doesn't
    /// collide with the standard library name.
    fn array_chunked<const N: usize>(self) -> ArrayChunks<Self, N>
    where
        Self: Sized,
    {
        ArrayChunks::new(self)
    }
}

impl<I: ?Sized> IterChunks for I where I: Iterator {}

/// An iterator over `N` elements of the iterator at a time.
///
/// This struct is created by the [`array_chunks`] method on iterators. See its
/// documentation for more.
///
/// [`array_chunks`]: IterChunks::array_chunks
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayChunks<I, const N: usize> {
    iter: I,
}

impl<I, const N: usize> ArrayChunks<I, N>
where
    I: Iterator,
{
    #[track_caller]
    fn new(iter: I) -> Self {
        assert!(N != 0, "chunk size must be non-zero");
        Self { iter }
    }
}

impl<I: Iterator, const N: usize> Iterator for ArrayChunks<I, N>
where
    I: Iterator,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter } = self;
        arrays::collect(iter.by_ref())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (lower / N, upper.map(|n| n / N))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count() / N
    }
}

impl<I, const N: usize> DoubleEndedIterator for ArrayChunks<I, N>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let rem = self.iter.len() % N;
        let mut rev = self.iter.by_ref().rev().skip(rem);
        let mut chunk = IterChunks::next_chunk(&mut rev)?;
        chunk.reverse();
        Some(chunk)
    }
}

/// An extension trait that provides the [`next_chunk`] method for iterators.
///
/// Note: the method provided here has a nightly API: [`Iterator::next_chunk`].
/// The nightly APIs handle remainders better and will likely have better
/// performance, so they should be preferred if possible.
///
/// [`next_chunk`]: IterNextChunk::next_chunk
#[cfg_attr(docsrs, doc(cfg(feature = "next_chunk")))]
pub trait IterNextChunk: Iterator {
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
    /// use itermore::IterNextChunk;
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
    /// use itermore::IterNextChunk;
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
        arrays::collect(self)
    }

    /// Identical to [`next_chunk`][IterNextChunk::next_chunk] but doesn't
    /// collide with the standard library name.
    #[inline]
    fn next_array<const N: usize>(&mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        arrays::collect(self)
    }
}

impl<I: ?Sized> IterNextChunk for I where I: Iterator {}

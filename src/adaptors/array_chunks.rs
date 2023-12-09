use core::iter::FusedIterator;

/// An extension trait that provides the [`array_chunks`] method for iterators.
///
/// Note: the method provided here has a nightly API:
/// [`Iterator::array_chunks`]. The nightly API handles remainders better and
/// will likely have better performance, so it should be preferred if possible.
///
/// [`array_chunks`]: IterArrayChunks::array_chunks
#[cfg_attr(docsrs, doc(cfg(feature = "array_chunks")))]
pub trait IterArrayChunks: Iterator {
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
    /// use itermore::IterArrayChunks;
    ///
    /// let mut iter = "lorem".chars().array_chunks();
    /// assert_eq!(iter.next(), Some(['l', 'o']));
    /// assert_eq!(iter.next(), Some(['r', 'e']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use itermore::IterArrayChunks;
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

    /// Identical to [`array_chunks`][IterArrayChunks::array_chunks] but doesn't
    /// collide with the standard library name.
    fn array_chunked<const N: usize>(self) -> ArrayChunks<Self, N>
    where
        Self: Sized,
    {
        ArrayChunks::new(self)
    }
}

impl<I: ?Sized> IterArrayChunks for I where I: Iterator {}

/// An iterator over `N` elements of the iterator at a time.
///
/// This struct is created by the [`array_chunks`] method on iterators. See its
/// documentation for more.
///
/// [`array_chunks`]: IterArrayChunks::array_chunks
#[cfg_attr(docsrs, doc(cfg(feature = "array_chunks")))]
#[derive(Debug, Clone)]
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
        arrays::next_chunk(iter).ok()
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
        let mut chunk = arrays::next_chunk(&mut rev).ok()?;
        chunk.reverse();
        Some(chunk)
    }
}

impl<I, const N: usize> ExactSizeIterator for ArrayChunks<I, N>
where
    I: ExactSizeIterator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len() / N
    }
}

impl<I, const N: usize> FusedIterator for ArrayChunks<I, N> where I: FusedIterator {}

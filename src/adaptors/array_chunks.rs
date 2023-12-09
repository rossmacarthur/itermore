use core::iter::FusedIterator;

use arrays::IntoIter;

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
    #[inline]
    fn arrays<const N: usize>(self) -> ArrayChunks<Self, N>
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
pub struct ArrayChunks<I, const N: usize>
where
    I: Iterator,
{
    iter: I,
    remainder: Option<IntoIter<I::Item, N>>,
}

impl<I, const N: usize> ArrayChunks<I, N>
where
    I: Iterator,
{
    #[track_caller]
    fn new(iter: I) -> Self {
        assert!(N != 0, "chunk size must be non-zero");
        Self {
            iter,
            remainder: None,
        }
    }

    /// Returns an iterator over the remaining elements of the original iterator
    /// that are not going to be yielded. The returned iterator will yield at
    /// most `N-1` elements. Returns `None` if the remainder is not yet known.
    #[inline]
    pub fn into_remainder(self) -> Option<IntoIter<I::Item, N>> {
        self.remainder
    }
}

impl<I: Iterator, const N: usize> Iterator for ArrayChunks<I, N>
where
    I: Iterator,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, remainder } = self;
        match arrays::next_chunk(iter) {
            Ok(chunk) => Some(chunk),
            Err(rem) => {
                remainder.get_or_insert(rem);
                None
            }
        }
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
        self.next_back_remainder();
        let mut rev = self.iter.by_ref().rev();
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

impl<I, const N: usize> ArrayChunks<I, N>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    /// Updates `self.remainder` such that `self.iter.len` is divisible by `N`.
    #[inline]
    fn next_back_remainder(&mut self) {
        if self.remainder.is_some() {
            return;
        }

        let rem = self.iter.len() % N;
        let mut rev = self.iter.by_ref().rev().take(rem);
        // SAFETY: `unwrap_err` always succeeds because x % N < N for all x.
        let mut remainder = unsafe { arrays::next_chunk(&mut rev).unwrap_err_unchecked() };

        // We used `.rev()` above, so we need to re-reverse the remainder.
        remainder.as_mut_slice().reverse();
        self.remainder = Some(remainder);
    }
}

impl<I, const N: usize> FusedIterator for ArrayChunks<I, N> where I: FusedIterator {}

use core::iter::{DoubleEndedIterator, Fuse, FusedIterator};

use crate::array;

/// An iterator over `N` elements of the iterator at a time.
///
/// The chunks do not overlap. If `N` does not divide the length of the
/// iterator, then the last up to `N-1` elements will be omitted.
///
/// This struct is created by the [`array_chunks`] method on iterators. See its
/// documentation for more.
///
/// [`array_chunks`]: crate::Itermore::array_chunks
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayChunks<I, const N: usize> {
    iter: Fuse<I>,
}

impl<I, const N: usize> ArrayChunks<I, N>
where
    I: Iterator,
{
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        assert!(N != 0, "chunk size must be non-zero");
        Self { iter: iter.fuse() }
    }
}

impl<I, const N: usize> Iterator for ArrayChunks<I, N>
where
    I: Iterator,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        array::collect(&mut self.iter)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        // Keep infinite iterator size hint lower bound as `usize::MAX`
        if lower == usize::MAX {
            (lower, upper)
        } else {
            (lower / N, upper.map(|n| n / N))
        }
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
        for _ in 0..(self.iter.len() % N) {
            self.iter.next_back()?;
        }
        array::collect_fn(|| self.iter.next_back()).map(|mut arr| {
            arr.reverse();
            arr
        })
    }
}

impl<I, const N: usize> FusedIterator for ArrayChunks<I, N> where I: FusedIterator {}

impl<I, const N: usize> ExactSizeIterator for ArrayChunks<I, N> where I: ExactSizeIterator {}

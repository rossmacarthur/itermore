use core::iter::{DoubleEndedIterator, FusedIterator};

use crate::array;

/// An iterator that yields `N` elements of `T` at a time.
///
/// This struct is created by the
/// [`array_chunks`][crate::Itermore::array_chunks] method on iterators.
#[derive(Debug, Clone)]
pub struct ArrayChunks<I, T, const N: usize>
where
    I: Iterator<Item = T>,
{
    iter: I,
}

impl<I, T, const N: usize> ArrayChunks<I, T, N>
where
    I: Iterator<Item = T>,
{
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I, T, const N: usize> Iterator for ArrayChunks<I, T, N>
where
    I: Iterator<Item = T>,
{
    type Item = [T; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        array::collect(&mut self.iter)
    }
}

impl<I, T, const N: usize> DoubleEndedIterator for ArrayChunks<I, T, N>
where
    I: DoubleEndedIterator<Item = T>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        array::collect_fn(|| self.iter.next_back())
    }
}

impl<I, T, const N: usize> FusedIterator for ArrayChunks<I, T, N>
//
where
    I: FusedIterator<Item = T>
{
}

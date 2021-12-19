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

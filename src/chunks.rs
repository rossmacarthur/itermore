use crate::array;

/// An iterator that yields `N` elements of `T` at a time.
///
/// This struct is created by the [`chunks`][crate::IterMore::chunks] method on
/// iterators.
#[derive(Debug, Clone)]
pub struct Chunks<I, T, const N: usize>
where
    I: Iterator<Item = T>,
{
    iter: I,
}

impl<I, T, const N: usize> Chunks<I, T, N>
where
    I: Iterator<Item = T>,
{
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I, T, const N: usize> Iterator for Chunks<I, T, N>
where
    I: Iterator<Item = T>,
{
    type Item = [T; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        array::collect(&mut self.iter)
    }
}

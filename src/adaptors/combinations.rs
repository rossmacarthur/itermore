use core::fmt;
use core::iter::FusedIterator;

use crate::adaptors::generic_combinations::GenericCombinations;

/// An extension trait that provides the [`combinations`] method for iterators.
///
/// [`combinations`]: IterCombinations::combinations
#[cfg_attr(docsrs, doc(cfg(feature = "combinations")))]
pub trait IterCombinations: Iterator {
    /// Returns an iterator adaptor that iterates over `k` length combinations
    /// of all the elements in the underlying iterator.
    ///
    /// The iterator is consumed as elements are required. In the first
    /// iteration `k` elements will be consumed by the iterator.
    ///
    /// # Panics
    ///
    /// If called with `k = 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use itermore::IterCombinations;
    ///
    /// let mut iter = "abcd".chars().combinations(3);
    /// assert_eq!(iter.next(), Some(vec!['a', 'b', 'c']));
    /// assert_eq!(iter.next(), Some(vec!['a', 'b', 'd']));
    /// assert_eq!(iter.next(), Some(vec!['a', 'c', 'd']));
    /// assert_eq!(iter.next(), Some(vec!['b', 'c', 'd']));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    fn combinations(self, k: usize) -> Combinations<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Combinations::new(self, k)
    }
}

impl<I: ?Sized> IterCombinations for I where I: Iterator {}

/// An iterator that iterates over `k` length combinations of all the elements
/// in the underlying iterator.
///
/// This struct is created by the [`combinations`] method on iterators.
/// See its documentation for more.
///
/// [`combinations`]: IterCombinations::combinations
#[cfg_attr(docsrs, doc(cfg(feature = "combinations")))]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Combinations<I>(GenericCombinations<I, Vec<usize>>)
where
    I: Iterator;

impl<I> Combinations<I>
where
    I: Iterator,
    I::Item: Clone,
{
    #[track_caller]
    pub(crate) fn new(iter: I, k: usize) -> Self {
        assert!(k != 0, "combination size must be non-zero");
        Self(GenericCombinations::new(iter, Vec::from_iter(0..k)))
    }
}

impl<I> Clone for Combinations<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<I> fmt::Debug for Combinations<I>
where
    I: Iterator + fmt::Debug,
    I::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_with(f, "Combinations")
    }
}

impl<I> Iterator for Combinations<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.fill_next().map(Vec::from_iter)
    }
}

impl<I> FusedIterator for Combinations<I>
where
    I: Iterator,
    I::Item: Clone,
{
}

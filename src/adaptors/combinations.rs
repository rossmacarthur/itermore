use core::fmt;
use core::iter::FusedIterator;

use crate::adaptors::generic_combinations::GenericCombinations;

/// An extension trait that provides the [`combinations`] and
/// [`combinations_with_reps`] methods for iterators.
///
/// [`combinations`]: IterCombinations::combinations
/// [`combinations_with_reps`]: IterCombinations::combinations_with_reps
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

    /// Returns an iterator adaptor that iterates over `k` length combinations
    /// with repetitions/replacements of all the elements in the underlying
    /// iterator.
    ///
    /// The iterator is consumed as elements are required.
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
    /// let mut iter = "ab".chars().combinations_with_reps(3);
    /// assert_eq!(iter.next(), Some(vec!['a', 'a', 'a']));
    /// assert_eq!(iter.next(), Some(vec!['a', 'a', 'b']));
    /// assert_eq!(iter.next(), Some(vec!['a', 'b', 'a']));
    /// assert_eq!(iter.next(), Some(vec!['a', 'b', 'b']));
    /// assert_eq!(iter.next(), Some(vec!['b', 'a', 'a']));
    /// // etc
    /// ```
    #[inline]
    fn combinations_with_reps(self, k: usize) -> CombinationsWithReps<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        CombinationsWithReps::new(self, k)
    }
}

impl<I: ?Sized> IterCombinations for I where I: Iterator {}

////////////////////////////////////////////////////////////////////////////////
// Without repetitions/replacement
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
// With repetitions/replacement
////////////////////////////////////////////////////////////////////////////////

/// An iterator that iterates over `K` length combinations with
/// repetitions/replacements of all the elements in the underlying iterator.
///
/// This struct is created by the [`combinations_with_reps`] method on
/// iterators. See its documentation for more.
///
/// [`combinations_with_reps`]: IterCombinations::combinations_with_reps
#[cfg_attr(docsrs, doc(cfg(feature = "combinations")))]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CombinationsWithReps<I>(GenericCombinations<I, Vec<usize>>)
where
    I: Iterator;

impl<I> CombinationsWithReps<I>
where
    I: Iterator,
{
    #[track_caller]
    pub(crate) fn new(iter: I, k: usize) -> Self {
        assert!(k != 0, "combination size must be non-zero");
        Self(GenericCombinations::new(iter, vec![0; k]))
    }
}

impl<I> fmt::Debug for CombinationsWithReps<I>
where
    I: Iterator + fmt::Debug,
    I::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_with(f, "CombinationsWithReps")
    }
}

impl<I> Iterator for CombinationsWithReps<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.fill_next_with_reps().map(Vec::from_iter)
    }
}

impl<I> FusedIterator for CombinationsWithReps<I>
where
    I: Iterator,
    I::Item: Clone,
{
}

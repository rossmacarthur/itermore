use core::fmt;
use core::fmt::Debug;
use core::iter::FusedIterator;

use crate::adaptors::generic_combinations::GenericCombinations;

/// An extension trait that provides the [`combinations_with_reps`] method for
/// iterators.
///
/// [`combinations_with_reps`]: IterCombinationsWithReps::combinations_with_reps
#[cfg_attr(docsrs, doc(cfg(feature = "combinations_with_reps")))]
pub trait IterCombinationsWithReps: Iterator {
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
    /// use itermore::IterCombinationsWithReps;
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

impl<I: ?Sized> IterCombinationsWithReps for I where I: Iterator {}

/// An iterator that iterates over `K` length combinations with
/// repetitions/replacements of all the elements in the underlying iterator.
///
/// This struct is created by the [`combinations_with_reps`] method on
/// iterators. See its documentation for more.
///
/// [`combinations_with_reps`]: IterCombinationsWithReps::combinations_with_reps
#[cfg_attr(docsrs, doc(cfg(feature = "combinations_with_reps")))]
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

impl<I> Debug for CombinationsWithReps<I>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_with(f, "CombinationsWithReps")
    }
}

impl<I> Clone for CombinationsWithReps<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
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

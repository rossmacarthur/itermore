use core::fmt;
use core::fmt::Debug;
use core::iter::FusedIterator;

use crate::adaptors::generic_combinations::GenericCombinations;

/// An extension trait that provides the [`array_combinations_with_reps`] method for
/// iterators.
///
/// [`array_combinations_with_reps`]: IterArrayCombinationsWithReps::array_combinations_with_reps
#[cfg_attr(docsrs, doc(cfg(feature = "array_combinations_with_reps")))]
pub trait IterArrayCombinationsWithReps: Iterator {
    /// Returns an iterator adaptor that iterates over `K` length combinations
    /// with repetitions/replacements of all the elements in the underlying
    /// iterator.
    ///
    /// The iterator is consumed as elements are required.
    ///
    /// # Panics
    ///
    /// If called with `K = 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use itermore::IterArrayCombinationsWithReps;
    ///
    /// let mut iter = "ab".chars().array_combinations_with_reps();
    /// assert_eq!(iter.next(), Some(['a', 'a', 'a']));
    /// assert_eq!(iter.next(), Some(['a', 'a', 'b']));
    /// assert_eq!(iter.next(), Some(['a', 'b', 'a']));
    /// assert_eq!(iter.next(), Some(['a', 'b', 'b']));
    /// assert_eq!(iter.next(), Some(['b', 'a', 'a']));
    /// // etc
    /// ```
    #[inline]
    fn array_combinations_with_reps<const K: usize>(self) -> ArrayCombinationsWithReps<Self, K>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        ArrayCombinationsWithReps::new(self)
    }
}

impl<I: ?Sized> IterArrayCombinationsWithReps for I where I: Iterator {}

/// An iterator that iterates over `K` length combinations with
/// repetitions/replacements of all the elements in the underlying iterator.
///
/// This struct is created by the [`array_combinations_with_reps`] method on
/// iterators. See its documentation for more.
///
/// [`array_combinations_with_reps`]: IterArrayCombinationsWithReps::array_combinations_with_reps
#[cfg_attr(docsrs, doc(cfg(feature = "array_combinations_with_reps")))]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayCombinationsWithReps<I, const K: usize>(GenericCombinations<I, [usize; K]>)
where
    I: Iterator;

impl<I, const K: usize> ArrayCombinationsWithReps<I, K>
where
    I: Iterator,
{
    #[track_caller]
    pub(crate) fn new(iter: I) -> Self {
        assert!(K != 0, "combination size must be non-zero");
        Self(GenericCombinations::new(iter, [0; K]))
    }
}

impl<I, const K: usize> Clone for ArrayCombinationsWithReps<I, K>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<I, const K: usize> Debug for ArrayCombinationsWithReps<I, K>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_with(f, "ArrayCombinationsWithReps")
    }
}

impl<I, const K: usize> Iterator for ArrayCombinationsWithReps<I, K>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; K];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.fill_next_with_reps().map(|it| {
            // SAFETY: The iterator is guaranteed to yield K elements because
            // it is derived from `self.0.comb` which is an array of length K.
            unsafe { arrays::collect_unchecked(it) }
        })
    }
}

impl<I, const K: usize> FusedIterator for ArrayCombinationsWithReps<I, K>
where
    I: Iterator,
    I::Item: Clone,
{
}

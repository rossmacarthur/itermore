use core::fmt;
use core::iter::FusedIterator;

use crate::adaptors::generic_combinations::GenericCombinations;

/// An extension trait that provides the [`array_combinations`] method for
/// iterators.
///
/// [`array_combinations`]: IterArrayCombinations::array_combinations
pub trait IterArrayCombinations: Iterator {
    /// Returns an iterator adaptor that iterates over `K` length combinations
    /// of all the elements in the underlying iterator.
    ///
    /// The iterator is consumed as elements are required. In the first
    /// iteration `K` elements will be consumed by the iterator.
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
    /// use itermore::IterArrayCombinations;
    ///
    /// let mut iter = "abcd".chars().array_combinations();
    /// assert_eq!(iter.next(), Some(['a', 'b', 'c']));
    /// assert_eq!(iter.next(), Some(['a', 'b', 'd']));
    /// assert_eq!(iter.next(), Some(['a', 'c', 'd']));
    /// assert_eq!(iter.next(), Some(['b', 'c', 'd']));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    fn array_combinations<const K: usize>(self) -> ArrayCombinations<Self, K>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        ArrayCombinations::new(self)
    }

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
    /// use itermore::IterArrayCombinations;
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

impl<I: ?Sized> IterArrayCombinations for I where I: Iterator {}

////////////////////////////////////////////////////////////////////////////////
// Without repetitions/replacement
////////////////////////////////////////////////////////////////////////////////

/// An iterator that iterates over `K` length combinations of all the elements
/// in the underlying iterator.
///
/// This struct is created by the [`array_combinations`] method on iterators.
/// See its documentation for more.
///
/// [`array_combinations`]: IterArrayCombinations::array_combinations
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayCombinations<I, const K: usize>(GenericCombinations<I, [usize; K]>)
where
    I: Iterator;

impl<I, const K: usize> ArrayCombinations<I, K>
where
    I: Iterator,
    I::Item: Clone,
{
    #[track_caller]
    pub(crate) fn new(iter: I) -> Self {
        assert!(K != 0, "combination size must be non-zero");

        // SAFETY: The range 0..K yields at least K elements.
        let comb = unsafe { arrays::collect_unchecked(0..K) };

        Self(GenericCombinations::new(iter, comb))
    }
}

impl<I, const K: usize> Clone for ArrayCombinations<I, K>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<I, const K: usize> fmt::Debug for ArrayCombinations<I, K>
where
    I: Iterator + fmt::Debug,
    I::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_with(f, "ArrayCombinations")
    }
}

impl<I, const K: usize> Iterator for ArrayCombinations<I, K>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; K];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.fill_next().map(|it| {
            // SAFETY: The iterator is guaranteed to yield K elements because
            // it is derived from `self.0.comb` which is an array of length K.
            unsafe { arrays::collect_unchecked(it) }
        })
    }
}

impl<I, const K: usize> FusedIterator for ArrayCombinations<I, K>
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
/// This struct is created by the [`array_combinations_with_reps`] method on
/// iterators. See its documentation for more.
///
/// [`array_combinations_with_reps`]: IterArrayCombinations::array_combinations_with_reps
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

impl<I, const K: usize> fmt::Debug for ArrayCombinationsWithReps<I, K>
where
    I: Iterator + fmt::Debug,
    I::Item: fmt::Debug,
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

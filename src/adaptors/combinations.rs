use core::iter::{Fuse, FusedIterator};

use alloc::vec::Vec;

/// An extension trait that provides the [`array_combinations`] method for
/// iterators.
///
/// [`array_combinations`]: IterCombinations::array_combinations
pub trait IterCombinations: Iterator {
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
    /// use itermore::IterCombinations;
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
}

impl<I: ?Sized> IterCombinations for I where I: Iterator {}

/// An iterator that iterates over `K` length combinations of all the elements
/// in the underlying iterator.
///
/// This struct is created by the [`array_combinations`] method on iterators.
/// See its documentation for more.
///
/// [`array_combinations`]: IterCombinations::array_combinations
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayCombinations<I, const K: usize>
where
    I: Iterator,
{
    /// The underlying iterator.
    iter: Fuse<I>,

    /// A number representing the combination.
    ///
    /// If we consider the iterator as a number of K digits in base N where N is
    /// the length of the iterator (unknown at this point) then each digit
    /// represents a position in the iterator. Incrementing this number will
    /// find the next permutation, to find the next combination we need to
    /// only the cases where all digits are in increasing order.
    comb: [usize; K],

    /// A buffer containing already yielded elements that are needed for later
    /// combinations.
    buf: Vec<I::Item>,

    /// Whether this is the first iteration.
    first: bool,
}

impl<I, const K: usize> ArrayCombinations<I, K>
where
    I: Iterator,
{
    #[track_caller]
    pub(crate) fn new(iter: I) -> Self
    where
        I: Iterator,
    {
        assert!(K != 0, "combination size must be non-zero");

        // We need to be fuse the iterator because we want to use `None` as a
        // terminating condition for the combinations algorithm.
        let iter = iter.fuse();

        // SAFETY: The range 0..K yields at least K elements.
        let comb = unsafe { arrays::collect_unchecked(0..K) };

        Self {
            iter,
            comb,
            buf: Vec::with_capacity(K),
            first: true,
        }
    }
}

impl<I, const K: usize> Iterator for ArrayCombinations<I, K>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; K];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            // Prefill the buffer with K elements from the iterator.
            self.buf.reserve(K);
            for _ in 0..K {
                self.buf.push(self.iter.next()?);
            }
            self.first = false;
        } else {
            // If the last digit in the combination points to the last element
            // in the buffer then we need to get another element from the
            // iterator because the next combination will need this element.
            let d = unsafe { self.comb.last_mut().unwrap_unchecked() };
            if *d == self.buf.len() - 1 {
                if let Some(item) = self.iter.next() {
                    self.buf.push(item);
                }
            }

            // Now we find the digit that needs to be incremented. Looking from
            // the back we find the first digit that is not the final expected
            // combination for that digit.
            //
            // For example given K = 3 and a total N = 5
            //
            // 0 1 3 ^--- finds this because at this point we think N = 4
            //
            // 0 1 4 ^----- finds this because we know N = 5
            //
            // 0 2 3 ^--- finds this again since it is not 4 yet
            //
            // The base case in the above example would be the following which
            // returns `None` and is propagated using `?`.
            //
            // 2 3 4
            //
            let n = self.buf.len();
            let i = self
                .comb
                .iter()
                .enumerate()
                .rposition(|(i, &d)| d != i + n - K)?;

            // Increment the digit, and reset the ones to its right
            //
            // For example given K = 3 and N = 5 and the following combination.
            //
            // 0 1 4 ^----- i
            //
            // We would increment digit i and then reset all digits to the
            // right.
            //
            // 0 2 3 ^---- was reset to 3 ^------ was incremented
            //
            self.comb[i] += 1;
            for j in (i + 1)..K {
                self.comb[j] = self.comb[j - 1] + 1;
            }
        }

        let next = {
            // Map the combination digits to actual elements in the buffer.
            let arr = self.comb.iter().map(|&d| self.buf[d].clone());
            // SAFETY: The iterator is guaranteed to yield K elements because
            // `self.comb` is an array of length K.
            unsafe { arrays::collect_unchecked(arr) }
        };

        Some(next)
    }
}

impl<I, const K: usize> FusedIterator for ArrayCombinations<I, K>
where
    I: Iterator,
    I::Item: Clone,
{
}

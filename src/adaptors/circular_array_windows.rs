use core::fmt;
use core::fmt::Debug;
use core::iter::{Cycle, FusedIterator};

use crate::{ArrayWindows, IterArrayWindows};

#[cfg_attr(docsrs, doc(cfg(feature = "circular_array_windows")))]
pub trait IterCircularArrayWindows: Iterator {
    /// Returns an iterator over all contiguous windows of length `N` wrapping
    /// back to the first elements when the window would otherwise exceed the
    /// length of the iterator.
    ///
    /// This adaptor clones the iterator elements so that they can be part of
    /// successive windows, this makes this it most suited for iterators of
    /// references and other values that are cheap to clone or copy.
    ///
    /// # Panics
    ///
    /// If called with `N = 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use itermore::IterCircularArrayWindows;
    ///
    /// let mut iter = (1..5).into_iter().circular_array_windows();
    /// assert_eq!(iter.next(), Some([1, 2]));
    /// assert_eq!(iter.next(), Some([2, 3]));
    /// assert_eq!(iter.next(), Some([3, 4]));
    /// assert_eq!(iter.next(), Some([4, 1]));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// If the window is smaller than the iterator length, then the iterator
    /// will wrap around multiple times.
    ///
    /// ```
    /// use itermore::IterCircularArrayWindows;
    ///
    /// let mut iter = (1..2).into_iter().circular_array_windows::<3>();
    /// assert_eq!(iter.next(), Some([1, 1, 1]));
    /// ```
    #[inline]
    fn circular_array_windows<const N: usize>(self) -> CircularArrayWindows<Self, N>
    where
        Self: Sized + Clone + ExactSizeIterator,
        Self::Item: Clone,
    {
        CircularArrayWindows::new(self)
    }
}

impl<I: ?Sized> IterCircularArrayWindows for I where I: Iterator {}

/// An iterator over all contiguous windows of length `N` wrapping back to the
/// first elements when the window would otherwise exceed the length of the
/// iterator.
///
/// This struct is created by the [`circular_array_windows`] method on
/// iterators. See its documentation for more.
///
/// [`circular_array_windows`]: IterCircularArrayWindows::circular_array_windows
#[cfg_attr(docsrs, doc(cfg(feature = "circular_array_windows")))]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CircularArrayWindows<I, const N: usize>
where
    I: Iterator + Clone,
{
    iter: ArrayWindows<Cycle<I>, N>,
    len: usize,
}

impl<I, const N: usize> CircularArrayWindows<I, N>
where
    I: ExactSizeIterator + Clone,
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        let len = iter.len();
        let iter = iter.cycle().array_windows();
        Self { iter, len }
    }
}

impl<I, const N: usize> Debug for CircularArrayWindows<I, N>
where
    I: Iterator + Clone + Debug,
    I::Item: Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircularArrayWindows")
            .field("iter", &self.iter)
            .field("len", &self.len)
            .finish()
    }
}

impl<I, const N: usize> Clone for CircularArrayWindows<I, N>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            len: self.len,
        }
    }
}

impl<I, const N: usize> Iterator for CircularArrayWindows<I, N>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len != 0 {
            self.len -= 1;
            self.iter.next()
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len
    }
}

impl<I, const N: usize> ExactSizeIterator for CircularArrayWindows<I, N>
where
    I: ExactSizeIterator + Clone,
    I::Item: Clone,
{
}

impl<I, const N: usize> FusedIterator for CircularArrayWindows<I, N>
where
    I: ExactSizeIterator + Clone,
    I::Item: Clone,
{
}

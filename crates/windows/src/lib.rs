//! This crate provides an iterator adapter to iterate over all contiguous
//! windows of length `N`.
//!
//! # Getting started
//!
//! Add the crate to your Cargo manifest.
//! ```sh
//! cargo add iterwindows
//! ```
//!
//! And bring the [`IterArrayWindows`] trait into scope.
//!
//! ```
//! use iterwindows::IterArrayWindows;
//! ```
//!
//! Now you can use the [`array_windows`] method on any iterator.
//!
//! ```
//! # use iterwindows::IterArrayWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.array_windows() {
//!     println!("{} {} {}", a, b, c)
//! }
//! ```
//!
//! Generally the size of `N` can be inferred by the compiler but you can also
//! specify it manually.
//! ```
//! # use iterwindows::IterArrayWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! let w = iter.array_windows::<3>();
//! ```
//!
//! [`array_windows`]: IterArrayWindows::array_windows
#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

/// An extension trait that provides the [`array_windows`] method for iterators.
///
/// [`array_windows`]: IterArrayWindows::array_windows
pub trait IterArrayWindows: Iterator {
    /// Returns an iterator over all contiguous windows of length `N`.
    ///
    /// The windows overlap. If the iterator is shorter than `N`, the iterator
    /// returns no values.
    ///
    /// This adapter clones the iterator elements so that they can be part of
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
    /// use iterwindows::IterArrayWindows;
    ///
    /// let mut iter = "rust".chars().array_windows();
    /// assert_eq!(iter.next(), Some(['r', 'u']));
    /// assert_eq!(iter.next(), Some(['u', 's']));
    /// assert_eq!(iter.next(), Some(['s', 't']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use iterwindows::IterArrayWindows;
    ///
    /// let seq: &[i32] = &[0, 1, 1, 2, 3, 5, 8, 13];
    /// for [x, y, z] in seq.iter().copied().array_windows() {
    ///     assert_eq!(x + y, z);
    /// }
    /// ```
    #[inline]
    fn array_windows<const N: usize>(self) -> ArrayWindows<Self, N>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        ArrayWindows::new(self)
    }
}

impl<I: ?Sized> IterArrayWindows for I where I: Iterator {}

/// An iterator over all contiguous windows of length `N`.
///
/// This struct is created by the [`array_windows`] method on iterators. See its
/// documentation for more.
///
/// [`array_windows`]: IterArrayWindows::array_windows
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayWindows<I, const N: usize>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    prev: Option<[I::Item; N]>,
    prev_back: Option<[I::Item; N]>,

    /// Items shared between `prev` and `prev_back`.
    overlap: usize,
}

impl<I, const N: usize> ArrayWindows<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        assert!(N != 0, "window size must be non-zero");
        Self {
            iter,
            prev: None,
            prev_back: None,
            overlap: 0,
        }
    }

    /// After `iter` is exhausted, this provides the `next` value.
    fn next_overlapping(
        prev_back: &mut Option<[I::Item; N]>,
        overlap: &mut usize,
    ) -> Option<I::Item> {
        if *overlap < N {
            if let Some(prev_back) = prev_back {
                let item = prev_back[*overlap].clone();
                *overlap += 1;
                return Some(item);
            }
        }
        None
    }

    /// After `iter` is exhausted, this provides the `next_back` value.
    fn next_back_overlapping(
        prev: &mut Option<[I::Item; N]>,
        overlap: &mut usize,
    ) -> Option<I::Item> {
        if *overlap < N {
            if let Some(prev) = prev {
                *overlap += 1;
                let item = prev[N - *overlap].clone();
                return Some(item);
            }
        }
        None
    }

    /// Number of items contained within `prev` and `prev_back` combined.
    fn extra_len(&self) -> usize {
        let prev_len = self.prev.as_ref().map_or(0, |p| p.len());
        let prev_back_len = self.prev_back.as_ref().map_or(0, |p| p.len());
        (prev_len + prev_back_len).saturating_sub(N + 1)
    }
}

impl<I, const N: usize> Iterator for ArrayWindows<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            iter,
            prev,
            prev_back,
            overlap,
        } = self;

        match prev {
            Some(prev) => {
                let item = iter
                    .next()
                    .or_else(|| Self::next_overlapping(prev_back, overlap))?;
                prev.rotate_left(1);
                prev[N - 1] = item;
                Some(prev.clone())
            }
            None => {
                let tmp = arrays::collect(iter.chain(core::iter::from_fn(|| {
                    Self::next_overlapping(prev_back, overlap)
                })))?;
                *prev = Some(tmp.clone());
                Some(tmp)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let extra = self.extra_len();
        (
            lower.saturating_sub(N - 1) + extra,
            upper.map(|n| n.saturating_sub(N - 1) + extra),
        )
    }

    #[inline]
    fn count(self) -> usize {
        let extra = self.extra_len();
        self.iter.count().saturating_sub(N - 1) + extra
    }
}

impl<I, const N: usize> DoubleEndedIterator for ArrayWindows<I, N>
where
    I: DoubleEndedIterator,
    I::Item: Clone,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let Self {
            iter,
            prev,
            prev_back,
            overlap,
        } = self;

        match prev_back {
            Some(prev_back) => {
                let item = iter
                    .next_back()
                    .or_else(|| Self::next_back_overlapping(prev, overlap))?;
                prev_back.rotate_right(1);
                prev_back[0] = item;
                Some(prev_back.clone())
            }
            None => {
                let tmp = arrays::collect_reversed(iter.rev().chain(core::iter::from_fn(|| {
                    Self::next_back_overlapping(prev, overlap)
                })))?;
                *prev_back = Some(tmp.clone());
                Some(tmp)
            }
        }
    }
}

#[cfg(test)]
mod tests {}

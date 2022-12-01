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
//! And bring the [`IterWindows`] trait into scope.
//!
//! ```
//! use iterwindows::IterWindows;
//! ```
//!
//! Now you can use the [`array_windows`] method on any iterator.
//!
//! ```
//! # use iterwindows::IterWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.array_windows() {
//!     println!("{} {} {}", a, b, c)
//! }
//! ```
//!
//! Generally the size of `N` can be inferred by the compiler but you can also
//! specify it manually.
//! ```
//! # use iterwindows::IterWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! let w = iter.array_windows::<3>();
//! ```
//!
//! [`array_windows`]: IterWindows::array_windows

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

/// An extension trait that provides the [`array_windows`] method for iterators.
///
/// [`array_windows`]: IterWindows::array_windows
pub trait IterWindows: Iterator {
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
    /// use iterwindows::IterWindows;
    ///
    /// let mut iter = "rust".chars().array_windows();
    /// assert_eq!(iter.next(), Some(['r', 'u']));
    /// assert_eq!(iter.next(), Some(['u', 's']));
    /// assert_eq!(iter.next(), Some(['s', 't']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use iterwindows::IterWindows;
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

impl<I: ?Sized> IterWindows for I where I: Iterator {}

/// An iterator over all contiguous windows of length `N`.
///
/// This struct is created by the [`array_windows`] method on iterators. See its
/// documentation for more.
///
/// [`array_windows`]: IterWindows::array_windows
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayWindows<I, const N: usize>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    last: Option<[I::Item; N]>,
}

impl<I, const N: usize> ArrayWindows<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        assert!(N != 0, "window size must be non-zero");
        Self { iter, last: None }
    }
}

impl<I: Iterator, const N: usize> Iterator for ArrayWindows<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, last } = self;

        match last {
            Some(last) => {
                let item = iter.next()?;
                last.rotate_left(1);
                if let Some(end) = last.last_mut() {
                    *end = item;
                }
                Some(last.clone())
            }
            None => {
                let tmp = arrays::collect(iter)?;
                *last = Some(tmp.clone());
                Some(tmp)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (
            lower.saturating_sub(N - 1),
            upper.map(|n| n.saturating_sub(N - 1)),
        )
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count().saturating_sub(N - 1)
    }
}

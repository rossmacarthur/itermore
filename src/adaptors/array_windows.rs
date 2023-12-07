/// An extension trait that provides the [`array_windows`] method for iterators.
///
/// [`array_windows`]: IterArrayWindows::array_windows
#[cfg_attr(docsrs, doc(cfg(feature = "array_windows")))]
pub trait IterArrayWindows: Iterator {
    /// Returns an iterator over all contiguous windows of length `N`.
    ///
    /// The windows overlap. If the iterator is shorter than `N`, the iterator
    /// returns no values.
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
    /// use itermore::IterArrayWindows;
    ///
    /// let mut iter = "rust".chars().array_windows();
    /// assert_eq!(iter.next(), Some(['r', 'u']));
    /// assert_eq!(iter.next(), Some(['u', 's']));
    /// assert_eq!(iter.next(), Some(['s', 't']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use itermore::IterArrayWindows;
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
#[cfg_attr(docsrs, doc(cfg(feature = "array_windows")))]
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayWindows<I, const N: usize>
where
    I: Iterator,
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

use core::iter;

/// An extension trait that provides the [`multi_collect_array`] method for iterators.
///
/// [`multi_collect_array`]: IterMultiCollectArray::multi_collect_array
#[cfg_attr(docsrs, doc(cfg(feature = "multi_collect_array")))]
pub trait IterMultiCollectArray: Iterator {
    /// Consumes an iterator of arrays collecting it into an array of
    /// collections.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use itermore::IterMultiCollectArray;
    ///
    /// let inputs = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    ///
    /// let arr: [Vec<_>; 3] = inputs.into_iter().multi_collect_array();
    /// assert_eq!(arr, [vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
    /// ```
    #[inline]
    fn multi_collect_array<C, T, const N: usize>(self) -> [C; N]
    where
        Self: Sized,
        Self: Iterator<Item = [T; N]>,
        C: Default + Extend<T>,
    {
        // SAFETY: The iterator is guaranteed to have at least `N` elements
        // since the property of `iter::repeat_with` is to repeat forever.
        let acc: [C; N] = unsafe { arrays::from_iter_unchecked(iter::repeat_with(C::default)) };
        self.fold(acc, |mut acc, item| {
            for (c, item) in iter::zip(acc.iter_mut(), item) {
                c.extend(Some(item));
            }
            acc
        })
    }
}

impl<I: ?Sized> IterMultiCollectArray for I where I: Iterator {}

/// An extension trait that provides the [`collect_array`] method for iterators.
///
/// [`collect_array`]: IterCollectArray::collect_array
#[cfg_attr(docsrs, doc(cfg(feature = "collect_array")))]
pub trait IterCollectArray: Iterator {
    /// Consumes the entire iterator collecting it into an array.
    ///
    /// # Panics
    ///
    /// If the iterator contains too little or too many elements to fit in the
    /// array.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use itermore::IterCollectArray;
    ///
    /// let mut iter = "a,b,c".split(",");
    ///
    /// let arr: [_; 3] = iter.collect_array();
    /// assert_eq!(arr, ["a", "b", "c"]);
    /// ```
    #[inline]
    #[track_caller]
    fn collect_array<const N: usize>(mut self) -> [Self::Item; N]
    where
        Self: Sized,
    {
        match arrays::from_iter(self.by_ref()) {
            Ok(arr) => {
                if self.next().is_some() {
                    panic!("expected exactly {} elements, but collected more", N);
                }
                arr
            }
            Err(arr) => {
                let got = arr.as_slice().len();
                panic!("expected exactly {} elements, but collected {}", N, got);
            }
        }
    }
}

impl<I: ?Sized> IterCollectArray for I where I: Iterator {}

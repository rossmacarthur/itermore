use core::iter::FusedIterator;

/// An extension trait that provides the [`cartesian_product`] method for
/// iterators.
///
/// [`cartesian_product`]: IterCartesianProduct::cartesian_product
#[cfg_attr(docsrs, doc(cfg(feature = "cartesian_product")))]
pub trait IterCartesianProduct: Iterator {
    /// Returns an iterator adaptor that iterates over the cartesian product of
    /// the element sets of two iterators `self` and `other.into_iter()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use itermore::IterCartesianProduct;
    ///
    /// let v = Vec::from_iter((0..3).cartesian_product("αβ".chars()));
    /// assert_eq!(v, [(0, 'α'), (0, 'β'), (1, 'α'), (1, 'β'), (2, 'α'), (2, 'β')]);
    /// ```
    fn cartesian_product<J>(self, other: J) -> CartesianProduct<Self, J::IntoIter>
    where
        Self: Sized,
        Self::Item: Clone,
        J: IntoIterator,
        J::IntoIter: Clone,
    {
        CartesianProduct::new(self, other.into_iter())
    }
}

impl<I: ?Sized> IterCartesianProduct for I where I: Iterator {}

/// An iterator over the cartesian product of the element sets of two iterators
/// `I` and `J`.
///
/// This struct is created by the [`cartesian_product`] method on iterators. See
/// its documentation for more.
///
/// [`cartesian_product`]: IterCartesianProduct::cartesian_product
#[cfg_attr(docsrs, doc(cfg(feature = "cartesian_product")))]
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CartesianProduct<I, J>
where
    I: Iterator,
{
    a: I,
    b: J,
    a_item: Option<I::Item>,
    b_curr: J,
}

impl<I, J> CartesianProduct<I, J>
where
    I: Iterator,
    J: Iterator + Clone,
{
    fn new(mut a: I, b: J) -> Self {
        CartesianProduct {
            a_item: a.next(),
            a,
            b_curr: b.clone(),
            b,
        }
    }
}

impl<I, J> Iterator for CartesianProduct<I, J>
where
    I: Iterator,
    J: Iterator + Clone,
    I::Item: Clone,
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let b_item = match self.b_curr.next() {
            Some(b_item) => b_item,
            None => {
                self.b_curr = self.b.clone();
                let b_item = self.b_curr.next()?;
                self.a_item = self.a.next();
                b_item
            }
        };
        self.a_item.as_ref().map(|a| (a.clone(), b_item))
    }
}

impl<I, J> FusedIterator for CartesianProduct<I, J>
where
    I: FusedIterator,
    J: FusedIterator + Clone,
    I::Item: Clone,
{
}

////////////////////////////////////////////////////////////////////////////////
// Macro
////////////////////////////////////////////////////////////////////////////////

/// Returns an iterator over the cartesian product of the element sets of
/// multiple iterators (up to 12).
///
/// This is essentially the equivalent of calling [`cartesian_product`] multiple
/// times and "flattening" each item e.g. `((A, B), C)` to `(A, B, C)`.
///
/// # Examples
///
/// ```
/// use itermore::{cartesian_product, IterCartesianProduct};
///
/// // With macro
/// let i = cartesian_product!(0..3, "αβ".chars(), [-1, 0, 1]);
///
/// // Without macro
/// let j = (0..3)
///     .cartesian_product("αβ".chars())
///     .cartesian_product([-1, 0, 1])
///     .map(|((a, b), c)| (a, b, c));
///
/// assert_eq!(Vec::from_iter(i), Vec::from_iter(j));
/// ```
///
/// Iterate over the 3D coordinates of a 10 x 10 x 10 cube.
///
/// ```
/// use itermore::cartesian_product;
///
/// // With macro
/// for (i, j, k) in cartesian_product!(0..10, 0..10, 0..10) {
///     // ...
/// }
///
/// // Without macro
/// for i in 0..10 {
///     for j in 0..10 {
///         for k in 0..10 {
///             // ...
///         }
///     }
/// }
/// ```
///
/// [`cartesian_product`]: IterCartesianProduct::cartesian_product
#[cfg_attr(docsrs, doc(cfg(feature = "cartesian_product")))]
#[macro_export]
macro_rules! cartesian_product {
    ($I:expr $(,)?) => {
        $crate::core::iter::IntoIterator::into_iter($I)
    };

    ($I:expr, $J:expr $(,)?) => {
        $crate::IterCartesianProduct::cartesian_product(
            $crate::cartesian_product!($I),
            $crate::cartesian_product!($J),
        )
    };

    ($I:expr, $J:expr, $($K:expr),+ $(,)?) => {{
        $crate::cartesian_product!($crate::cartesian_product!($I, $J), $($K),+)
            .map($crate::flatten_tuple)
    }};
}

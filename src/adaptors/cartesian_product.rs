/// An extension trait that provides the [`cartesian_product`] method for
/// iterators.
///
/// [`cartesian_product`]: IterCartesianProduct::cartesian_product
pub trait IterCartesianProduct: Iterator {
    /// Return an iterator adaptor that iterates over the cartesian product of
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

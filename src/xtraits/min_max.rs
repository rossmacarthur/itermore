use core::cmp::Ordering;

/// An extension trait that provides the [`min_max`] method and friends for
/// iterators.
///
/// [`min_max`]: IterMinMax::min_max
#[cfg_attr(docsrs, doc(cfg(feature = "min_max")))]
pub trait IterMinMax: Iterator {
    /// Returns the minimum and maximum element in the iterator.
    ///
    /// - If there are no elements then `None` is returned.
    /// - In the case of a single element the element is cloned and returned in
    ///   both places.
    /// - If several elements are equally minimum or maximum, the first element
    ///   is returned.
    /// - On an iterator of length `n`, `min_max` does `1.5 * n` comparisons, so
    ///   it is faster than calling [`min`] and [`max`] separately which does
    ///   `2 * n` comparisons.
    ///
    /// [`min`]: Iterator::min
    /// [`max`]: Iterator::max
    fn min_max(self) -> Option<(Self::Item, Self::Item)>
    where
        Self: Sized,
        Self::Item: Ord + Clone,
    {
        min_max(self, Ord::cmp)
    }

    /// Returns the minimum and maximum element with respect to the given
    /// comparison function.
    ///
    /// See [`min_max`] for more details.
    ///
    /// [`min_max`]: IterMinMax::min_max
    fn min_max_by<F>(self, compare: F) -> Option<(Self::Item, Self::Item)>
    where
        Self: Sized,
        Self::Item: Clone,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        min_max(self, compare)
    }

    /// Returns the minimum and maximum element with respect to element returned
    /// from the given key function.
    ///
    /// See [`min_max`] for more details.
    ///
    /// [`min_max`]: IterMinMax::min_max
    //
    // FIXME: `Clone` bound on `K` is unnecessary but requires refactoring
    fn min_max_by_key<F, K>(self, mut key: F) -> Option<(Self::Item, Self::Item)>
    where
        Self: Sized,
        Self::Item: Clone,
        K: Ord + Clone,
        F: FnMut(&Self::Item) -> K,
    {
        self.map(move |item| (key(&item), item))
            .min_max_by(|(k1, _), (k2, _)| k1.cmp(k2))
            .map(|((_, min), (_, max))| (min, max))
    }
}

impl<I: ?Sized> IterMinMax for I where I: Iterator {}

fn min_max<I, F>(mut iter: I, mut compare: F) -> Option<(I::Item, I::Item)>
where
    I::Item: Clone,
    I: Iterator,
    F: FnMut(&I::Item, &I::Item) -> Ordering,
{
    let (mut min, mut max) = {
        let a = iter.next()?;
        match iter.next() {
            None => return Some((a.clone(), a)),
            Some(b) => match compare(&a, &b) {
                Ordering::Less => (a, b),
                _ => (b, a),
            },
        }
    };
    while let Some(a) = iter.next() {
        let b = match iter.next() {
            Some(b) => b,
            None => {
                if compare(&a, &min) == Ordering::Less {
                    min = a;
                } else if compare(&a, &max) == Ordering::Greater {
                    max = a;
                }
                break;
            }
        };
        let (a, b) = match compare(&a, &b) {
            Ordering::Less => (a, b),
            _ => (b, a),
        };
        if compare(&a, &min) == Ordering::Less {
            min = a;
        } else if compare(&b, &max) == Ordering::Greater {
            max = b;
        }
    }
    Some((min, max))
}

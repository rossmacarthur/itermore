use core::cmp::Ordering;

use alloc::vec::IntoIter;

/// An extension trait that provides the [`sorted`] method and friends for
/// iterators.
///
/// [`sorted`]: IterSorted::sorted
#[cfg(feature = "sorted")]
pub trait IterSorted: Iterator {
    /// Sorts the iterator.
    ///
    /// Simply collects into a [`Vec`] and sorts it using [`slice::sort`].
    fn sorted(self) -> IntoIter<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut v = Vec::from_iter(self);
        v.sort();
        v.into_iter()
    }

    /// Sorts the iterator with a comparator function.
    ///
    /// Simply collects into a [`Vec`] and sorts it using [`slice::sort_by`].
    fn sorted_by<F>(self, cmp: F) -> IntoIter<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut v = Vec::from_iter(self);
        v.sort_by(cmp);
        v.into_iter()
    }

    /// Sorts the iterator with a key extraction function.
    ///
    /// Simply collects into a [`Vec`] and sorts it using
    /// [`slice::sort_by_key`].
    fn sorted_by_key<K, F>(self, f: F) -> IntoIter<Self::Item>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        let mut v = Vec::from_iter(self);
        v.sort_by_key(f);
        v.into_iter()
    }

    /// Sorts the iterator with a key extraction function.
    ///
    /// Simply collects into a [`Vec`] and sorts it using
    /// [`slice::sort_by_cached_key`].
    fn sorted_by_cached_key<K, F>(self, f: F) -> IntoIter<Self::Item>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        let mut v = Vec::from_iter(self);
        v.sort_by_cached_key(f);
        v.into_iter()
    }

    /// Sorts the iterator, but might not preserve the order of equal elements.
    ///
    /// Simply collects into a [`Vec`] and sorts it using
    /// [`slice::sort_unstable`].
    fn sorted_unstable(self) -> IntoIter<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut v = Vec::from_iter(self);
        v.sort_unstable();
        v.into_iter()
    }

    /// Sorts the iterator with a comparator function, but might not preserve
    /// the order of equal elements.
    ///
    /// Simply collects into a [`Vec`] and sorts it using
    /// [`slice::sort_unstable_by`].
    fn sorted_unstabled_by<F>(self, cmp: F) -> IntoIter<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut v = Vec::from_iter(self);
        v.sort_unstable_by(cmp);
        v.into_iter()
    }

    /// Sorts the iterator with a key extraction function, but might not
    /// preserve the order of equal elements.
    ///
    /// Simply collects into a [`Vec`] and sorts it using
    /// [`slice::sort_unstable_by_key`].
    fn sorted_unstable_by_key<K, F>(self, f: F) -> IntoIter<Self::Item>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        let mut v = Vec::from_iter(self);
        v.sort_unstable_by_key(f);
        v.into_iter()
    }
}

impl<I: ?Sized> IterSorted for I where I: Iterator {}

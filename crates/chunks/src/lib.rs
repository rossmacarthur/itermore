//! This crate provides an iterator adapter that yields N elements of the
//! iterator at a time.
//!
//! # Getting started
//!
//! Add the crate to your Cargo manifest.
//! ```sh
//! cargo add iterchunks
//! ```
//!
//! And bring the [`IterChunks`] trait into scope.
//!
//! ```
//! use iterchunks::IterChunks;
//! ```
//!
//! Now you can use the [`chunks`][IterChunks::chunks] method on any iterator.
//!
//! ```
//! # use iterchunks::IterChunks;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.chunks() {
//!     println!("{} {} {}", a, b, c)
//! }
//! ```
//!
//! Generally the size of `N` can be inferred by the compiler but you can also
//! specify it manually.
//! ```
//! # use iterchunks::IterChunks;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! let c = iter.chunks::<3>();
//! ```

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

use core::mem;
use core::mem::MaybeUninit;
use core::ptr;

/// An extension trait that provides the [`chunks`][IterChunks::chunks]
/// method for iterators.
pub trait IterChunks: Iterator {
    /// Advances the iterator and returns an array containing the next `N`
    /// values.
    ///
    /// If there are not enough elements to fill the array then `None` is
    /// returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterchunks::IterChunks;
    ///
    /// let mut iter = "lorem".chars();
    ///
    /// assert_eq!(iter.next_chunk().unwrap(), ['l', 'o']);              // N is inferred as 2
    /// assert_eq!(iter.next_chunk().unwrap(), ['r', 'e', 'm']);         // N is inferred as 3
    /// assert!(iter.next_chunk::<4>().is_none()); // N is explicitly 4
    /// ```
    ///
    /// Split a string and get the first three items.
    ///
    /// ```
    /// use iterchunks::IterChunks;
    ///
    /// let quote = "not all those who wander are lost";
    /// let [first, second, third] = quote.split_whitespace().next_chunk().unwrap();
    /// assert_eq!(first, "not");
    /// assert_eq!(second, "all");
    /// assert_eq!(third, "those");
    /// ```
    #[inline]
    fn next_chunk<const N: usize>(&mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        collect(self.by_ref())
    }

    /// Returns an iterator over `N` elements of the iterator at a time.
    ///
    /// The chunks do not overlap. If `N` does not divide the length of the
    /// iterator, then the last up to `N-1` elements will be omitted.
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
    /// use iterchunks::IterChunks;
    ///
    /// let mut iter = "lorem".chars().chunks();
    /// assert_eq!(iter.next(), Some(['l', 'o']));
    /// assert_eq!(iter.next(), Some(['r', 'e']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// ```
    /// use iterchunks::IterChunks;
    ///
    /// let data = [1, 1, 2, -2, 6, 0, 3, 1];
    /// //          ^-----^  ^------^
    /// for [x, y, z] in data.iter().chunks() {
    ///     assert_eq!(x + y + z, 4);
    /// }
    /// ```
    #[inline]
    fn chunks<const N: usize>(self) -> Chunks<Self, N>
    where
        Self: Sized,
    {
        Chunks::new(self)
    }
}

impl<I: ?Sized> IterChunks for I where I: Iterator {}

/// An iterator over `N` elements of the iterator at a time.
///
/// This struct is created by the [`chunks`][IterChunks::chunks] method on
/// iterators. See its documentation for more.
pub struct Chunks<I, const N: usize> {
    iter: I,
}

impl<I, const N: usize> Chunks<I, N>
where
    I: Iterator,
{
    #[track_caller]
    fn new(iter: I) -> Self {
        assert!(N != 0, "chunk size must be non-zero");
        Self { iter }
    }
}

impl<I: Iterator, const N: usize> Iterator for Chunks<I, N>
where
    I: Iterator,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter } = self;
        collect(iter.by_ref())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (lower / N, upper.map(|n| n / N))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count() / N
    }
}

impl<I, const N: usize> DoubleEndedIterator for Chunks<I, N>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let rem = self.iter.len() % N;
        let mut rev = self.iter.by_ref().rev().skip(rem);
        let mut chunk = IterChunks::next_chunk(&mut rev)?;
        chunk.reverse();
        Some(chunk)
    }
}

/// Consumes `N` elements from the iterator and returns them as an array. If the
/// iterator yields fewer than `N` items, `None` is returned and all already
/// yielded items are dropped.
///
/// Since the iterator is passed as a mutable reference and this function calls
/// `next` at most `N` times, the iterator can still be used afterwards to
/// retrieve the remaining items.
///
// Based on the array collect implementation in the Rust standard library.
// https://github.com/rust-lang/rust/blob/master/library/core/src/array/mod.rs#L476-L531
#[inline]
fn collect<I, T, const N: usize>(mut iter: I) -> Option<[T; N]>
where
    I: Iterator<Item = T>,
{
    struct Guard<'a, T, const N: usize> {
        array: &'a mut [MaybeUninit<T>; N],
        init: usize,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
        fn drop(&mut self) {
            for elem in &mut self.array.as_mut_slice()[..self.init] {
                // SAFETY: this raw slice up to `self.len` will only contain
                // the initialized objects.
                unsafe { ptr::drop_in_place(elem.as_mut_ptr()) };
            }
        }
    }

    // SAFETY: The `assume_init` is safe because the type we are claiming to
    // have initialized here is a bunch of `MaybeUninit`s, which do not
    // require initialization.
    //
    // This is not the most ideal way of doing this. In the future when Rust
    // allows inline const expressions we might be able to use the following.
    //
    //      let mut array = [const { MaybeUninit::<T>::uninit() }; N];
    //
    // See https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
    let mut array: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut guard = Guard {
        array: &mut array,
        init: 0,
    };

    for _ in 0..N {
        match iter.next() {
            Some(item) => {
                // SAFETY: `guard.init` starts at zero, is increased by 1 each
                // iteration of the loop, and the loop is aborted once M * N
                // is reached, which is the length of the array.
                unsafe { guard.array.get_unchecked_mut(guard.init).write(item) };
                guard.init += 1;
            }
            None => {
                return None;
                // <-- guard is dropped here with already initialized elements
            }
        }
    }

    mem::forget(guard);
    // SAFETY: the loop above loops exactly N times which is the size of the
    // array, so all elements in the array are initialized.
    Some(unsafe { transmute_unchecked(array) })
}

/// Size-heterogeneous transmutation.
///
/// This is required because the compiler doesn't yet know how to deal with the
/// size of const arrays. We should be able to use [`mem::transmute()`] but it
/// doesn't work yet :(.
#[inline]
unsafe fn transmute_unchecked<A, B>(a: A) -> B {
    let b = unsafe { ptr::read(&a as *const A as *const B) };
    mem::forget(a);
    b
}

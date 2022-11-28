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
//! Now you can use the [`windows`][IterWindows::windows] method on any
//! iterator.
//!
//! ```
//! # use iterwindows::IterWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! for [a, b, c] in iter.windows() {
//!     println!("{} {} {}", a, b, c)
//! }
//! ```
//!
//! Generally the size of `N` can be inferred by the compiler but you can also
//! specify it manually.
//! ```
//! # use iterwindows::IterWindows;
//! # let iter = [1, 2, 3, 4, 5].into_iter();
//! let w = iter.windows::<3>();
//! ```

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

use core::mem;
use core::mem::MaybeUninit;
use core::ptr;

/// An extension trait that provides the [`windows`][IterWindows::windows]
/// method for iterators.
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
    /// let mut iter = "rust".chars().windows();
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
    /// for [x, y, z] in seq.iter().copied().windows() {
    ///     assert_eq!(x + y, z);
    /// }
    /// ```
    #[inline]
    fn windows<const N: usize>(self) -> Windows<Self, N>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Windows::new(self)
    }
}

impl<I: ?Sized> IterWindows for I where I: Iterator {}

/// An iterator over all contiguous windows of length `N`. The windows overlap.
/// If the iterator is shorter than `N`, the iterator returns no values.
///
/// This struct is created by the [`windows`][IterWindows::windows] method on
/// iterators. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Windows<I, const N: usize>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    last: Option<[I::Item; N]>,
}

impl<I, const N: usize> Windows<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        assert!(N != 0, "window size must be non-zero");
        Self { iter, last: None }
    }
}

impl<I: Iterator, const N: usize> Iterator for Windows<I, N>
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
                let tmp = collect(iter)?;
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

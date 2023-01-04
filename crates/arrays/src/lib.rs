//! Collect an iterator into an array
//!
//! # Getting started
//!
//! Add the `arrays` crate to your Cargo manifest.
//!
//! ```sh
//! cargo add arrays
//! ```
//!
//! Now collect any iterator into an array.
//!
//! ```
//! # let iter = 1..5;
//! let arr: [_; 3] = arrays::collect(iter).unwrap();
//! ```

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

use core::hint;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ptr;

/// # Safety
///
/// Caller must ensure the following invariant:
///
/// * `idxs` only returns indices in range 0..N
/// * `curr_idx_to_range` will, for any given index given by `idxs`,
///    return a range which only includes indices yielded so far (preferably, all of them)
///
/// Essentially, we can use `idxs` and `curr_idx_to_range` to properly write the
/// items to the array in the right order and drop them if we don't get enough.
unsafe fn collect_impl<T, const N: usize>(
    mut next: impl FnMut() -> Option<T>,
    idxs: impl Iterator<Item = usize>,
    curr_idx_to_range: impl Fn(usize) -> Range<usize>,
) -> Option<[T; N]> {
    struct Guard<'a, T, const N: usize> {
        array: &'a mut [MaybeUninit<T>; N],
        init: Range<usize>,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
        fn drop(&mut self) {
            for elem in &mut self.array.as_mut_slice()[self.init.clone()] {
                // SAFETY: see function docs for invariants upheld by caller;
                //   we basically guarantee that only the range defined by self.init
                //   is actually initialized
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

        // any empty range will work; it doesn't have to start at the right index
        init: 0..0,
    };

    for idx in idxs {
        match next() {
            Some(item) => {
                // SAFETY: see function docs for invariants upheld by caller;
                //   we basically guarantee that the indices used here are always
                //   valid and the range put in the guard will cover all the
                //   initialized indices
                unsafe { guard.array.get_unchecked_mut(idx).write(item) };
                guard.init = curr_idx_to_range(idx);
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

/// Consumes `N` elements from the iterator and returns them as an array. If the
/// iterator yields fewer than `N` items, `None` is returned and all already
/// yielded items are dropped.
///
/// # Panics
///
/// If the iterator panics then all already yielded elements will be dropped.
///
// Based on the array collect implementation in the Rust standard library.
// https://github.com/rust-lang/rust/blob/master/library/core/src/array/mod.rs#L476-L531
#[inline]
pub fn collect<I, T, const N: usize>(mut iter: I) -> Option<[T; N]>
where
    I: Iterator<Item = T>,
{
    unsafe { collect_impl(move || iter.next(), 0..N, |i| 0..i + 1) }
}

/// Consumes `N` elements from the iterator and returns them as an array in
/// reverse order. If the iterator yields fewer than `N` items, `None` is
/// returned and all already yielded items are dropped.
///
/// # Panics
///
/// If the iterator panics then all already yielded elements will be dropped.
///
// Based on the array collect implementation in the Rust standard library.
// https://github.com/rust-lang/rust/blob/master/library/core/src/array/mod.rs#L476-L531
#[inline]
pub fn collect_reversed<I, T, const N: usize>(mut iter: I) -> Option<[T; N]>
where
    I: Iterator<Item = T>,
{
    unsafe { collect_impl(move || iter.next(), (0..N).rev(), |i| i..N) }
}

/// Consumes `N` elements from the iterator and returns them as an array.
///
/// # Safety
///
/// This function is the same as [`collect`] but the caller must guarantee that
/// the iterator yields at least N items.
///
/// # Panics
///
/// If the iterator panics then all already yielded elements will be dropped.
#[inline]
pub unsafe fn collect_unchecked<I, T, const N: usize>(iter: I) -> [T; N]
where
    I: Iterator<Item = T>,
{
    match collect(iter) {
        Some(arr) => arr,
        None =>
        // SAFETY: Guaranteed by the caller.
        unsafe { hint::unreachable_unchecked() },
    }
}

/// Consumes `N` elements from the iterator and returns them as an array in
/// reverse order.
///
/// # Safety
///
/// This function is the same as [`collect_reversed`] but the caller must guarantee
/// that the iterator yields at least N items.
///
/// # Panics
///
/// If the iterator panics then all already yielded elements will be dropped.
#[inline]
pub unsafe fn collect_reversed_unchecked<I, T, const N: usize>(iter: I) -> [T; N]
where
    I: Iterator<Item = T>,
{
    match collect_reversed(iter) {
        Some(arr) => arr,
        None =>
        // SAFETY: Guaranteed by the caller.
        unsafe { hint::unreachable_unchecked() },
    }
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

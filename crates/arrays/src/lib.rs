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

mod into_iter;
mod transmute;

use core::hint;
use core::mem;
use core::mem::MaybeUninit;
use core::ptr;

use crate::transmute::transmute_unchecked;

pub use crate::into_iter::IntoIter;

/// Consumes `N` elements from the iterator and returns them as an array. If the
/// iterator yields fewer than `N` items, `Err` is returned containing the
/// already yielded items.
///
/// # Panics
///
/// If the iterator panics then all already yielded elements will be dropped.
#[inline]
pub fn collect<I, T, const N: usize>(mut iter: I) -> Result<[T; N], IntoIter<T, N>>
where
    I: Iterator<Item = T>,
{
    struct Guard<'a, T, const N: usize> {
        arr: &'a mut [MaybeUninit<T>; N],
        init: usize,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
        fn drop(&mut self) {
            for elem in &mut self.arr.as_mut_slice()[..self.init] {
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
    //      let mut arr = [const { MaybeUninit::<T>::uninit() }; N];
    //
    // See https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
    let mut arr: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut guard = Guard {
        arr: &mut arr,
        init: 0,
    };

    for _ in 0..N {
        match iter.next() {
            Some(item) => {
                // SAFETY: `guard.init` starts at zero, is increased by 1 each
                // iteration of the loop, and the loop is aborted once M * N
                // is reached, which is the length of the array.
                unsafe { guard.arr.get_unchecked_mut(guard.init) }.write(item);
                guard.init += 1;
            }
            None => {
                let init = guard.init;
                mem::forget(guard);
                // SAFETY: Exactly `guard.init` elements of `arr` have been
                // initialized.
                let rem = unsafe { IntoIter::new_unchecked(arr, 0..init) };
                return Err(rem);
            }
        }
    }

    mem::forget(guard);
    // SAFETY: the loop above loops exactly N times which is the size of the
    // array, so all elements in the array are initialized.
    Ok(unsafe { transmute_unchecked(arr) })
}

/// Consumes `N` elements from the iterator and returns them as an array.
///
/// # Safety
///
/// This function is the same as [`collect`] but the caller must guarantee that
/// the iterator yields at least N items or panic.
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
        Ok(arr) => arr,
        Err(_) =>
        // SAFETY: Guaranteed by the caller.
        unsafe { hint::unreachable_unchecked() },
    }
}

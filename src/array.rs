//! Some array related functions.

use core::mem;
use core::mem::MaybeUninit;
use core::ptr;

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
pub fn collect<I, T, const N: usize>(iter: &mut I) -> Option<[T; N]>
where
    I: Iterator<Item = T>,
{
    collect_fn(|| iter.next())
}

pub fn collect_fn<F, T, const N: usize>(mut f: F) -> Option<[T; N]>
where
    F: FnMut() -> Option<T>,
{
    struct Guard<T, const N: usize> {
        ptr: *mut T,
        len: usize,
    }

    let mut array: [MaybeUninit<T>; N] = {
        // SAFETY: An uninitialized `[MaybeUninit<_>; N]` is valid.
        unsafe { MaybeUninit::uninit().assume_init() }
    };
    let mut guard: Guard<_, N> = Guard {
        ptr: array.as_mut_ptr() as *mut T,
        len: 0,
    };

    impl<T, const N: usize> Drop for Guard<T, N> {
        fn drop(&mut self) {
            let partial = core::ptr::slice_from_raw_parts_mut(self.ptr, self.len);
            // SAFETY: this raw slice will contain only initialized objects.
            unsafe {
                core::ptr::drop_in_place(partial);
            }
        }
    }

    while let Some(item) = f() {
        // SAFETY: `guard.initialized` starts at 0, is increased by one in the
        // loop and the loop is aborted once it reaches N (which is
        // `array.len()`).
        unsafe {
            array.get_unchecked_mut(guard.len).write(item);
        }
        guard.len += 1;

        // Check if the whole array was initialized.
        if guard.len == N {
            mem::forget(guard);

            // SAFETY:
            // - The condition above asserts that all elements are initialized.
            // - We know that `[T; N]` is the same size as `[MaybeUninit<T>; N]`.
            return Some(unsafe { transmute_unchecked(array) });
        }
    }

    // This is only reached if the iterator is exhausted before
    // `guard.initialized` reaches `N`. Also note that `guard` is dropped here,
    // dropping all already initialized elements.
    None
}

#[inline]
unsafe fn transmute_unchecked<A, B>(a: A) -> B {
    let b = unsafe { ptr::read(&a as *const A as *const B) };
    mem::forget(a);
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::iter::repeat_with;

    use alloc::boxed::Box;

    #[test]
    #[should_panic]
    fn collect_no_leak() {
        // Test that no undefined behaviour occurs if the iterator provided to
        // `collect` panics on `next()`.
        //
        // Valgrind and Miri should be run on this test.

        let mut state = 0;
        let mut iter = repeat_with(|| {
            state += 1;
            if state > 2 {
                panic!()
            } else {
                Box::new(state)
            }
        });

        drop(collect::<_, _, 3>(&mut iter));
    }
}

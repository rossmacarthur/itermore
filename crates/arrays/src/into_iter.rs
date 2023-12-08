use core::fmt;
use core::fmt::Debug;
use core::iter;
use core::iter::FusedIterator;
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ptr;

use crate::transmute::transmute_unchecked;

/// A by-value array iterator.
///
/// This is a reimplementation of the [`IntoIter`] iterator from the Rust
/// standard library. using stable Rust.
///
/// [`IntoIter`]: core::array::IntoIter
pub struct IntoIter<T, const N: usize> {
    /// This is the array we are iterating over.
    arr: [MaybeUninit<T>; N],

    /// The elements in `arr` that have not been yielded yet.
    init: Range<usize>,
}

impl<T, const N: usize> IntoIter<T, N> {
    /// Creates a new iterator over an array.
    #[inline]
    pub fn new(arr: [T; N]) -> Self {
        // SAFETY: The transmute here is actually safe because `MaybeUninit<T>`
        // is guaranteed to have the same size and alignment as `T`.
        let arr = unsafe { transmute_unchecked(arr) };
        let init = 0..N;
        Self { arr, init }
    }

    /// Creates a new iterator over a partially initialized array.
    ///
    /// # Safety
    ///
    /// - The caller must ensure that the elements `arr[init]` are initialized.
    /// - The range must be canonical, i.e. `init.start <= init.end`.
    /// - The range must be within the bounds of the array, i.e. `init.end <= N`.
    #[inline]
    pub unsafe fn new_unchecked(arr: [MaybeUninit<T>; N], init: Range<usize>) -> Self {
        Self { arr, init }
    }

    /// Returns an immutable slice of all elements that have not been yielded
    /// yet.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: We know that the elements `init` are initialized in the array
        // and `MaybeUninit<T>` is guaranteed to have the same size and
        // alignment as `T`.
        unsafe {
            let slice = self.arr.get_unchecked(self.init.clone());
            &*(slice as *const [MaybeUninit<T>] as *const [T])
        }
    }

    /// Returns a mutable slice of all elements that have not been yielded yet.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: We know that the elements `init` are initialized in the array
        // and `MaybeUninit<T>` is guaranteed to have the same size and
        // alignment as `T`.
        unsafe {
            let slice = self.arr.get_unchecked_mut(self.init.clone());
            &mut *(slice as *mut [MaybeUninit<T>] as *mut [T])
        }
    }
}

impl<T, const N: usize> Debug for IntoIter<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only print the elements that have not been yielded yet.
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T, const N: usize> Clone for IntoIter<T, N>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        let mut new = Self {
            arr: unsafe { MaybeUninit::uninit().assume_init() },
            init: 0..0,
        };
        for (src, dst) in iter::zip(self.as_slice(), &mut new.arr) {
            // Write the clone of the element into the new array, if the clone
            // panics we will correctly drop the elements that have already been
            // cloned.
            dst.write(src.clone());
            // This addition cannot overflow because we are iterating over a slice.
            new.init.end += 1;
        }
        new
    }
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
    #[inline]
    fn drop(&mut self) {
        let slice = self.as_mut_slice();
        // SAFETY: We know that the slice is initialized and contain exactly
        // the elements that have not be yielded yet and need to be dropped.
        unsafe { ptr::drop_in_place(slice) }
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.init.next().map(|i| {
            // SAFETY: We know that the elements `init` are initialized and
            // within the bounds of the array. We can safely assume that it is
            // initialized and read it. Since we have consumed this index it
            // will now be considered uninitialized and won't be touched again.
            unsafe { self.arr.get_unchecked(i).assume_init_read() }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.init.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.init.len()
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    #[inline]
    fn len(&self) -> usize {
        self.init.len()
    }
}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> {}

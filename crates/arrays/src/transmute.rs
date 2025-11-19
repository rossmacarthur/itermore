use core::mem;
use core::ptr;

/// Size-heterogeneous transmutation.
///
/// This is required because the compiler doesn't yet know how to deal with the
/// size of const arrays. We should be able to use [`mem::transmute()`] but it
/// doesn't work yet :(.
///
/// # Safety
///
/// In addition to the usual requirements of [`mem::transmute()`], the caller
/// needs to ensure that the source and target types have the same size and
/// alignment.
#[inline]
pub unsafe fn transmute_unchecked<A, B>(a: A) -> B {
    let src = &a as *const A as *const B;
    // SAFETY: The caller ensures all the requirements are met
    let b = unsafe { ptr::read(src) };
    mem::forget(a);
    b
}

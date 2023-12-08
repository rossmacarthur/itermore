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
    let b = unsafe { ptr::read(&a as *const A as *const B) };
    mem::forget(a);
    b
}

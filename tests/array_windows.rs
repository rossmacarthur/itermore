#![cfg(feature = "array_windows")]

use core::iter;

use itermore::prelude::*;

#[test]
fn array_windows_debug() {
    let iter = (0..6).array_windows::<2>();
    let _ = format!("{:?}", iter);
}

#[test]
fn array_windows_clone() {
    let mut iter = (0..6).array_windows::<3>();
    let mut iter2 = iter.clone();
    assert_eq!(iter.next(), Some([0, 1, 2]));
    assert_eq!(iter2.next(), Some([0, 1, 2]));
}

#[test]
fn array_windows_infer() {
    let s = [0, 1, 0, 1, 0, 1];
    for [a, b] in s.iter().copied().array_windows() {
        assert_eq!(a + b, 1);
    }
    for [a, b, c, d] in s.iter().copied().array_windows() {
        assert_eq!(a + b + c + d, 2);
    }
}

#[test]
fn array_windows_size_hint() {
    let iter = (0..6).array_windows::<1>();
    assert_eq!(iter.size_hint(), (6, Some(6)));

    let iter = (0..6).array_windows::<3>();
    assert_eq!(iter.size_hint(), (4, Some(4)));

    let iter = (0..6).array_windows::<5>();
    assert_eq!(iter.size_hint(), (2, Some(2)));

    let iter = (0..6).array_windows::<7>();
    assert_eq!(iter.size_hint(), (0, Some(0)));

    let iter = (1..).array_windows::<2>();
    assert_eq!(iter.size_hint(), (usize::MAX - 1, None));

    let iter = (1..).filter(|x| x % 2 != 0).array_windows::<2>();
    assert_eq!(iter.size_hint(), (0, None));
}

#[test]
fn array_windows_count() {
    let iter = (0..6).array_windows::<1>();
    assert_eq!(iter.count(), 6);

    let iter = (0..6).array_windows::<3>();
    assert_eq!(iter.count(), 4);

    let iter = (0..6).array_windows::<5>();
    assert_eq!(iter.count(), 2);

    let iter = (0..6).array_windows::<7>();
    assert_eq!(iter.count(), 0);

    let iter = (0..6).filter(|x| x % 2 == 0).array_windows::<2>();
    assert_eq!(iter.count(), 2);

    let iter = iter::empty::<i32>().array_windows::<2>();
    assert_eq!(iter.count(), 0);

    let iter = [(); usize::MAX].iter().array_windows::<2>();
    assert_eq!(iter.count(), usize::MAX - 1);
}

#[allow(clippy::iter_nth_zero)]
#[test]
fn array_windows_nth() {
    let mut iter = (0..6).array_windows::<4>();
    assert_eq!(iter.nth(1), Some([1, 2, 3, 4]));
    assert_eq!(iter.nth(0), Some([2, 3, 4, 5]));
    assert_eq!(iter.nth(1), None);
}

#[test]
fn array_windows_len() {
    let iter = (0..6).array_windows::<1>();
    assert_eq!(iter.len(), 6);

    let iter = (0..6).array_windows::<2>();
    assert_eq!(iter.len(), 5);

    let iter = (0..6).array_windows::<3>();
    assert_eq!(iter.len(), 4);

    let iter = (0..6).array_windows::<4>();
    assert_eq!(iter.len(), 3);

    let iter = (0..6).array_windows::<5>();
    assert_eq!(iter.len(), 2);

    let iter = (0..6).array_windows::<6>();
    assert_eq!(iter.len(), 1);

    let iter = (0..6).array_windows::<7>();
    assert_eq!(iter.len(), 0);

    let iter = iter::empty::<i32>().array_windows::<2>();
    assert_eq!(iter.len(), 0);

    let iter = [(); usize::MAX].iter().array_windows::<2>();
    assert_eq!(iter.len(), usize::MAX - 1);
}

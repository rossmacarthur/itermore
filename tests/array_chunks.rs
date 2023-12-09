#![cfg(feature = "array_chunks")]
#![allow(unstable_name_collisions)]

use core::iter;

use itermore::prelude::*;

#[test]
fn array_chunks_debug() {
    let iter = (0..6).array_chunks::<2>();
    let _ = format!("{:?}", iter);
}

#[test]
fn array_chunks_clone() {
    let mut iter = (0..6).array_chunks::<2>();
    let mut iter2 = iter.clone();
    assert_eq!(iter.next(), Some([0, 1]));
    assert_eq!(iter2.next(), Some([0, 1]));
}

#[test]
fn array_chunks_infer() {
    let s = [1, 1, 2, -2, 6, 0, 3, 1];
    for [a, b, c] in s.iter().copied().array_chunks() {
        assert_eq!(a + b + c, 4);
    }
}

#[test]
fn array_chunks_size_hint() {
    let iter = (0..6).array_chunks::<1>();
    assert_eq!(iter.size_hint(), (6, Some(6)));

    let iter = (0..6).array_chunks::<3>();
    assert_eq!(iter.size_hint(), (2, Some(2)));

    let iter = (0..6).array_chunks::<5>();
    assert_eq!(iter.size_hint(), (1, Some(1)));

    let iter = (0..6).array_chunks::<7>();
    assert_eq!(iter.size_hint(), (0, Some(0)));

    let iter = (1..).array_chunks::<2>();
    assert_eq!(iter.size_hint(), (usize::MAX / 2, None));

    let iter = (1..).filter(|x| x % 2 != 0).array_chunks::<2>();
    assert_eq!(iter.size_hint(), (0, None));
}

#[test]
fn array_chunks_count() {
    let iter = (0..6).array_chunks::<1>();
    assert_eq!(iter.count(), 6);

    let iter = (0..6).array_chunks::<3>();
    assert_eq!(iter.count(), 2);

    let iter = (0..6).array_chunks::<5>();
    assert_eq!(iter.count(), 1);

    let iter = (0..6).array_chunks::<7>();
    assert_eq!(iter.count(), 0);

    let iter = (0..6).filter(|x| x % 2 == 0).array_chunks::<2>();
    assert_eq!(iter.count(), 1);

    let iter = iter::empty::<i32>().array_chunks::<2>();
    assert_eq!(iter.count(), 0);

    let iter = [(); usize::MAX].iter().array_chunks::<2>();
    assert_eq!(iter.count(), usize::MAX / 2);
}

#[test]
fn array_chunks_remainder() {
    let mut iter = (0..4).array_chunks::<2>();
    assert_eq!(iter.next(), Some([0, 1]));
    assert_eq!(iter.next(), Some([2, 3]));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.into_remainder().unwrap().collect::<Vec<_>>(), []);

    let mut iter = (0..5).array_chunks::<2>();
    assert_eq!(iter.next(), Some([0, 1]));
    assert_eq!(iter.next(), Some([2, 3]));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.into_remainder().unwrap().collect::<Vec<_>>(), [4]);

    let mut iter = (0..5).array_chunks::<2>();
    assert_eq!(iter.next(), Some([0, 1]));
    assert!(iter.into_remainder().is_none());
}

#[test]
fn array_chunks_next_back() {
    let mut iter = (0..7).array_chunks::<2>();
    assert_eq!(iter.next(), Some([0, 1]));
    assert_eq!(iter.next_back(), Some([4, 5]));
    assert_eq!(iter.next(), Some([2, 3]));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn array_chunks_next_back_remainder() {
    let mut iter = (0..7).array_chunks::<2>();
    assert_eq!(iter.next(), Some([0, 1]));
    assert_eq!(iter.next_back(), Some([4, 5]));
    assert_eq!(iter.next(), Some([2, 3]));
    assert_eq!(iter.into_remainder().unwrap().collect::<Vec<_>>(), [6]);
}

#[allow(clippy::iter_nth_zero)]
#[test]
fn array_chunks_nth() {
    let mut iter = (0..6).array_chunks::<2>();
    assert_eq!(iter.nth(1), Some([2, 3]));
    assert_eq!(iter.nth(0), Some([4, 5]));
    assert_eq!(iter.nth(1), None);
}

#[test]
fn array_chunks_len() {
    let iter = (0..6).array_chunks::<1>();
    assert_eq!(iter.len(), 6);

    let iter = (0..6).array_chunks::<2>();
    assert_eq!(iter.len(), 3);

    let iter = (0..6).array_chunks::<3>();
    assert_eq!(iter.len(), 2);

    let iter = (0..6).array_chunks::<4>();
    assert_eq!(iter.len(), 1);

    let iter = (0..6).array_chunks::<5>();
    assert_eq!(iter.len(), 1);

    let iter = (0..6).array_chunks::<6>();
    assert_eq!(iter.len(), 1);

    let iter = (0..6).array_chunks::<7>();
    assert_eq!(iter.len(), 0);

    let iter = iter::empty::<i32>().array_chunks::<2>();
    assert_eq!(iter.len(), 0);

    let iter = [(); usize::MAX].iter().array_chunks::<2>();
    assert_eq!(iter.len(), usize::MAX / 2);
}

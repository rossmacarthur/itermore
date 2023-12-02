#![cfg(feature = "min_max")]

use std::cmp::Reverse;
use std::iter;

use itermore::prelude::*;

#[test]
fn min_max() {
    assert_eq!(iter::empty::<i32>().min_max(), None);

    assert_eq!([1].into_iter().min_max(), Some((1, 1)));

    assert_eq!([1, 2].into_iter().min_max(), Some((1, 2)));
    assert_eq!([2, 1].into_iter().min_max(), Some((1, 2)));

    assert_eq!([1, 2, 3].into_iter().min_max(), Some((1, 3)));
    assert_eq!([1, 3, 2].into_iter().min_max(), Some((1, 3)));
    assert_eq!([2, 1, 3].into_iter().min_max(), Some((1, 3)));
    assert_eq!([2, 3, 1].into_iter().min_max(), Some((1, 3)));
    assert_eq!([3, 1, 2].into_iter().min_max(), Some((1, 3)));
    assert_eq!([3, 2, 1].into_iter().min_max(), Some((1, 3)));
}

#[test]
fn min_max_by() {
    let rev = |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b));
    assert_eq!(iter::empty::<i32>().min_max_by(rev), None);

    assert_eq!([1].into_iter().min_max_by(rev), Some((1, 1)));

    assert_eq!([1, 2].into_iter().min_max_by(rev), Some((2, 1)));
    assert_eq!([2, 1].into_iter().min_max_by(rev), Some((2, 1)));

    assert_eq!([1, 2, 3].into_iter().min_max_by(rev), Some((3, 1)));
    assert_eq!([1, 3, 2].into_iter().min_max_by(rev), Some((3, 1)));
    assert_eq!([2, 1, 3].into_iter().min_max_by(rev), Some((3, 1)));
    assert_eq!([2, 3, 1].into_iter().min_max_by(rev), Some((3, 1)));
    assert_eq!([3, 1, 2].into_iter().min_max_by(rev), Some((3, 1)));
    assert_eq!([3, 2, 1].into_iter().min_max_by(rev), Some((3, 1)));
}

#[test]
fn min_max_by_key() {
    let key = |item: &i32| -item;
    assert_eq!(iter::empty::<i32>().min_max_by_key(key), None);

    assert_eq!([1].into_iter().min_max_by_key(key), Some((1, 1)));

    assert_eq!([1, 2].into_iter().min_max_by_key(key), Some((2, 1)));
    assert_eq!([2, 1].into_iter().min_max_by_key(key), Some((2, 1)));

    assert_eq!([1, 2, 3].into_iter().min_max_by_key(key), Some((3, 1)));
    assert_eq!([1, 3, 2].into_iter().min_max_by_key(key), Some((3, 1)));
    assert_eq!([2, 1, 3].into_iter().min_max_by_key(key), Some((3, 1)));
    assert_eq!([2, 3, 1].into_iter().min_max_by_key(key), Some((3, 1)));
    assert_eq!([3, 1, 2].into_iter().min_max_by_key(key), Some((3, 1)));
    assert_eq!([3, 2, 1].into_iter().min_max_by_key(key), Some((3, 1)));
}

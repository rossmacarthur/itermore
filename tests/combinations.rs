#![cfg(feature = "combinations")]

use itermore::prelude::*;

#[test]
fn combinations_debug() {
    let iter = (0..6).combinations(2);
    let _ = format!("{:?}", iter);
}

#[test]
fn combinations_clone() {
    let mut iter = (0..6).combinations(2);
    let mut iter2 = iter.clone();
    assert_eq!(iter.next(), Some(vec![0, 1]));
    assert_eq!(iter2.next(), Some(vec![0, 1]));
}

#[test]
#[should_panic]
fn combinations_zero_k() {
    let _it = (1..5).combinations(0);
}

#[test]
fn combinations_smoke() {
    // N = 4, K = 1
    let v = Vec::from_iter((1..5).combinations(1));
    assert_eq!(v, [[1], [2], [3], [4]]);

    // N = 4, K = 2
    let v = Vec::from_iter((1..5).combinations(2));
    assert_eq!(v, [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]);

    // N = 4, K = 3
    let v = Vec::from_iter((1..5).combinations(3));
    assert_eq!(v, [[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]);

    // N = 4, K = 4
    let v = Vec::from_iter((1..5).combinations(4));
    assert_eq!(v, [[1, 2, 3, 4]]);

    // N = 4, K = 5
    let v = Vec::from_iter((1..5).combinations(5));
    assert!(v.is_empty());
}

#[test]
fn combinations_edge_cases() {
    // N = 1, K = 1
    let mut it = (1..2).combinations(1);
    assert_eq!(it.next(), Some(vec![1]));
    assert!(it.next().is_none());

    // N = 1, K = 2
    let mut it = (1..2).combinations(2);
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());

    // N = 2, K = 3
    let mut it = (1..3).combinations(3);
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());
}

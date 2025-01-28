#![cfg(feature = "sorted")]

use itermore::prelude::*;

#[test]
fn sorted() {
    let v: Vec<_> = [1, 3, 2].into_iter().sorted().collect();
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn sorted_by() {
    let v: Vec<_> = [1, 3, 2].into_iter().sorted_by(|a, b| b.cmp(a)).collect();
    assert_eq!(v, [3, 2, 1]);
}

#[test]
fn sorted_by_key() {
    let v: Vec<_> = [1, 3, 2].into_iter().sorted_by_key(|&x| x % 2).collect();
    assert_eq!(v, [2, 1, 3]);
}

#[test]
fn sorted_unstable() {
    let v: Vec<_> = [1, 3, 2].into_iter().sorted_unstable().collect();
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn sorted_unstable_by() {
    let v: Vec<_> = [1, 3, 2]
        .into_iter()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .collect();
    assert_eq!(v, [3, 2, 1]);
}

#[test]
fn sorted_unstable_by_key() {
    let v: Vec<_> = [1, 3, 2]
        .into_iter()
        .sorted_unstable_by_key(|&x| x % 2)
        .collect();
    assert_eq!(v, [2, 1, 3]);
}

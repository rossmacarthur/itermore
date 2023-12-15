#![cfg(feature = "collect_array")]

use itermore::prelude::*;

#[test]
fn collect_array() {
    let arr: [_; 0] = (0..0).collect_array();
    assert_eq!(arr, []);

    let arr: [_; 3] = (0..3).collect_array();
    assert_eq!(arr, [0, 1, 2]);

    let arr: [_; 3] = (0..3).rev().collect_array::<3>();
    assert_eq!(arr, [2, 1, 0]);

    let arr: [_; 3] = (0..10).take(3).collect_array();
    assert_eq!(arr, [0, 1, 2]);
}

#[test]
#[should_panic]
fn collect_array_too_few() {
    let _: [_; 3] = (0..2).collect_array();
}

#[test]
#[should_panic]
fn collect_array_too_many() {
    let _: [_; 3] = (0..4).collect_array();
}

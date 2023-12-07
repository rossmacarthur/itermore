#![cfg(feature = "array_combinations_with_reps")]

use itermore::prelude::*;

#[test]
fn array_combinations_with_reps_debug() {
    let iter = (0..6).array_combinations_with_reps::<2>();
    let _ = format!("{:?}", iter);
}

#[test]
fn array_combinations_with_reps_clone() {
    let mut iter = (0..6).array_combinations_with_reps::<2>();
    let mut iter2 = iter.clone();
    assert_eq!(iter.next(), Some([0, 0]));
    assert_eq!(iter2.next(), Some([0, 0]));
}

#[test]
#[should_panic]
fn array_combinations_with_reps_zero_k() {
    let _it = (1..5).array_combinations_with_reps::<0>();
}

#[test]
fn array_combinations_with_reps_smoke() {
    // N = 4, K = 1
    let v = Vec::from_iter((1..5).array_combinations_with_reps());
    assert_eq!(v, [[1], [2], [3], [4]]);

    // N = 2, K = 2
    let v = Vec::from_iter((1..3).array_combinations_with_reps());
    assert_eq!(v, [[1, 1], [1, 2], [2, 1], [2, 2]]);

    // N = 3, K = 2
    let v = Vec::from_iter((1..4).array_combinations_with_reps());
    assert_eq!(
        v,
        [
            [1, 1],
            [1, 2],
            [1, 3],
            [2, 1],
            [2, 2],
            [2, 3],
            [3, 1],
            [3, 2],
            [3, 3]
        ]
    );

    // N = 3, K = 3
    let v = Vec::from_iter((1..4).array_combinations_with_reps());
    assert_eq!(v.len(), 27);
    assert_eq!(
        &v[..8],
        [
            [1, 1, 1],
            [1, 1, 2],
            [1, 1, 3],
            [1, 2, 1],
            [1, 2, 2],
            [1, 2, 3],
            [1, 3, 1],
            [1, 3, 2]
        ]
    );

    // N = 4, K = 4
    let v = Vec::from_iter((1..5).array_combinations_with_reps::<4>());
    assert_eq!(v.len(), 256);
}

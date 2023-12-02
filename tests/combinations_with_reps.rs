#![cfg(feature = "combinations_with_reps")]

use itermore::prelude::*;

#[test]
fn combinations_with_reps_smoke() {
    // N = 4, K = 1
    let v = Vec::from_iter((1..5).combinations_with_reps(1));
    assert_eq!(v, [[1], [2], [3], [4]]);

    // N = 2, K = 2
    println!("here");
    let v = Vec::from_iter((1..3).combinations_with_reps(2));
    assert_eq!(v, [[1, 1], [1, 2], [2, 1], [2, 2]]);

    // N = 3, K = 2
    let v = Vec::from_iter((1..4).combinations_with_reps(2));
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
    let v = Vec::from_iter((1..4).combinations_with_reps(3));
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
    let v = Vec::from_iter((1..5).combinations_with_reps(4));
    assert_eq!(v.len(), 256);
}

#![cfg(feature = "combinations")]

use itermore::IterCombinations;

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

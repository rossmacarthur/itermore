#![cfg(feature = "array_combinations")]

use itermore::IterArrayCombinations;

#[test]
#[should_panic]
fn array_combinations_zero_k() {
    let _it = (1..5).array_combinations::<0>();
}

#[test]
fn array_combinations_smoke() {
    // N = 4, K = 1
    let v = Vec::from_iter((1..5).array_combinations());
    assert_eq!(v, [[1], [2], [3], [4]]);

    // N = 4, K = 2
    let v = Vec::from_iter((1..5).array_combinations());
    assert_eq!(v, [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]);

    // N = 4, K = 3
    let v = Vec::from_iter((1..5).array_combinations());
    assert_eq!(v, [[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]);

    // N = 4, K = 4
    let v = Vec::from_iter((1..5).array_combinations());
    assert_eq!(v, [[1, 2, 3, 4]]);

    // N = 4, K = 5
    let v = Vec::from_iter((1..5).array_combinations::<5>());
    assert!(v.is_empty());
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

#[test]
fn array_combinations_edge_cases() {
    // N = 1, K = 1
    let mut it = (1..2).array_combinations::<1>();
    assert_eq!(it.next(), Some([1]));
    assert!(it.next().is_none());

    // N = 1, K = 2
    let mut it = (1..2).array_combinations::<2>();
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());

    // N = 2, K = 3
    let mut it = (1..3).array_combinations::<3>();
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());
    assert!(it.next().is_none());
}

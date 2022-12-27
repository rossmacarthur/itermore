#![cfg(feature = "cartesian_product")]

use std::iter;

use itermore::IterCartesianProduct;

#[test]
fn cartesian_product() {
    let v: Vec<_> = [1i64]
        .into_iter()
        .cartesian_product(iter::empty::<i32>())
        .collect();
    assert_eq!(v, []);

    let v: Vec<_> = iter::empty::<i64>()
        .into_iter()
        .cartesian_product([4i32])
        .collect();
    assert_eq!(v, []);

    let v: Vec<_> = [1i64].into_iter().cartesian_product([4i32]).collect();
    assert_eq!(v, [(1, 4)]);

    let v: Vec<_> = [1i64, 2].into_iter().cartesian_product([4i32]).collect();
    assert_eq!(v, [(1, 4), (2, 4)]);

    let v: Vec<_> = [1i64, 2].into_iter().cartesian_product([4i32, 5]).collect();
    assert_eq!(v, [(1, 4), (1, 5), (2, 4), (2, 5)]);

    let v: Vec<_> = [1i64, 2]
        .into_iter()
        .cartesian_product([4i32, 5, 6])
        .collect();
    assert_eq!(v, [(1, 4), (1, 5), (1, 6), (2, 4), (2, 5), (2, 6)]);

    let v: Vec<_> = [1i64, 2, 3]
        .into_iter()
        .cartesian_product([4i32, 5, 6])
        .collect();
    assert_eq!(
        v,
        [
            (1, 4),
            (1, 5),
            (1, 6),
            (2, 4),
            (2, 5),
            (2, 6),
            (3, 4),
            (3, 5),
            (3, 6)
        ]
    );
}

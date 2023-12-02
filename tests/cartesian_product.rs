#![cfg(feature = "cartesian_product")]
#![allow(clippy::type_complexity)]

use std::iter;

use itermore::cartesian_product;
use itermore::prelude::*;

#[test]
fn cartesian_product() {
    let v: Vec<_> = [1i64]
        .into_iter()
        .cartesian_product(iter::empty::<i32>())
        .collect();
    assert_eq!(v, []);

    let v: Vec<_> = iter::empty::<i64>().cartesian_product([4i32]).collect();
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

#[test]
fn cartesian_product_macro() {
    let v: Vec<i32> = cartesian_product!(1..2).collect();
    assert_eq!(v, [1]);

    let v: Vec<(i8, i16)> = cartesian_product!(1..2, 3..4).collect();
    assert_eq!(v, [(1, 3)]);

    let v: Vec<(i8, i16, i32)> = cartesian_product!(1..2, 3..4, 5..6).collect();
    assert_eq!(v, [(1, 3, 5)]);

    let v: Vec<(i8, i16, i32, i64)> = cartesian_product!(1..2, 3..4, 5..6, 7..8).collect();
    assert_eq!(v, [(1, 3, 5, 7)]);

    let v: Vec<(i8, i16, i32, i64, u8)> =
        cartesian_product!(1..2, 3..4, 5..6, 7..8, 9..10).collect();
    assert_eq!(v, [(1, 3, 5, 7, 9)]);

    let v: Vec<(i8, i16, i32, i64, u8, u16)> =
        cartesian_product!(1..2, 3..4, 5..6, 7..8, 9..10, 11..12).collect();
    assert_eq!(v, [(1, 3, 5, 7, 9, 11)]);

    let v: Vec<(i8, i16, i32, i64, u8, u16, u32)> =
        cartesian_product!(1..2, 3..4, 5..6, 7..8, 9..10, 11..12, 13..14).collect();
    assert_eq!(v, [(1, 3, 5, 7, 9, 11, 13)]);

    let v: Vec<(i8, i16, i32, i64, u8, u16, u32, u64)> =
        cartesian_product!(1..2, 3..4, 5..6, 7..8, 9..10, 11..12, 13..14, 15..16).collect();
    assert_eq!(v, [(1, 3, 5, 7, 9, 11, 13, 15)]);

    let v: Vec<(i8, i16, i32, i64, u8, u16, u32, u64, i8)> = cartesian_product!(
        1..2,
        3..4,
        5..6,
        7..8,
        9..10,
        11..12,
        13..14,
        15..16,
        17..18,
    )
    .collect();
    assert_eq!(v, [(1, 3, 5, 7, 9, 11, 13, 15, 17)]);

    let v: Vec<(i8, i16, i32, i64, u8, u16, u32, u64, i8, i16)> = cartesian_product!(
        1..2,
        3..4,
        5..6,
        7..8,
        9..10,
        11..12,
        13..14,
        15..16,
        17..18,
        19..20,
    )
    .collect();
    assert_eq!(v, [(1, 3, 5, 7, 9, 11, 13, 15, 17, 19)]);

    let v: Vec<(i8, i16, i32, i64, u8, u16, u32, u64, i8, i16, i32)> = cartesian_product!(
        1..2,
        3..4,
        5..6,
        7..8,
        9..10,
        11..12,
        13..14,
        15..16,
        17..18,
        19..20,
        21..22,
    )
    .collect();
    assert_eq!(v, [(1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21)]);

    let v: Vec<(i8, i16, i32, i64, u8, u16, u32, u64, i8, i16, i32, i64)> = cartesian_product!(
        1..2,
        3..4,
        5..6,
        7..8,
        9..10,
        11..12,
        13..14,
        15..16,
        17..18,
        19..20,
        21..22,
        23..24,
    )
    .collect();
    assert_eq!(v, [(1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23)]);
}

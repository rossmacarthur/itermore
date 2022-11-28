use core::iter;

use iterwindows::IterWindows;

#[test]
fn windows_infer() {
    let s = [0, 1, 0, 1, 0, 1];
    for [a, b] in s.iter().copied().windows() {
        assert_eq!(a + b, 1);
    }
    for [a, b, c, d] in s.iter().copied().windows() {
        assert_eq!(a + b + c + d, 2);
    }
}

#[test]
fn windows_size_hint() {
    let iter = (0..6).windows::<1>();
    assert_eq!(iter.size_hint(), (6, Some(6)));

    let iter = (0..6).windows::<3>();
    assert_eq!(iter.size_hint(), (4, Some(4)));

    let iter = (0..6).windows::<5>();
    assert_eq!(iter.size_hint(), (2, Some(2)));

    let iter = (0..6).windows::<7>();
    assert_eq!(iter.size_hint(), (0, Some(0)));

    let iter = (1..).windows::<2>();
    assert_eq!(iter.size_hint(), (usize::MAX - 1, None));

    let iter = (1..).filter(|x| x % 2 != 0).windows::<2>();
    assert_eq!(iter.size_hint(), (0, None));
}

#[test]
fn windows_count() {
    let iter = (0..6).windows::<1>();
    assert_eq!(iter.count(), 6);

    let iter = (0..6).windows::<3>();
    assert_eq!(iter.count(), 4);

    let iter = (0..6).windows::<5>();
    assert_eq!(iter.count(), 2);

    let iter = (0..6).windows::<7>();
    assert_eq!(iter.count(), 0);

    let iter = (0..6).filter(|x| x % 2 == 0).windows::<2>();
    assert_eq!(iter.count(), 2);

    let iter = iter::empty::<i32>().windows::<2>();
    assert_eq!(iter.count(), 0);

    let iter = [(); usize::MAX].iter().windows::<2>();
    assert_eq!(iter.count(), usize::MAX - 1);
}

#[test]
fn windows_nth() {
    let mut iter = (0..6).windows::<4>();
    assert_eq!(iter.nth(1), Some([1, 2, 3, 4]));
    assert_eq!(iter.nth(0), Some([2, 3, 4, 5]));
    assert_eq!(iter.nth(1), None);
}

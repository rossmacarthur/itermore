use core::iter;

use iterchunks::IterChunks;

#[test]
fn chunks_infer() {
    let s = [1, 1, 2, -2, 6, 0, 3, 1];
    for [a, b, c] in s.iter().copied().chunks() {
        assert_eq!(a + b + c, 4);
    }
}

#[test]
fn chunks_size_hint() {
    let iter = (0..6).chunks::<1>();
    assert_eq!(iter.size_hint(), (6, Some(6)));

    let iter = (0..6).chunks::<3>();
    assert_eq!(iter.size_hint(), (2, Some(2)));

    let iter = (0..6).chunks::<5>();
    assert_eq!(iter.size_hint(), (1, Some(1)));

    let iter = (0..6).chunks::<7>();
    assert_eq!(iter.size_hint(), (0, Some(0)));

    let iter = (1..).chunks::<2>();
    assert_eq!(iter.size_hint(), (usize::MAX / 2, None));

    let iter = (1..).filter(|x| x % 2 != 0).chunks::<2>();
    assert_eq!(iter.size_hint(), (0, None));
}

#[test]
fn chunks_count() {
    let iter = (0..6).chunks::<1>();
    assert_eq!(iter.count(), 6);

    let iter = (0..6).chunks::<3>();
    assert_eq!(iter.count(), 2);

    let iter = (0..6).chunks::<5>();
    assert_eq!(iter.count(), 1);

    let iter = (0..6).chunks::<7>();
    assert_eq!(iter.count(), 0);

    let iter = (0..6).filter(|x| x % 2 == 0).chunks::<2>();
    assert_eq!(iter.count(), 1);

    let iter = iter::empty::<i32>().chunks::<2>();
    assert_eq!(iter.count(), 0);

    let iter = [(); usize::MAX].iter().chunks::<2>();
    assert_eq!(iter.count(), usize::MAX / 2);
}

#[test]
fn chunks_next_back() {
    let mut iter = (0..7).chunks::<2>();
    assert_eq!(iter.next(), Some([0, 1]));
    assert_eq!(iter.next_back(), Some([4, 5]));
    assert_eq!(iter.next(), Some([2, 3]));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}

#[allow(clippy::iter_nth_zero)]
#[test]
fn chunks_nth() {
    let mut iter = (0..6).chunks::<2>();
    assert_eq!(iter.nth(1), Some([2, 3]));
    assert_eq!(iter.nth(0), Some([4, 5]));
    assert_eq!(iter.nth(1), None);
}
